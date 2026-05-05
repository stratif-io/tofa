pub mod aegis;
pub mod andotp;
pub mod bitwarden;
pub mod ente;
pub mod freeotp;
pub mod google_authenticator;
pub mod keepassxc;
pub mod raivo;
pub mod twofas;

use crate::qr::OtpSecret;

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

    // Root array: Raivo (has "kind") or andOTP (has "type")
    if let Some(entries) = v.as_array() {
        let first = entries.first();
        if first.and_then(|e| e.get("kind")).is_some() {
            return raivo::parse(entries);
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
