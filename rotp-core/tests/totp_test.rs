use rotp_core::totp::{generate_code, seconds_remaining};

// RFC 6238 test vector: secret = "12345678901234567890" (ASCII), SHA1, 30s window
// Base32 of that ASCII string: GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ
// At T=59: code = 287082
#[test]
fn rfc6238_vector_t59() {
    let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
    let code = generate_code(secret, 59).unwrap();
    assert_eq!(code, "287082");
}

#[test]
fn code_is_six_digits() {
    // "12345678901234567890" in Base32 — same as RFC vector, 160-bit secret
    let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
    let code = generate_code(secret, 0).unwrap();
    assert_eq!(code.len(), 6);
    assert!(code.chars().all(|c| c.is_ascii_digit()));
}

#[test]
fn invalid_secret_returns_error() {
    let result = generate_code("!!!INVALID!!!", 0);
    assert!(result.is_err());
}

#[test]
fn seconds_remaining_in_range() {
    let secs = seconds_remaining(0);
    assert!(secs <= 30);
}

#[test]
fn seconds_remaining_at_boundary() {
    assert_eq!(seconds_remaining(30), 30);
    assert_eq!(seconds_remaining(59), 1);
    assert_eq!(seconds_remaining(60), 30);
}
