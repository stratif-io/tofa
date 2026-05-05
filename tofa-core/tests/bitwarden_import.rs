use tofa_core::qr::parse_json_bytes;

fn fixture_bytes(name: &str) -> Vec<u8> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/bitwarden")
        .join(name);
    std::fs::read(path).expect("fixture file not found")
}

#[test]
fn bitwarden_parses_three_entries() {
    let bytes = fixture_bytes("bitwarden-fake.json");
    let entries = parse_json_bytes(&bytes).expect("should parse Bitwarden file");
    assert_eq!(entries.len(), 3, "non-login item must be skipped");
}

#[test]
fn bitwarden_github_entry() {
    let bytes = fixture_bytes("bitwarden-fake.json");
    let entries = parse_json_bytes(&bytes).unwrap();
    let e = entries
        .iter()
        .find(|e| e.meta.issuer.as_deref() == Some("GitHub"))
        .expect("GitHub entry should be present");
    assert_eq!(e.secret, "JBSWY3DPEHPK3PXP");
    assert_eq!(e.meta.account.as_deref(), Some("carlo@example.com"));
    assert_eq!(e.meta.digits, Some(6));
    assert_eq!(e.meta.period, Some(30));
}

#[test]
fn bitwarden_aws_entry_has_non_default_params() {
    let bytes = fixture_bytes("bitwarden-fake.json");
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
fn bitwarden_bare_secret_entry() {
    let bytes = fixture_bytes("bitwarden-fake.json");
    let entries = parse_json_bytes(&bytes).unwrap();
    let e = entries
        .iter()
        .find(|e| e.meta.issuer.as_deref() == Some("BareSecret"))
        .expect("BareSecret entry should be present");
    assert_eq!(e.secret, "JBSWY3DPEHPK3PXP");
}
