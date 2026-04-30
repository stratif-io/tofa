use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn setup() -> TempDir {
    let tmp = TempDir::new().unwrap();
    Command::cargo_bin("rotp").unwrap()
        .env("ROTP_PASSPHRASE", "testpass")
        .env("ROTP_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
        .arg("init").assert().success();
    for (name, secret) in &[
        ("GitHub:carlo", "JBSWY3DPEHPK3PXP"),
        ("Authelia:carlo", "JBSWY3DPEHPK3PXQ"),
    ] {
        Command::cargo_bin("rotp").unwrap()
            .env("ROTP_PASSPHRASE", "testpass")
            .env("ROTP_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
            .args(["add", "--name", name, "--secret", secret])
            .assert().success();
    }
    tmp
}

fn rotp(tmp: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("rotp").unwrap();
    cmd.env("ROTP_PASSPHRASE", "testpass")
       .env("ROTP_VAULT", tmp.path().join("vault.enc").to_str().unwrap());
    cmd
}

#[test]
fn list_shows_names() {
    let tmp = setup();
    rotp(&tmp).arg("list").assert().success()
        .stdout(contains("GitHub:carlo"))
        .stdout(contains("Authelia:carlo"));
}

#[test]
fn list_codes_shows_digits() {
    let tmp = setup();
    let out = rotp(&tmp).args(["list", "--codes"]).assert().success();
    let stdout = String::from_utf8(out.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("GitHub:carlo"));
    assert!(stdout.chars().any(|c| c.is_ascii_digit()));
}

#[test]
fn list_empty_vault_exits_ok() {
    let tmp = TempDir::new().unwrap();
    Command::cargo_bin("rotp").unwrap()
        .env("ROTP_PASSPHRASE", "testpass")
        .env("ROTP_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
        .arg("init").assert().success();
    Command::cargo_bin("rotp").unwrap()
        .env("ROTP_PASSPHRASE", "testpass")
        .env("ROTP_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
        .arg("list").assert().success().stdout("");
}
