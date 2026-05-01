/// Generates fake fixture QR images. Run once with:
///   cargo test -p rotp-core gen_fixtures -- --nocapture
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
}

#[test]
fn gen_fixtures() {
    use image::Luma;
    use qrcode::QrCode;

    // Standard otpauth:// QR — fake Authelia account
    let standard_uri = "otpauth://totp/Authelia:demo?secret=JBSWY3DPEHPK3PXP&issuer=Authelia&algorithm=SHA1&digits=6&period=30";
    let code = QrCode::new(standard_uri.as_bytes()).unwrap();
    let img = code.render::<Luma<u8>>().quiet_zone(true).module_dimensions(8, 8).build();
    img.save(fixture("demo_standard.png")).unwrap();
    println!("demo_standard.png saved  (uri={})", standard_uri);
}
