use crate::cli::{find_entry, open_vault, read_passphrase, CliResult};
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct QrArgs {
    /// Account name (partial, case-insensitive)
    pub name: Option<String>,
    /// Export all accounts as a migration QR
    #[arg(long, conflicts_with = "name")]
    pub all: bool,
    /// Save QR as PNG instead of displaying in terminal
    #[arg(long, value_name = "PATH")]
    pub output: Option<PathBuf>,
}

fn percent_encode(s: &str) -> String {
    let mut out = String::new();
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9'
            | b'-' | b'_' | b'.' | b'~' => out.push(byte as char),
            b => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

pub fn run(args: QrArgs, vault_path: PathBuf) -> CliResult {
    let pass = read_passphrase("Passphrase: ")?;
    let vault = open_vault(&vault_path, &pass)?;

    let uri = if args.all {
        // Build tuples of (name, issuer, secret) for all entries.
        // VaultEntry.name may be "issuer:account" — use it as both name and issuer="" for simplicity.
        let entries = vault.entries();
        let tuples: Vec<(&str, &str, &str)> = entries
            .iter()
            .map(|e| (e.name.as_str(), "", e.secret.as_str()))
            .collect();
        tofa_core::generate_migration_uri(&tuples)
            .map_err(|e| format!("QR generation failed: {e}"))?
    } else {
        let name = args.name.as_deref().ok_or("provide a name or --all")?;
        let (_, entry) = find_entry(&vault, name)?;
        format!(
            "otpauth://totp/{}?secret={}",
            percent_encode(&entry.name),
            entry.secret
        )
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
