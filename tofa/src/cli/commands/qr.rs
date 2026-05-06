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
        // build_selection_uri picks the right format: a single otpauth:// when
        // there's exactly one entry, otpauth-migration:// when multiple all-30s
        // entries, and refuses with NonStandardPeriod when a multi-selection
        // includes a non-30s entry that the migration format can't carry.
        let entries: Vec<_> = vault.entries().to_vec();
        tofa_core::build_selection_uri(&entries).map_err(|e| e.to_string())?
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
