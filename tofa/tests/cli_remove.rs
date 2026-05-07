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
            "CLIREMOVEAAAAAAA",
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
fn remove_confirms_and_deletes() {
    let tmp = setup();
    tofa(&tmp)
        .args(["remove", "GitHub:carlo"])
        .write_stdin("y\n")
        .assert()
        .success()
        .stderr(contains("Removed"));
    tofa(&tmp).arg("list").assert().success().stdout("");
}

#[test]
fn remove_aborts_on_n() {
    let tmp = setup();
    tofa(&tmp)
        .args(["remove", "GitHub:carlo"])
        .write_stdin("n\n")
        .assert()
        .success()
        .stderr(contains("Aborted"));
    tofa(&tmp)
        .arg("list")
        .assert()
        .success()
        .stdout(contains("GitHub:carlo"));
}

#[test]
fn remove_not_found() {
    let tmp = setup();
    tofa(&tmp)
        .args(["remove", "nope"])
        .write_stdin("y\n")
        .assert()
        .failure()
        .stderr(contains("no account matching"));
}
