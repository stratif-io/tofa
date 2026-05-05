use crate::qr::{parse_uri, OtpMeta, OtpSecret};

/// Parse a Bitwarden JSON export. Extracts TOTP from `items[].login.totp`.
/// The `totp` field may be a full `otpauth://` URI or a bare base32 secret.
pub fn parse(v: &serde_json::Value) -> Result<Vec<OtpSecret>, String> {
    let items = v["items"]
        .as_array()
        .ok_or("Bitwarden: missing 'items' array")?;

    let mut otps = Vec::new();
    for item in items {
        // type 1 = login
        if item["type"].as_u64() != Some(1) {
            continue;
        }
        let totp_field = match item.pointer("/login/totp").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s,
            _ => continue,
        };

        let otp = if totp_field.starts_with("otpauth://") {
            parse_uri(totp_field).map_err(|e| e.to_string())?
        } else {
            // Bare base32 secret — attach the item name as issuer
            let name = item["name"].as_str().unwrap_or("").to_string();
            OtpSecret {
                secret: totp_field.to_uppercase(),
                meta: OtpMeta {
                    issuer: if name.is_empty() { None } else { Some(name) },
                    account: None,
                    algorithm: None,
                    digits: None,
                    period: None,
                },
            }
        };
        otps.push(otp);
    }

    if otps.is_empty() {
        Err("No TOTP entries found in Bitwarden export.".to_string())
    } else {
        Ok(otps)
    }
}
