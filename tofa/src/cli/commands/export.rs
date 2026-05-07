use crate::cli::{open_vault, read_passphrase, CliResult};
use clap::{Args, ValueEnum};
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum ExportFormat {
    /// Native tofa JSON (re-importable, preserves all metadata).
    Json,
    /// Plain-text list of `otpauth://` URIs, one per line. Round-trips
    /// through `tofa import <file>.txt` and is what other apps tend to
    /// accept (Ente Auth, Aegis "scan from text", etc.).
    Uris,
}

#[derive(Args)]
pub struct ExportArgs {
    /// Write to a file instead of stdout
    #[arg(long, value_name = "PATH")]
    pub output: Option<PathBuf>,
    /// Output format. Defaults to `json` for backwards compatibility.
    #[arg(long, value_enum, default_value_t = ExportFormat::Json)]
    pub format: ExportFormat,
}

pub fn run(args: ExportArgs, vault_path: PathBuf) -> CliResult {
    let pass = read_passphrase("Passphrase: ")?;
    let vault = open_vault(&vault_path, &pass)?;

    let default_ext = match args.format {
        ExportFormat::Json => "json",
        ExportFormat::Uris => "txt",
    };
    let out_path = args.output.unwrap_or_else(|| {
        let date = tofa_core::today_iso();
        PathBuf::from(format!("tofa-export-{date}.{default_ext}"))
    });

    eprintln!("⚠  This file contains your secrets in plain text.");
    eprintln!("   Store it securely and delete it after use.");

    let body = match args.format {
        ExportFormat::Json => serde_json::to_string_pretty(vault.entries())?,
        ExportFormat::Uris => tofa_core::qr::entries_to_uri_list(vault.entries()),
    };
    std::fs::write(&out_path, body)?;

    let count = vault.entries().len();
    println!("{count} account(s) exported to {}", out_path.display());
    Ok(())
}
