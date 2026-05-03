use crate::cli::{open_vault, read_passphrase, CliResult};
use clap::Args;
use std::path::PathBuf;
use tofa_core::totp::{generate_code_now, seconds_remaining_now};
use tofa_theme::ansi;

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
        // Header
        let bc = ansi::box_color();
        let rs = ansi::RESET;
        println!("{bc}┌{:─<36}┬{:─<10}┬{:─<9}┐{rs}", "", "", "");
        println!(
            "{bc}│ {}{:<35}{bc}│ {}{:<9}{bc}│ {}{:<8}{bc}│{rs}",
            ansi::muted(),
            "id",
            ansi::muted(),
            "code",
            ansi::muted(),
            "expires"
        );
        println!("{bc}├{:─<36}┼{:─<10}┼{:─<9}┤{rs}", "", "", "");

        for entry in vault.entries() {
            let secs = seconds_remaining_now(entry);
            let code = generate_code_now(entry).unwrap_or_else(|_| "------".into());
            let formatted = format!("{} {}", &code[..3], &code[3..]);
            let id = if entry.id.is_empty() {
                entry.name.as_str()
            } else {
                entry.id.as_str()
            };
            println!(
                "{bc}│ {rs}{id:<35}{bc}│ {}{formatted:<9}{bc}│ {}{:<8}{bc}│{rs}",
                ansi::brand(),
                ansi::timer(secs),
                format!("{secs}s")
            );
        }

        println!("{bc}└{:─<36}┴{:─<10}┴{:─<9}┘{rs}", "", "", "");
    } else {
        for entry in vault.entries() {
            let id = if entry.id.is_empty() {
                entry.name.as_str()
            } else {
                entry.id.as_str()
            };
            println!("{}", id);
        }
    }
    Ok(())
}
