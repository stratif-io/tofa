/// Generates fixture QR images for all TOTP parameter combinations.
/// Run with:
///   cargo test -p tofa-core gen_fixtures -- --nocapture
use image::Luma;
use qrcode::QrCode;
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/qr")
        .join(name)
}

fn save_qr(uri: &str, path: PathBuf) {
    let code = QrCode::new(uri.as_bytes()).expect("QrCode::new");
    let img = code
        .render::<Luma<u8>>()
        .quiet_zone(true)
        .module_dimensions(8, 8)
        .build();
    img.save(&path).expect("save PNG");
    println!("Saved {}  ({})", path.display(), uri);
}

#[test]
fn gen_fixtures() {
    // All combinations: 3 algorithms × 2 digit counts × 2 periods = 12 QR codes
    let algorithms = [("SHA1", "SHA1"), ("SHA256", "SHA256"), ("SHA512", "SHA512")];
    let digits_variants = [(6u8, "6"), (8u8, "8")];
    let periods = [(30u32, "30s"), (60u32, "60s")];
    let secret = "JBSWY3DPEHPK3PXP";

    for (algo_id, _algo_label) in &algorithms {
        for (digits, digits_label) in &digits_variants {
            for (period, period_label) in &periods {
                let name = format!("Demo TOTP {algo_id} {digits_label}d {period_label}");
                let uri = format!(
                    "otpauth://totp/{name}:demo@example.com?secret={secret}&issuer={name}&algorithm={algo_id}&digits={digits}&period={period}"
                );
                let filename = format!(
                    "demo_{}_{}d_{}.png",
                    algo_id.to_lowercase(),
                    digits_label,
                    period_label
                );
                save_qr(&uri, fixture(&filename));
            }
        }
    }

    // Also regenerate the legacy standard QR (SHA1/6d/30s, used in existing tests)
    let standard_uri = "otpauth://totp/Authelia:demo?secret=JBSWY3DPEHPK3PXP&issuer=Authelia&algorithm=SHA1&digits=6&period=30";
    save_qr(standard_uri, fixture("demo_standard.png"));

    println!("\nGenerated 12 combination QR codes + 1 legacy standard QR.");
}
