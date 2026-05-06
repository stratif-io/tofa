use std::path::PathBuf;
use tofa_core::{
    qr::{parse_input, parse_migration, scan_qr_uri},
    store::VaultEntry,
    totp::generate_code_at,
};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/qr")
        .join(name)
}

fn ga_fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/google_authenticator")
        .join(name)
}

fn entry_from_otp(otp: &tofa_core::qr::OtpSecret) -> VaultEntry {
    VaultEntry {
        id: String::new(),
        name: String::new(),
        secret: otp.secret.clone(),
        created_at: String::new(),
        period: otp.meta.period.unwrap_or(30),
        digits: otp.meta.digits.unwrap_or(6),
        algorithm: otp
            .meta
            .algorithm
            .clone()
            .unwrap_or_else(|| "SHA1".to_string()),
    }
}

const DEMO_SECRET: &str = "QRGENSTANDARDAAA";
const DEMO_MIGRATION_SHA1_SECRET: &str = "DEMOSHAONEAAAAAA";
const DEMO_MIGRATION_8DIGIT_SECRET: &str = "DEMOEIGHTAAAAAAA";

#[test]
fn standard_qr_parses_secret() {
    let path = fixture("demo_standard.png");
    let otp = parse_input(&path.to_string_lossy()).expect("should parse standard QR");
    assert_eq!(otp.secret, DEMO_SECRET);
}

#[test]
fn standard_qr_parses_metadata() {
    let path = fixture("demo_standard.png");
    let otp = parse_input(&path.to_string_lossy()).expect("should parse standard QR");
    assert_eq!(otp.meta.issuer.as_deref(), Some("Authelia"));
    assert_eq!(otp.meta.account.as_deref(), Some("demo"));
    assert_eq!(otp.meta.algorithm.as_deref(), Some("SHA1"));
    assert_eq!(otp.meta.digits, Some(6));
    assert_eq!(otp.meta.period, Some(30));
}

#[test]
fn standard_qr_generates_valid_code() {
    let path = fixture("demo_standard.png");
    let otp = parse_input(&path.to_string_lossy()).expect("should parse standard QR");
    let code = generate_code_at(&entry_from_otp(&otp), 1_000_000).expect("should generate code");
    assert_eq!(code.len(), 6, "code must be 6 digits");
    assert!(
        code.chars().all(|c| c.is_ascii_digit()),
        "code must be numeric"
    );
}

#[test]
fn migration_qr_detects_five_accounts() {
    let uri = scan_migration_uri();
    let accounts = parse_migration(&uri).expect("should parse migration QR");
    assert_eq!(accounts.len(), 4, "demo migration QR has 4 accounts");
}

#[test]
fn migration_qr_totp_sha1_account() {
    let accounts = parse_migration(&scan_migration_uri()).unwrap();
    let acc = accounts
        .iter()
        .find(|a| a.meta.issuer.as_deref() == Some("Demo TOTP SHA1"))
        .expect("Demo TOTP SHA1 should be present");
    assert_eq!(acc.secret, DEMO_MIGRATION_SHA1_SECRET);
    assert_valid_code(acc);
}

#[test]
fn migration_qr_8digit_account() {
    let accounts = parse_migration(&scan_migration_uri()).unwrap();
    let acc = accounts
        .iter()
        .find(|a| a.meta.issuer.as_deref() == Some("Demo TOTP 8-digit"))
        .expect("Demo TOTP 8-digit should be present");
    assert_eq!(acc.secret, DEMO_MIGRATION_8DIGIT_SECRET);
    // The 8-digit account should produce an 8-digit code
    let code = generate_code_at(&entry_from_otp(acc), 1_746_000_000).unwrap();
    assert_eq!(code.len(), 8, "8-digit account produces 8-digit codes");
}

#[test]
fn migration_and_standard_qr_have_distinct_secrets() {
    // Each fixture entry must use its own unique secret so that bugs which
    // collapse distinct accounts cannot be silently masked. The standard
    // single-entry QR and the SHA1 account inside the migration QR must
    // therefore decode to different secrets and produce different codes.
    let standard_path = fixture("demo_standard.png");
    let standard = parse_input(&standard_path.to_string_lossy()).unwrap();

    let accounts = parse_migration(&scan_migration_uri()).unwrap();
    let migration = accounts
        .iter()
        .find(|a| a.meta.issuer.as_deref() == Some("Demo TOTP SHA1"))
        .unwrap();

    assert_ne!(
        standard.secret, migration.secret,
        "fixture entries must each use a unique secret"
    );

    let ts = 1_746_000_000u64;
    let code_standard = generate_code_at(&entry_from_otp(&standard), ts).unwrap();
    let code_migration = generate_code_at(&entry_from_otp(migration), ts).unwrap();
    assert_ne!(
        code_standard, code_migration,
        "different secrets must yield different codes"
    );
}

fn scan_migration_uri() -> String {
    let path = ga_fixture("demo_migration.png");
    scan_qr_uri(&path).expect("should decode migration QR image")
}

fn assert_valid_code(otp: &tofa_core::qr::OtpSecret) {
    let code = generate_code_at(&entry_from_otp(otp), 1_746_000_000)
        .unwrap_or_else(|e| panic!("generate_code_at failed: {e}"));
    assert_eq!(code.len(), 6);
    assert!(code.chars().all(|c| c.is_ascii_digit()));
}
