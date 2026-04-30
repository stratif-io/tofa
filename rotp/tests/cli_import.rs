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

fn rotp(tmp: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("rotp").unwrap();
    cmd.env("ROTP_PASSPHRASE", "testpass")
       .env("ROTP_VAULT", tmp.path().join("vault.enc").to_str().unwrap());
    cmd
}

#[test]
fn import_json_roundtrip() {
    let tmp_a = TempDir::new().unwrap();
    Command::cargo_bin("rotp").unwrap()
        .env("ROTP_PASSPHRASE", "testpass")
        .env("ROTP_VAULT", tmp_a.path().join("vault.enc").to_str().unwrap())
        .arg("init").assert().success();
    Command::cargo_bin("rotp").unwrap()
        .env("ROTP_PASSPHRASE", "testpass")
        .env("ROTP_VAULT", tmp_a.path().join("vault.enc").to_str().unwrap())
        .args(["add", "--name", "GitHub:carlo", "--secret", "JBSWY3DPEHPK3PXP"])
        .assert().success();

    let export_file = tmp_a.path().join("export.json");
    Command::cargo_bin("rotp").unwrap()
        .env("ROTP_PASSPHRASE", "testpass")
        .env("ROTP_VAULT", tmp_a.path().join("vault.enc").to_str().unwrap())
        .args(["export", "--output", export_file.to_str().unwrap()])
        .assert().success();

    let tmp_b = setup();
    rotp(&tmp_b).args(["import", export_file.to_str().unwrap()])
        .assert().success().stdout(contains("Imported 1"));

    rotp(&tmp_b).arg("list")
        .assert().success().stdout(contains("GitHub:carlo"));
}

#[test]
fn import_skips_duplicates() {
    let tmp = setup();
    rotp(&tmp).args(["add", "--name", "GitHub:carlo", "--secret", "JBSWY3DPEHPK3PXP"])
        .assert().success();

    let json = r#"[{"name":"GitHub:carlo","secret":"JBSWY3DPEHPK3PXP","created_at":"2026-01-01"}]"#;
    let f = tmp.path().join("dup.json");
    std::fs::write(&f, json).unwrap();

    rotp(&tmp).args(["import", f.to_str().unwrap()])
        .assert().success().stdout(contains("Skipped 1"));
}
