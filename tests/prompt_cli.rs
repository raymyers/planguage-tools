use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn convert_emits_template_and_inline_text() {
    let temp = tempdir().unwrap();
    fs::create_dir(temp.path().join("prompts")).unwrap();
    fs::copy(
        "/home/ray/dev/planguage-tools/prompts/planguage_conversion.md",
        temp.path().join("prompts/planguage_conversion.md"),
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("plg").unwrap();
    cmd.current_dir(temp.path())
        .args(["convert", "--text", "Users need faster feedback."])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Planguage Conversion Specialist Prompt",
        ))
        .stdout(predicate::str::contains("Users need faster feedback."));
}

#[test]
fn qa_emits_template_and_file_input() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join("prompts")).unwrap();
    fs::copy(
        "/home/ray/dev/planguage-tools/prompts/planguage_spec_quality_control.md",
        temp.path()
            .join("prompts/planguage_spec_quality_control.md"),
    )
    .unwrap();
    fs::create_dir_all(temp.path().join("docs")).unwrap();
    fs::write(
        temp.path().join("docs/spec.md"),
        "Tag: Quality
Owner: Team
",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("plg").unwrap();
    cmd.current_dir(temp.path())
        .args(["qa", "--file", "docs/spec.md"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Professional Specification Quality Control",
        ))
        .stdout(predicate::str::contains("Tag: Quality"));
}
