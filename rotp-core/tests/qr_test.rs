use rotp_core::qr::parse_input;

#[test]
fn parse_raw_base32_secret() {
    let result = parse_input("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ").unwrap();
    assert_eq!(result.secret, "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ");
}

#[test]
fn parse_otpauth_uri() {
    let uri = "otpauth://totp/GitHub?secret=GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ&issuer=GitHub";
    let result = parse_input(uri).unwrap();
    assert_eq!(result.secret, "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ");
}

#[test]
fn parse_otpauth_uri_missing_secret_returns_error() {
    let uri = "otpauth://totp/GitHub?issuer=GitHub";
    assert!(parse_input(uri).is_err());
}

#[test]
fn invalid_input_returns_error() {
    assert!(parse_input("!!!notvalid!!!").is_err());
}

#[test]
fn parse_qr_image_file() {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/test_qr.png");

    if fixture.exists() {
        let result = parse_input(fixture.to_str().unwrap()).unwrap();
        assert_eq!(result.secret, "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ");
    } else {
        eprintln!("Skipping QR image test — fixture not found");
    }
}
