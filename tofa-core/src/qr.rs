use base64::{
    engine::general_purpose::{STANDARD as B64, URL_SAFE, URL_SAFE_NO_PAD},
    Engine as _,
};
use data_encoding::BASE32_NOPAD;
use qrcode::{Color as QrColor, EcLevel, QrCode};
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
    #[error("could not decode Google Authenticator migration payload")]
    MigrationDecode,
    #[error("could not generate QR code: {0}")]
    QrGenerate(String),
}

#[derive(Debug, Clone, Default)]
pub struct OtpMeta {
    pub issuer: Option<String>,
    pub account: Option<String>,
    pub algorithm: Option<String>,
    pub digits: Option<u8>,
    pub period: Option<u32>,
}

impl OtpMeta {
    /// Derive a vault entry name from issuer/account metadata.
    /// Format: "Issuer:account" when both present, else whichever is available.
    pub fn derive_name(&self) -> String {
        match (&self.issuer, &self.account) {
            (Some(i), Some(a)) => format!("{i}:{a}"),
            (Some(i), None) => i.clone(),
            (None, Some(a)) => a.clone(),
            (None, None) => "Imported".to_string(),
        }
    }

    /// Split a vault entry name back into (issuer, account).
    /// "GitHub:alice@example.com" → ("GitHub", "alice@example.com")
    /// "alice@example.com"        → ("",        "alice@example.com")
    pub fn split_name(name: &str) -> (String, String) {
        if let Some(pos) = name.find(':') {
            (name[..pos].to_string(), name[pos + 1..].to_string())
        } else {
            (String::new(), name.to_string())
        }
    }
}

#[derive(Debug, Clone)]
pub struct OtpSecret {
    pub secret: String,
    pub meta: OtpMeta,
}

/// Parse a Google Authenticator `otpauth-migration://` URI.
/// Returns all TOTP accounts found in the payload.
pub fn parse_migration(uri: &str) -> Result<Vec<OtpSecret>, QrError> {
    // Extract the `data` query parameter
    let data_param = uri
        .split('?')
        .nth(1)
        .and_then(|q| {
            q.split('&')
                .find(|p| p.starts_with("data="))
                .map(|p| &p[5..])
        })
        .ok_or(QrError::MigrationDecode)?;

    // Percent-decode then base64-decode
    let decoded_str = percent_decode(data_param);
    let b = decoded_str.as_bytes();
    let proto_bytes = B64
        .decode(b)
        .or_else(|_| URL_SAFE.decode(b))
        .or_else(|_| URL_SAFE_NO_PAD.decode(b))
        .map_err(|_| QrError::MigrationDecode)?;

    // Parse protobuf manually (MigrationPayload, field 1 = repeated OtpParameters)
    let mut pos = 0;
    let mut results = Vec::new();

    while pos < proto_bytes.len() {
        let tag = proto_decode_varint(&proto_bytes, &mut pos).ok_or(QrError::MigrationDecode)?;
        let field_number = tag >> 3;
        let wire_type = tag & 0x7;

        match (field_number, wire_type) {
            (1, 2) => {
                // OtpParameters message
                let msg =
                    proto_decode_bytes(&proto_bytes, &mut pos).ok_or(QrError::MigrationDecode)?;
                if let Some(otp) = parse_otp_parameters(msg) {
                    results.push(otp);
                }
            }
            (_, 0) => {
                proto_decode_varint(&proto_bytes, &mut pos);
            }
            (_, 2) => {
                proto_decode_bytes(&proto_bytes, &mut pos);
            }
            (_, 1) => {
                if pos + 8 <= proto_bytes.len() {
                    pos += 8;
                } else {
                    break;
                }
            }
            (_, 5) => {
                if pos + 4 <= proto_bytes.len() {
                    pos += 4;
                } else {
                    break;
                }
            }
            _ => break,
        }
    }

    if results.is_empty() {
        Err(QrError::MigrationDecode)
    } else {
        Ok(results)
    }
}

