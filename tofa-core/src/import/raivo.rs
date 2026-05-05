use crate::qr::{OtpMeta, OtpSecret};

/// Parse a Raivo OTP JSON export (root array; `digits` and `timer` are strings).
pub fn parse(entries: &[serde_json::Value]) -> Result<Vec<OtpSecret>, String> {
    let mut otps = Vec::new();
    for e in entries {
        if e["kind"].as_str().map(|t| t.to_uppercase()) != Some("TOTP".to_string()) {
            continue;
        }

        let secret = e["secret"].as_str().unwrap_or("").to_string();
        if secret.is_empty() {
            continue;
        }

        let issuer = e["issuer"].as_str().unwrap_or("").to_string();
        let account = e["account"].as_str().unwrap_or("").to_string();

        // Raivo stores digits and timer as strings
        let digits: Option<u8> = e["digits"].as_str().and_then(|s| s.parse().ok());
        let period: Option<u32> = e["timer"].as_str().and_then(|s| s.parse().ok());
        let algorithm = e["algorithm"].as_str().map(String::from);

        otps.push(OtpSecret {
            secret,
            meta: OtpMeta {
                issuer: if issuer.is_empty() {
                    None
                } else {
                    Some(issuer)
                },
                account: if account.is_empty() {
                    None
                } else {
                    Some(account)
                },
                algorithm,
                digits,
                period,
            },
        });
    }

    if otps.is_empty() {
        Err("No TOTP entries found in Raivo export.".to_string())
    } else {
        Ok(otps)
    }
}
