use crate::qr::{parse_uri, OtpSecret};

/// Parse an Ente Auth plain-text export: one `otpauth://` URI per line.
pub fn parse(text: &str) -> Result<Vec<OtpSecret>, String> {
    let otps: Vec<OtpSecret> = text
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("otpauth://totp/"))
        .map(|line| parse_uri(line).map_err(|e| e.to_string()))
        .collect::<Result<_, _>>()?;

    if otps.is_empty() {
        Err("No TOTP entries found in Ente Auth export.".to_string())
    } else {
        Ok(otps)
    }
}
