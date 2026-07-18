use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use std::{error::Error, fs};
use tempfile::tempdir;

#[test]
fn persists_and_updates_relationship_intent_as_yaml() -> Result<(), Box<dyn Error>> {
    let directory = tempdir()?;
    let workspace = directory.path().join("People");

    Command::cargo_bin("liaison")?
        .arg("--workspace")
        .arg(&workspace)
        .args(["workspace", "init", "--name", "People"])
        .assert()
        .success();

    let output = Command::cargo_bin("liaison")?
        .arg("--workspace")
        .arg(&workspace)
        .args([
            "--output",
            "json",
            "person",
            "create",
            "--name",
            "Alex Example",
            "--email",
            "alex@example.test",
        ])
        .output()?;
    assert!(output.status.success());
    let person: Value = serde_json::from_slice(&output.stdout)?;
    let person_id = person["id"]
        .as_str()
        .ok_or("person output did not include an ID")?;

    Command::cargo_bin("liaison")?
        .arg("--workspace")
        .arg(&workspace)
        .args([
            "--output",
            "json",
            "relationship",
            "set",
            "--person-id",
            person_id,
            "--relationship-type",
            "friend",
            "--tier",
            "core",
            "--cadence",
            "quarterly",
            "--last-contacted",
            "2026-06-01",
            "--reason",
            "Ask how the move went",
            "--circle",
            "Close friends",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"relationship_type\": \"friend\""))
        .stdout(predicate::str::contains("\"revision\": 1"));

    let relationship_path = workspace
        .join("relationships")
        .join(format!("{person_id}.yaml"));
    let yaml = fs::read_to_string(&relationship_path)?;
    assert!(yaml.contains("format: liaison-relationship"));
    assert!(yaml.contains("Close friends"));

    Command::cargo_bin("liaison")?
        .arg("--workspace")
        .arg(&workspace)
        .args([
            "--output",
            "json",
            "relationship",
            "set",
            "--person-id",
            person_id,
            "--expected-revision",
            "1",
            "--topic",
            "Starting a new role",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"revision\": 2"))
        .stdout(predicate::str::contains("Starting a new role"))
        .stdout(predicate::str::contains("Close friends"));

    Command::cargo_bin("liaison")?
        .arg("--workspace")
        .arg(&workspace)
        .args([
            "--output",
            "json",
            "relationship",
            "show",
            "--person-id",
            person_id,
            "--as-of",
            "2026-10-01",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"maintenance_status\": \"overdue\""));

    Command::cargo_bin("liaison")?
        .arg("--workspace")
        .arg(&workspace)
        .args([
            "--output",
            "json",
            "relationship",
            "set",
            "--person-id",
            person_id,
            "--expected-revision",
            "1",
            "--reason",
            "Stale edit",
        ])
        .assert()
        .code(4)
        .stderr(predicate::str::contains("relationship.revision-conflict"));

    Ok(())
}
