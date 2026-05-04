use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn tofa(tmp: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("tofa").unwrap();
    cmd.env("TOFA_PASSPHRASE", "testpass")
        .env("TOFA_VAULT", tmp.path().join("vault.enc").to_str().unwrap());
    cmd
}

#[test]
fn init_creates_vault() {
    let tmp = TempDir::new().unwrap();
    tofa(&tmp).arg("init").assert().success();
    assert!(tmp.path().join("vault.enc").exists());
}

#[test]
fn init_fails_if_vault_exists() {
    let tmp = TempDir::new().unwrap();
    tofa(&tmp).arg("init").assert().success();
    tofa(&tmp)
        .arg("init")
        .assert()
        .failure()
        .stderr(contains("already exists"));
}

#[test]
fn init_creates_parent_dirs() {
    let tmp = TempDir::new().unwrap();
    let vault = tmp.path().join("a").join("b").join("vault.enc");
    Command::cargo_bin("tofa")
        .unwrap()
        .env("TOFA_PASSPHRASE", "testpass")
        .env("TOFA_VAULT", vault.to_str().unwrap())
        .arg("init")
        .assert()
        .success();
    assert!(vault.exists());
}
