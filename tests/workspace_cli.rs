use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn commands_find_workspace_from_nested_directory() {
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
    fs::create_dir_all(temp.path().join("docs/specs/nested")).unwrap();
    fs::write(temp.path().join("docs/specs/spec.md"), "# Spec\n").unwrap();

    let mut cmd = Command::cargo_bin("plg").unwrap();
    cmd.current_dir(temp.path().join("docs/specs/nested"))
        .arg("get")
        .assert()
        .success()
        .stdout(predicate::str::contains("docs/specs/spec.md"));
}

#[test]
fn commands_use_current_directory_when_no_workspace_markers_exist() {
    let temp = tempdir().unwrap();
    fs::write(temp.path().join("notes.md"), "# Notes\n").unwrap();

    let mut cmd = Command::cargo_bin("plg").unwrap();
    cmd.current_dir(temp.path())
        .arg("get")
        .assert()
        .success()
        .stdout(predicate::str::contains("notes.md"));
}
