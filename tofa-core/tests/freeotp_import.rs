use tofa_core::qr::parse_json_bytes;

fn fixture_bytes(name: &str) -> Vec<u8> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/freeotp")
        .join(name);
    std::fs::read(path).expect("fixture file not found")
}

#[test]
fn freeotp_parses_two_totp_entries() {
    let bytes = fixture_bytes("freeotp-fake.json");
    let entries = parse_json_bytes(&bytes).expect("should parse FreeOTP+ file");
    assert_eq!(entries.len(), 2, "HOTP entry must be skipped");
}

#[test]
fn freeotp_github_entry_secret_decoded() {
    let bytes = fixture_bytes("freeotp-fake.json");
    let entries = parse_json_bytes(&bytes).unwrap();
    let e = entries
        .iter()
        .find(|e| e.meta.issuer.as_deref() == Some("GitHub"))
        .expect("GitHub entry should be present");
    // Signed bytes [72,101,108,108,111,33,-34,-83,-66,-17] decode to JBSWY3DPEHPK3PXP
    assert_eq!(e.secret, "JBSWY3DPEHPK3PXP");
    assert_eq!(e.meta.account.as_deref(), Some("carlo@example.com"));
    assert_eq!(e.meta.digits, Some(6));
    assert_eq!(e.meta.period, Some(30));
}

#[test]
fn freeotp_aws_entry_has_non_default_params() {
    let bytes = fixture_bytes("freeotp-fake.json");
    let entries = parse_json_bytes(&bytes).unwrap();
    let e = entries
        .iter()
        .find(|e| e.meta.issuer.as_deref() == Some("AWS"))
        .expect("AWS entry should be present");
    assert_eq!(e.meta.digits, Some(8));
    assert_eq!(e.meta.period, Some(60));
}

#[test]
fn freeotp_hotp_entry_is_skipped() {
    let bytes = fixture_bytes("freeotp-fake.json");
    let entries = parse_json_bytes(&bytes).unwrap();
    assert!(
        entries
            .iter()
            .all(|e| e.meta.issuer.as_deref() != Some("ShouldBeIgnored")),
        "HOTP entry must not appear in parsed output"
    );
}
