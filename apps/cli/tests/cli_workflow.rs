use assert_cmd::Command;
use liaison_application::{LiaisonApplication, OpenWorkspaceCommand};
use predicates::prelude::*;
use serde_json::Value;
use std::{error::Error, fs};
use tempfile::tempdir;

fn copy_directory(source: &std::path::Path, destination: &std::path::Path) -> std::io::Result<()> {
    fs::create_dir(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let destination_entry = destination.join(entry.file_name());
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_directory(&entry.path(), &destination_entry)?;
        } else if file_type.is_file() {
            fs::copy(entry.path(), destination_entry)?;
        } else {
            return Err(std::io::Error::other(
                "test workspace copy refuses non-file entries",
            ));
        }
    }
    Ok(())
}

fn parity_fixture() -> Result<Value, serde_json::Error> {
    serde_json::from_str(include_str!(
        "../../../spec/fixtures/application-parity.json"
    ))
}

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
        .stdout(predicate::str::contains("command_id"))
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
fn every_command_requires_an_explicit_workspace() -> Result<(), Box<dyn Error>> {
    let mut command = Command::cargo_bin("liaison")?;
    command
        .args(["workspace", "inspect"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("--workspace <PATH>"));
    Ok(())
}

#[test]
fn review_cli_defaults_to_an_honest_connected_local_manifest() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    let workspace = directory.path().join("Review");
    let mut initialise = Command::cargo_bin("liaison")?;
    initialise
        .arg("--workspace")
        .arg(&workspace)
        .args(["workspace", "init", "--name", "Review"])
        .assert()
        .success();

    let manifest = fs::read_to_string(workspace.join(".liaison/workspace.yaml"))?;
    assert!(manifest.contains("build_profile: connected-local"));
    assert!(manifest.contains("profile: workplace"));
    Ok(())
}

#[test]
fn reports_missing_workspace_with_the_application_error_envelope() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    let workspace = directory.path().join("missing");

    let mut command = Command::cargo_bin("liaison")?;
    let output = command
        .arg("--workspace")
        .arg(workspace)
        .args(["--output", "json", "workspace", "inspect"])
        .output()?;
    assert_eq!(output.status.code(), Some(3));
    let error: Value = serde_json::from_slice(&output.stderr)?;
    assert_eq!(error["error"]["code"], "workspace.not-found");
    assert!(
        error["error"]["recovery"]
            .as_str()
            .is_some_and(|value| !value.is_empty())
    );
    assert!(error["error"]["details"].is_object());
    assert!(
        error["error"]["correlation_id"]
            .as_str()
            .is_some_and(|value| !value.is_empty())
    );
    assert_eq!(error["exit_code"], 3);

    Ok(())
}

#[test]
fn invalid_workspace_emits_report_and_returns_validation_exit() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    let workspace = directory.path().join("People");

    let mut initialise = Command::cargo_bin("liaison")?;
    initialise
        .arg("--workspace")
        .arg(&workspace)
        .args(["workspace", "init", "--name", "People"])
        .assert()
        .success();
    fs::write(
        workspace.join("people/malformed.md"),
        "# Missing front matter\n",
    )?;

    let mut validate = Command::cargo_bin("liaison")?;
    let output = validate
        .arg("--workspace")
        .arg(&workspace)
        .args(["--output", "json", "workspace", "validate"])
        .output()?;
    assert_eq!(output.status.code(), Some(6));
    assert!(output.stderr.is_empty());
    let report: Value = serde_json::from_slice(&output.stdout)?;
    let expected = parity_fixture()?;
    assert_eq!(report["contract_version"], expected["contract_version"]);
    assert_eq!(report["value"]["valid"], false);
    for field in ["code", "severity", "path", "message", "recovery"] {
        assert_eq!(
            report["value"]["findings"][0][field],
            expected["malformed_health"][field]
        );
    }
    assert!(
        report["command_id"]
            .as_str()
            .is_some_and(|value| !value.is_empty())
    );

    let mut validate_human = Command::cargo_bin("liaison")?;
    validate_human
        .arg("--workspace")
        .arg(&workspace)
        .args(["workspace", "validate"])
        .assert()
        .code(6)
        .stdout(predicate::str::contains(
            "ERROR people.invalid-record [people/malformed.md]",
        ))
        .stdout(predicate::str::contains(
            "person record format or schema is invalid",
        ))
        .stdout(predicate::str::contains("Recovery: inspect the file"))
        .stdout(predicate::str::contains("Correlation ID:"));

    Ok(())
}

#[test]
fn cli_error_boundary_matches_the_shared_application_fixture_without_leaking_path()
-> Result<(), Box<dyn Error>> {
    let expected = parity_fixture()?;
    let private_path = "relative/PRIVATE-client";
    let mut command = Command::cargo_bin("liaison")?;
    let output = command
        .arg("--workspace")
        .arg(private_path)
        .args(["--output", "json", "workspace", "inspect"])
        .output()?;
    assert_eq!(output.status.code(), Some(1));
    let envelope: Value = serde_json::from_slice(&output.stderr)?;
    let mut normalized = envelope["error"].clone();
    normalized["correlation_id"] = Value::String("<uuid>".to_owned());
    assert_eq!(normalized, expected["relative_path_error"]);
    assert!(!String::from_utf8_lossy(&output.stderr).contains(private_path));

    let mut human = Command::cargo_bin("liaison")?;
    human
        .arg("--workspace")
        .arg(private_path)
        .args(["workspace", "inspect"])
        .assert()
        .code(1)
        .stderr(predicate::str::contains(private_path).not())
        .stderr(predicate::str::contains(
            "application.workspace-path-not-absolute",
        ));
    Ok(())
}

