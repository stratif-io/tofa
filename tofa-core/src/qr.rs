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

impl OtpSecret {
    /// Build a `VaultEntry` for this OTP, applying the standard
    /// defaults (SHA1 / 6 digits / 30s period) anywhere the source
    /// didn't specify them. Every import surface — CLI, TUI, desktop
    /// app, drag-drop, file picker, paste — funnels through this so
    /// the defaults can't drift between platforms. The resulting
    /// entry's `id` is left blank; `Vault::add_entry*` generates it.
    pub fn into_vault_entry(self, name: String, created_at: String) -> crate::store::VaultEntry {
        use crate::store::{DEFAULT_ALGORITHM, DEFAULT_DIGITS, DEFAULT_PERIOD};
        crate::store::VaultEntry {
            id: String::new(),
            name,
            secret: self.secret,
            created_at,
            period: self.meta.period.unwrap_or(DEFAULT_PERIOD),
            digits: self.meta.digits.unwrap_or(DEFAULT_DIGITS),
            algorithm: self
                .meta
                .algorithm
                .unwrap_or_else(|| DEFAULT_ALGORITHM.to_string()),
        }
    }
}

/// One account to encode into a Google Authenticator migration URI.
///
/// Note: the migration protobuf has no period field, so any non-default
/// period (e.g. 60s) is silently dropped on export — Google Authenticator
/// migration assumes 30s. Algorithm and digits are preserved.
#[derive(Debug, Clone, Copy)]
pub struct MigrationAccount<'a> {
    pub name: &'a str,
    pub issuer: &'a str,
    pub secret_b32: &'a str,
    pub algorithm: &'a str,
    pub digits: u8,
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

/// Builds a single-account `otpauth://totp/...` URI from a vault entry,
/// preserving algorithm, digits, and period as query parameters. The label
/// is `Issuer:account` (or just `account` when the entry name has no colon).
/// All user-controlled fields are percent-encoded per RFC 3986 unreserved.
pub fn build_otpauth_uri(entry: &crate::store::VaultEntry) -> String {
    let (issuer, account) = OtpMeta::split_name(&entry.name);
    let label = if issuer.is_empty() {
        account.clone()
    } else {
        format!("{issuer}:{account}")
    };
    let mut uri = format!(
        "otpauth://totp/{}?secret={}",
        percent_encode(&label),
        entry.secret,
    );
    if !issuer.is_empty() {
        uri.push_str(&format!("&issuer={}", percent_encode(&issuer)));
    }
    uri.push_str(&format!("&algorithm={}", entry.algorithm));
    uri.push_str(&format!("&digits={}", entry.digits));
    uri.push_str(&format!("&period={}", entry.period));
    uri
}

fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char)
            }
            b => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Why a selection couldn't be encoded as a single QR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionExportError {
    /// No entries selected.
    Empty,
    /// Selection has multiple entries with non-30s periods. The Google
    /// Authenticator migration QR (the only widely-supported multi-account
    /// format) has no period field, so non-30s entries can't be combined
    /// with others. Caller should either deselect the offending entries,
    /// export them individually, or use a multi-otpauth list export.
    NonStandardPeriod {
        offending_count: usize,
        total: usize,
    },
    /// The migration encoder failed (e.g. an entry's stored secret isn't
    /// valid base32). Effectively unreachable for entries that came out
    /// of a tofa vault, but surfaced rather than panicked.
    EncodingFailed(String),
}

impl std::fmt::Display for SelectionExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "no entries selected"),
            Self::NonStandardPeriod {
                offending_count,
                total,
            } => write!(
                f,
                "{offending_count} of {total} selected entries use a non-30s period; \
                 the Google Authenticator migration QR cannot include them. \
                 Export those entries individually."
            ),
            Self::EncodingFailed(msg) => write!(f, "QR encoding failed: {msg}"),
        }
    }
}

impl std::error::Error for SelectionExportError {}

/// Replace the `secret` substring in an `otpauth://` URI with 16
/// bullets, returning the result unchanged if the secret can't be
/// found. Used by every detail-view surface (TUI fullscreen, desktop
/// app's masked-URI command) so the bullet count is the same on every
/// platform regardless of whether the secret is 16, 26, or 32 chars
/// long. Centralising the rule means a future change (different
/// length, different glyph) lands in one place.
pub fn mask_otpauth_uri(uri: &str, secret: &str) -> String {
    if !uri.contains(secret) {
        return uri.to_string();
    }
    uri.replacen(secret, &"•".repeat(16), 1)
}

/// Build a plain-text URI list for the given entries — one
/// `otpauth://totp/...` line per entry, no trailing newline. This is the
/// inverse of `tofa_core::import::parse_text_uris` (Ente Auth's export
/// format), so a vault → URI list → vault round trip preserves every
/// Replace path-unsafe characters in a vault entry name so it can
/// safely be a filename. Used by every QR-PNG export surface (CLI
/// `tofa qr --multi`, desktop app print/zip flow) so they all produce
/// the same filenames and a future change to the rule lands in one
/// place. Keeps ASCII letters, digits, `-`, `_`, `.`; everything
/// else (including `:`, `/`, spaces) becomes `_`.
pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' => c,
            _ => '_',
        })
        .collect()
}

