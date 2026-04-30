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
fn export_produces_json_file() {
    let tmp = setup();
    let out = tmp.path().join("export.json");
    rotp(&tmp).args(["export", "--output", out.to_str().unwrap()])
        .assert().success().stderr(contains("plain text"));
    let content = std::fs::read_to_string(&out).unwrap();
    assert!(content.contains("GitHub:carlo"));
    assert!(content.contains("JBSWY3DPEHPK3PXP"));
}

#[test]
fn export_json_is_valid() {
    let tmp = setup();
    let out = tmp.path().join("export.json");
    rotp(&tmp).args(["export", "--output", out.to_str().unwrap()])
        .assert().success();
    let content = std::fs::read_to_string(&out).unwrap();
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&content).unwrap();
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0]["name"], "GitHub:carlo");
}
