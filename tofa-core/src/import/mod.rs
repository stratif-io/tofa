pub mod aegis;
pub mod andotp;
pub mod bitwarden;
pub mod ente;
pub mod freeotp;
pub mod google_authenticator;
pub mod keepassxc;
pub mod native;
pub mod raivo;
pub mod twofas;

use crate::qr::OtpSecret;
use std::io::Read;
use std::path::Path;

const IMAGE_EXTS: &[&str] = &["png", "jpg", "jpeg", "gif", "bmp", "webp", "tiff"];

/// Single entry point for importing any file format the user can hand us.
/// Dispatches by extension first, then by content where the extension is
/// ambiguous. Used by the CLI's `tofa import`, the TUI's drop handler, and
/// the desktop app's drop handler so format support is identical across
/// all three.
///
/// Supported:
/// - Image (`png`/`jpg`/`gif`/`bmp`/`webp`/`tiff`) containing one or many
///   `otpauth://` and/or `otpauth-migration://` QRs.
/// - Zip archive of any number of QR images (round-trips the desktop
///   app's "Save All" output).
/// - JSON: Aegis, andOTP, 2FAS, Raivo, Bitwarden, FreeOTP+, native tofa.
/// - CSV: KeePassXC.
/// - Plain text: newline-separated `otpauth://` URIs (Ente Auth format),
///   or a single `otpauth-migration://` URI.
pub fn parse_file(path: &Path) -> Result<Vec<OtpSecret>, String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "zip" => parse_zip(path),
        e if IMAGE_EXTS.contains(&e) => parse_image(path),
        "json" | "2fas" => {
            let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
            parse_json_bytes(&bytes)
        }
        "csv" => {
            let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
            parse_csv(&text)
        }
        "txt" => {
            let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
            parse_text_dispatch(&text)
        }
        "" => Err("File has no extension; can't determine format.".to_string()),
        other => Err(format!("Unsupported file extension: .{other}")),
    }
}

/// Run the multi-resolution QR scanner on an image and parse every URI
/// it finds. Each URI may be a single-account `otpauth://` or a
/// Google-Authenticator `otpauth-migration://` (which expands to several
/// accounts), so the result is flattened.
fn parse_image(path: &Path) -> Result<Vec<OtpSecret>, String> {
    let uris = crate::qr::scan_all_qr_uris(path).map_err(|e| e.to_string())?;
    parse_uri_list(&uris)
}

/// Iterate the zip's entries by index. Slip-path attacks aren't a
/// concern because we never write extracted files to disk — we decode
/// each image entry in memory and feed it to the QR scanner. Non-image
/// entries (a README, etc.) are silently skipped so users don't have to
/// curate their archive before importing.
fn parse_zip(path: &Path) -> Result<Vec<OtpSecret>, String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    let mut all_uris: Vec<String> = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| e.to_string())?;
        if !entry.is_file() {
            continue;
        }
        let name = entry.name().to_lowercase();
        let entry_ext = std::path::Path::new(&name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        if !IMAGE_EXTS.contains(&entry_ext) {
            continue;
        }
        let mut bytes = Vec::with_capacity(entry.size() as usize);
        entry.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
        let img = match image::load_from_memory(&bytes) {
            Ok(i) => i,
            Err(_) => continue,
        };
        if let Ok(uris) = crate::qr::scan_dynamic_image_with_progress(img, |_| {}) {
            for u in uris {
                if seen.insert(u.clone()) {
                    all_uris.push(u);
                }
            }
        }
    }

    if all_uris.is_empty() {
        return Err("Zip contained no decodable QR codes.".to_string());
    }
    parse_uri_list(&all_uris)
}

/// Plain-text dispatch: a `otpauth-migration://` URI on its own gets
/// expanded; otherwise we treat the file as a newline-separated list of
/// `otpauth://` URIs (Ente Auth's export format).
fn parse_text_dispatch(text: &str) -> Result<Vec<OtpSecret>, String> {
    let trimmed = text.trim();
    if trimmed.starts_with("otpauth-migration://") {
        return parse_migration_uri(trimmed);
    }
    parse_text_uris(text)
}

fn parse_uri_list(uris: &[String]) -> Result<Vec<OtpSecret>, String> {
    let mut out = Vec::new();
    for uri in uris {
        if uri.starts_with("otpauth-migration://") {
            out.extend(parse_migration_uri(uri)?);
        } else if uri.starts_with("otpauth://") {
            out.push(crate::qr::parse_input(uri).map_err(|e| e.to_string())?);
        }
        // Anything else from the QR (a vCard, a URL) is silently
        // ignored — the user's stated intent was "import OTPs".
    }
    if out.is_empty() {
        return Err("No otpauth URIs found.".to_string());
    }
    Ok(out)
}

/// Parse a JSON import from raw bytes.
/// Supports: Aegis, andOTP, 2FAS, Raivo, Bitwarden, FreeOTP+.
pub fn parse_json_bytes(bytes: &[u8]) -> Result<Vec<OtpSecret>, String> {
    let v: serde_json::Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;

    // Aegis: has "version" or "db" key
    if v.get("version").is_some() || v.get("db").is_some() {
        if v.pointer("/db/is_locked")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            return Err(
                "Aegis export is encrypted — re-export without encryption first.".to_string(),
            );
        }
        if let Some(entries) = v.pointer("/db/entries").and_then(|e| e.as_array()) {
            return aegis::parse(entries);
        }
        return Err(
            "Aegis export is encrypted or unsupported — re-export without encryption.".to_string(),
        );
    }

    // 2FAS: has "services" array
    if v.get("services").is_some() {
        return twofas::parse(&v);
    }

    // Bitwarden: has "items" array
    if v.get("items").is_some() {
        return bitwarden::parse(&v);
    }

    // FreeOTP+: has "tokens" array
    if v.get("tokens").is_some() {
        return freeotp::parse(&v);
    }

    // Root array: Raivo (has "kind"), native tofa export (has "name", no "type"), or andOTP
    if let Some(entries) = v.as_array() {
        let first = entries.first();
        if first.and_then(|e| e.get("kind")).is_some() {
            return raivo::parse(entries);
        }
        // Native tofa export: entries have "name" and "secret" but no "type" field
        if first
            .map(|e| e.get("name").is_some() && e.get("type").is_none())
            .unwrap_or(false)
        {
            return native::parse(entries);
        }
        return andotp::parse(entries);
    }

    Err(
        "Unrecognised JSON format. Supported: Aegis, andOTP, 2FAS, Raivo, Bitwarden, FreeOTP+."
            .to_string(),
    )
}

/// Parse a Google Authenticator `otpauth-migration://` URI (from an exported QR code).
pub fn parse_migration_uri(uri: &str) -> Result<Vec<OtpSecret>, String> {
    google_authenticator::parse(uri).map_err(|e| e.to_string())
}

/// Parse an Ente Auth plain-text export (newline-separated `otpauth://` URIs).
pub fn parse_text_uris(text: &str) -> Result<Vec<OtpSecret>, String> {
    ente::parse(text)
}

/// Parse a KeePassXC CSV export.
pub fn parse_csv(csv: &str) -> Result<Vec<OtpSecret>, String> {
    keepassxc::parse(csv)
}
