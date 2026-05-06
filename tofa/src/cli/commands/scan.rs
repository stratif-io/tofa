use crate::cli::{open_vault, read_passphrase, CliResult};
use clap::Args;
use std::collections::HashSet;
use std::path::PathBuf;
use tofa_core::store::Vault;
use tofa_core::{
    totp::{generate_code_now, seconds_remaining_now},
    VaultEntry,
};

#[derive(Args)]
pub struct ScanArgs {
    /// Override the account name (only applied when exactly one entry is found)
    #[arg(long, value_name = "NAME")]
    pub name: Option<String>,
}

pub fn run(args: ScanArgs, vault_path: PathBuf) -> CliResult {
    let captures = capture_screens()?;
    if captures.is_empty() {
        return Err("No displays available to capture.".into());
    }

    let uris = scan_paths_for_qrs(&captures);
    for path in &captures {
        let _ = std::fs::remove_file(path);
    }

    if uris.is_empty() {
        return Err(format!("No QR codes found across {} screen(s).", captures.len()).into());
    }

    let pass = read_passphrase("Passphrase: ")?;
    let mut vault = open_vault(&vault_path, &pass)?;

    let count = import_uris_into_vault(&uris, &mut vault, args.name.as_deref())?;
    vault.save(&vault_path, &pass)?;

    println!(
        "Imported {count} account(s) from {} screen(s).",
        captures.len()
    );
    if count == 1 {
        let entry = vault.entries().last().expect("just added");
        let code = generate_code_now(entry).unwrap_or_else(|_| "------".into());
        let secs = seconds_remaining_now(entry);
        let formatted = if code.len() >= 6 {
            format!("{} {}", &code[..3], &code[3..])
        } else {
            code.clone()
        };
        println!("Current code: {formatted}  ({secs}s)");
    }
    Ok(())
}

fn scan_paths_for_qrs(paths: &[PathBuf]) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut uris: Vec<String> = Vec::new();
    for path in paths {
        if let Ok(found) = tofa_core::qr::scan_all_qr_uris(path) {
            for uri in found {
                if seen.insert(uri.clone()) {
                    uris.push(uri);
                }
            }
        }
    }
    uris
}

/// Parse each URI (otpauth:// or otpauth-migration://) and add the resulting
/// account(s) to the vault. Returns the total number of entries added.
///
/// `name_override` is applied only when the scan results in exactly one new
/// entry — `--name` doesn't make sense when one capture yields several
/// accounts, so we fall back to per-entry derived names in that case.
pub fn import_uris_into_vault(
    uris: &[String],
    vault: &mut Vault,
    name_override: Option<&str>,
) -> Result<usize, Box<dyn std::error::Error>> {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    // Pre-parse so we know the total count before deciding whether to apply
    // the name override.
    let mut parsed: Vec<tofa_core::OtpSecret> = Vec::new();
    for uri in uris {
        if uri.starts_with("otpauth-migration://") {
            for otp in tofa_core::qr::parse_migration(uri)? {
                parsed.push(otp);
            }
        } else {
            parsed.push(tofa_core::qr::parse_input(uri)?);
        }
    }

    let total = parsed.len();
    let apply_override = total == 1 && name_override.is_some();

    for otp in parsed {
        let name = if apply_override {
            name_override.unwrap().to_string()
        } else {
            make_name(&otp)
        };
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

    Ok(total)
}

fn make_name(otp: &tofa_core::OtpSecret) -> String {
    match (&otp.meta.issuer, &otp.meta.account) {
        (Some(i), Some(a)) => format!("{i}:{a}"),
        (Some(i), None) => i.clone(),
        (None, Some(a)) => a.clone(),
        (None, None) => "unknown".to_string(),
    }
}

/// Capture every connected display to a PNG (or several PNGs) in the system
/// temp directory. Returns the file paths in capture order. Callers are
/// responsible for removing the files after use.
///
/// Implementation is platform-specific because the cross-platform crates
/// (xcap, screenshots) drag in the full desktop graphics stack — EGL,
/// PipeWire, Wayland, XCB — none of which we need just to scan a static
/// pixel buffer for QR codes:
/// - **macOS**: `screencapture -D N` per display, stopping when N becomes
///   invalid. Yields one PNG per display.
/// - **Linux Wayland**: `grim` (no args) writes all outputs merged into one PNG.
/// - **Linux X11**: `scrot -m` writes all monitors merged into one PNG.
/// - **Other**: unsupported.
///
/// `scan_all_qr_uris` handles both cases (per-display PNGs and one merged PNG)
/// because it finds every QR code in the image regardless of layout.
#[cfg(target_os = "macos")]
fn capture_screens() -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut paths = Vec::new();
    for n in 1.. {
        let path = std::env::temp_dir().join(format!("tofa-scan-{n}.png"));
        let status = std::process::Command::new("screencapture")
            .args(["-x", "-t", "png", "-D", &n.to_string()])
            .arg(&path)
            .status()?;
        if !status.success() {
            // No more displays. screencapture exits non-zero when -D is out
            // of range, having written nothing.
            let _ = std::fs::remove_file(&path);
            break;
        }
        // Defensive: if the file wasn't written for some reason, stop.
        if !path.exists() {
            break;
        }
        paths.push(path);
    }
    if paths.is_empty() {
        return Err("screencapture failed for display 1".into());
    }
    Ok(paths)
}

#[cfg(target_os = "linux")]
fn capture_screens() -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let path = std::env::temp_dir().join("tofa-scan.png");
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    let is_wayland = session_type == "wayland" || std::env::var("WAYLAND_DISPLAY").is_ok();

    if is_wayland {
        let status = std::process::Command::new("grim").arg(&path).status();
        if let Ok(s) = status {
            if s.success() {
                return Ok(vec![path]);
            }
        }
        return Err("grim failed; install grim for Wayland multi-monitor screen capture".into());
    }

    // X11: scrot -m captures all monitors into one image.
    let status = std::process::Command::new("scrot")
        .arg("-m")
        .arg(&path)
        .status();
    if let Ok(s) = status {
        if s.success() {
            return Ok(vec![path]);
        }
    }
    // Fallback: gnome-screenshot (single display only).
    let status = std::process::Command::new("gnome-screenshot")
        .arg("-f")
        .arg(&path)
        .status()?;
    if !status.success() {
        return Err("screenshot capture failed (install scrot, grim, or gnome-screenshot)".into());
    }
    Ok(vec![path])
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn capture_screens() -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    Err("Screen capture is not supported on this platform.".into())
}
