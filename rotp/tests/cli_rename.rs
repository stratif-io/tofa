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
fn rename_happy_path() {
    let tmp = setup();
    rotp(&tmp).args(["rename", "GitHub:carlo", "GitHub:work"])
        .assert().success().stdout(contains("GitHub:work"));
    rotp(&tmp).arg("list").assert().success().stdout(contains("GitHub:work"));
}

#[test]
fn rename_not_found() {
    let tmp = setup();
    rotp(&tmp).args(["rename", "nope", "something"])
        .assert().failure().stderr(contains("no account matching"));
}

#[test]
fn rename_duplicate_name_fails() {
    let tmp = setup();
    rotp(&tmp).args(["add", "--name", "Authelia:carlo", "--secret", "JBSWY3DPEHPK3PXQ"])
        .assert().success();
    rotp(&tmp).args(["rename", "GitHub:carlo", "Authelia:carlo"])
        .assert().failure().stderr(contains("already exists"));
}