fn parse_otp_parameters(data: &[u8]) -> Option<OtpSecret> {
    let mut pos = 0;
    let mut secret_bytes: Option<Vec<u8>> = None;
    let mut name: Option<String> = None;
    let mut issuer: Option<String> = None;
    let mut algorithm: Option<String> = None;
    let mut digits: Option<u8> = None;
    let mut otp_type: u64 = 0;

    while pos < data.len() {
        let tag = proto_decode_varint(data, &mut pos)?;
        let field_number = tag >> 3;
        let wire_type = tag & 0x7;

        match (field_number, wire_type) {
            (1, 2) => {
                secret_bytes = Some(proto_decode_bytes(data, &mut pos)?.to_vec());
            }
            (2, 2) => {
                name =
                    Some(String::from_utf8_lossy(proto_decode_bytes(data, &mut pos)?).into_owned());
            }
            (3, 2) => {
                issuer =
                    Some(String::from_utf8_lossy(proto_decode_bytes(data, &mut pos)?).into_owned());
            }
            (4, 0) => {
                let algo_id = proto_decode_varint(data, &mut pos)?;
                algorithm = Some(
                    match algo_id {
                        2 => "SHA256",
                        3 => "SHA512",
                        _ => "SHA1",
                    }
                    .to_string(),
                );
            }
            (5, 0) => {
                let d = proto_decode_varint(data, &mut pos)?;
                digits = Some(if d == 2 { 8 } else { 6 });
            }
            (6, 0) => {
                otp_type = proto_decode_varint(data, &mut pos)?;
            }
            (_, 0) => {
                proto_decode_varint(data, &mut pos)?;
            }
            (_, 2) => {
                proto_decode_bytes(data, &mut pos)?;
            }
            _ => break,
        }
    }

    // Only handle TOTP (type 2); skip HOTP
    if otp_type != 0 && otp_type != 2 {
        return None;
    }

    let raw_secret = secret_bytes?;
    let secret = base32_encode(&raw_secret);

    // Derive account from name field ("issuer:account" or just "account")
    let (resolved_issuer, account) = match (&issuer, &name) {
        (Some(i), Some(n)) => {
            let acc = n
                .strip_prefix(&format!("{i}:"))
                .unwrap_or(n)
                .trim()
                .to_string();
            (Some(i.clone()), Some(acc))
        }
        (None, Some(n)) => {
            if let Some((i, a)) = n.split_once(':') {
                (Some(i.trim().to_string()), Some(a.trim().to_string()))
            } else {
                (None, Some(n.clone()))
            }
        }
        _ => (issuer.clone(), None),
    };

    Some(OtpSecret {
        secret,
        meta: OtpMeta {
            issuer: resolved_issuer,
            account,
            algorithm,
            digits,
            period: None,
        },
    })
}

fn base32_encode(data: &[u8]) -> String {
    BASE32_NOPAD.encode(data)
}

fn proto_decode_varint(data: &[u8], pos: &mut usize) -> Option<u64> {
    let mut result: u64 = 0;
    let mut shift = 0u32;
    loop {
        let byte = *data.get(*pos)?;
        *pos += 1;
        result |= ((byte & 0x7F) as u64) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift >= 64 {
            return None;
        }
    }
    Some(result)
}

fn proto_decode_bytes<'a>(data: &'a [u8], pos: &mut usize) -> Option<&'a [u8]> {
    let len = proto_decode_varint(data, pos)? as usize;
    let slice = data.get(*pos..*pos + len)?;
    *pos += len;
    Some(slice)
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

