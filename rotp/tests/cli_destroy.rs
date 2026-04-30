use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn init_vault(tmp: &TempDir) {
    Command::cargo_bin("rotp").unwrap()
        .env("ROTP_PASSPHRASE", "testpass")
        .env("ROTP_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
        .arg("init").assert().success();
}

fn rotp(tmp: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("rotp").unwrap();
    cmd.env("ROTP_PASSPHRASE", "testpass")
       .env("ROTP_VAULT", tmp.path().join("vault.enc").to_str().unwrap());
    cmd
}

#[test]
fn destroy_removes_vault() {
    let tmp = TempDir::new().unwrap();
    init_vault(&tmp);
    rotp(&tmp).arg("destroy").write_stdin("y\n").assert().success();
    assert!(!tmp.path().join("vault.enc").exists());
}

#[test]
fn destroy_aborts_on_n() {
    let tmp = TempDir::new().unwrap();
    init_vault(&tmp);
    rotp(&tmp).arg("destroy").write_stdin("n\n").assert().success();
    assert!(tmp.path().join("vault.enc").exists());
}

#[test]
fn destroy_fails_wrong_passphrase() {
    let tmp = TempDir::new().unwrap();
    init_vault(&tmp);
    Command::cargo_bin("rotp").unwrap()
        .env("ROTP_PASSPHRASE", "wrongpass")
        .env("ROTP_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
        .arg("destroy").write_stdin("y\n")
        .assert().failure().stderr(contains("wrong passphrase"));
}

#[test]
fn destroy_fails_no_vault() {
    let tmp = TempDir::new().unwrap();
    rotp(&tmp).arg("destroy").assert().failure()
        .stderr(contains("no vault"));
}
