use tofa_core::qr::parse_json_bytes;

fn fixture_bytes(name: &str) -> Vec<u8> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/aegis")
        .join(name);
    std::fs::read(path).expect("fixture file not found")
}

#[test]
fn aegis_parses_four_totp_entries() {
    let bytes = fixture_bytes("aegis-fake.json");
    let entries = parse_json_bytes(&bytes).expect("should parse Aegis file");
    assert_eq!(entries.len(), 4, "HOTP entry must be skipped");
}

#[test]
fn aegis_github_entry() {
    let bytes = fixture_bytes("aegis-fake.json");
    let entries = parse_json_bytes(&bytes).unwrap();
    let e = entries
        .iter()
        .find(|e| e.meta.issuer.as_deref() == Some("GitHub"))
        .expect("GitHub entry should be present");
    assert_eq!(e.secret, "JBSWY3DPEHPK3PXP");
    assert_eq!(e.meta.account.as_deref(), Some("carlo@example.com"));
    assert_eq!(e.meta.algorithm.as_deref(), Some("SHA1"));
    assert_eq!(e.meta.digits, Some(6));
    assert_eq!(e.meta.period, Some(30));
}

#[test]
fn aegis_aws_entry_has_non_default_params() {
    let bytes = fixture_bytes("aegis-fake.json");
    let entries = parse_json_bytes(&bytes).unwrap();
    let e = entries
        .iter()
        .find(|e| e.meta.issuer.as_deref() == Some("AWS"))
        .expect("AWS entry should be present");
    assert_eq!(e.meta.algorithm.as_deref(), Some("SHA256"));
    assert_eq!(e.meta.digits, Some(8));
    assert_eq!(e.meta.period, Some(60));
}

#[test]
fn aegis_hotp_entry_is_skipped() {
    let bytes = fixture_bytes("aegis-fake.json");
    let entries = parse_json_bytes(&bytes).unwrap();
    assert!(
        entries
            .iter()
            .all(|e| e.meta.issuer.as_deref() != Some("ShouldBeIgnored")),
        "HOTP entry must not appear in parsed output"
    );
}

#[test]
fn aegis_encrypted_returns_error() {
    let json =
        br#"{"version":1,"header":{"slots":[{"type":1}],"params":{}},"db":{"is_locked":true}}"#;
    let err = parse_json_bytes(json).unwrap_err();
    assert!(err.contains("encrypted"), "error should mention encryption");
}
