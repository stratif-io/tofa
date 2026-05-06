use tofa_core::qr::parse_input;

#[test]
fn parse_raw_base32_secret() {
    let result = parse_input("QRTESTRAWSECRETA").unwrap();
    assert_eq!(result.secret, "QRTESTRAWSECRETA");
}

#[test]
fn parse_otpauth_uri() {
    let uri = "otpauth://totp/GitHub?secret=QRTESTOTPAUTHURI&issuer=GitHub";
    let result = parse_input(uri).unwrap();
    assert_eq!(result.secret, "QRTESTOTPAUTHURI");
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
    let fixture =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/qr/test_qr.png");

    if fixture.exists() {
        let result = parse_input(fixture.to_str().unwrap()).unwrap();
        assert_eq!(result.secret, "QRTESTUNIQUEAAAA");
    } else {
        eprintln!("Skipping QR image test — fixture not found");
    }
}
