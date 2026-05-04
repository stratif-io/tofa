use crate::cli::{find_entry, open_vault, read_passphrase, CliResult};
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct RenameArgs {
    pub id: String,
    pub new_name: String,
}

pub fn run(args: RenameArgs, vault_path: PathBuf) -> CliResult {
    if args.new_name.trim().is_empty() {
        return Err("New name cannot be empty.".into());
    }
    let pass = read_passphrase("Passphrase: ")?;
    let mut vault = open_vault(&vault_path, &pass)?;

    if vault.entries().iter().any(|e| e.name == args.new_name) {
        return Err(format!("\"{}\" already exists.", args.new_name).into());
    }

    let (index, entry) = find_entry(&vault, &args.id)?;
    let old_name = entry.name.clone();
    vault.rename_entry(index, args.new_name.clone());
    vault.save(&vault_path, &pass)?;
    println!("Renamed \"{old_name}\" → \"{}\"", args.new_name);
    Ok(())
}
