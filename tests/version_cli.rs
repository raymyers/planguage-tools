use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn prints_version() {
    let mut cmd = Command::cargo_bin("plg").unwrap();

    cmd.arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("plg 0.1.0"));
}

#[test]
fn help_mentions_planguage() {
    let mut cmd = Command::cargo_bin("plg").unwrap();

    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Navigate and analyze Planguage markdown",
        ));
}
