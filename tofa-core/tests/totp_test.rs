use tofa_core::{
    store::VaultEntry,
    totp::{generate_code_at, seconds_remaining},
};

fn entry(secret: &str) -> VaultEntry {
    VaultEntry::new("test".into(), secret.into(), "2026-01-01".into())
}

fn entry_with(secret: &str, period: u32, digits: u8, algorithm: &str) -> VaultEntry {
    let base = entry(secret);
    VaultEntry {
        period,
        digits,
        algorithm: algorithm.into(),
        name: base.name.clone(),
        secret: base.secret.clone(),
        created_at: base.created_at.clone(),
    }
}

// RFC 6238 test vector: secret = "12345678901234567890" (ASCII), SHA1, 30s window
// Base32: GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ
// At T=59: code = 287082
#[test]
fn rfc6238_vector_t59() {
    let code = generate_code_at(&entry("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ"), 59).unwrap();
    assert_eq!(code, "287082");
}

#[test]
fn code_is_six_digits() {
    let code = generate_code_at(&entry("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ"), 0).unwrap();
    assert_eq!(code.len(), 6);
    assert!(code.chars().all(|c| c.is_ascii_digit()));
}

#[test]
fn invalid_secret_returns_error() {
    let result = generate_code_at(&entry("!!!INVALID!!!"), 0);
    assert!(result.is_err());
}

#[test]
fn seconds_remaining_30s_period() {
    let e = entry("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ");
    assert!(seconds_remaining(&e, 0) <= 30);
    assert_eq!(seconds_remaining(&e, 30), 30);
    assert_eq!(seconds_remaining(&e, 59), 1);
    assert_eq!(seconds_remaining(&e, 60), 30);
}

#[test]
fn seconds_remaining_60s_period() {
    let e = entry_with("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ", 60, 6, "SHA1");
    assert_eq!(seconds_remaining(&e, 60), 60);
    assert_eq!(seconds_remaining(&e, 119), 1);
    assert_eq!(seconds_remaining(&e, 120), 60);
}

#[test]
fn custom_period_generates_different_code() {
    let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
    let e30 = entry_with(secret, 30, 6, "SHA1");
    let e60 = entry_with(secret, 60, 6, "SHA1");
    // At T=0 the 30s and 60s windows differ in step count → different codes
    let code30 = generate_code_at(&e30, 30).unwrap();
    let code60 = generate_code_at(&e60, 30).unwrap();
    assert_ne!(code30, code60);
}

#[test]
fn eight_digit_code_has_correct_length() {
    let e = entry_with("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ", 30, 8, "SHA1");
    let code = generate_code_at(&e, 0).unwrap();
    assert_eq!(code.len(), 8);
}

#[test]
fn sha256_generates_code() {
    // SHA-256 secret — just check it produces a 6-digit string without error
    let e = entry_with(
        "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ",
        30,
        6,
        "SHA256",
    );
    let code = generate_code_at(&e, 0).unwrap();
    assert_eq!(code.len(), 6);
    assert!(code.chars().all(|c| c.is_ascii_digit()));
}
