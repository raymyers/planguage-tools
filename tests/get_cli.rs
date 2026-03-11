use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn get_lists_markdown_files_in_workspace() {
    let temp = tempdir().unwrap();
    fs::write(
        temp.path().join("root.md"),
        "# Root
",
    )
    .unwrap();
    fs::create_dir(temp.path().join("nested")).unwrap();
    fs::write(
        temp.path().join("nested/spec.md"),
        "# Spec
",
    )
    .unwrap();
    fs::write(
        temp.path().join("notes.txt"),
        "ignore me
",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("plg").unwrap();
    cmd.current_dir(temp.path())
        .arg("get")
        .assert()
        .success()
        .stdout(predicate::str::contains("root.md"))
        .stdout(predicate::str::contains("nested/spec.md"))
        .stdout(predicate::str::contains("notes.txt").not());
}

#[test]
fn get_filters_by_path_prefix() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("docs/planguage")).unwrap();
    fs::write(
        temp.path().join("docs/planguage/spec.md"),
        "# Spec
",
    )
    .unwrap();
    fs::create_dir_all(temp.path().join("prompts")).unwrap();
    fs::write(
        temp.path().join("prompts/input.md"),
        "# Prompt
",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("plg").unwrap();
    cmd.current_dir(temp.path())
        .args(["get", "--path-prefix", "docs/"])
        .assert()
        .success()
        .stdout(predicate::str::contains("docs/planguage/spec.md"))
        .stdout(predicate::str::contains("prompts/input.md").not());
}
