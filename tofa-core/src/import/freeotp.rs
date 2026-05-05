use crate::qr::{OtpMeta, OtpSecret};
use data_encoding::BASE32_NOPAD;

/// Parse a FreeOTP+ JSON export.
/// Secrets are stored as arrays of signed Java bytes (i8 range, -128..=127).
pub fn parse(v: &serde_json::Value) -> Result<Vec<OtpSecret>, String> {
    let tokens = v["tokens"]
        .as_array()
        .ok_or("FreeOTP+: missing 'tokens' array")?;

    let mut otps = Vec::new();
    for t in tokens {
        if t["type"].as_str().map(|s| s.to_uppercase()) != Some("TOTP".to_string()) {
            continue;
        }

        let secret = decode_secret(t)?;
        if secret.is_empty() {
            continue;
        }

        let issuer = t["issuerExt"]
            .as_str()
            .or_else(|| t["issuerInt"].as_str())
            .unwrap_or("")
            .to_string();
        let account = t["label"].as_str().unwrap_or("").to_string();

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
                algorithm: t["algo"].as_str().map(String::from),
                digits: t["digits"].as_u64().map(|d| d as u8),
                period: t["period"].as_u64().map(|p| p as u32),
            },
        });
    }

    if otps.is_empty() {
        Err("No TOTP entries found in FreeOTP+ export.".to_string())
    } else {
        Ok(otps)
    }
}

fn decode_secret(token: &serde_json::Value) -> Result<String, String> {
    // Prefer pre-encoded base32 if present
    if let Some(b32) = token["secret_base32"].as_str() {
        if !b32.is_empty() {
            return Ok(b32.to_uppercase());
        }
    }

    // Signed Java byte array → unsigned bytes → base32
    let arr = token["secret"]
        .as_array()
        .ok_or("FreeOTP+: missing 'secret' field")?;

    let bytes: Vec<u8> = arr
        .iter()
        .map(|b| {
            b.as_i64()
                .map(|v| (v & 0xFF) as u8)
                .ok_or_else(|| "FreeOTP+: invalid byte in secret array".to_string())
        })
        .collect::<Result<_, _>>()?;

    Ok(BASE32_NOPAD.encode(&bytes))
}