pub(crate) fn parse_uri(uri: &str) -> Result<OtpSecret, QrError> {
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

/// Builds a Google Authenticator `otpauth-migration://` URI from a list of
/// (name, issuer, base32_secret) tuples.  Returns the URI string ready to
/// encode into a QR code.
pub fn generate_migration_uri(
    accounts: &[(&str, &str, &str)], // (name, issuer, base32_secret)
) -> Result<String, QrError> {
    let mut payload: Vec<u8> = Vec::new();

    for (name, issuer, secret_b32) in accounts {
        let secret_bytes = BASE32_NOPAD
            .decode(secret_b32.trim_end_matches('=').as_bytes())
            .map_err(|_| QrError::MigrationDecode)?;

        let mut otp: Vec<u8> = Vec::new();
        // field 1: secret bytes
        otp.extend(proto_field_bytes(1, &secret_bytes));
        // field 2: name
        otp.extend(proto_field_bytes(2, name.as_bytes()));
        // field 3: issuer (skip if empty)
        if !issuer.is_empty() {
            otp.extend(proto_field_bytes(3, issuer.as_bytes()));
        }
        // field 4: algorithm = 1 (SHA1)
        otp.extend(proto_field_varint(4, 1));
        // field 5: digits = 1 (SIX)
        otp.extend(proto_field_varint(5, 1));
        // field 6: type = 2 (TOTP)
        otp.extend(proto_field_varint(6, 2));

        // wrap as field 1 of MigrationPayload
        payload.extend(proto_field_bytes(1, &otp));
    }

    // version=1, batch_size=1, batch_index=0
    payload.extend(proto_field_varint(2, 1));
    payload.extend(proto_field_varint(3, 1));
    payload.extend(proto_field_varint(4, 0));

    let b64 = B64.encode(&payload);
    let encoded = b64
        .replace('+', "%2B")
        .replace('/', "%2F")
        .replace('=', "%3D");

    Ok(format!("otpauth-migration://offline?data={encoded}"))
}

/// Generates a demo migration URI containing one account per common OTP variant,
/// useful for testing import across authenticator apps.
pub fn generate_demo_migration_uri() -> Result<String, QrError> {
    // (name, issuer, base32_secret, algorithm, digits, otp_type)
    // algorithm: 1=SHA1  2=SHA256  3=SHA512
    // digits:    1=SIX   2=EIGHT
    // otp_type:  1=HOTP  2=TOTP
    let accounts: &[(&str, &str, &str, u64, u64, u64)] = &[
        (
            "demo@example.com",
            "Demo TOTP SHA1",
            "JBSWY3DPEHPK3PXP",
            1,
            1,
            2,
        ),
        (
            "demo@example.com",
            "Demo TOTP SHA256",
            "JBSWY3DPEHPK3PXP",
            2,
            1,
            2,
        ),
        (
            "demo@example.com",
            "Demo TOTP SHA512",
            "JBSWY3DPEHPK3PXP",
            3,
            1,
            2,
        ),
        (
            "demo@example.com",
            "Demo TOTP 8-digit",
            "JBSWY3DPEHPK3PXP",
            1,
            2,
            2,
        ),
        (
            "demo2@example.com",
            "Demo HOTP",
            "MFRA22LOMFRA22LO",
            1,
            1,
            1,
        ),
    ];

    let mut payload: Vec<u8> = Vec::new();
    for (name, issuer, secret_b32, algo, digits, otp_type) in accounts {
        let secret_bytes = BASE32_NOPAD
            .decode(secret_b32.as_bytes())
            .map_err(|_| QrError::MigrationDecode)?;

        let mut otp: Vec<u8> = Vec::new();
        otp.extend(proto_field_bytes(1, &secret_bytes));
        otp.extend(proto_field_bytes(2, name.as_bytes()));
        otp.extend(proto_field_bytes(3, issuer.as_bytes()));
        otp.extend(proto_field_varint(4, *algo));
        otp.extend(proto_field_varint(5, *digits));
        otp.extend(proto_field_varint(6, *otp_type));
        payload.extend(proto_field_bytes(1, &otp));
    }
    payload.extend(proto_field_varint(2, 1));
    payload.extend(proto_field_varint(3, 1));
    payload.extend(proto_field_varint(4, 0));

    let b64 = B64.encode(&payload);
    let encoded = b64
        .replace('+', "%2B")
        .replace('/', "%2F")
        .replace('=', "%3D");
    Ok(format!("otpauth-migration://offline?data={encoded}"))
}

fn proto_encode_varint(mut val: u64) -> Vec<u8> {
    let mut buf = Vec::new();
    loop {
        let mut byte = (val & 0x7F) as u8;
        val >>= 7;
        if val != 0 {
            byte |= 0x80;
        }
        buf.push(byte);
        if val == 0 {
            break;
        }
    }
    buf
}

fn proto_field_varint(field: u32, val: u64) -> Vec<u8> {
    let tag = (field as u64) << 3;
    let mut buf = proto_encode_varint(tag);
    buf.extend(proto_encode_varint(val));
    buf
}

fn proto_field_bytes(field: u32, data: &[u8]) -> Vec<u8> {
    let tag = ((field as u64) << 3) | 2;
    let mut buf = proto_encode_varint(tag);
    buf.extend(proto_encode_varint(data.len() as u64));
    buf.extend_from_slice(data);
    buf
}

/// Renders a string as a QR code into terminal lines using Unicode half-block chars.
pub fn uri_to_qr_lines(data: &str) -> Vec<String> {
    // Use lowest error-correction level (L) to minimise the number of modules,
    // falling back to M if L fails for the given data length.
    let code = match QrCode::with_error_correction_level(data.as_bytes(), EcLevel::L)
        .or_else(|_| QrCode::with_error_correction_level(data.as_bytes(), EcLevel::M))
    {
        Ok(c) => c,
        Err(_) => return vec!["QR generation failed".to_string()],
    };
    let colors = code.to_colors();
    let width = code.width();
    let quiet = 2usize;
    let padded = width + quiet * 2;

    let is_dark = |row: isize, col: isize| -> bool {
        let (r, c) = (row as usize, col as usize);
        if row < 0 || col < 0 || r < quiet || c < quiet || r >= quiet + width || c >= quiet + width
        {
            return false;
        }
        colors[(r - quiet) * width + (c - quiet)] == QrColor::Dark
    };

    let mut lines = Vec::new();
    let mut row = 0isize;
    while row < padded as isize {
        let mut line = String::new();
        for col in 0..padded as isize {
            let top = is_dark(row, col);
            let bot = is_dark(row + 1, col);
            line.push(match (top, bot) {
                (true, true) => '█',
                (true, false) => '▀',
                (false, true) => '▄',
                (false, false) => ' ',
            });
        }
        lines.push(line);
        row += 2;
    }
    lines
}

pub fn scan_qr_uri(path: &std::path::Path) -> Result<String, QrError> {
    let img = image::open(path)
        .map_err(|e| QrError::ImageLoad(e.to_string()))?
        .to_luma8();
    let mut img = rqrr::PreparedImage::prepare(img);
    let grids = img.detect_grids();
    grids
        .into_iter()
        .find_map(|grid| grid.decode().ok().map(|(_, content)| content))
        .ok_or(QrError::NoQrFound)
}

pub fn scan_all_qr_uris(path: &std::path::Path) -> Result<Vec<String>, QrError> {
    let raw = image::open(path).map_err(|e| QrError::ImageLoad(e.to_string()))?;

    // rqrr misses QR codes when the image is too large (Retina) or when codes
    // appear at different sizes. Scan at multiple scales and deduplicate results.
    let widths: &[u32] = &[1920, 1280, 960];
    let mut seen = std::collections::HashSet::new();
    let mut uris: Vec<String> = Vec::new();

    for &max_w in widths {
        let gray = if raw.width() > max_w {
            let scale = max_w as f32 / raw.width() as f32;
            let h = (raw.height() as f32 * scale) as u32;
            raw.resize(max_w, h, image::imageops::FilterType::Lanczos3)
                .to_luma8()
        } else {
            raw.to_luma8()
        };

        let mut prepared = rqrr::PreparedImage::prepare(gray);
        for grid in prepared.detect_grids() {
            if let Ok((_, content)) = grid.decode() {
                if seen.insert(content.clone()) {
                    uris.push(content);
                }
            }
        }
    }

    if uris.is_empty() {
        Err(QrError::NoQrFound)
    } else {
        Ok(uris)
    }
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
    } else if content.starts_with("otpauth-migration://") {
        // Return first account; caller can use parse_migration for all
        parse_migration(&content).and_then(|mut v| v.pop().ok_or(QrError::MigrationDecode))
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

pub use crate::import::parse_json_bytes;

pub fn uri_to_qr_png(data: &str, path: &std::path::Path) -> Result<(), QrError> {
    use image::Luma;
    let code = QrCode::with_error_correction_level(data.as_bytes(), EcLevel::L)
        .or_else(|_| QrCode::with_error_correction_level(data.as_bytes(), EcLevel::M))
        .map_err(|e| QrError::QrGenerate(e.to_string()))?;
    let img = code
        .render::<Luma<u8>>()
        .quiet_zone(true)
        .module_dimensions(8, 8)
        .build();
    img.save(path)
        .map_err(|e| QrError::ImageLoad(e.to_string()))?;
    Ok(())
}
