use tofa_core::qr::{mask_otpauth_uri, parse_input, OtpMeta, OtpSecret};

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

#[test]
fn mask_otpauth_uri_replaces_secret_with_fixed_16_bullets() {
    // Always 16 bullets so the masked URI's shape doesn't leak the
    // secret's length, regardless of whether the source is 16-char
    // (RFC default) or 32-char (some authenticators).
    let bullets = "•".repeat(16);

    // 16-char secret
    let uri = "otpauth://totp/Discord:bob?secret=DISCORDFAKEAAAAA&issuer=Discord";
    let masked = mask_otpauth_uri(uri, "DISCORDFAKEAAAAA");
    assert_eq!(
        masked,
        format!("otpauth://totp/Discord:bob?secret={bullets}&issuer=Discord")
    );

    // 32-char secret — same number of bullets
    let long = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let uri32 = format!("otpauth://totp/Vault:user?secret={long}&issuer=Vault");
    let masked32 = mask_otpauth_uri(&uri32, long);
    assert_eq!(
        masked32,
        format!("otpauth://totp/Vault:user?secret={bullets}&issuer=Vault")
    );
}

#[test]
fn mask_otpauth_uri_only_replaces_first_occurrence() {
    // If the secret happens to appear elsewhere in the URI (shouldn't,
    // but be defensive), we only mask the actual `secret=` value once.
    // `replacen(_, _, 1)` is the right primitive — pin it.
    let uri = "otpauth://totp/Issuer:account?secret=AAAAAAAAAAAAAAAA&note=AAAAAAAAAAAAAAAA";
    let masked = mask_otpauth_uri(uri, "AAAAAAAAAAAAAAAA");
    let bullets = "•".repeat(16);
    assert_eq!(
        masked,
        format!("otpauth://totp/Issuer:account?secret={bullets}&note=AAAAAAAAAAAAAAAA")
    );
}

#[test]
fn into_vault_entry_applies_standard_defaults_when_meta_is_empty() {
    // RFC 6238 / Google Authenticator defaults: SHA1 / 6 digits / 30s.
    // Every import path used to spell these out inline (`unwrap_or(30)`,
    // `unwrap_or(6)`, `unwrap_or_else(|| "SHA1".to_string())`) — pin
    // the rule centrally so a future change lands in one place.
    let otp = OtpSecret {
        secret: "DEFAULTTESTAAAAA".to_string(),
        meta: OtpMeta::default(),
    };
    let entry = otp.into_vault_entry("Discord:bob".to_string(), "2026-05-07".to_string());
    assert_eq!(entry.name, "Discord:bob");
    assert_eq!(entry.secret, "DEFAULTTESTAAAAA");
    assert_eq!(entry.created_at, "2026-05-07");
    assert_eq!(entry.algorithm, "SHA1");
    assert_eq!(entry.digits, 6);
    assert_eq!(entry.period, 30);
    assert!(
        entry.id.is_empty(),
        "id is generated on add_entry, not here"
    );
}

#[test]
fn into_vault_entry_preserves_explicit_meta() {
    // 8 digits, 60s period, SHA256 — not common, but the migration
    // protobuf and otpauth URI both can carry them. The conversion
    // must not silently downgrade.
    let otp = OtpSecret {
        secret: "EXPLICITTESTAAAA".to_string(),
        meta: OtpMeta {
            issuer: Some("Vault".to_string()),
            account: Some("admin".to_string()),
            algorithm: Some("SHA256".to_string()),
            digits: Some(8),
            period: Some(60),
        },
    };
    let entry = otp.into_vault_entry("Vault:admin".to_string(), "2026-05-07".to_string());
    assert_eq!(entry.algorithm, "SHA256");
    assert_eq!(entry.digits, 8);
    assert_eq!(entry.period, 60);
}

#[test]
fn mask_otpauth_uri_returns_input_unchanged_when_secret_absent() {
    // Defensive: if the caller hands us a URI that doesn't contain the
    // claimed secret, don't panic — return the URI as-is. The display
    // path can decide how to handle the inconsistency.
    let uri = "otpauth://totp/Issuer:account?secret=DIFFERENT&issuer=Issuer";
    let masked = mask_otpauth_uri(uri, "MISMATCH");
    assert_eq!(masked, uri);
}
