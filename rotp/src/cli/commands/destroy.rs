use crate::cli::{open_vault, read_passphrase, CliResult};
use std::io::{self, Write};
use std::path::PathBuf;

pub fn run(vault_path: PathBuf) -> CliResult {
    let pass = read_passphrase("Passphrase: ")?;
    open_vault(&vault_path, &pass)?; // auth check only

    eprint!(
        "Destroy vault at {}? [y/N] ",
        vault_path.display()
    );
    io::stderr().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.trim().to_lowercase() != "y" {
        eprintln!("Aborted.");
        return Ok(());
    }
    std::fs::remove_file(&vault_path)?;
    eprintln!("Vault destroyed.");
    Ok(())
}
