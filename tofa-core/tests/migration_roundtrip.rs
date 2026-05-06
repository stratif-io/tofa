use std::path::PathBuf;
use tofa_core::{
    generate_migration_uri,
    qr::{parse_migration, scan_qr_uri, MigrationAccount, OtpSecret},
    store::VaultEntry,
};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/google_authenticator")
        .join(name)
}

fn entry_from_otp(otp: &OtpSecret) -> VaultEntry {
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

fn build_accounts(otps: &[OtpSecret]) -> Vec<(String, String, String, String, u8)> {
    otps.iter()
        .map(|otp| {
            (
                otp.meta.account.clone().unwrap_or_default(),
                otp.meta.issuer.clone().unwrap_or_default(),
                otp.secret.clone(),
                otp.meta
                    .algorithm
                    .clone()
                    .unwrap_or_else(|| "SHA1".to_string()),
                otp.meta.digits.unwrap_or(6),
            )
        })
        .collect()
}

fn refs_from(owned: &[(String, String, String, String, u8)]) -> Vec<MigrationAccount<'_>> {
    owned
        .iter()
        .map(|(n, i, s, a, d)| MigrationAccount {
            name: n.as_str(),
            issuer: i.as_str(),
            secret_b32: s.as_str(),
            algorithm: a.as_str(),
            digits: *d,
        })
        .collect()
}

#[test]
fn migration_uri_roundtrip() {
    let original_uri = scan_migration_uri();
    let original = parse_migration(&original_uri).expect("parse original");

    let owned = build_accounts(&original);
    let refs = refs_from(&owned);

    let generated_uri = generate_migration_uri(&refs).expect("generate URI");
    assert!(generated_uri.starts_with("otpauth-migration://"));

    let decoded = parse_migration(&generated_uri).expect("parse generated URI");
    assert_eq!(decoded.len(), original.len(), "same number of accounts");

    for (orig, dec) in original.iter().zip(decoded.iter()) {
        assert_eq!(
            orig.secret, dec.secret,
            "secrets must match for {:?}",
            orig.meta.account
        );
        assert_eq!(
            orig.meta.algorithm, dec.meta.algorithm,
            "algorithm must round-trip for {:?}",
            orig.meta.account
        );
        assert_eq!(
            orig.meta.digits, dec.meta.digits,
            "digits must round-trip for {:?}",
            orig.meta.account
        );
    }
}

#[test]
fn migration_uri_preserves_per_account_algorithm_and_digits() {
    // Two accounts that share a secret but differ in algorithm/digits — the
    // exact case where hardcoding algorithm=SHA1/digits=6 caused all imported
    // entries to display the same TOTP code.
    let owned = vec![
        (
            "demo@example.com".to_string(),
            "Issuer A".to_string(),
            "JBSWY3DPEHPK3PXP".to_string(),
            "SHA256".to_string(),
            6u8,
        ),
        (
            "demo@example.com".to_string(),
            "Issuer B".to_string(),
            "JBSWY3DPEHPK3PXP".to_string(),
            "SHA1".to_string(),
            8u8,
        ),
    ];
    let refs = refs_from(&owned);

    let uri = generate_migration_uri(&refs).expect("generate URI");
    let decoded = parse_migration(&uri).expect("parse URI");
    assert_eq!(decoded.len(), 2);

    assert_eq!(decoded[0].meta.algorithm.as_deref(), Some("SHA256"));
    assert_eq!(decoded[0].meta.digits, Some(6));
    assert_eq!(decoded[1].meta.algorithm.as_deref(), Some("SHA1"));
    assert_eq!(decoded[1].meta.digits, Some(8));
}

#[test]
fn migration_qr_image_roundtrip() {
    let original_uri = scan_migration_uri();
    let original = parse_migration(&original_uri).expect("parse original");

    let owned = build_accounts(&original);
    let refs = refs_from(&owned);

    let uri = generate_migration_uri(&refs).expect("generate URI");

    let tmp = tempfile::NamedTempFile::with_suffix(".png").expect("tmp file");
    render_uri_to_png(&uri, tmp.path());

    let decoded_uri = scan_qr_uri(tmp.path()).expect("decode QR image");
    assert_eq!(decoded_uri, uri, "scanned URI must match generated URI");

    let decoded = parse_migration(&decoded_uri).expect("parse decoded migration");
    assert_eq!(decoded.len(), original.len());

    for (orig, dec) in original.iter().zip(decoded.iter()) {
        assert_eq!(orig.secret, dec.secret);
    }
}

fn scan_migration_uri() -> String {
    let path = fixture("demo_migration.png");
    scan_qr_uri(&path).expect("decode migration QR fixture")
}

fn render_uri_to_png(uri: &str, path: &std::path::Path) {
    use image::Luma;
    use qrcode::QrCode;
    let code = QrCode::new(uri.as_bytes()).expect("QrCode::new");
    let img = code
        .render::<Luma<u8>>()
        .quiet_zone(true)
        .module_dimensions(8, 8)
        .build();
    img.save(path).expect("save QR PNG");
}
