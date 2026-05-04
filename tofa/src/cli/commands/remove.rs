use crate::cli::{find_entry, open_vault, read_passphrase, CliResult};
use clap::Args;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Args)]
pub struct RemoveArgs {
    pub id: String,
}

pub fn run(args: RemoveArgs, vault_path: PathBuf) -> CliResult {
    let pass = read_passphrase("Passphrase: ")?;
    let mut vault = open_vault(&vault_path, &pass)?;
    let (index, entry) = find_entry(&vault, &args.id)?;
    let full_name = entry.name.clone();

    eprint!("Remove \"{full_name}\"? [y/N] ");
    io::stderr().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.trim().to_lowercase() != "y" {
        eprintln!("Aborted.");
        return Ok(());
    }

    vault.remove_entry(index);
    vault.save(&vault_path, &pass)?;
    eprintln!("Removed \"{full_name}\".");
    Ok(())
}
