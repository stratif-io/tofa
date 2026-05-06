use crate::cli::{find_entry, open_vault, read_passphrase, CliResult};
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct QrArgs {
    /// Entry id or name (partial match)
    pub name: Option<String>,
    /// Export all accounts as a migration QR
    #[arg(long, conflicts_with = "name")]
    pub all: bool,
    /// Save QR as PNG instead of displaying in terminal
    #[arg(long, value_name = "PATH")]
    pub output: Option<PathBuf>,
}

pub fn run(args: QrArgs, vault_path: PathBuf) -> CliResult {
    let pass = read_passphrase("Passphrase: ")?;
    let vault = open_vault(&vault_path, &pass)?;

    let uri = if args.all {
        // Build a MigrationAccount per entry. VaultEntry.name may be "issuer:account",
        // but the migration encoder doesn't try to split — it stores the full string
        // as the name and leaves issuer empty.
        let entries = vault.entries();
        let accounts: Vec<tofa_core::MigrationAccount<'_>> = entries
            .iter()
            .map(|e| tofa_core::MigrationAccount {
                name: e.name.as_str(),
                issuer: "",
                secret_b32: e.secret.as_str(),
                algorithm: e.algorithm.as_str(),
                digits: e.digits,
            })
            .collect();
        tofa_core::generate_migration_uri(&accounts)
            .map_err(|e| format!("QR generation failed: {e}"))?
    } else {
        let name = args.name.as_deref().ok_or("provide a name or --all")?;
        let (_, entry) = find_entry(&vault, name)?;
        tofa_core::qr::build_otpauth_uri(entry)
    };

    if let Some(out_path) = args.output {
        tofa_core::uri_to_qr_png(&uri, &out_path)
            .map_err(|e| format!("PNG generation failed: {e}"))?;
        eprintln!("QR saved to {}", out_path.display());
    } else {
        for line in tofa_core::uri_to_qr_lines(&uri) {
            println!("{line}");
        }
    }
    Ok(())
}
