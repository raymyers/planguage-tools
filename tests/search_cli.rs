use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn search_lists_matching_markdown_files() {
    let temp = tempdir().unwrap();
    fs::write(
        temp.path().join("overview.md"),
        "Tag: Product
Owner: Team
",
    )
    .unwrap();
    fs::create_dir(temp.path().join("docs")).unwrap();
    fs::write(
        temp.path().join("docs/spec.md"),
        "Tag: Quality
Owner: Team
",
    )
    .unwrap();
    fs::write(
        temp.path().join("docs/notes.md"),
        "Nothing relevant here
",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("plg").unwrap();
    cmd.current_dir(temp.path())
        .args(["search", "Owner: Team"])
        .assert()
        .success()
        .stdout(predicate::str::contains("overview.md"))
        .stdout(predicate::str::contains("docs/spec.md"))
        .stdout(predicate::str::contains("docs/notes.md").not());
}

#[test]
fn search_respects_path_prefix() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("docs/planguage")).unwrap();
    fs::write(
        temp.path().join("docs/planguage/spec.md"),
        "Tag: Performance
Owner: Ops
",
    )
    .unwrap();
    fs::create_dir_all(temp.path().join("prompts")).unwrap();
    fs::write(
        temp.path().join("prompts/example.md"),
        "Tag: Performance
Owner: Ops
",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("plg").unwrap();
    cmd.current_dir(temp.path())
        .args(["search", "Owner: Ops", "--path-prefix", "docs/"])
        .assert()
        .success()
        .stdout(predicate::str::contains("docs/planguage/spec.md"))
        .stdout(predicate::str::contains("prompts/example.md").not());
}
