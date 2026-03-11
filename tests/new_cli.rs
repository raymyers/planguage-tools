use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn new_creates_requirement_document() {
    let temp = tempdir().unwrap();
    fs::write(
        temp.path().join("Cargo.toml"),
        r#"[package]
name = "fixture"
version = "0.1.0"
edition = "2024"
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("plg").unwrap();
    cmd.current_dir(temp.path())
        .args(["new", "docs/planguage/req.md"])
        .assert()
        .success()
        .stdout(predicate::str::contains("created"));

    let content = fs::read_to_string(temp.path().join("docs/planguage/req.md")).unwrap();
    assert!(content.contains("Tag: Example.Requirement"));
}

#[test]
fn new_creates_performance_template() {
    let temp = tempdir().unwrap();
    fs::write(
        temp.path().join("Cargo.toml"),
        r#"[package]
name = "fixture"
version = "0.1.0"
edition = "2024"
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("plg").unwrap();
    cmd.current_dir(temp.path())
        .args(["new", "--template", "performance", "docs/planguage/perf.md"])
        .assert()
        .success();

    let content = fs::read_to_string(temp.path().join("docs/planguage/perf.md")).unwrap();
    assert!(content.contains("Type: Performance"));
    assert!(content.contains("Ambition:"));
}
