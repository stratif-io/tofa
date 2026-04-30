use thiserror::Error;

#[derive(Debug, Error)]
pub enum QrError {
    #[error("could not read image file: {0}")]
    ImageLoad(String),
    #[error("no QR code found in image")]
    NoQrFound,
    #[error("QR code does not contain a valid otpauth:// URI")]
    InvalidQrContent,
    #[error("missing 'secret' parameter in otpauth:// URI")]
    MissingSecret,
    #[error("input is not a valid Base32 secret, otpauth:// URI, or image path")]
    UnrecognizedInput,
}

#[derive(Debug, Clone, Default)]
pub struct OtpMeta {
    pub issuer: Option<String>,
    pub account: Option<String>,
    pub algorithm: Option<String>,
    pub digits: Option<u8>,
    pub period: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct OtpSecret {
    pub secret: String,
    pub meta: OtpMeta,
}

pub fn parse_input(input: &str) -> Result<OtpSecret, QrError> {
    let trimmed = input.trim();

    if trimmed.starts_with("otpauth://") {
        return parse_uri(trimmed);
    }

    // Strip shell backslash-escapes (e.g. from terminal drag & drop: "path\ with\ spaces")
    let unescaped = trimmed.replace("\\ ", " ");
    let path = std::path::Path::new(unescaped.as_str());
    if path.exists() {
        return parse_qr_image(path);
    }

    let path = std::path::Path::new(trimmed);
    if path.exists() {
        return parse_qr_image(path);
    }

    if is_valid_base32(trimmed) {
        return Ok(OtpSecret {
            secret: trimmed.to_uppercase(),
            meta: OtpMeta::default(),
        });
    }

    Err(QrError::UnrecognizedInput)
}

fn parse_uri(uri: &str) -> Result<OtpSecret, QrError> {
    // Format: otpauth://totp/LABEL?secret=SECRET&issuer=ISSUER&...
    // LABEL may be "issuer:account" or just "account"
    let after_scheme = uri.strip_prefix("otpauth://totp/").unwrap_or("");
    let (raw_label, query) = match after_scheme.split_once('?') {
        Some((l, q)) => (l, q),
        None => (after_scheme, ""),
    };

    let label = percent_decode(raw_label);
    let (label_issuer, label_account) = if let Some((a, b)) = label.split_once(':') {
        (Some(a.trim().to_string()), Some(b.trim().to_string()))
    } else if !label.is_empty() {
        (None, Some(label.clone()))
    } else {
        (None, None)
    };

    let mut secret = None;
    let mut issuer: Option<String> = None;
    let mut algorithm: Option<String> = None;
    let mut digits: Option<u8> = None;
    let mut period: Option<u32> = None;

    for param in query.split('&') {
        let mut parts = param.splitn(2, '=');
        let key = parts.next().unwrap_or("");
        let val = percent_decode(parts.next().unwrap_or(""));
        match key {
            "secret" => secret = Some(val.to_uppercase()),
            "issuer" => issuer = Some(val),
            "algorithm" => algorithm = Some(val),
            "digits" => digits = val.parse().ok(),
            "period" => period = val.parse().ok(),
            _ => {}
        }
    }

    let secret = secret.ok_or(QrError::MissingSecret)?;
    let resolved_issuer = issuer.or(label_issuer);
    let resolved_account = label_account;

    Ok(OtpSecret {
        secret,
        meta: OtpMeta {
            issuer: resolved_issuer,
            account: resolved_account,
            algorithm,
            digits,
            period,
        },
    })
}

fn parse_qr_image(path: &std::path::Path) -> Result<OtpSecret, QrError> {
    let img = image::open(path)
        .map_err(|e| QrError::ImageLoad(e.to_string()))?
        .to_luma8();

    let mut img = rqrr::PreparedImage::prepare(img);
    let grids = img.detect_grids();

    let content = grids
        .into_iter()
        .find_map(|grid| grid.decode().ok().map(|(_, content)| content))
        .ok_or(QrError::NoQrFound)?;

    if content.starts_with("otpauth://") {
        parse_uri(&content)
    } else {
        Err(QrError::InvalidQrContent)
    }
}

fn percent_decode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            let h1 = chars.next();
            let h2 = chars.next();
            if let (Some(h1), Some(h2)) = (h1, h2) {
                if let Ok(byte) = u8::from_str_radix(&format!("{h1}{h2}"), 16) {
                    out.push(byte as char);
                    continue;
                }
            }
        }
        if c == '+' {
            out.push(' ');
        } else {
            out.push(c);
        }
    }
    out
}

fn is_valid_base32(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '2'..='7' | '='))
}
