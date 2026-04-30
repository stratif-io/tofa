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

#[derive(Debug, Clone)]
pub struct OtpSecret {
    pub secret: String,
}

pub fn parse_input(input: &str) -> Result<OtpSecret, QrError> {
    let trimmed = input.trim();

    if trimmed.starts_with("otpauth://") {
        return parse_uri(trimmed);
    }

    let path = std::path::Path::new(trimmed);
    if path.exists() {
        return parse_qr_image(path);
    }

    if is_valid_base32(trimmed) {
        return Ok(OtpSecret {
            secret: trimmed.to_uppercase(),
        });
    }

    Err(QrError::UnrecognizedInput)
}

fn parse_uri(uri: &str) -> Result<OtpSecret, QrError> {
    let secret = uri
        .split('?')
        .nth(1)
        .and_then(|query| {
            query.split('&').find_map(|param| {
                let mut parts = param.splitn(2, '=');
                if parts.next() == Some("secret") {
                    parts.next()
                } else {
                    None
                }
            })
        })
        .ok_or(QrError::MissingSecret)?;

    Ok(OtpSecret {
        secret: secret.to_uppercase(),
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

fn is_valid_base32(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '2'..='7' | '='))
}
