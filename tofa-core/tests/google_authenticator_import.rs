use tofa_core::import::parse_migration_uri;
use tofa_core::qr::scan_qr_uri;

fn fixture_uri() -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/google_authenticator/demo_migration.png");
    scan_qr_uri(&path).expect("should decode migration QR image")
}

#[test]
fn ga_migration_parses_four_accounts() {
    let uri = fixture_uri();
    let entries = parse_migration_uri(&uri).expect("should parse migration URI");
    assert_eq!(entries.len(), 4);
}

#[test]
fn ga_migration_totp_sha1_account() {
    let uri = fixture_uri();
    let entries = parse_migration_uri(&uri).unwrap();
    let e = entries
        .iter()
        .find(|e| e.meta.issuer.as_deref() == Some("Demo TOTP SHA1"))
        .expect("Demo TOTP SHA1 should be present");
    assert_eq!(e.secret, "DEMOSHAONEAAAAAA");
}

#[test]
fn ga_migration_hotp_is_excluded() {
    let uri = fixture_uri();
    let entries = parse_migration_uri(&uri).unwrap();
    assert!(
        entries
            .iter()
            .all(|e| e.meta.issuer.as_deref() != Some("Demo HOTP")),
        "HOTP entry must be excluded"
    );
}
