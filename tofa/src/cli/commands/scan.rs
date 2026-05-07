use crate::cli::{open_vault, read_passphrase, CliResult};
use clap::Args;
use std::collections::HashSet;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tofa_core::store::Vault;
use tofa_core::totp::{format_code, generate_code_now, seconds_remaining_now};

/// Tiny stderr braille spinner used around long-ish work (screen capture +
/// QR scanning). The message slot is shared so the scan loop can update it
/// with progress info without needing to stop and restart the animation.
/// Stops on Drop so a `?` early-return still clears the line.
struct Spinner {
    message: Arc<Mutex<String>>,
    stop: Arc<AtomicBool>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl Spinner {
    fn start(message: &str) -> Self {
        let message = Arc::new(Mutex::new(message.to_string()));
        let stop = Arc::new(AtomicBool::new(false));
        let stop_clone = stop.clone();
        let msg_clone = message.clone();
        let handle = std::thread::spawn(move || {
            let frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
            let mut i = 0;
            while !stop_clone.load(Ordering::Relaxed) {
                let f = frames[i % frames.len()];
                let m = msg_clone.lock().map(|g| g.clone()).unwrap_or_default();
                // \x1b[2K clears the whole line so a shorter new message
                // doesn't leave trailing characters from a longer one.
                eprint!("\r\x1b[2K{f} {m}");
                let _ = std::io::stderr().flush();
                std::thread::sleep(std::time::Duration::from_millis(80));
                i += 1;
            }
            eprint!("\r\x1b[2K");
            let _ = std::io::stderr().flush();
        });
        Self {
            message,
            stop,
            handle: Some(handle),
        }
    }

    fn set(&self, message: impl Into<String>) {
        if let Ok(mut g) = self.message.lock() {
            *g = message.into();
        }
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

#[derive(Args)]
pub struct ScanArgs {
    /// Override the account name (only applied when exactly one entry is found)
    #[arg(long, value_name = "NAME")]
    pub name: Option<String>,
}

pub fn run(args: ScanArgs, vault_path: PathBuf) -> CliResult {
    eprintln!(
        "⚠  Experimental — screen scan may miss QR codes at the edge of \
         rqrr's detection threshold. If a code is missing, rerun the scan \
         or import that one separately."
    );

    let captures = {
        let _spin = Spinner::start("Capturing screens…");
        capture_screens()?
    };
    if captures.is_empty() {
        return Err("No displays available to capture.".into());
    }

    let uris = scan_with_progress(&captures);
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
        println!("Current code: {}  ({secs}s)", format_code(&code));
    }
    Ok(())
}

/// Drive a stderr spinner whose message updates after each resolution pass —
/// so the user sees something like "screen 1/2 • pass @ 3840px • 7 found"
/// instead of a static label while the multi-second native + rescale ladder
/// runs. Per-screen and per-pass progress is the only feedback users get for
/// scans that can take 5–30s on a Retina display.
fn scan_with_progress(paths: &[PathBuf]) -> Vec<String> {
    let spin = Spinner::start("Scanning for QR codes…");
    let mut seen: HashSet<String> = HashSet::new();
    let mut uris: Vec<String> = Vec::new();
    let total_screens = paths.len();
    for (i, path) in paths.iter().enumerate() {
        let prefix = if total_screens > 1 {
            format!("screen {}/{} • ", i + 1, total_screens)
        } else {
            String::new()
        };
        let prior = uris.len();
        spin.set(format!("{prefix}preparing image…"));
        let result = tofa_core::qr::scan_all_qr_uris_with_progress(path, |p| {
            spin.set(format!(
                "{prefix}pass @ {}px • {} found",
                p.pass_width,
                prior + p.found
            ));
        });
        if let Ok(found) = result {
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
    let today = tofa_core::today_iso();

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
        vault.add_entry(otp.into_vault_entry(name, today.clone()));
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
    use std::process::Stdio;
    let mut paths = Vec::new();
    for n in 1.. {
        let path = std::env::temp_dir().join(format!("tofa-scan-{n}.png"));
        // Silence stderr — when -D N is out of range, screencapture writes
        // "Invalid display specified..." which is how we discover we're done.
        // It's expected control flow, not something the user needs to see.
        let status = std::process::Command::new("screencapture")
            .args(["-x", "-t", "png", "-D", &n.to_string()])
            .arg(&path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        if !status.success() {
            let _ = std::fs::remove_file(&path);
            break;
        }
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
    use std::process::Stdio;
    let path = std::env::temp_dir().join("tofa-scan.png");
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    let is_wayland = session_type == "wayland" || std::env::var("WAYLAND_DISPLAY").is_ok();

    if is_wayland {
        let status = std::process::Command::new("grim")
            .arg(&path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        if let Ok(s) = status {
            if s.success() {
                return Ok(vec![path]);
            }
        }
        return Err("grim failed; install grim for Wayland multi-monitor screen capture".into());
    }

    let status = std::process::Command::new("scrot")
        .arg("-m")
        .arg(&path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    if let Ok(s) = status {
        if s.success() {
            return Ok(vec![path]);
        }
    }
    let status = std::process::Command::new("gnome-screenshot")
        .arg("-f")
        .arg(&path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
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
