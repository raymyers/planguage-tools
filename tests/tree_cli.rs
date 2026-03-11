use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn tree_shows_markdown_hierarchy() {
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
        .arg("tree")
        .assert()
        .success()
        .stdout(predicate::str::contains("docs"))
        .stdout(predicate::str::contains("planguage"))
        .stdout(predicate::str::contains("goal.md"))
        .stdout(predicate::str::contains("spec.md"))
        .stdout(predicate::str::contains("root.md"));
}

#[test]
fn tree_respects_path_prefix() {
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
        .args(["tree", "--path-prefix", "docs/"])
        .assert()
        .success()
        .stdout(predicate::str::contains("docs"))
        .stdout(predicate::str::contains("spec.md"))
        .stdout(predicate::str::contains("prompts").not())
        .stdout(predicate::str::contains("example.md").not());
}
