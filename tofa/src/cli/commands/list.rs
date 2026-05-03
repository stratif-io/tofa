use crate::cli::{open_vault, read_passphrase, CliResult};
use clap::Args;
use tofa_core::totp::{generate_code_now, seconds_remaining_now};
use tofa_theme::ansi;
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
        // Header
        println!("{}┌{:─<14}┬{:─<17}┬{:─<10}┬{:─<9}┐{}",
            ansi::box_color(), "", "", "", "", ansi::RESET);
        println!("{}│ {}{:<13}{}│ {}{:<16}{}│ {}{:<9}{}│ {}{:<8}{}│{}",
            ansi::box_color(),
            ansi::muted(), "issuer", ansi::box_color(),
            ansi::muted(), "account", ansi::box_color(),
            ansi::muted(), "code", ansi::box_color(),
            ansi::muted(), "expires", ansi::box_color(),
            ansi::RESET);
        println!("{}├{:─<14}┼{:─<17}┼{:─<10}┼{:─<9}┤{}",
            ansi::box_color(), "", "", "", "", ansi::RESET);

        for entry in vault.entries() {
            let secs = seconds_remaining_now(entry);
            let code = generate_code_now(entry).unwrap_or_else(|_| "------".into());
            let formatted = format!("{} {}", &code[..3], &code[3..]);
            let (issuer, account) = if let Some(pos) = entry.name.find(':') {
                (&entry.name[..pos], &entry.name[pos + 1..])
            } else {
                (entry.name.as_str(), "")
            };
            println!("{}│ {}{:<13}{}│ {}{:<16}{}│ {}{}{:<9}{}{}│ {}{}{:<8}{}{}│{}",
                ansi::box_color(),
                ansi::RESET, issuer, ansi::box_color(),
                ansi::RESET, account, ansi::box_color(),
                ansi::RESET, ansi::brand(), formatted, ansi::RESET, ansi::box_color(),
                ansi::RESET, ansi::timer(secs), format!("{}s", secs), ansi::RESET, ansi::box_color(),
                ansi::RESET);
        }

        println!("{}└{:─<14}┴{:─<17}┴{:─<10}┴{:─<9}┘{}",
            ansi::box_color(), "", "", "", "", ansi::RESET);
    } else {
        for entry in vault.entries() {
            println!("{}", entry.name);
        }
    }
    Ok(())
}
