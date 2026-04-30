use crate::cli::{open_vault, read_passphrase, CliResult};
use clap::Args;
use rotp_core::VaultEntry;
use std::path::PathBuf;

#[derive(Args)]
pub struct ImportArgs {
    pub file: PathBuf,
}

pub fn run(args: ImportArgs, vault_path: PathBuf) -> CliResult {
    let pass = read_passphrase("Passphrase: ")?;
    let mut vault = open_vault(&vault_path, &pass)?;

    let ext = args.file.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    let (imported, skipped) = if ext == "json" {
        import_json(&args.file, &mut vault)?
    } else {
        import_qr(&args.file, &mut vault)?
    };

    vault.save(&vault_path, &pass)?;
    print!("Imported {imported} account(s).");
    if skipped > 0 {
        print!(" Skipped {skipped} duplicate(s).");
    }
    println!();
    Ok(())
}

fn import_json(
    path: &PathBuf,
    vault: &mut rotp_core::Vault,
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let entries: Vec<VaultEntry> = serde_json::from_str(&content)?;
    let mut imported = 0;
    let mut skipped = 0;
    for entry in entries {
        let dup = vault
            .entries()
            .iter()
            .any(|e| e.name == entry.name && e.secret == entry.secret);
        if dup {
            skipped += 1;
        } else {
            vault.add_entry(entry);
            imported += 1;
        }
    }
    Ok((imported, skipped))
}

fn import_qr(
    path: &PathBuf,
    vault: &mut rotp_core::Vault,
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let uri = rotp_core::qr::scan_qr_uri(path.as_path())?;
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    if uri.starts_with("otpauth-migration://") {
        let accounts = rotp_core::qr::parse_migration(&uri)?;
        let mut imported = 0;
        let mut skipped = 0;
        for otp in accounts {
            let name = match (&otp.meta.issuer, &otp.meta.account) {
                (Some(i), Some(a)) => format!("{i}:{a}"),
                (Some(i), None) => i.clone(),
                (None, Some(a)) => a.clone(),
                (None, None) => format!("imported-{}", vault.entries().len() + 1),
            };
            let dup = vault
                .entries()
                .iter()
                .any(|e| e.name == name && e.secret == otp.secret);
            if dup {
                skipped += 1;
            } else {
                vault.add_entry(VaultEntry {
                    name,
                    secret: otp.secret,
                    created_at: today.clone(),
                });
                imported += 1;
            }
        }
        Ok((imported, skipped))
    } else {
        Err("QR image is not a migration QR. Use 'rotp add --qr' for single-account QRs.".into())
    }
}
