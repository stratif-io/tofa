use crate::cli::{find_entry, open_vault, read_passphrase, CliResult};
use clap::Args;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use tofa_core::{
    store::VaultEntry,
    totp::{generate_code_now, seconds_remaining_now},
};
use tofa_theme::{ansi, voice};

#[derive(Args)]
pub struct CodeArgs {
    /// Entry id or name (partial match)
    pub name: String,
    /// Output bare digits without space (for scripting)
    #[arg(long)]
    pub raw: bool,
    /// Copy code to clipboard
    #[arg(long)]
    pub copy: bool,
    /// Refresh every second until Ctrl+C
    #[arg(long)]
    pub watch: bool,
}

pub fn run(args: CodeArgs, vault_path: PathBuf) -> CliResult {
    let pass = read_passphrase("Passphrase: ")?;
    let vault = open_vault(&vault_path, &pass)?;
    let (_, entry) = find_entry(&vault, &args.name)?;
    let entry = entry.clone();

    if args.watch {
        return watch_loop(&entry);
    }

    let code = generate_code_now(&entry)?;
    if args.raw {
        println!("{}{}{}", ansi::brand(), code, ansi::RESET);
    } else {
        println!(
            "{}{} {}{}",
            ansi::brand(),
            &code[..3],
            &code[3..],
            ansi::RESET
        );
    }
    if args.copy {
        match arboard::Clipboard::new().and_then(|mut cb| cb.set_text(code)) {
            Ok(_) => eprintln!(
                "{}{}{}",
                ansi::success(),
                voice::COPIED.replace("{account}", &args.name),
                ansi::RESET
            ),
            Err(_) => eprintln!("{}Clipboard unavailable.{}", ansi::danger(), ansi::RESET),
        }
    }
    Ok(())
}

fn watch_loop(entry: &VaultEntry) -> CliResult {
    let bar_w = 20usize;
    loop {
        let code = generate_code_now(entry)?;
        let secs = seconds_remaining_now(entry);
        let filled = ((secs as usize * bar_w) / entry.period as usize).min(bar_w);
        let bar = format!("{}{}", "█".repeat(filled), "░".repeat(bar_w - filled));
        let col = ansi::timer(secs);
        let line = format!(
            "{}{}   {} {}   {bar}   {secs}s{}",
            col,
            entry.name,
            &code[..3],
            &code[3..],
            ansi::RESET
        );
        print!("\r\x1b[K{line}");
        std::io::stdout().flush()?;
        std::thread::sleep(Duration::from_secs(1));
    }
}
