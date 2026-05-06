use tofa_core::qr::{build_otpauth_uri, parse_input};
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
        "JBSWY3DPEHPK3PXP",
        "SHA256",
        8,
        60,
    );
    let uri = build_otpauth_uri(&e);
    assert!(uri.starts_with("otpauth://totp/"), "uri: {uri}");

    let parsed = parse_input(&uri).expect("parse uri");
    assert_eq!(parsed.secret, "JBSWY3DPEHPK3PXP");
    assert_eq!(parsed.meta.issuer.as_deref(), Some("GitHub"));
    assert_eq!(parsed.meta.account.as_deref(), Some("alice@example.com"));
    assert_eq!(parsed.meta.algorithm.as_deref(), Some("SHA256"));
    assert_eq!(parsed.meta.digits, Some(8));
    assert_eq!(parsed.meta.period, Some(60));
}

#[test]
fn otpauth_roundtrip_with_no_issuer_in_name() {
    let e = entry("alice@example.com", "JBSWY3DPEHPK3PXP", "SHA1", 6, 30);
    let uri = build_otpauth_uri(&e);
    let parsed = parse_input(&uri).expect("parse uri");
    assert_eq!(parsed.meta.issuer, None);
    assert_eq!(parsed.meta.account.as_deref(), Some("alice@example.com"));
    assert_eq!(parsed.meta.algorithm.as_deref(), Some("SHA1"));
    assert_eq!(parsed.meta.digits, Some(6));
    assert_eq!(parsed.meta.period, Some(30));
}

#[test]
fn otpauth_roundtrip_handles_special_characters_in_name() {
    let e = entry(
        "Issuer With Spaces:user+tag@example.com",
        "JBSWY3DPEHPK3PXP",
        "SHA1",
        6,
        30,
    );
    let uri = build_otpauth_uri(&e);
    let parsed = parse_input(&uri).expect("parse uri");
    assert_eq!(parsed.meta.issuer.as_deref(), Some("Issuer With Spaces"));
    assert_eq!(parsed.meta.account.as_deref(), Some("user+tag@example.com"));
}