/// entry. Used by the CLI (`tofa export --format uris`), TUI (export
/// screen "save as URI list"), and desktop app ("Save as URI list").
pub fn entries_to_uri_list(entries: &[crate::store::VaultEntry]) -> String {
    entries
        .iter()
        .map(build_otpauth_uri)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Encodes an export selection as a single QR-ready URI, dispatching by
/// shape:
/// - **Empty selection** → `Err(Empty)`.
/// - **Single entry** → `otpauth://...` (preserves all five fields).
/// - **Multiple all-30s entries** → `otpauth-migration://...` (Google's
///   migration format; preserves algorithm and digits, period is implicitly 30).
/// - **Multiple entries containing any non-30s** → `Err(NonStandardPeriod)`.
pub fn build_selection_uri(
    entries: &[crate::store::VaultEntry],
) -> Result<String, SelectionExportError> {
    if entries.is_empty() {
        return Err(SelectionExportError::Empty);
    }
    if entries.len() == 1 {
        return Ok(build_otpauth_uri(&entries[0]));
    }
    let offending = entries.iter().filter(|e| e.period != 30).count();
    if offending > 0 {
        return Err(SelectionExportError::NonStandardPeriod {
            offending_count: offending,
            total: entries.len(),
        });
    }
    let accounts: Vec<MigrationAccount<'_>> = entries
        .iter()
        .map(|e| MigrationAccount {
            name: e.name.as_str(),
            issuer: "",
            secret_b32: e.secret.as_str(),
            algorithm: e.algorithm.as_str(),
            digits: e.digits,
        })
        .collect();
    generate_migration_uri(&accounts)
        .map_err(|e| SelectionExportError::EncodingFailed(e.to_string()))
}

/// Builds a Google Authenticator `otpauth-migration://` URI from a list of
/// accounts. Returns the URI string ready to encode into a QR code.
///
/// Algorithm strings are mapped to the migration enum: `SHA1`/`SHA256`/`SHA512`/`MD5`
/// (case-insensitive); unknown values fall back to SHA1. Digits accept 6 or 8;
/// other values fall back to 6 — Google Authenticator migration only encodes
/// those two. The migration protobuf has no period field, so any custom period
/// is silently dropped (importers assume 30s).
pub fn generate_migration_uri(accounts: &[MigrationAccount<'_>]) -> Result<String, QrError> {
    let mut payload: Vec<u8> = Vec::new();

    for account in accounts {
        let secret_bytes = BASE32_NOPAD
            .decode(account.secret_b32.trim_end_matches('=').as_bytes())
            .map_err(|_| QrError::MigrationDecode)?;

        let mut otp: Vec<u8> = Vec::new();
        // field 1: secret bytes
        otp.extend(proto_field_bytes(1, &secret_bytes));
        // field 2: name
        otp.extend(proto_field_bytes(2, account.name.as_bytes()));
        // field 3: issuer (skip if empty)
        if !account.issuer.is_empty() {
            otp.extend(proto_field_bytes(3, account.issuer.as_bytes()));
        }
        // field 4: algorithm enum
        otp.extend(proto_field_varint(
            4,
            algorithm_to_migration_enum(account.algorithm),
        ));
        // field 5: digits enum
        otp.extend(proto_field_varint(
            5,
            digits_to_migration_enum(account.digits),
        ));
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
            "DEMOSHAONEAAAAAA",
            1,
            1,
            2,
        ),
        (
            "demo@example.com",
            "Demo TOTP SHA256",
            "DEMOSHATWOAAAAAA",
            2,
            1,
            2,
        ),
        (
            "demo@example.com",
            "Demo TOTP SHA512",
            "DEMOSHAFIVAAAAAA",
            3,
            1,
            2,
        ),
        (
            "demo@example.com",
            "Demo TOTP 8-digit",
            "DEMOEIGHTAAAAAAA",
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

fn algorithm_to_migration_enum(s: &str) -> u64 {
    match s.to_ascii_uppercase().as_str() {
        "SHA256" => 2,
        "SHA512" => 3,
        "MD5" => 4,
        _ => 1, // SHA1
    }
}

fn digits_to_migration_enum(n: u8) -> u64 {
    match n {
        8 => 2, // EIGHT
        _ => 1, // SIX
    }
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

/// Reports on a single resolution pass during a scan. Emitted after each
/// pass completes, so callers (CLI spinner, app progress UI) can surface
/// "pass @ 1920px • 7 found" feedback while the full ladder is still running.
#[derive(Debug, Clone, Copy)]
pub struct ScanProgress {
    /// Width of the image fed to the detector for this pass, in pixels.
    pub pass_width: u32,
    /// Total unique URIs decoded so far across all completed passes for
    /// this image. Monotonically non-decreasing across events.
    pub found: usize,
}

pub fn scan_all_qr_uris(path: &std::path::Path) -> Result<Vec<String>, QrError> {
    scan_all_qr_uris_with_progress(path, |_| {})
}

pub fn scan_all_qr_uris_with_progress<F: FnMut(ScanProgress)>(
    path: &std::path::Path,
    on_progress: F,
) -> Result<Vec<String>, QrError> {
    let raw = image::open(path).map_err(|e| QrError::ImageLoad(e.to_string()))?;
    scan_dynamic_image_with_progress(raw, on_progress)
}

/// Same as `scan_all_qr_uris_with_progress` but operating on an
/// already-decoded image. Used by the zip-import path so callers can
/// scan an image in memory without writing it to a tempfile first.
pub fn scan_dynamic_image_with_progress<F: FnMut(ScanProgress)>(
    raw: image::DynamicImage,
    mut on_progress: F,
) -> Result<Vec<String>, QrError> {
    // rqrr's grid detector behaves differently at different resolutions. Dense
    // QRs (long URIs) need enough pixels per module; sparse QRs at very high
    // res can carry noise that confuses detection. Cover both ends with a
    // small ladder, stopping early when nothing new is found.
    //
    // Cap the highest pass at 3840px even when the source is wider (5K Retina
    // captures are ~5120). At 3840 each 180-CSS-px QR still has ~8 pixels per
    // module — plenty for rqrr — and we avoid the ~3-5× cost of running the
    // detector on a 14M-pixel native image.
    //
    // Resize the *RGB* image and convert to luma after; Lanczos3 on RGB
    // preserves more information than on already-greyscaled pixels (recall
    // on the synthetic dense-grid drops 10/11 → 7/11 when we collapse to
    // luma before resizing).
    //
    // Filter-diversity rung: when the source is wide enough that 1920 is a
    // real resize (not native), insert an extra pass at 1920 using Triangle
    // alongside the Lanczos one. Lanczos preserves sharper edges but
    // introduces ringing; Triangle is softer with no ringing. A QR that
    // lands just below rqrr's detection threshold under one filter often
    // decodes under the other. Doing this only at 1920 (not at 1280/960)
    // is deliberate — Triangle's bilinear blur smears dense QRs at small
    // widths where each module is only 3-4 pixels (recall on the synthetic
    // dense-grid drops 10/11 → 9/11 if we use Triangle at 960). 1920 is
    // wide enough to keep modules sharp under either filter.
    use image::imageops::FilterType;
    let raw_w = raw.width();
    let mut candidates: Vec<(u32, FilterType)> = vec![
        (raw_w.min(3840), FilterType::Lanczos3),
        (1920, FilterType::Lanczos3),
        (1280, FilterType::Lanczos3),
        (960, FilterType::Lanczos3),
    ];
    if raw_w > 1920 {
        candidates.insert(2, (1920, FilterType::Triangle));
    }
    candidates.retain(|(w, _)| *w > 0 && *w <= raw_w);
    candidates.sort_by_key(|c| std::cmp::Reverse(c.0));
    candidates.dedup();

    let mut seen = std::collections::HashSet::new();
    let mut uris: Vec<String> = Vec::new();
    let mut last_count = 0usize;
    let mut unproductive = 0u8;

    for (w, filter) in candidates.iter().copied() {
        let gray = if w == raw_w {
            raw.to_luma8()
        } else {
            let h = (raw.height() as f32 * (w as f32 / raw_w as f32)) as u32;
            raw.resize(w, h, filter).to_luma8()
        };
        scan_into(gray, &mut seen, &mut uris);
        on_progress(ScanProgress {
            pass_width: w,
            found: uris.len(),
        });
        if uris.len() == last_count {
            unproductive += 1;
            if unproductive >= 2 {
                break;
            }
        } else {
            unproductive = 0;
            last_count = uris.len();
        }
    }

    if uris.is_empty() {
        Err(QrError::NoQrFound)
    } else {
        Ok(uris)
    }
}

fn scan_into(
    gray: image::ImageBuffer<image::Luma<u8>, Vec<u8>>,
    seen: &mut std::collections::HashSet<String>,
    uris: &mut Vec<String>,
) {
    let mut prepared = rqrr::PreparedImage::prepare(gray);
    for grid in prepared.detect_grids() {
        if let Ok((_, content)) = grid.decode() {
            if seen.insert(content.clone()) {
                uris.push(content);
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_filename_strips_path_separators_and_colons() {
        assert_eq!(sanitize_filename("Issuer:account"), "Issuer_account");
        assert_eq!(sanitize_filename("a/b\\c"), "a_b_c");
        assert_eq!(sanitize_filename("plain"), "plain");
        assert_eq!(sanitize_filename("with space"), "with_space");
        assert_eq!(
            sanitize_filename("dot.kept-and_under"),
            "dot.kept-and_under"
        );
    }
}
