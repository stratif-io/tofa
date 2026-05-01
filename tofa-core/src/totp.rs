use crate::store::VaultEntry;
use thiserror::Error;
use totp_rs::{Algorithm, Secret, TOTP};

#[derive(Debug, Error)]
pub enum TotpError {
    #[error("invalid TOTP secret: {0}")]
    InvalidSecret(String),
    #[error("code generation failed: {0}")]
    Generation(String),
    #[error("unsupported algorithm: {0}")]
    UnsupportedAlgorithm(String),
}

fn parse_algorithm(s: &str) -> Result<Algorithm, TotpError> {
    match s.to_uppercase().as_str() {
        "SHA1"   => Ok(Algorithm::SHA1),
        "SHA256" => Ok(Algorithm::SHA256),
        "SHA512" => Ok(Algorithm::SHA512),
        other    => Err(TotpError::UnsupportedAlgorithm(other.to_string())),
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Generate a TOTP code for the given entry at a specific Unix timestamp.
pub fn generate_code_at(entry: &VaultEntry, timestamp: u64) -> Result<String, TotpError> {
    let secret = Secret::Encoded(entry.secret.to_uppercase())
        .to_bytes()
        .map_err(|e| TotpError::InvalidSecret(e.to_string()))?;
    let algo = parse_algorithm(&entry.algorithm)?;
    // new_unchecked bypasses the ≥128-bit check so short GA export secrets work
    let totp = TOTP::new_unchecked(algo, entry.digits as usize, 1, entry.period as u64, secret);
    Ok(totp.generate(timestamp))
}

/// Generate a TOTP code for the given entry using the current system time.
pub fn generate_code_now(entry: &VaultEntry) -> Result<String, TotpError> {
    generate_code_at(entry, now_secs())
}

/// Format a raw TOTP code with a space in the middle.
/// 6 digits → "XXX XXX", 8 digits → "XXXX XXXX", others → as-is.
pub fn format_code(raw: &str) -> String {
    match raw.len() {
        6 => format!("{} {}", &raw[..3], &raw[3..]),
        8 => format!("{} {}", &raw[..4], &raw[4..]),
        _ => raw.to_string(),
    }
}

/// Format a masked placeholder matching the entry's digit count.
pub fn mask_code(entry: &VaultEntry) -> &'static str {
    if entry.digits == 8 { "•••• ••••" } else { "••• •••" }
}

/// Seconds remaining in the entry's TOTP window at a specific timestamp.
pub fn seconds_remaining(entry: &VaultEntry, timestamp: u64) -> u64 {
    let p = entry.period as u64;
    p - (timestamp % p)
}

/// Seconds remaining in the entry's TOTP window using the current system time.
pub fn seconds_remaining_now(entry: &VaultEntry) -> u64 {
    seconds_remaining(entry, now_secs())
}
