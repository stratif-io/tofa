use std::path::PathBuf;
use tofa_core::generate_demo_migration_uri;

/// Generates the demo migration QR and saves it as a fixture PNG.
/// Run with: cargo test -p rotp-core generate_demo_qr -- --nocapture
#[test]
fn generate_demo_qr() {
    let uri = generate_demo_migration_uri().expect("generate demo URI");
    assert!(uri.starts_with("otpauth-migration://"));

    let out = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/demo_migration.png");

    use image::Luma;
    use qrcode::QrCode;
    let code = QrCode::new(uri.as_bytes()).expect("QrCode::new");
    let img = code
        .render::<Luma<u8>>()
        .quiet_zone(true)
        .module_dimensions(8, 8)
        .build();
    img.save(&out).expect("save demo QR PNG");

    println!("Demo migration QR saved to {}", out.display());
    println!("URI: {uri}");
}
