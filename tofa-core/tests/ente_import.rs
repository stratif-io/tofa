use tofa_core::import::parse_text_uris;

fn fixture_text(name: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/ente")
        .join(name);
    std::fs::read_to_string(path).expect("fixture file not found")
}

#[test]
fn ente_parses_two_entries() {
    let text = fixture_text("ente-fake.txt");
    let entries = parse_text_uris(&text).expect("should parse Ente Auth export");
    assert_eq!(entries.len(), 2);
}

#[test]
fn ente_figma_entry() {
    let text = fixture_text("ente-fake.txt");
    let entries = parse_text_uris(&text).unwrap();
    let e = entries
        .iter()
        .find(|e| e.meta.issuer.as_deref() == Some("Figma"))
        .expect("Figma entry should be present");
    assert_eq!(e.secret, "ENTEFIGMAAAAAAAA");
    assert_eq!(e.meta.account.as_deref(), Some("grace@example.com"));
    assert_eq!(e.meta.digits, Some(6));
    assert_eq!(e.meta.period, Some(30));
}

#[test]
fn ente_gitlab_entry_has_non_default_params() {
    let text = fixture_text("ente-fake.txt");
    let entries = parse_text_uris(&text).unwrap();
    let e = entries
        .iter()
        .find(|e| e.meta.issuer.as_deref() == Some("GitLab"))
        .expect("GitLab entry should be present");
    assert_eq!(e.meta.algorithm.as_deref(), Some("SHA256"));
    assert_eq!(e.meta.digits, Some(8));
    assert_eq!(e.meta.period, Some(60));
}
