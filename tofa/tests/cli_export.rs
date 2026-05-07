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
            "CLIEXPORTAAAAAAA",
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
fn export_produces_json_file() {
    let tmp = setup();
    let out = tmp.path().join("export.json");
    tofa(&tmp)
        .args(["export", "--output", out.to_str().unwrap()])
        .assert()
        .success()
        .stderr(contains("plain text"));
    let content = std::fs::read_to_string(&out).unwrap();
    assert!(content.contains("GitHub:carlo"));
    assert!(content.contains("CLIEXPORTAAAAAAA"));
}

#[test]
fn export_json_is_valid() {
    let tmp = setup();
    let out = tmp.path().join("export.json");
    tofa(&tmp)
        .args(["export", "--output", out.to_str().unwrap()])
        .assert()
        .success();
    let content = std::fs::read_to_string(&out).unwrap();
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&content).unwrap();
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0]["name"], "GitHub:carlo");
}

#[test]
fn export_format_uris_writes_one_otpauth_per_line() {
    // The new --format uris flag writes a plain-text file that the
    // unified import dispatcher accepts back via `tofa import`. Round
    // trip the full vault through it and assert the entry survives.
    let tmp = setup();
    let out = tmp.path().join("export.txt");
    tofa(&tmp)
        .args([
            "export",
            "--format",
            "uris",
            "--output",
            out.to_str().unwrap(),
        ])
        .assert()
        .success();
    let content = std::fs::read_to_string(&out).unwrap();
    assert!(
        content.starts_with("otpauth://totp/"),
        "expected URI list, got: {content:?}"
    );
    assert!(content.contains("CLIEXPORTAAAAAAA"));

    // Round-trip: import the .txt into a fresh vault and assert the
    // entry survives.
    let tmp2 = TempDir::new().unwrap();
    Command::cargo_bin("tofa")
        .unwrap()
        .env("TOFA_PASSPHRASE", "testpass")
        .env(
            "TOFA_VAULT",
            tmp2.path().join("vault.enc").to_str().unwrap(),
        )
        .arg("init")
        .assert()
        .success();
    Command::cargo_bin("tofa")
        .unwrap()
        .env("TOFA_PASSPHRASE", "testpass")
        .env(
            "TOFA_VAULT",
            tmp2.path().join("vault.enc").to_str().unwrap(),
        )
        .args(["import", out.to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("Imported 1"));
}
