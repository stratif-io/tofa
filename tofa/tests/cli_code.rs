use assert_cmd::Command;
use predicates::str::{contains, is_match};
use predicates::Predicate;
use std::time::Duration;
use tempfile::TempDir;

fn strip_ansi(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // skip until 'm'
            for ch in chars.by_ref() {
                if ch == 'm' {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

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
            "CLICODEPRIMARYAA",
        ])
        .assert()
        .success();
    Command::cargo_bin("tofa")
        .unwrap()
        .env("TOFA_PASSPHRASE", "testpass")
        .env("TOFA_VAULT", tmp.path().join("vault.enc").to_str().unwrap())
        .args([
            "add",
            "--name",
            "GitHub:perso",
            "--secret",
            "CLICODESECONDAAA",
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
fn code_exact_substring() {
    let tmp = setup();
    let out = tofa(&tmp).args(["code", "GitHub:carlo"]).assert().success();
    let stdout = String::from_utf8(out.get_output().stdout.clone()).unwrap();
    let clean = strip_ansi(&stdout);
    assert!(
        is_match(r"^\d{3} \d{3}\n$").unwrap().eval(clean.as_str()),
        "stdout was: {clean:?}"
    );
}

#[test]
fn code_raw_flag() {
    let tmp = setup();
    let out = tofa(&tmp)
        .args(["code", "GitHub:carlo", "--raw"])
        .assert()
        .success();
    let stdout = String::from_utf8(out.get_output().stdout.clone()).unwrap();
    let clean = strip_ansi(&stdout);
    let trimmed = clean.trim();
    assert_eq!(trimmed.len(), 6, "stdout was: {trimmed:?}");
    assert!(trimmed.chars().all(|c| c.is_ascii_digit()));
}

#[test]
fn code_not_found() {
    let tmp = setup();
    tofa(&tmp)
        .args(["code", "nope"])
        .assert()
        .failure()
        .stderr(contains("no account matching"));
}

#[test]
fn code_ambiguous() {
    let tmp = setup();
    tofa(&tmp)
        .args(["code", "github"])
        .assert()
        .failure()
        .stderr(contains("matches multiple"));
}

#[test]
fn code_watch_produces_output() {
    let tmp = setup();
    tofa(&tmp)
        .args(["code", "GitHub:carlo", "--watch"])
        .timeout(Duration::from_secs(4))
        .assert()
        .stdout(is_match(r"\d{3} \d{3}").unwrap());
}

#[test]
fn code_uri_flag_prints_otpauth_uri_instead_of_digits() {
    // `tofa code <name> --uri` is the way to grab a single entry's
    // otpauth:// URI (e.g. to move it to another authenticator).
    // It prints the URI to stdout; --copy puts it on the clipboard.
    let tmp = setup();
    let out = tofa(&tmp)
        .args(["code", "GitHub:carlo", "--uri"])
        .assert()
        .success();
    let stdout = String::from_utf8(out.get_output().stdout.clone()).unwrap();
    let line = stdout.trim();
    // The label's `:` is percent-encoded per the otpauth spec.
    assert!(line.starts_with("otpauth://totp/"), "stdout: {line:?}");
    assert!(
        line.contains("GitHub%3Acarlo") || line.contains("GitHub:carlo"),
        "label should include GitHub:carlo (raw or percent-encoded): {line:?}"
    );
    assert!(line.contains("CLICODEPRIMARYAA"));
    // No code digits should appear on stdout — that's the point of --uri.
    assert!(
        !is_match(r"\d{3} \d{3}").unwrap().eval(line),
        "URI mode should not also print the code: {line:?}"
    );
}
