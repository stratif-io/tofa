use crate::cli::{open_vault, read_passphrase, CliResult};
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct ImportArgs {
    pub file: PathBuf,
}

pub fn run(args: ImportArgs, vault_path: PathBuf) -> CliResult {
    let pass = read_passphrase("Passphrase: ")?;
    let mut vault = open_vault(&vault_path, &pass)?;

    let secrets = tofa_core::import::parse_file(&args.file)?;
    let today = tofa_core::today_iso();
    let mut imported = 0;
    let mut skipped = 0;
    for otp in secrets {
        let name = otp.meta.derive_name();
        let entry = otp.into_vault_entry(name, today.clone());
        if vault.add_entry_if_unique(entry) {
            imported += 1;
        } else {
            skipped += 1;
        }
    }

    vault.save(&vault_path, &pass)?;
    print!("Imported {imported} account(s).");
    if skipped > 0 {
        print!(" Skipped {skipped} duplicate(s).");
    }
    println!();
    Ok(())
}
