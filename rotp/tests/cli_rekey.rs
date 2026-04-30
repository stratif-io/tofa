use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn setup() -> TempDir {
    let tmp = TempDir::new().unwrap();
    Command::cargo_bin("rotp").unwrap()
        .env("ROTP_PASSPHRASE", "testpass")
        .env("ROTP_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
        .arg("init").assert().success();
    tmp
}

#[test]
fn rekey_changes_passphrase() {
    let tmp = setup();
    Command::cargo_bin("rotp").unwrap()
        .env("ROTP_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
        .env("ROTP_PASSPHRASE", "testpass")
        .env("ROTP_NEW_PASSPHRASE", "newpass")
        .arg("rekey").assert().success().stderr(contains("updated"));

    Command::cargo_bin("rotp").unwrap()
        .env("ROTP_PASSPHRASE", "testpass")
        .env("ROTP_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
        .arg("list").assert().failure().stderr(contains("wrong passphrase"));

    Command::cargo_bin("rotp").unwrap()
        .env("ROTP_PASSPHRASE", "newpass")
        .env("ROTP_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
        .arg("list").assert().success();
}

#[test]
fn rekey_wrong_current_passphrase() {
    let tmp = setup();
    Command::cargo_bin("rotp").unwrap()
        .env("ROTP_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
        .env("ROTP_PASSPHRASE", "wrongpass")
        .env("ROTP_NEW_PASSPHRASE", "newpass")
        .arg("rekey").assert().failure().stderr(contains("wrong passphrase"));
}
