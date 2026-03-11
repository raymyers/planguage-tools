use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn stats_reports_markdown_file_and_directory_counts() {
    let temp = tempdir().unwrap();
    fs::write(
        temp.path().join("root.md"),
        "# Root
",
    )
    .unwrap();
    fs::create_dir_all(temp.path().join("docs/planguage")).unwrap();
    fs::write(
        temp.path().join("docs/planguage/spec.md"),
        "# Spec
",
    )
    .unwrap();
    fs::write(
        temp.path().join("docs/planguage/goal.md"),
        "# Goal
",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("plg").unwrap();
    cmd.current_dir(temp.path())
        .arg("stats")
        .assert()
        .success()
        .stdout(predicate::str::contains("markdown_files	3"))
        .stdout(predicate::str::contains("directories_with_markdown	2"));
}

#[test]
fn stats_respects_path_prefix() {
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
        temp.path().join("prompts/example.md"),
        "# Example
",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("plg").unwrap();
    cmd.current_dir(temp.path())
        .args(["stats", "--path-prefix", "docs/"])
        .assert()
        .success()
        .stdout(predicate::str::contains("markdown_files	1"))
        .stdout(predicate::str::contains("directories_with_markdown	1"));
}
