use crate::cli::{open_vault, read_passphrase, CliResult};
use crate::theme::ansi;
use clap::Args;
use std::path::PathBuf;
use tofa_core::totp::{format_code, generate_code_now, seconds_remaining_now};

#[derive(Args)]
pub struct ListArgs {
    /// Also display current codes and time remaining
    #[arg(long)]
    pub codes: bool,
}

pub fn run(args: ListArgs, vault_path: PathBuf) -> CliResult {
    let pass = read_passphrase("Passphrase: ")?;
    let vault = open_vault(&vault_path, &pass)?;

    let bc = ansi::box_color();
    let rs = ansi::RESET;
    let m = ansi::muted();

    if args.codes {
        println!("{bc}┌{:─<38}┬{:─<36}┬{:─<10}┬{:─<9}┐{rs}", "", "", "", "");
        println!(
            "{bc}│ {m}{:<37}{bc}│ {m}{:<35}{bc}│ {m}{:<9}{bc}│ {m}{:<8}{bc}│{rs}",
            "id", "name", "code", "expires"
        );
        println!("{bc}├{:─<38}┼{:─<36}┼{:─<10}┼{:─<9}┤{rs}", "", "", "", "");

        for entry in vault.entries() {
            let secs = seconds_remaining_now(entry);
            let code = generate_code_now(entry).unwrap_or_else(|_| "------".into());
            let formatted = format_code(&code);
            println!(
                "{bc}│ {m}{:<37}{bc}│ {rs}{:<35}{bc}│ {}{formatted:<9}{bc}│ {}{:<8}{bc}│{rs}",
                entry.id,
                entry.name,
                ansi::brand(),
                ansi::timer(secs),
                format!("{secs}s")
            );
        }

        println!("{bc}└{:─<38}┴{:─<36}┴{:─<10}┴{:─<9}┘{rs}", "", "", "", "");
    } else {
        for entry in vault.entries() {
            println!("{m}{}{rs}  {}", entry.id, entry.name);
        }
    }
    Ok(())
}
