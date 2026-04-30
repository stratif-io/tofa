use crate::cli::{find_entry, open_vault, read_passphrase, CliResult};
use clap::Args;
use rotp_core::totp::{generate_code_now, seconds_remaining_now};
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Args)]
pub struct CodeArgs {
    /// Account name (partial, case-insensitive)
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
    let secret = entry.secret.clone();
    let name = entry.name.clone();

    if args.watch {
        return watch_loop(&name, &secret);
    }

    let code = generate_code_now(&secret)?;
    if args.raw {
        println!("{code}");
    } else {
        println!("{} {}", &code[..3], &code[3..]);
    }
    if args.copy {
        match arboard::Clipboard::new().and_then(|mut cb| cb.set_text(code)) {
            Ok(_) => eprintln!("Copied to clipboard."),
            Err(_) => eprintln!("Clipboard unavailable."),
        }
    }
    Ok(())
}

fn watch_loop(name: &str, secret: &str) -> CliResult {
    loop {
        let code = generate_code_now(secret)?;
        let secs = seconds_remaining_now();
        let bar_w = 20usize;
        let filled = ((secs as usize * bar_w) / 30).min(bar_w);
        let bar = format!("{}{}", "█".repeat(filled), "░".repeat(bar_w - filled));
        let line = format!("{name}   {} {}   {bar}   {secs}s", &code[..3], &code[3..]);
        print!("\r\x1b[K{line}");
        std::io::stdout().flush()?;
        std::thread::sleep(Duration::from_secs(1));
    }
}
