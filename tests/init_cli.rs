use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn init_creates_workspace_layout_in_current_directory() {
    let temp = tempdir().unwrap();

    let mut cmd = Command::cargo_bin("plg").unwrap();
    cmd.current_dir(temp.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("initialized"));

    assert!(temp.path().join("docs/planguage/templates").is_dir());
    assert!(temp.path().join("docs/planguage/fragments").is_dir());
    assert!(temp.path().join("docs/planguage/example.md").is_file());
    assert!(temp.path().join("plg.toml").is_file());
}

#[test]
fn init_respects_explicit_target_directory() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("workspace")).unwrap();

    let mut cmd = Command::cargo_bin("plg").unwrap();
    cmd.current_dir(temp.path())
        .args(["init", "--dir", "workspace"])
        .assert()
        .success();

    assert!(
        temp.path()
            .join("workspace/docs/planguage/example.md")
            .is_file()
    );
    assert!(temp.path().join("workspace/plg.toml").is_file());
}
