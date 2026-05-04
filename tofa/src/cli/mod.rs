pub mod commands;

use clap::{Parser, Subcommand};
use rpassword::prompt_password;
use std::path::PathBuf;
use tofa_core::Vault;

pub type CliResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Parser)]
#[command(
    name = "tofa",
    about = "Eye-candy terminal OTP manager",
    long_about = "tofa manages TOTP codes in an encrypted vault.\n\nRun without arguments to launch the interactive TUI.",
    version,
    arg_required_else_help = false
)]
pub struct Cli {
    /// Path to the vault file (overrides TOFA_VAULT env var)
    #[arg(long, global = true, env = "TOFA_VAULT", value_name = "PATH")]
    pub vault: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new encrypted vault
    Init(commands::init::InitArgs),
    /// Permanently delete the vault
    Destroy,
    /// List all accounts
    List(commands::list::ListArgs),
    /// Show the current TOTP code for an account
    Code(commands::code::CodeArgs),
    /// Add a new account
    Add(commands::add::AddArgs),
    /// Remove an account
    Remove(commands::remove::RemoveArgs),
    /// Rename an account
    Rename(commands::rename::RenameArgs),
    /// Export a QR code for one or all accounts
    Qr(commands::qr::QrArgs),
    /// Change the vault passphrase
    Rekey,
    /// Print shell completions
    Completions(commands::completions::CompletionsArgs),
    /// Export all accounts as plain-text JSON
    Export(commands::export::ExportArgs),
    /// Import accounts from JSON or a migration QR image
    Import(commands::import::ImportArgs),
    /// Scan screen for a QR code and add the account
    Scan(commands::scan::ScanArgs),
    /// Open camera and wait for a QR code to add an account
    Cam(commands::cam::CamArgs),
}

pub fn default_vault_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("tofa")
        .join("vault.enc")
}

pub fn resolve_vault_path(flag: Option<PathBuf>) -> PathBuf {
    flag.unwrap_or_else(default_vault_path)
}

pub fn read_passphrase(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(pass) = std::env::var("TOFA_PASSPHRASE") {
        eprintln!("⚠  Passphrase read from TOFA_PASSPHRASE. Avoid this in production.");
        return Ok(pass);
    }
    let pass = prompt_password(prompt)?;
    Ok(pass)
}

pub fn open_vault(
    path: &std::path::Path,
    passphrase: &str,
) -> Result<Vault, Box<dyn std::error::Error>> {
    if !path.exists() {
        return Err(format!(
            "no vault at {}. Run 'tofa init' to create one.",
            path.display()
        )
        .into());
    }
    let vault = Vault::load(path, passphrase).map_err(|_| "wrong passphrase.")?;
    Ok(vault)
}

pub fn find_entry<'a>(
    vault: &'a Vault,
    query: &str,
) -> Result<(usize, &'a tofa_core::VaultEntry), Box<dyn std::error::Error>> {
    let entries = vault.entries();

    // 1. Exact id match
    if let Some(hit) = entries.iter().enumerate().find(|(_, e)| e.id == query) {
        return Ok(hit);
    }

    // 2. Id prefix match (e.g. "GitHub:carlo@174" or just "GitHub:carlo")
    let id_prefix: Vec<_> = entries
        .iter()
        .enumerate()
        .filter(|(_, e)| !e.id.is_empty() && e.id.starts_with(query))
        .collect();
    if id_prefix.len() == 1 {
        return Ok(id_prefix.into_iter().next().unwrap());
    }
    if id_prefix.len() > 1 {
        let list = id_prefix
            .iter()
            .map(|(_, e)| format!("  {}", e.id))
            .collect::<Vec<_>>()
            .join("\n");
        return Err(format!(
            "\"{query}\" matches multiple accounts:\n{list}\nUse a more specific id."
        )
        .into());
    }

    // 3. Fuzzy name match (backward compat / entries without id)
    let lower = query.to_lowercase();
    let name_matches: Vec<_> = entries
        .iter()
        .enumerate()
        .filter(|(_, e)| e.name.to_lowercase().contains(&lower))
        .collect();
    match name_matches.len() {
        0 => Err(format!("no account matching \"{query}\".").into()),
        1 => Ok(name_matches.into_iter().next().unwrap()),
        _ => {
            let list = name_matches
                .iter()
                .map(|(_, e)| format!("  {} (id: {})", e.name, e.id))
                .collect::<Vec<_>>()
                .join("\n");
            Err(format!(
                "\"{query}\" matches multiple accounts:\n{list}\nUse the full id to disambiguate."
            )
            .into())
        }
    }
}

pub fn dispatch(cmd: Commands, vault_flag: Option<PathBuf>) -> CliResult {
    let vault_path = resolve_vault_path(vault_flag);
    match cmd {
        Commands::Init(args) => commands::init::run(args, vault_path),
        Commands::Destroy => commands::destroy::run(vault_path),
        Commands::List(args) => commands::list::run(args, vault_path),
        Commands::Code(args) => commands::code::run(args, vault_path),
        Commands::Add(args) => commands::add::run(args, vault_path),
        Commands::Remove(args) => commands::remove::run(args, vault_path),
        Commands::Rename(args) => commands::rename::run(args, vault_path),
        Commands::Qr(args) => commands::qr::run(args, vault_path),
        Commands::Rekey => commands::rekey::run(vault_path),
        Commands::Completions(args) => commands::completions::run(args),
        Commands::Export(args) => commands::export::run(args, vault_path),
        Commands::Import(args) => commands::import::run(args, vault_path),
        Commands::Scan(args) => commands::scan::run(args, vault_path),
        Commands::Cam(args) => commands::cam::run(args, vault_path),
    }
}
