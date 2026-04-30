use crate::cli::{open_vault, read_passphrase, CliResult};
use clap::Args;
use rotp_core::totp::{generate_code_now, seconds_remaining_now};
use std::path::PathBuf;

#[derive(Args)]
pub struct ListArgs {
    /// Also display current codes and time remaining
    #[arg(long)]
    pub codes: bool,
}

pub fn run(args: ListArgs, vault_path: PathBuf) -> CliResult {
    let pass = read_passphrase("Passphrase: ")?;
    let vault = open_vault(&vault_path, &pass)?;

    if args.codes {
        let secs = seconds_remaining_now();
        for entry in vault.entries() {
            let code = generate_code_now(&entry.secret).unwrap_or_else(|_| "------".into());
            let formatted = format!("{} {}", &code[..3], &code[3..]);
            println!("{:<30} {}   {}s", entry.name, formatted, secs);
        }
    } else {
        for entry in vault.entries() {
            println!("{}", entry.name);
        }
    }
    Ok(())
}
