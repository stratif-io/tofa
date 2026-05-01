use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn init_vault() -> TempDir {
    let tmp = TempDir::new().unwrap();
    Command::cargo_bin("tofa").unwrap()
        .env("TOFA_PASSPHRASE", "testpass")
        .env("TOFA_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
        .arg("init").assert().success();
    tmp
}

fn tofa(tmp: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("tofa").unwrap();
    cmd.env("TOFA_PASSPHRASE", "testpass")
       .env("TOFA_VAULT", tmp.path().join("vault.enc").to_str().unwrap());
    cmd
}

#[test]
fn add_with_secret_and_name() {
    let tmp = init_vault();
    tofa(&tmp)
        .args(["add", "--name", "GitHub:carlo", "--secret", "JBSWY3DPEHPK3PXP"])
        .assert().success().stdout(contains("Added GitHub:carlo"));
}

#[test]
fn add_with_uri() {
    let tmp = init_vault();
    tofa(&tmp)
        .args(["add", "--uri", "otpauth://totp/GitHub:carlo?secret=JBSWY3DPEHPK3PXP&issuer=GitHub"])
        .assert().success().stdout(contains("Added"));
}

#[test]
fn add_secret_without_name_fails() {
    let tmp = init_vault();
    tofa(&tmp)
        .args(["add", "--secret", "JBSWY3DPEHPK3PXP"])
        .assert().failure().stderr(contains("--name"));
}

#[test]
fn add_no_args_fails() {
    let tmp = init_vault();
    tofa(&tmp)
        .arg("add")
        .assert().failure();
}
