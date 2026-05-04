use std::path::PathBuf;
use tofa_core::{
    generate_migration_uri,
    qr::{parse_migration, scan_qr_uri, OtpSecret},
    store::VaultEntry,
};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
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

#[test]
fn migration_uri_roundtrip() {
    let original_uri = scan_migration_uri();
    let original = parse_migration(&original_uri).expect("parse original");

    let tuples: Vec<(String, String, String)> = original
        .iter()
        .map(|otp| {
            let name = otp.meta.account.clone().unwrap_or_default();
            let issuer = otp.meta.issuer.clone().unwrap_or_default();
            (name, issuer, otp.secret.clone())
        })
        .collect();

    let refs: Vec<(&str, &str, &str)> = tuples
        .iter()
        .map(|(n, i, s)| (n.as_str(), i.as_str(), s.as_str()))
        .collect();

    let generated_uri = generate_migration_uri(&refs).expect("generate URI");
    assert!(generated_uri.starts_with("otpauth-migration://"));

    let decoded = parse_migration(&generated_uri).expect("parse generated URI");
    assert_eq!(decoded.len(), original.len(), "same number of accounts");

    // generate_migration_uri doesn't preserve period/digits/algorithm yet,
    // so only verify that secrets round-trip correctly.
    for (orig, dec) in original.iter().zip(decoded.iter()) {
        assert_eq!(
            orig.secret, dec.secret,
            "secrets must match for {:?}",
            orig.meta.account
        );
    }
}

#[test]
fn migration_qr_image_roundtrip() {
    let original_uri = scan_migration_uri();
    let original = parse_migration(&original_uri).expect("parse original");

    let tuples: Vec<(String, String, String)> = original
        .iter()
        .map(|otp| {
            (
                otp.meta.account.clone().unwrap_or_default(),
                otp.meta.issuer.clone().unwrap_or_default(),
                otp.secret.clone(),
            )
        })
        .collect();

    let refs: Vec<(&str, &str, &str)> = tuples
        .iter()
        .map(|(n, i, s)| (n.as_str(), i.as_str(), s.as_str()))
        .collect();

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
