use crate::cli::{open_vault, read_passphrase, CliResult};
use clap::Args;
use std::path::PathBuf;
use tofa_core::{
    totp::{generate_code_now, seconds_remaining_now},
    VaultEntry,
};

#[derive(Args)]
pub struct ScanArgs {
    /// Override the account name (default: derived from QR metadata)
    #[arg(long, value_name = "NAME")]
    pub name: Option<String>,
}

pub fn run(args: ScanArgs, vault_path: PathBuf) -> CliResult {
    let tmp = std::env::temp_dir().join("tofa-scan.png");
    capture_screen(&tmp)?;

    let uri = tofa_core::qr::scan_qr_uri(&tmp).map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        format!("No QR code found on screen: {e}")
    })?;
    let _ = std::fs::remove_file(&tmp);

    let pass = read_passphrase("Passphrase: ")?;
    let mut vault = open_vault(&vault_path, &pass)?;

    if uri.starts_with("otpauth-migration://") {
        let accounts = tofa_core::qr::parse_migration(&uri)?;
        let count = accounts.len();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        for otp in accounts {
            let name = args.name.clone().unwrap_or_else(|| make_name(&otp));
            vault.add_entry(VaultEntry {
                id: String::new(),
                name,
                secret: otp.secret,
                created_at: today.clone(),
                period: otp.meta.period.unwrap_or(30),
                digits: otp.meta.digits.unwrap_or(6),
                algorithm: otp.meta.algorithm.unwrap_or_else(|| "SHA1".to_string()),
            });
        }
        vault.save(&vault_path, &pass)?;
        println!("Imported {count} account(s) from migration QR.");
        return Ok(());
    }

    let otp = tofa_core::qr::parse_input(&uri)?;
    let name = args.name.unwrap_or_else(|| make_name(&otp));
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let entry = VaultEntry {
        id: String::new(),
        name: name.clone(),
        secret: otp.secret,
        created_at: today,
        period: otp.meta.period.unwrap_or(30),
        digits: otp.meta.digits.unwrap_or(6),
        algorithm: otp.meta.algorithm.unwrap_or_else(|| "SHA1".to_string()),
    };
    let code = generate_code_now(&entry).unwrap_or_else(|_| "------".into());
    let secs = seconds_remaining_now(&entry);
    vault.add_entry(entry);
    vault.save(&vault_path, &pass)?;
    println!("Added {name}");
    println!("Current code: {} {}  ({}s)", &code[..3], &code[3..], secs);
    Ok(())
}

fn make_name(otp: &tofa_core::OtpSecret) -> String {
    match (&otp.meta.issuer, &otp.meta.account) {
        (Some(i), Some(a)) => format!("{i}:{a}"),
        (Some(i), None) => i.clone(),
        (None, Some(a)) => a.clone(),
        (None, None) => "unknown".to_string(),
    }
}

#[cfg(target_os = "macos")]
fn capture_screen(path: &PathBuf) -> CliResult {
    let status = std::process::Command::new("screencapture")
        .args(["-x", "-t", "png"])
        .arg(path)
        .status()?;
    if !status.success() {
        return Err("screencapture failed".into());
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn capture_screen(path: &PathBuf) -> CliResult {
    let ok = std::process::Command::new("scrot")
        .arg(path)
        .status()
        .ok()
        .is_some_and(|s: std::process::ExitStatus| s.success());
    if ok {
        return Ok(());
    }
    let status = std::process::Command::new("gnome-screenshot")
        .args(["-f"])
        .arg(path)
        .status()?;
    if !status.success() {
        return Err("screenshot capture failed (install scrot or gnome-screenshot)".into());
    }
    Ok(())
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn capture_screen(_path: &PathBuf) -> CliResult {
    Err("Screen capture is not supported on this platform.".into())
}
