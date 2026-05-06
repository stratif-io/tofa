/// Generates fixture QR images for all TOTP parameter combinations.
/// Run with:
///   cargo test -p tofa-core gen_fixtures -- --nocapture
use image::Luma;
use qrcode::QrCode;
use std::path::PathBuf;
use tofa_core::qr::generate_demo_migration_uri;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/qr")
        .join(name)
}

fn ga_fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/google_authenticator")
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
    // All combinations: 3 algorithms × 2 digit counts × 2 periods = 12 QR codes.
    // Each combination uses a unique base32 secret so any bug that collapses
    // accounts to a single one cannot be silently masked by the fixture.
    let algorithms = [("SHA1", "SHA1"), ("SHA256", "SHA256"), ("SHA512", "SHA512")];
    let digits_variants = [(6u8, "6"), (8u8, "8")];
    let periods = [(30u32, "30s"), (60u32, "60s")];

    // 12 unique secrets, ordered to match the iteration below.
    let secrets = [
        "QRGENAAAAAAAAAAA", // SHA1 6d 30s
        "QRGENBBBBBBBBBBB", // SHA1 6d 60s
        "QRGENCCCCCCCCCCC", // SHA1 8d 30s
        "QRGENDDDDDDDDDDD", // SHA1 8d 60s
        "QRGENEEEEEEEEEEE", // SHA256 6d 30s
        "QRGENFFFFFFFFFFF", // SHA256 6d 60s
        "QRGENGGGGGGGGGGG", // SHA256 8d 30s
        "QRGENHHHHHHHHHHH", // SHA256 8d 60s
        "QRGENIIIIIIIIIII", // SHA512 6d 30s
        "QRGENJJJJJJJJJJJ", // SHA512 6d 60s
        "QRGENKKKKKKKKKKK", // SHA512 8d 30s
        "QRGENLLLLLLLLLLL", // SHA512 8d 60s
    ];
    let mut idx = 0;

    for (algo_id, _algo_label) in &algorithms {
        for (digits, digits_label) in &digits_variants {
            for (period, period_label) in &periods {
                let secret = secrets[idx];
                idx += 1;
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

    // Also regenerate the legacy standard QR (SHA1/6d/30s, used in existing tests).
    let standard_uri = "otpauth://totp/Authelia:demo?secret=QRGENSTANDARDAAA&issuer=Authelia&algorithm=SHA1&digits=6&period=30";
    save_qr(standard_uri, fixture("demo_standard.png"));

    // The optional test_qr.png referenced by qr_test.rs. Use a secret unique to
    // that file so no two test files share a fixture secret value.
    let test_qr_uri = "otpauth://totp/QrTest:demo?secret=QRTESTUNIQUEAAAA&issuer=QrTest&algorithm=SHA1&digits=6&period=30";
    save_qr(test_qr_uri, fixture("test_qr.png"));

    // Regenerate the Google Authenticator demo migration QR from the library
    // helper so its accounts (each with a unique secret) match the in-source
    // definition in `generate_demo_migration_uri`.
    let migration_uri = generate_demo_migration_uri().expect("generate demo migration URI");
    save_qr(&migration_uri, ga_fixture("demo_migration.png"));

    println!("\nGenerated 12 combination QR codes + standard QR + GA demo migration QR.");
}
