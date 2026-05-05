use tofa_core::import::parse_csv;

fn fixture_text(name: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/keepassxc")
        .join(name);
    std::fs::read_to_string(path).expect("fixture file not found")
}

#[test]
fn keepassxc_parses_three_totp_entries() {
    let csv = fixture_text("keepassxc-fake.csv");
    let entries = parse_csv(&csv).expect("should parse KeePassXC CSV");
    assert_eq!(
        entries.len(),
        3,
        "entry without TOTP column must be skipped"
    );
}

#[test]
fn keepassxc_shopify_entry() {
    let csv = fixture_text("keepassxc-fake.csv");
    let entries = parse_csv(&csv).unwrap();
    let e = entries
        .iter()
        .find(|e| e.meta.issuer.as_deref() == Some("Shopify"))
        .expect("Shopify entry should be present");
    assert_eq!(e.secret, "JBSWY3DPEHPK3PXP");
    assert_eq!(e.meta.account.as_deref(), Some("henry@example.com"));
    assert_eq!(e.meta.digits, Some(6));
    assert_eq!(e.meta.period, Some(30));
}

#[test]
fn keepassxc_flyio_entry_has_non_default_params() {
    let csv = fixture_text("keepassxc-fake.csv");
    let entries = parse_csv(&csv).unwrap();
    let e = entries
        .iter()
        .find(|e| e.meta.issuer.as_deref() == Some("Fly.io"))
        .expect("Fly.io entry should be present");
    assert_eq!(e.meta.algorithm.as_deref(), Some("SHA256"));
    assert_eq!(e.meta.digits, Some(8));
    assert_eq!(e.meta.period, Some(60));
}

#[test]
fn keepassxc_bare_secret_entry() {
    let csv = fixture_text("keepassxc-fake.csv");
    let entries = parse_csv(&csv).unwrap();
    let e = entries
        .iter()
        .find(|e| e.meta.issuer.as_deref() == Some("DigitalOcean"))
        .expect("DigitalOcean entry should be present");
    assert_eq!(e.secret, "JBSWY3DPEHPK3PXP");
}
