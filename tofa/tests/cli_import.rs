use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn setup() -> TempDir {
    let tmp = TempDir::new().unwrap();
    Command::cargo_bin("tofa")
        .unwrap()
        .env("TOFA_PASSPHRASE", "testpass")
        .env("TOFA_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
        .arg("init")
        .assert()
        .success();
    tmp
}

fn tofa(tmp: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("tofa").unwrap();
    cmd.env("TOFA_PASSPHRASE", "testpass")
        .env("TOFA_VAULT", tmp.path().join("vault.enc").to_str().unwrap());
    cmd
}

#[test]
fn import_json_roundtrip() {
    let tmp_a = TempDir::new().unwrap();
    Command::cargo_bin("tofa")
        .unwrap()
        .env("TOFA_PASSPHRASE", "testpass")
        .env(
            "TOFA_VAULT",
            tmp_a.path().join("vault.enc").to_str().unwrap(),
        )
        .arg("init")
        .assert()
        .success();
    Command::cargo_bin("tofa")
        .unwrap()
        .env("TOFA_PASSPHRASE", "testpass")
        .env(
            "TOFA_VAULT",
            tmp_a.path().join("vault.enc").to_str().unwrap(),
        )
        .args([
            "add",
            "--name",
            "GitHub:carlo",
            "--secret",
            "CLIIMPORTAAAAAAA",
        ])
        .assert()
        .success();

    let export_file = tmp_a.path().join("export.json");
    Command::cargo_bin("tofa")
        .unwrap()
        .env("TOFA_PASSPHRASE", "testpass")
        .env(
            "TOFA_VAULT",
            tmp_a.path().join("vault.enc").to_str().unwrap(),
        )
        .args(["export", "--output", export_file.to_str().unwrap()])
        .assert()
        .success();

    let tmp_b = setup();
    tofa(&tmp_b)
        .args(["import", export_file.to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("Imported 1"));

    tofa(&tmp_b)
        .arg("list")
        .assert()
        .success()
        .stdout(contains("GitHub:carlo"));
}

/// Path to a tofa-core test fixture, resolved via CARGO_MANIFEST_DIR
/// so it works whether tests are run from the workspace root or from
/// inside `tofa/`.
fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .join("tofa-core/tests/fixtures")
        .join(rel)
}

#[test]
fn import_png_with_single_otpauth_qr_succeeds() {
    // Pre-unified-import the CLI rejected single-account QR images
    // with "Use 'tofa add --qr'". Now `import` accepts them via the
    // shared dispatcher. Use the demo standard QR fixture which encodes
    // a known otpauth:// URI.
    let tmp = setup();
    let png = fixture("qr/demo_standard.png");
    assert!(png.is_file(), "fixture missing: {}", png.display());
    tofa(&tmp)
        .args(["import", png.to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("Imported 1"));
}

#[test]
fn import_png_with_migration_qr_imports_every_account() {
    // demo_migration.png is a Google-Authenticator export QR with
    // multiple accounts encoded inside. The CLI's import command
    // should expand it into N entries in one pass.
    let tmp = setup();
    let png = fixture("google_authenticator/demo_migration.png");
    assert!(png.is_file(), "fixture missing: {}", png.display());
    let assertion = tofa(&tmp)
        .args(["import", png.to_str().unwrap()])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assertion.get_output().stdout).to_string();
    assert!(
        stdout.contains("Imported "),
        "expected import success line, got: {stdout}"
    );
    // The fixture has at least 2 accounts; allow >= so a regenerated
    // fixture with more accounts doesn't break the test.
    assert!(
        !stdout.contains("Imported 1 account") && !stdout.contains("Imported 0"),
        "expected multiple accounts from migration QR, got: {stdout}"
    );
}

#[test]
fn import_zip_of_qr_images_decodes_every_entry() {
    // Build a .zip in the test temp dir containing the existing single
    // QR fixture, then verify `tofa import` walks the archive and
    // imports the QR. Uses zip's stored (no compression) variant so
    // there's no flake risk from deflate non-determinism.
    let tmp = setup();
    let png_bytes = std::fs::read(fixture("qr/demo_standard.png")).expect("read fixture");

    let zip_path = tmp.path().join("backup.zip");
    let mut writer = zip::ZipWriter::new(std::fs::File::create(&zip_path).expect("create zip"));
    let opts: zip::write::SimpleFileOptions =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    use std::io::Write;
    writer.start_file("qr.png", opts).expect("start");
    writer.write_all(&png_bytes).expect("write");
    writer.finish().expect("finish");

    tofa(&tmp)
        .args(["import", zip_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("Imported 1"));
}

#[test]
fn import_txt_with_otpauth_uri_list_imports_each_line() {
    // The .txt branch routes through parse_text_uris (Ente Auth's
    // newline-separated otpauth:// list format). Two URIs in, two
    // entries out.
    let tmp = setup();
    let text = "otpauth://totp/Discord:bob?secret=DISCORDFAKEAAAAA&issuer=Discord&algorithm=SHA1&digits=6&period=30\n\
                otpauth://totp/GitHub:bob?secret=GITHUBFAKEAAAAAA&issuer=GitHub&algorithm=SHA1&digits=6&period=30\n";
    let txt_path = tmp.path().join("uris.txt");
    std::fs::write(&txt_path, text).expect("write txt");

    tofa(&tmp)
        .args(["import", txt_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("Imported 2"));
}

#[test]
fn import_skips_duplicates() {
    let tmp = setup();
    tofa(&tmp)
        .args([
            "add",
            "--name",
            "GitHub:carlo",
            "--secret",
            "CLIIMPORTAAAAAAA",
        ])
        .assert()
        .success();

    let json = r#"[{"name":"GitHub:carlo","secret":"CLIIMPORTAAAAAAA","created_at":"2026-01-01"}]"#;
    let f = tmp.path().join("dup.json");
    std::fs::write(&f, json).unwrap();

    tofa(&tmp)
        .args(["import", f.to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("Skipped 1"));
}
