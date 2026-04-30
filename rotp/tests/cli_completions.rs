use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn completions_bash() {
    Command::cargo_bin("rotp").unwrap()
        .args(["completions", "bash"])
        .assert().success()
        .stdout(contains("rotp"));
}

#[test]
fn completions_zsh() {
    Command::cargo_bin("rotp").unwrap()
        .args(["completions", "zsh"])
        .assert().success()
        .stdout(contains("rotp"));
}

#[test]
fn completions_fish() {
    Command::cargo_bin("rotp").unwrap()
        .args(["completions", "fish"])
        .assert().success()
        .stdout(contains("rotp"));
}
