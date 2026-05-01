use crate::cli::{open_vault, read_passphrase, CliResult};
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct ExportArgs {
    #[arg(long, value_name = "PATH")]
    pub output: Option<PathBuf>,
}

pub fn run(args: ExportArgs, vault_path: PathBuf) -> CliResult {
    let pass = read_passphrase("Passphrase: ")?;
    let vault = open_vault(&vault_path, &pass)?;

    let out_path = args.output.unwrap_or_else(|| {
        let date = chrono::Local::now().format("%Y-%m-%d");
        PathBuf::from(format!("tofa-export-{date}.json"))
    });

    eprintln!("⚠  This file contains your secrets in plain text.");
    eprintln!("   Store it securely and delete it after use.");

    let json = serde_json::to_string_pretty(vault.entries())?;
    std::fs::write(&out_path, json)?;

    let count = vault.entries().len();
    println!("{count} account(s) exported to {}", out_path.display());
    Ok(())
}
