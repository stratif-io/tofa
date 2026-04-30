use thiserror::Error;
use totp_rs::{Algorithm, Secret, TOTP};

#[derive(Debug, Error)]
pub enum TotpError {
    #[error("invalid TOTP secret: {0}")]
    InvalidSecret(String),
    #[error("code generation failed: {0}")]
    Generation(String),
}

/// Generates a 6-digit TOTP code for the given Base32 secret at a specific Unix timestamp.
pub fn generate_code(base32_secret: &str, timestamp: u64) -> Result<String, TotpError> {
    let secret = Secret::Encoded(base32_secret.to_uppercase())
        .to_bytes()
        .map_err(|e| TotpError::InvalidSecret(e.to_string()))?;

    let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, secret)
        .map_err(|e| TotpError::InvalidSecret(e.to_string()))?;

    Ok(totp.generate(timestamp))
}

/// Returns seconds remaining in the current 30-second TOTP window for the given timestamp.
pub fn seconds_remaining(timestamp: u64) -> u64 {
    30 - (timestamp % 30)
}

/// Generates a code using the current system time.
pub fn generate_code_now(base32_secret: &str) -> Result<String, TotpError> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    generate_code(base32_secret, timestamp)
}

/// Returns seconds remaining in the current TOTP window using system time.
pub fn seconds_remaining_now() -> u64 {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    seconds_remaining(timestamp)
}
