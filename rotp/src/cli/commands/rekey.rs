use crate::cli::{open_vault, read_passphrase, CliResult};
use std::path::PathBuf;

pub fn run(vault_path: PathBuf) -> CliResult {
    let current = read_passphrase("Current passphrase: ")?;
    let vault = open_vault(&vault_path, &current)?;

    let new_pass = if let Ok(p) = std::env::var("ROTP_NEW_PASSPHRASE") {
        eprintln!("⚠  New passphrase read from ROTP_NEW_PASSPHRASE.");
        p
    } else {
        let p = rpassword::prompt_password("New passphrase: ")?;
        let confirm = rpassword::prompt_password("Confirm new passphrase: ")?;
        if p != confirm {
            return Err("Passphrases do not match.".into());
        }
        p
    };

    vault.save(&vault_path, &new_pass)?;
    eprintln!("Passphrase updated.");
    Ok(())
}
