use crate::qr::{OtpMeta, OtpSecret};

pub fn parse(entries: &[serde_json::Value]) -> Result<Vec<OtpSecret>, String> {
    let mut otps = Vec::new();
    for e in entries {
        if e["type"].as_str().map(|t| t.to_uppercase()) != Some("TOTP".to_string()) {
            continue;
        }
        let secret = e["secret"].as_str().unwrap_or("").to_string();
        if secret.is_empty() {
            continue;
        }
        let account = e["label"].as_str().unwrap_or("").to_string();
        let issuer = e["issuer"].as_str().unwrap_or("").to_string();
        otps.push(OtpSecret {
            secret,
            meta: OtpMeta {
                account: if account.is_empty() {
                    None
                } else {
                    Some(account)
                },
                issuer: if issuer.is_empty() {
                    None
                } else {
                    Some(issuer)
                },
                algorithm: e["algorithm"].as_str().map(String::from),
                digits: e["digits"].as_u64().map(|d| d as u8),
                period: e["period"].as_u64().map(|p| p as u32),
            },
        });
    }
    if otps.is_empty() {
        Err("No TOTP entries found in andOTP export.".to_string())
    } else {
        Ok(otps)
    }
}
