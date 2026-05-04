use crate::cli::{read_passphrase, CliResult};
use clap::Args;
use std::path::PathBuf;
use tofa_core::Vault;

#[derive(Args)]
pub struct InitArgs {}

pub fn run(_args: InitArgs, vault_path: PathBuf) -> CliResult {
    if vault_path.exists() {
        return Err(format!(
            "Vault already exists at {}. Use 'tofa rekey' to change the passphrase.",
            vault_path.display()
        )
        .into());
    }

    let pass = if std::env::var("TOFA_PASSPHRASE").is_ok() {
        read_passphrase("")?
    } else {
        let p = read_passphrase("Passphrase: ")?;
        let confirm = rpassword::prompt_password("Confirm passphrase: ")?;
        if p != confirm {
            return Err("Passphrases do not match.".into());
        }
        p
    };

    let vault = Vault::new();
    vault.save(&vault_path, &pass)?;

    eprintln!("Vault created at {}", vault_path.display());
    Ok(())
}
