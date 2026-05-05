use crate::qr::{parse_migration, OtpSecret, QrError};

/// Parse a Google Authenticator `otpauth-migration://` URI.
/// This is the format produced by Google Authenticator's "Export accounts" QR codes.
pub fn parse(uri: &str) -> Result<Vec<OtpSecret>, QrError> {
    parse_migration(uri)
}
