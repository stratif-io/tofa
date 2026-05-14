pub mod crypto;
pub mod import;
pub mod qr;
pub mod store;
pub mod totp;

pub use crypto::CryptoError;
pub use qr::{
    build_otpauth_uri, build_selection_uri, generate_demo_migration_uri, generate_migration_uri,
    uri_to_qr_lines, uri_to_qr_png, MigrationAccount, OtpSecret, QrError, SelectionExportError,
};
pub use store::{today_iso, Vault, VaultEntry, DEFAULT_ALGORITHM, DEFAULT_DIGITS, DEFAULT_PERIOD};
pub use totp::{
    generate_code_at, generate_code_now, seconds_remaining, seconds_remaining_now, TotpError,
};

/// Crate version, surfaced to consumers (e.g. the Mac app's About panel).
///
/// ```
/// assert!(!tofa_core::VERSION.is_empty());
/// assert_eq!(tofa_core::VERSION.split('.').count(), 3);
/// ```
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Vendored `jsQR.min.js` (v1.4.0). Embedded so the camera-scan flow never
/// has to fetch a third-party CDN at runtime — see AUDIT.md FINDING 1.
pub const JSQR_MIN_JS: &str = include_str!("../vendor/jsQR.min.js");
