use crate::qr::{parse_uri, OtpMeta, OtpSecret};

/// Parse a KeePassXC CSV export. Expects the standard column layout:
/// `Group,Title,Username,Password,URL,Notes,TOTP,Icon,Modified Time,Created`
/// The `TOTP` column may hold an `otpauth://` URI or a bare base32 secret.
pub fn parse(csv: &str) -> Result<Vec<OtpSecret>, String> {
    let mut lines = csv.lines();

    // Consume header row
    let header = lines.next().ok_or("KeePassXC CSV: empty file")?;
    let totp_col =
        find_column(header, "TOTP").ok_or("KeePassXC CSV: no 'TOTP' column in header")?;
    let title_col = find_column(header, "Title").unwrap_or(1);

    let mut otps = Vec::new();
    for line in lines {
        let fields = split_csv_line(line);
        let totp = fields.get(totp_col).map(|s| s.as_str()).unwrap_or("");
        if totp.is_empty() {
            continue;
        }

        let otp = if totp.starts_with("otpauth://") {
            parse_uri(totp).map_err(|e| e.to_string())?
        } else {
            let title = fields.get(title_col).cloned().unwrap_or_default();
            OtpSecret {
                secret: totp.to_uppercase(),
                meta: OtpMeta {
                    issuer: if title.is_empty() { None } else { Some(title) },
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
        Err("No TOTP entries found in KeePassXC CSV export.".to_string())
    } else {
        Ok(otps)
    }
}

fn find_column(header: &str, name: &str) -> Option<usize> {
    split_csv_line(header)
        .into_iter()
        .position(|col| col.eq_ignore_ascii_case(name))
}

/// Minimal RFC 4180 CSV field splitter: handles double-quoted fields with
/// embedded commas and escaped quotes (`""`).
fn split_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' if in_quotes => {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    field.push('"');
                } else {
                    in_quotes = false;
                }
            }
            '"' => in_quotes = true,
            ',' if !in_quotes => {
                fields.push(field.clone());
                field.clear();
            }
            _ => field.push(c),
        }
    }
    fields.push(field);
    fields
}
