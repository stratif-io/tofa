use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn setup() -> TempDir {
    let tmp = TempDir::new().unwrap();
    Command::cargo_bin("rotp").unwrap()
        .env("ROTP_PASSPHRASE", "testpass")
        .env("ROTP_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
        .arg("init").assert().success();
    Command::cargo_bin("rotp").unwrap()
        .env("ROTP_PASSPHRASE", "testpass")
        .env("ROTP_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
        .args(["add", "--name", "GitHub:carlo", "--secret", "JBSWY3DPEHPK3PXP"])
        .assert().success();
    tmp
}

fn rotp(tmp: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("rotp").unwrap();
    cmd.env("ROTP_PASSPHRASE", "testpass")
       .env("ROTP_VAULT", tmp.path().join("vault.enc").to_str().unwrap());
    cmd
}

#[test]
fn qr_terminal_output_is_non_empty() {
    let tmp = setup();
    let out = rotp(&tmp).args(["qr", "GitHub:carlo"]).assert().success();
    let stdout = String::from_utf8(out.get_output().stdout.clone()).unwrap();
    assert!(!stdout.trim().is_empty());
}

#[test]
fn qr_output_png() {
    let tmp = setup();
    let out_file = tmp.path().join("qr.png");
    rotp(&tmp).args(["qr", "GitHub:carlo", "--output", out_file.to_str().unwrap()])
        .assert().success();
    assert!(out_file.exists());
    assert!(out_file.metadata().unwrap().len() > 100);
}

#[test]
fn qr_all_terminal() {
    let tmp = setup();
    let out = rotp(&tmp).args(["qr", "--all"]).assert().success();
    let stdout = String::from_utf8(out.get_output().stdout.clone()).unwrap();
    assert!(!stdout.trim().is_empty());
}

#[test]
fn qr_all_output_png() {
    let tmp = setup();
    let out_file = tmp.path().join("migration.png");
    rotp(&tmp).args(["qr", "--all", "--output", out_file.to_str().unwrap()])
        .assert().success();
    assert!(out_file.exists());
}

#[test]
fn qr_name_and_all_exclusive() {
    let tmp = setup();
    rotp(&tmp).args(["qr", "GitHub:carlo", "--all"])
        .assert().failure();
}
