use crate::qr::{OtpMeta, OtpSecret};

/// Parse a native tofa JSON export (root array of VaultEntry-shaped objects).
pub fn parse(entries: &[serde_json::Value]) -> Result<Vec<OtpSecret>, String> {
    let mut otps = Vec::new();
    for e in entries {
        let secret = e["secret"].as_str().unwrap_or("").to_string();
        if secret.is_empty() {
            continue;
        }
        let name = e["name"].as_str().unwrap_or("").to_string();
        let (issuer, account) = if let Some(pos) = name.find(':') {
            (
                Some(name[..pos].to_string()),
                Some(name[pos + 1..].to_string()),
            )
        } else {
            (None, Some(name))
        };
        otps.push(OtpSecret {
            secret,
            meta: OtpMeta {
                issuer,
                account,
                algorithm: e["algorithm"].as_str().map(String::from),
                digits: e["digits"].as_u64().map(|d| d as u8),
                period: e["period"].as_u64().map(|p| p as u32),
            },
        });
    }
    if otps.is_empty() {
        Err("No entries found in native tofa export.".to_string())
    } else {
        Ok(otps)
    }
}
