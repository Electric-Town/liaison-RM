use assert_cmd::Command;
use predicates::prelude::*;
use std::{error::Error, fs};
use tempfile::tempdir;

#[test]
fn creates_lists_and_validates_a_local_workspace() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    let workspace = directory.path().join("People");

    let mut initialise = Command::cargo_bin("liaison")?;
    initialise
        .arg("--workspace")
        .arg(&workspace)
        .args([
            "workspace",
            "init",
            "--name",
            "People",
            "--profile",
            "personal",
            "--build-profile",
            "airgap",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialised workspace 'People'"));

    let mut create = Command::cargo_bin("liaison")?;
    create
        .arg("--workspace")
        .arg(&workspace)
        .args([
            "person",
            "create",
            "--name",
            "Alex Example",
            "--email",
            "alex@example.test",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Created person 'Alex Example'"));

    let people_dir = workspace.join("people");
    let markdown_count = fs::read_dir(people_dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().and_then(|value| value.to_str()) == Some("md"))
        .count();
    assert_eq!(markdown_count, 1);

    let mut list = Command::cargo_bin("liaison")?;
    list.arg("--workspace")
        .arg(&workspace)
        .args(["--output", "json", "person", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alex Example"))
        .stdout(predicate::str::contains("alex@example.test"));

    let mut validate = Command::cargo_bin("liaison")?;
    validate
        .arg("--workspace")
        .arg(&workspace)
        .args(["workspace", "validate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("is valid"));

    Ok(())
}

#[test]
fn reports_missing_workspace_as_structured_error() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    let workspace = directory.path().join("missing");

    let mut command = Command::cargo_bin("liaison")?;
    command
        .arg("--workspace")
        .arg(workspace)
        .args(["--output", "json", "workspace", "inspect"])
        .assert()
        .code(3)
        .stderr(predicate::str::contains("workspace.not-found"))
        .stderr(predicate::str::contains("exit_code"));

    Ok(())
}
