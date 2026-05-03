use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn init_vault(tmp: &TempDir) {
    Command::cargo_bin("tofa")
        .unwrap()
        .env("TOFA_PASSPHRASE", "testpass")
        .env("TOFA_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
        .arg("init")
        .assert()
        .success();
}

fn tofa(tmp: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("tofa").unwrap();
    cmd.env("TOFA_PASSPHRASE", "testpass")
        .env("TOFA_VAULT", tmp.path().join("vault.enc").to_str().unwrap());
    cmd
}

#[test]
fn destroy_removes_vault() {
    let tmp = TempDir::new().unwrap();
    init_vault(&tmp);
    tofa(&tmp)
        .arg("destroy")
        .write_stdin("y\n")
        .assert()
        .success();
    assert!(!tmp.path().join("vault.enc").exists());
}

#[test]
fn destroy_aborts_on_n() {
    let tmp = TempDir::new().unwrap();
    init_vault(&tmp);
    tofa(&tmp)
        .arg("destroy")
        .write_stdin("n\n")
        .assert()
        .success();
    assert!(tmp.path().join("vault.enc").exists());
}

#[test]
fn destroy_fails_wrong_passphrase() {
    let tmp = TempDir::new().unwrap();
    init_vault(&tmp);
    Command::cargo_bin("tofa")
        .unwrap()
        .env("TOFA_PASSPHRASE", "wrongpass")
        .env("TOFA_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
        .arg("destroy")
        .write_stdin("y\n")
        .assert()
        .failure()
        .stderr(contains("wrong passphrase"));
}

#[test]
fn destroy_fails_no_vault() {
    let tmp = TempDir::new().unwrap();
    tofa(&tmp)
        .arg("destroy")
        .assert()
        .failure()
        .stderr(contains("no vault"));
}
