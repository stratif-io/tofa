use tofa_core::qr::{build_otpauth_uri, entries_to_uri_list, parse_input};
use tofa_core::store::VaultEntry;

fn entry(name: &str, secret: &str, algorithm: &str, digits: u8, period: u32) -> VaultEntry {
    VaultEntry {
        id: String::new(),
        name: name.to_string(),
        secret: secret.to_string(),
        created_at: String::new(),
        period,
        digits,
        algorithm: algorithm.to_string(),
    }
}

#[test]
fn otpauth_roundtrip_preserves_period_algorithm_digits() {
    let e = entry(
        "GitHub:alice@example.com",
        "OAUTHROUNDFULLAA",
        "SHA256",
        8,
        60,
    );
    let uri = build_otpauth_uri(&e);
    assert!(uri.starts_with("otpauth://totp/"), "uri: {uri}");

    let parsed = parse_input(&uri).expect("parse uri");
    assert_eq!(parsed.secret, "OAUTHROUNDFULLAA");
    assert_eq!(parsed.meta.issuer.as_deref(), Some("GitHub"));
    assert_eq!(parsed.meta.account.as_deref(), Some("alice@example.com"));
    assert_eq!(parsed.meta.algorithm.as_deref(), Some("SHA256"));
    assert_eq!(parsed.meta.digits, Some(8));
    assert_eq!(parsed.meta.period, Some(60));
}

#[test]
fn otpauth_roundtrip_with_no_issuer_in_name() {
    let e = entry("alice@example.com", "OAUTHROUNDNOISSU", "SHA1", 6, 30);
    let uri = build_otpauth_uri(&e);
    let parsed = parse_input(&uri).expect("parse uri");
    assert_eq!(parsed.meta.issuer, None);
    assert_eq!(parsed.meta.account.as_deref(), Some("alice@example.com"));
    assert_eq!(parsed.meta.algorithm.as_deref(), Some("SHA1"));
    assert_eq!(parsed.meta.digits, Some(6));
    assert_eq!(parsed.meta.period, Some(30));
}

#[test]
fn entries_to_uri_list_round_trips_through_parse_text_uris() {
    // The export-as-URI-list flow writes the result of this fn to a
    // .txt file. The unified import dispatcher routes .txt through
    // parse_text_uris (Ente Auth format), so the round trip must
    // preserve every entry's secret / period / digits / algorithm.
    let entries = vec![
        entry("GitHub:alice", "AAAAAAAAAAAAAAAA", "SHA1", 6, 30),
        entry("Vercel:bob@acme.io", "BBBBBBBBBBBBBBBB", "SHA256", 8, 60),
        entry("Discord:eve", "CCCCCCCCCCCCCCCC", "SHA1", 6, 30),
    ];
    let text = entries_to_uri_list(&entries);

    // No trailing newline — it's a join, not a list-with-terminator.
    assert!(
        !text.ends_with('\n'),
        "URI list should not have trailing newline"
    );

    let parsed = tofa_core::import::parse_text_uris(&text).expect("round-trip via parse_text_uris");
    assert_eq!(parsed.len(), entries.len());
    for (orig, got) in entries.iter().zip(parsed.iter()) {
        assert_eq!(got.secret, orig.secret);
        assert_eq!(got.meta.period, Some(orig.period));
        assert_eq!(got.meta.digits, Some(orig.digits));
        assert_eq!(got.meta.algorithm.as_deref(), Some(orig.algorithm.as_str()));
    }
}

#[test]
fn entries_to_uri_list_empty_input_is_empty_string() {
    assert_eq!(entries_to_uri_list(&[]), "");
}

#[test]
fn otpauth_roundtrip_handles_special_characters_in_name() {
    let e = entry(
        "Issuer With Spaces:user+tag@example.com",
        "OAUTHROUNDSPECAA",
        "SHA1",
        6,
        30,
    );
    let uri = build_otpauth_uri(&e);
    let parsed = parse_input(&uri).expect("parse uri");
    assert_eq!(parsed.meta.issuer.as_deref(), Some("Issuer With Spaces"));
    assert_eq!(parsed.meta.account.as_deref(), Some("user+tag@example.com"));
}
