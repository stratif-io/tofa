use crate::cli::{open_vault, read_passphrase, CliResult};
use crate::theme::{ansi, voice};
use clap::Args;
use std::path::PathBuf;
use tofa_core::{
    qr::OtpSecret,
    totp::{format_code, generate_code_now, seconds_remaining_now},
    Vault,
};

#[derive(Args)]
pub struct AddArgs {
    /// Account name (required when using --secret)
    #[arg(long, value_name = "NAME")]
    pub name: Option<String>,
    /// Base32-encoded TOTP secret
    #[arg(long, value_name = "BASE32")]
    pub secret: Option<String>,
    /// otpauth:// URI
    #[arg(long, value_name = "URI")]
    pub uri: Option<String>,
    /// Path to a QR code image
    #[arg(long, value_name = "PATH")]
    pub qr: Option<PathBuf>,
}

pub fn run(args: AddArgs, vault_path: PathBuf) -> CliResult {
    let pass = read_passphrase("Passphrase: ")?;
    let mut vault = open_vault(&vault_path, &pass)?;

    if let Some(qr_path) = &args.qr {
        let uri = tofa_core::qr::scan_qr_uri(qr_path)?;
        if uri.starts_with("otpauth-migration://") {
            return import_migration(&uri, &mut vault, &vault_path, &pass, &args.name);
        }
        let otp = tofa_core::qr::parse_input(&uri)?;
        let name = args.name.unwrap_or_else(|| otp.meta.derive_name());
        return add_single(&name, otp, &mut vault, &vault_path, &pass);
    }

    if let Some(uri) = &args.uri {
        let otp = tofa_core::qr::parse_input(uri)?;
        let name = args.name.unwrap_or_else(|| otp.meta.derive_name());
        return add_single(&name, otp, &mut vault, &vault_path, &pass);
    }

    if let Some(secret) = &args.secret {
        let name = args.name.ok_or("--name is required when using --secret")?;
        let otp = OtpSecret {
            secret: secret.clone(),
            meta: Default::default(),
        };
        return add_single(&name, otp, &mut vault, &vault_path, &pass);
    }

    Err("Provide --secret, --uri, or --qr.".into())
}

pub fn add_single(
    name: &str,
    otp: OtpSecret,
    vault: &mut Vault,
    path: &std::path::Path,
    pass: &str,
) -> CliResult {
    let today = tofa_core::today_iso();
    let entry = otp.into_vault_entry(name.to_string(), today);
    let code = generate_code_now(&entry).unwrap_or_else(|_| "------".into());
    let secs = seconds_remaining_now(&entry);
    vault.add_entry(entry);
    vault.save(path, pass)?;
    println!("{}{}{}", ansi::success(), voice::ADDED_OK, ansi::RESET);
    println!(
        "{}{}{}  {}({}s){}",
        ansi::brand(),
        format_code(&code),
        ansi::RESET,
        ansi::muted(),
        secs,
        ansi::RESET
    );
    Ok(())
}

fn import_migration(
    uri: &str,
    vault: &mut Vault,
    path: &std::path::Path,
    pass: &str,
    name_override: &Option<String>,
) -> CliResult {
    let accounts = tofa_core::qr::parse_migration(uri)?;
    let count = accounts.len();
    let today = tofa_core::today_iso();
    for otp in accounts {
        let name = name_override
            .clone()
            .unwrap_or_else(|| otp.meta.derive_name());
        vault.add_entry(otp.into_vault_entry(name, today.clone()));
    }
    vault.save(path, pass)?;
    println!("Imported {count} account(s).");
    Ok(())
}
