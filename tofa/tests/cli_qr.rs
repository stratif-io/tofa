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
    Command::cargo_bin("tofa")
        .unwrap()
        .env("TOFA_PASSPHRASE", "testpass")
        .env("TOFA_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
        .args([
            "add",
            "--name",
            "GitHub:carlo",
            "--secret",
            "JBSWY3DPEHPK3PXP",
        ])
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
fn qr_terminal_output_is_non_empty() {
    let tmp = setup();
    let out = tofa(&tmp).args(["qr", "GitHub:carlo"]).assert().success();
    let stdout = String::from_utf8(out.get_output().stdout.clone()).unwrap();
    assert!(!stdout.trim().is_empty());
}

#[test]
fn qr_output_png() {
    let tmp = setup();
    let out_file = tmp.path().join("qr.png");
    tofa(&tmp)
        .args(["qr", "GitHub:carlo", "--output", out_file.to_str().unwrap()])
        .assert()
        .success();
    assert!(out_file.exists());
    assert!(out_file.metadata().unwrap().len() > 100);
}

#[test]
fn qr_all_terminal() {
    let tmp = setup();
    let out = tofa(&tmp).args(["qr", "--all"]).assert().success();
    let stdout = String::from_utf8(out.get_output().stdout.clone()).unwrap();
    assert!(!stdout.trim().is_empty());
}

#[test]
fn qr_all_output_png() {
    let tmp = setup();
    let out_file = tmp.path().join("migration.png");
    tofa(&tmp)
        .args(["qr", "--all", "--output", out_file.to_str().unwrap()])
        .assert()
        .success();
    assert!(out_file.exists());
}

#[test]
fn qr_name_and_all_exclusive() {
    let tmp = setup();
    tofa(&tmp)
        .args(["qr", "GitHub:carlo", "--all"])
        .assert()
        .failure();
}

#[test]
fn qr_single_entry_preserves_full_otpauth_params() {
    // Hand-craft a vault containing one entry with non-default period,
    // algorithm, and digits. Run `tofa qr <name>` to export it as a PNG,
    // then scan the PNG back and verify all five fields round-trip.
    let tmp = TempDir::new().unwrap();
    let vault_path = tmp.path().join("vault.enc");
    let pass = "testpass";

    let mut vault = tofa_core::store::Vault::new();
    vault.add_entry(tofa_core::store::VaultEntry {
        id: String::new(),
        name: "GitHub:custom@example.com".to_string(),
        secret: "JBSWY3DPEHPK3PXP".to_string(),
        created_at: "2026-01-01".to_string(),
        period: 60,
        digits: 8,
        algorithm: "SHA256".to_string(),
    });
    vault.save(&vault_path, pass).unwrap();

    let out_png = tmp.path().join("qr.png");
    Command::cargo_bin("tofa")
        .unwrap()
        .env("TOFA_PASSPHRASE", pass)
        .env("TOFA_VAULT", vault_path.to_str().unwrap())
        .args(["qr", "custom", "--output", out_png.to_str().unwrap()])
        .assert()
        .success();

    let scanned_uri = tofa_core::qr::scan_qr_uri(&out_png).expect("scan QR");
    let parsed = tofa_core::qr::parse_input(&scanned_uri).expect("parse URI");

    assert_eq!(parsed.secret, "JBSWY3DPEHPK3PXP");
    assert_eq!(parsed.meta.issuer.as_deref(), Some("GitHub"));
    assert_eq!(parsed.meta.account.as_deref(), Some("custom@example.com"));
    assert_eq!(parsed.meta.algorithm.as_deref(), Some("SHA256"));
    assert_eq!(parsed.meta.digits, Some(8));
    assert_eq!(parsed.meta.period, Some(60));
}