#[test]
fn invalid_email_error_does_not_repeat_the_input() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    let workspace = directory.path().join("People");

    let mut initialise = Command::cargo_bin("liaison")?;
    initialise
        .arg("--workspace")
        .arg(&workspace)
        .args(["workspace", "init", "--name", "People"])
        .assert()
        .success();

    let mut create = Command::cargo_bin("liaison")?;
    create
        .arg("--workspace")
        .arg(&workspace)
        .args([
            "--output",
            "json",
            "person",
            "create",
            "--name",
            "Invalid Contact",
            "--email",
            "not-an-email",
        ])
        .assert()
        .code(1)
        .stderr(predicate::str::contains("people.invalid-email"))
        .stderr(predicate::str::contains("correlation_id"))
        .stderr(predicate::str::contains("not-an-email").not());

    let markdown_count = fs::read_dir(workspace.join("people"))?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().and_then(|value| value.to_str()) == Some("md"))
        .count();
    assert_eq!(markdown_count, 0);

    Ok(())
}

#[test]
fn writer_contention_is_typed_but_cli_health_stays_read_only() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    let workspace = directory.path().join("Contended");
    let mut initialise = Command::cargo_bin("liaison")?;
    initialise
        .arg("--workspace")
        .arg(&workspace)
        .args(["workspace", "init", "--name", "Contended"])
        .assert()
        .success();

    let owner = LiaisonApplication::new();
    let _authority = owner.open_workspace(OpenWorkspaceCommand {
        path: workspace.to_string_lossy().into_owned(),
    })?;

    let mut health = Command::cargo_bin("liaison")?;
    health
        .arg("--workspace")
        .arg(&workspace)
        .args(["--output", "json", "workspace", "validate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"valid\": true"));

    let mut create = Command::cargo_bin("liaison")?;
    let output = create
        .arg("--workspace")
        .arg(&workspace)
        .args([
            "--output",
            "json",
            "person",
            "create",
            "--name",
            "Blocked Person",
        ])
        .output()?;
    assert_eq!(output.status.code(), Some(4));
    let error: Value = serde_json::from_slice(&output.stderr)?;
    assert_eq!(error["error"]["code"], "workspace.writer-already-active");
    assert_eq!(error["error"]["details"], serde_json::json!({}));
    let rendered = String::from_utf8_lossy(&output.stderr);
    assert!(!rendered.contains(&workspace.to_string_lossy().into_owned()));
    assert!(!rendered.contains("process_id"));
    assert!(!rendered.contains("observed_diagnostic"));

    Ok(())
}

#[test]
fn copied_workspace_identity_is_typed_but_cli_health_stays_read_only() -> Result<(), Box<dyn Error>>
{
    let directory = tempdir()?;
    let source = directory.path().join("Source");
    let copied = directory.path().join("Copied");
    let mut initialise = Command::cargo_bin("liaison")?;
    initialise
        .arg("--workspace")
        .arg(&source)
        .args(["workspace", "init", "--name", "Copied identity"])
        .assert()
        .success();

    let owner = LiaisonApplication::new();
    let opened = owner.open_workspace(OpenWorkspaceCommand {
        path: source.to_string_lossy().into_owned(),
    })?;
    copy_directory(&source, &copied)?;

    let mut health = Command::cargo_bin("liaison")?;
    health
        .arg("--workspace")
        .arg(&copied)
        .args(["--output", "json", "workspace", "validate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"valid\": true"));

    let mut create = Command::cargo_bin("liaison")?;
    let output = create
        .arg("--workspace")
        .arg(&copied)
        .args([
            "--output",
            "json",
            "person",
            "create",
            "--name",
            "Blocked Copy",
        ])
        .output()?;
    assert_eq!(output.status.code(), Some(4));
    let error: Value = serde_json::from_slice(&output.stderr)?;
    assert_eq!(
        error["error"]["code"],
        "workspace.identity-writer-already-active"
    );
    assert_eq!(error["error"]["details"], serde_json::json!({}));
    let rendered = String::from_utf8_lossy(&output.stderr);
    assert!(!rendered.contains(&source.to_string_lossy().into_owned()));
    assert!(!rendered.contains(&copied.to_string_lossy().into_owned()));
    assert!(!rendered.contains(&opened.value.workspace.workspace_id.to_string()));
    assert!(!rendered.contains("process_id"));

    drop(owner);
    let mut create_after_exit = Command::cargo_bin("liaison")?;
    create_after_exit
        .arg("--workspace")
        .arg(&copied)
        .args(["person", "create", "--name", "Permitted Copy"])
        .assert()
        .success();

    Ok(())
}
