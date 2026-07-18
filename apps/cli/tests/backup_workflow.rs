use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

fn liaison() -> Command {
    Command::cargo_bin("liaison")
        .unwrap_or_else(|error| unreachable!("liaison binary is unavailable: {error}"))
}

fn run_success(workspace: &Path, arguments: &[&str]) {
    liaison()
        .arg("--workspace")
        .arg(workspace)
        .args(arguments)
        .assert()
        .success();
}

fn first_markdown_file(root: &Path) -> PathBuf {
    let people = root.join("payload/people");
    fs::read_dir(&people)
        .unwrap_or_else(|error| unreachable!("could not read backup people directory: {error}"))
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| path.extension().and_then(|value| value.to_str()) == Some("md"))
        .unwrap_or_else(|| unreachable!("backup did not contain a person Markdown file"))
}

#[test]
fn creates_verifies_and_restores_workspace_backup() {
    let directory = tempfile::tempdir()
        .unwrap_or_else(|error| unreachable!("could not create test directory: {error}"));
    let workspace = directory.path().join("workspace");
    let backup = directory.path().join("backup");
    let restored = directory.path().join("restored");

    run_success(
        &workspace,
        &["workspace", "init", "--name", "Relationships"],
    );
    run_success(
        &workspace,
        &[
            "person",
            "create",
            "--name",
            "Alex Example",
            "--email",
            "alex@example.test",
        ],
    );

    liaison()
        .arg("--workspace")
        .arg(&workspace)
        .args(["backup", "create", "--destination"])
        .arg(&backup)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created backup"));

    liaison()
        .args(["backup", "verify", "--backup"])
        .arg(&backup)
        .assert()
        .success()
        .stdout(predicate::str::contains("Verified backup"));

    liaison()
        .args(["backup", "restore", "--backup"])
        .arg(&backup)
        .arg("--target")
        .arg(&restored)
        .assert()
        .success()
        .stdout(predicate::str::contains("Restored workspace"));

    liaison()
        .arg("--workspace")
        .arg(&restored)
        .args(["person", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alex Example"));
    assert!(!restored.join(".liaison/restore-in-progress").exists());
}

#[test]
fn rejects_tampered_backup_without_creating_restore_target() {
    let directory = tempfile::tempdir()
        .unwrap_or_else(|error| unreachable!("could not create test directory: {error}"));
    let workspace = directory.path().join("workspace");
    let backup = directory.path().join("backup");
    let restored = directory.path().join("restored");

    run_success(
        &workspace,
        &["workspace", "init", "--name", "Relationships"],
    );
    run_success(&workspace, &["person", "create", "--name", "Alex Example"]);
    liaison()
        .arg("--workspace")
        .arg(&workspace)
        .args(["backup", "create", "--destination"])
        .arg(&backup)
        .assert()
        .success();

    let person = first_markdown_file(&backup);
    fs::write(&person, b"tampered\n")
        .unwrap_or_else(|error| unreachable!("could not tamper with backup: {error}"));

    liaison()
        .args(["backup", "verify", "--backup"])
        .arg(&backup)
        .assert()
        .code(5)
        .stderr(predicate::str::contains("backup.verification-failed"));

    liaison()
        .args(["backup", "restore", "--backup"])
        .arg(&backup)
        .arg("--target")
        .arg(&restored)
        .assert()
        .code(5)
        .stderr(predicate::str::contains("backup.verification-failed"));
    assert!(!restored.exists());
}
