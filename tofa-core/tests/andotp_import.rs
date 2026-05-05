use tofa_core::qr::parse_json_bytes;

fn fixture_bytes(name: &str) -> Vec<u8> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/andotp")
        .join(name);
    std::fs::read(path).expect("fixture file not found")
}

#[test]
fn andotp_parses_three_totp_entries() {
    let bytes = fixture_bytes("andotp-fake.json");
    let entries = parse_json_bytes(&bytes).expect("should parse andOTP file");
    assert_eq!(entries.len(), 3, "HOTP entry must be skipped");
}

#[test]
fn andotp_dropbox_entry() {
    let bytes = fixture_bytes("andotp-fake.json");
    let entries = parse_json_bytes(&bytes).unwrap();
    let e = entries
        .iter()
        .find(|e| e.meta.issuer.as_deref() == Some("Dropbox"))
        .expect("Dropbox entry should be present");
    assert_eq!(e.secret, "JBSWY3DPEHPK3PXP");
    assert_eq!(e.meta.account.as_deref(), Some("alice@example.com"));
    assert_eq!(e.meta.algorithm.as_deref(), Some("SHA1"));
    assert_eq!(e.meta.digits, Some(6));
    assert_eq!(e.meta.period, Some(30));
}

#[test]
fn andotp_cloudflare_entry_has_non_default_params() {
    let bytes = fixture_bytes("andotp-fake.json");
    let entries = parse_json_bytes(&bytes).unwrap();
    let e = entries
        .iter()
        .find(|e| e.meta.issuer.as_deref() == Some("Cloudflare"))
        .expect("Cloudflare entry should be present");
    assert_eq!(e.meta.algorithm.as_deref(), Some("SHA256"));
    assert_eq!(e.meta.digits, Some(8));
    assert_eq!(e.meta.period, Some(60));
}

#[test]
fn andotp_hotp_entry_is_skipped() {
    let bytes = fixture_bytes("andotp-fake.json");
    let entries = parse_json_bytes(&bytes).unwrap();
    assert!(
        entries
            .iter()
            .all(|e| e.meta.issuer.as_deref() != Some("ShouldBeIgnored")),
        "HOTP entry must not appear in parsed output"
    );
}
