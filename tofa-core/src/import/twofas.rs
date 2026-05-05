use crate::qr::{OtpMeta, OtpSecret};

/// Parse a 2FAS `.2fas` export (JSON with a top-level `services` array).
pub fn parse(v: &serde_json::Value) -> Result<Vec<OtpSecret>, String> {
    let services = v["services"]
        .as_array()
        .ok_or("2FAS: missing 'services' array")?;

    let mut otps = Vec::new();
    for svc in services {
        let token_type = svc
            .pointer("/otp/tokenType")
            .and_then(|t| t.as_str())
            .unwrap_or("");
        if token_type.to_uppercase() != "TOTP" {
            continue;
        }

        let secret = svc["secret"].as_str().unwrap_or("").to_string();
        if secret.is_empty() {
            continue;
        }

        let issuer = svc
            .pointer("/otp/issuer")
            .and_then(|v| v.as_str())
            .or_else(|| svc["name"].as_str())
            .unwrap_or("")
            .to_string();

        let account = svc
            .pointer("/otp/account")
            .and_then(|v| v.as_str())
            .or_else(|| svc.pointer("/otp/label").and_then(|v| v.as_str()))
            .unwrap_or("")
            .to_string();

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
                algorithm: svc
                    .pointer("/otp/algorithm")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                digits: svc
                    .pointer("/otp/digits")
                    .and_then(|v| v.as_u64())
                    .map(|d| d as u8),
                period: svc
                    .pointer("/otp/period")
                    .and_then(|v| v.as_u64())
                    .map(|p| p as u32),
            },
        });
    }

    if otps.is_empty() {
        Err("No TOTP entries found in 2FAS export.".to_string())
    } else {
        Ok(otps)
    }
}
