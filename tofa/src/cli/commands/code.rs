use crate::cli::{find_entry, open_vault, read_passphrase, CliResult};
use crate::theme::{ansi, voice};
use clap::Args;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use tofa_core::{
    store::VaultEntry,
    totp::{format_code, generate_code_now, seconds_remaining_now},
};

#[derive(Args)]
pub struct CodeArgs {
    /// Entry id or name (partial match)
    pub name: String,
    /// Output bare digits without space (for scripting)
    #[arg(long)]
    pub raw: bool,
    /// Copy to clipboard (the code by default; the otpauth:// URI when --uri is set)
    #[arg(long)]
    pub copy: bool,
    /// Refresh every second until Ctrl+C
    #[arg(long)]
    pub watch: bool,
    /// Print/copy the entry's `otpauth://` URI instead of the current code.
    /// Useful for moving an account to another authenticator app or
    /// piping into `tofa add --uri`.
    #[arg(long)]
    pub uri: bool,
}

pub fn run(args: CodeArgs, vault_path: PathBuf) -> CliResult {
    let pass = read_passphrase("Passphrase: ")?;
    let vault = open_vault(&vault_path, &pass)?;
    let (_, entry) = find_entry(&vault, &args.name)?;
    let entry = entry.clone();

    if args.uri {
        let uri = tofa_core::qr::build_otpauth_uri(&entry);
        println!("{uri}");
        if args.copy {
            copy_to_clipboard(&uri, &args.name);
        }
        return Ok(());
    }

    if args.watch {
        return watch_loop(&entry);
    }

    let code = generate_code_now(&entry)?;
    if args.raw {
        println!("{}{}{}", ansi::brand(), code, ansi::RESET);
    } else {
        println!("{}{}{}", ansi::brand(), format_code(&code), ansi::RESET);
    }
    if args.copy {
        copy_to_clipboard(&code, &args.name);
    }
    Ok(())
}

fn copy_to_clipboard(payload: &str, account: &str) {
    match arboard::Clipboard::new().and_then(|mut cb| cb.set_text(payload.to_string())) {
        Ok(_) => eprintln!(
            "{}{}{}",
            ansi::success(),
            voice::COPIED.replace("{account}", account),
            ansi::RESET
        ),
        Err(_) => eprintln!("{}Clipboard unavailable.{}", ansi::danger(), ansi::RESET),
    }
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
            "{}{}   {}   {bar}   {secs}s{}",
            col,
            entry.name,
            format_code(&code),
            ansi::RESET
        );
        print!("\r\x1b[K{line}");
        std::io::stdout().flush()?;
        std::thread::sleep(Duration::from_secs(1));
    }
}
