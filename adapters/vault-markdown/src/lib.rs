//! Markdown/YAML adapter for the open Liaison workspace.
//!
//! This crate implements context-owned repository ports. It translates
//! between versioned file documents and domain types; file documents are not
//! re-exported as domain models.

#![allow(clippy::module_name_repetitions, clippy::missing_errors_doc)]

use liaison_people::{PeopleError, PersonProfile, PersonRepository};
use liaison_shared_kernel::{PersonId, Revision};
use liaison_workspace::{
    FindingSeverity, ValidationFinding, WorkspaceError, WorkspaceManifest, WorkspaceStore,
};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};
use tempfile::NamedTempFile;
use walkdir::WalkDir;

const MANIFEST_PATH: &str = ".liaison/workspace.yaml";
const PERSON_FORMAT: &str = "liaison-person";
const PERSON_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Default)]
pub struct MarkdownVault;

impl MarkdownVault {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    fn manifest_path(root: &Path) -> PathBuf {
        root.join(MANIFEST_PATH)
    }

    fn people_path(root: &Path) -> PathBuf {
        root.join("people")
    }

    fn ensure_workspace(root: &Path) -> Result<(), WorkspaceError> {
        if Self::manifest_path(root).is_file() {
            Ok(())
        } else {
            Err(WorkspaceError::NotFound)
        }
    }

    fn find_person_path(root: &Path, id: PersonId) -> Result<PathBuf, PeopleError> {
        let people = Self::people_path(root);
        if !people.is_dir() {
            return Err(PeopleError::NotFound);
        }
        for entry in WalkDir::new(&people).min_depth(1).max_depth(1) {
            let entry = entry.map_err(storage_people)?;
            if !entry.file_type().is_file() || entry.path().extension().and_then(|value| value.to_str()) != Some("md") {
                continue;
            }
            let parsed = read_person_document(entry.path())?;
            if parsed.document.id == id {
                return Ok(entry.path().to_path_buf());
            }
        }
        Err(PeopleError::NotFound)
    }
}

impl WorkspaceStore for MarkdownVault {
    fn initialise(&self, root: &Path, manifest: &WorkspaceManifest) -> Result<(), WorkspaceError> {
        let manifest_path = Self::manifest_path(root);
        if manifest_path.exists() {
            return Err(WorkspaceError::AlreadyExists);
        }
        manifest.validate()?;
        for relative in [
            ".liaison/devices",
            ".liaison/members",
            ".liaison/grants",
            ".liaison/migrations",
            ".liaison/operations",
            ".liaison/projections",
            "people",
            "organisations",
            "locations",
            "groups",
            "relationships",
            "notes",
            "interactions",
            "reminders",
            "events",
            "views",
            "streams/access",
            "streams/email-metadata",
            "attachments/sha256",
            "audit",
        ] {
            fs::create_dir_all(root.join(relative)).map_err(storage_workspace)?;
        }
        let yaml = serde_yaml::to_string(manifest).map_err(storage_workspace)?;
        atomic_create(&manifest_path, yaml.as_bytes()).map_err(storage_workspace)
    }

    fn load(&self, root: &Path) -> Result<WorkspaceManifest, WorkspaceError> {
        let path = Self::manifest_path(root);
        let text = fs::read_to_string(path).map_err(|error| {
            if error.kind() == io::ErrorKind::NotFound {
                WorkspaceError::NotFound
            } else {
                storage_workspace(error)
            }
        })?;
        let manifest: WorkspaceManifest = serde_yaml::from_str(&text).map_err(storage_workspace)?;
        manifest.validate()?;
        Ok(manifest)
    }

    fn validate_layout(&self, root: &Path) -> Result<Vec<ValidationFinding>, WorkspaceError> {
        Self::ensure_workspace(root)?;
        let mut findings = Vec::new();
        for required in ["people", "organisations", "relationships", "interactions", "events"] {
            if !root.join(required).is_dir() {
                findings.push(ValidationFinding {
                    code: "workspace.missing-directory".to_owned(),
                    severity: FindingSeverity::Error,
                    path: required.to_owned(),
                    message: format!("required workspace directory is missing: {required}"),
                    recovery: format!("create the directory after taking a workspace backup: {required}"),
                });
            }
        }
        let people = Self::people_path(root);
        if people.is_dir() {
            for entry in WalkDir::new(&people).min_depth(1).max_depth(1) {
                let entry = entry.map_err(storage_workspace)?;
                if !entry.file_type().is_file() || entry.path().extension().and_then(|value| value.to_str()) != Some("md") {
                    continue;
                }
                if let Err(error) = read_person_document(entry.path()) {
                    let relative = entry
                        .path()
                        .strip_prefix(root)
                        .unwrap_or(entry.path())
                        .display()
                        .to_string();
                    findings.push(ValidationFinding {
                        code: "people.invalid-record".to_owned(),
                        severity: FindingSeverity::Error,
                        path: relative,
                        message: error.to_string(),
                        recovery: "inspect the file, preserve a copy, and repair its front matter; Liaison will not delete it automatically".to_owned(),
                    });
                }
            }
        }
        Ok(findings)
    }
}

impl PersonRepository for MarkdownVault {
    fn create(&self, workspace: &Path, person: &PersonProfile) -> Result<(), PeopleError> {
        MarkdownVault::ensure_workspace(workspace).map_err(|error| PeopleError::Storage(error.to_string()))?;
        if MarkdownVault::find_person_path(workspace, person.id).is_ok() {
            return Err(PeopleError::AlreadyExists);
        }
        let filename = format!("{}--{}.md", slug(&person.display_name), person.id);
        let path = Self::people_path(workspace).join(filename);
        let document = PersonDocument::from_domain(person, BTreeMap::new());
        let rendered = render_person(&document, &default_person_body(person));
        atomic_create(&path, rendered.as_bytes()).map_err(storage_people)
    }

    fn list(&self, workspace: &Path, include_archived: bool) -> Result<Vec<PersonProfile>, PeopleError> {
        MarkdownVault::ensure_workspace(workspace).map_err(|error| PeopleError::Storage(error.to_string()))?;
        let people = Self::people_path(workspace);
        let mut result = Vec::new();
        for entry in WalkDir::new(&people).min_depth(1).max_depth(1) {
            let entry = entry.map_err(storage_people)?;
            if !entry.file_type().is_file() || entry.path().extension().and_then(|value| value.to_str()) != Some("md") {
                continue;
            }
            let parsed = read_person_document(entry.path())?;
            let person = parsed.document.into_domain()?;
            if include_archived || !person.archived {
                result.push(person);
            }
        }
        result.sort_by(|left, right| {
            left.display_name
                .to_lowercase()
                .cmp(&right.display_name.to_lowercase())
                .then_with(|| left.id.to_string().cmp(&right.id.to_string()))
        });
        Ok(result)
    }

    fn find(&self, workspace: &Path, id: PersonId) -> Result<PersonProfile, PeopleError> {
        let path = Self::find_person_path(workspace, id)?;
        read_person_document(&path)?.document.into_domain()
    }

    fn save(
        &self,
        workspace: &Path,
        person: &PersonProfile,
        expected_revision: Revision,
    ) -> Result<(), PeopleError> {
        let path = Self::find_person_path(workspace, person.id)?;
        let existing = read_person_document(&path)?;
        let found = existing.document.revision;
        if found != expected_revision {
            return Err(PeopleError::RevisionConflict {
                expected: expected_revision.get(),
                found: found.get(),
            });
        }
        let required_revision = expected_revision
            .next()
            .map_err(|_| PeopleError::RevisionOverflow)?;
        if person.revision != required_revision {
            return Err(PeopleError::RevisionConflict {
                expected: required_revision.get(),
                found: person.revision.get(),
            });
        }
        let document = PersonDocument::from_domain(person, existing.document.extra);
        let rendered = render_person(&document, &existing.body);
        atomic_replace(&path, rendered.as_bytes()).map_err(storage_people)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersonDocument {
    format: String,
    schema_version: u32,
    id: PersonId,
    revision: Revision,
    display_name: String,
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default)]
    emails: Vec<liaison_people::EmailAddress>,
    #[serde(default)]
    phones: Vec<liaison_people::PhoneNumber>,
    #[serde(default)]
    birthday: Option<liaison_people::PartialDate>,
    #[serde(default)]
    archived: bool,
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl PersonDocument {
    fn from_domain(person: &PersonProfile, extra: BTreeMap<String, Value>) -> Self {
        Self {
            format: PERSON_FORMAT.to_owned(),
            schema_version: PERSON_SCHEMA_VERSION,
            id: person.id,
            revision: person.revision,
            display_name: person.display_name.clone(),
            aliases: person.aliases.clone(),
            emails: person.emails.clone(),
            phones: person.phones.clone(),
            birthday: person.birthday.clone(),
            archived: person.archived,
            extra,
        }
    }

    fn into_domain(self) -> Result<PersonProfile, PeopleError> {
        if self.format != PERSON_FORMAT {
            return Err(PeopleError::Storage(format!(
                "unexpected person format: {}",
                self.format
            )));
        }
        if self.schema_version != PERSON_SCHEMA_VERSION {
            return Err(PeopleError::Storage(format!(
                "unsupported person schema: {}",
                self.schema_version
            )));
        }
        if self.display_name.trim().is_empty() {
            return Err(PeopleError::RequiredField("display name"));
        }
        Ok(PersonProfile {
            id: self.id,
            revision: self.revision,
            display_name: self.display_name,
            aliases: self.aliases,
            emails: self.emails,
            phones: self.phones,
            birthday: self.birthday,
            archived: self.archived,
        })
    }
}

#[derive(Debug)]
struct ParsedPerson {
    document: PersonDocument,
    body: String,
}

fn read_person_document(path: &Path) -> Result<ParsedPerson, PeopleError> {
    let text = fs::read_to_string(path).map_err(storage_people)?;
    let (front_matter, body) = split_front_matter(&text)?;
    let document: PersonDocument = serde_yaml::from_str(front_matter).map_err(storage_people)?;
    if document.revision.get() == 0 {
        return Err(PeopleError::Storage("person revision must be positive".to_owned()));
    }
    Ok(ParsedPerson {
        document,
        body: body.to_owned(),
    })
}

fn split_front_matter(text: &str) -> Result<(&str, &str), PeopleError> {
    let mut lines = text.split_inclusive('\n');
    if lines.next().map(str::trim_end) != Some("---") {
        return Err(PeopleError::Storage("Markdown record is missing opening front matter delimiter".to_owned()));
    }
    let mut offset = 4;
    for line in lines {
        if line.trim_end() == "---" {
            let front_matter = &text[4..offset];
            let body_start = offset + line.len();
            return Ok((front_matter, &text[body_start..]));
        }
        offset += line.len();
    }
    Err(PeopleError::Storage("Markdown record is missing closing front matter delimiter".to_owned()))
}

fn render_person(document: &PersonDocument, body: &str) -> String {
    let yaml = serde_yaml::to_string(document).unwrap_or_else(|error| {
        // Serialization of the in-memory document should be infallible for the
        // supported value set. Returning an explicit marker keeps this helper
        // total; callers validate persisted documents on the next read.
        format!("serialization_error: {error}\n")
    });
    format!("---\n{yaml}---\n{}", body.trim_start_matches('\n'))
}

fn default_person_body(person: &PersonProfile) -> String {
    format!(
        "# {}\n\n## Snapshot\n\n## Personal notes\n\n## Relationship context\n\n## Open loops\n",
        person.display_name
    )
}

fn slug(value: &str) -> String {
    let mut result = String::new();
    let mut previous_dash = false;
    for character in value.chars().flat_map(char::to_lowercase) {
        if character.is_alphanumeric() {
            result.push(character);
            previous_dash = false;
        } else if !previous_dash && !result.is_empty() {
            result.push('-');
            previous_dash = true;
        }
    }
    let result = result.trim_matches('-');
    if result.is_empty() {
        "person".to_owned()
    } else {
        result.to_owned()
    }
}

fn atomic_create(path: &Path, bytes: &[u8]) -> io::Result<()> {
    if path.exists() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "target already exists"));
    }
    write_temp_and_persist(path, bytes)
}

fn atomic_replace(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let parent = path.parent().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "target has no parent"))?;
    let mut temporary = NamedTempFile::new_in(parent)?;
    temporary.write_all(bytes)?;
    temporary.as_file().sync_all()?;
    temporary.persist(path).map(|_| ()).map_err(|error| error.error)
}

fn write_temp_and_persist(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let parent = path.parent().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "target has no parent"))?;
    fs::create_dir_all(parent)?;
    let mut temporary = NamedTempFile::new_in(parent)?;
    temporary.write_all(bytes)?;
    temporary.as_file().sync_all()?;
    temporary.persist_noclobber(path).map(|_| ()).map_err(|error| error.error)
}

fn storage_workspace(error: impl std::fmt::Display) -> WorkspaceError {
    WorkspaceError::Storage(error.to_string())
}

fn storage_people(error: impl std::fmt::Display) -> PeopleError {
    PeopleError::Storage(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::MarkdownVault;
    use liaison_people::{CreatePerson, ListPeople, PersonRepository};
    use liaison_workspace::{BuildProfile, InitialiseWorkspace, WorkspaceProfile, WorkspaceStore};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn creates_workspace_and_person_as_readable_files() {
        let directory = tempdir();
        assert!(directory.is_ok());
        if let Ok(directory) = directory {
            let root = directory.path();
            let vault = MarkdownVault::new();
            let initialise = InitialiseWorkspace::new(vault.clone());
            let created = initialise.execute(
                root,
                "People",
                WorkspaceProfile::Personal,
                BuildProfile::Airgap,
                "en-IE",
            );
            assert!(created.is_ok());

            let create_person = CreatePerson::new(vault.clone());
            let person = create_person.execute(root, "Alex Murphy", Some("alex@example.test".to_owned()));
            assert!(person.is_ok());

            let files = fs::read_dir(root.join("people"));
            assert!(files.is_ok());
            if let Ok(files) = files {
                let names: Vec<String> = files
                    .filter_map(Result::ok)
                    .filter_map(|entry| entry.file_name().into_string().ok())
                    .collect();
                assert_eq!(names.len(), 1);
                assert!(names[0].starts_with("alex-murphy--"));
            }
        }
    }

    #[test]
    fn preserves_unknown_front_matter_when_saving() {
        let directory = tempdir();
        assert!(directory.is_ok());
        if let Ok(directory) = directory {
            let root = directory.path();
            let vault = MarkdownVault::new();
            let initialise = InitialiseWorkspace::new(vault.clone());
            assert!(initialise.execute(
                root,
                "People",
                WorkspaceProfile::Personal,
                BuildProfile::Airgap,
                "en-IE",
            ).is_ok());
            let create = CreatePerson::new(vault.clone());
            let person = create.execute(root, "Alex Murphy", None);
            assert!(person.is_ok());
            if let Ok(mut person) = person {
                let path = MarkdownVault::find_person_path(root, person.id);
                assert!(path.is_ok());
                if let Ok(path) = path {
                    let original = fs::read_to_string(&path);
                    assert!(original.is_ok());
                    if let Ok(original) = original {
                        let changed = original.replacen(
                            "archived: false",
                            "archived: false\nx-example-field: retained",
                            1,
                        );
                        assert!(fs::write(&path, changed).is_ok());
                        let expected = person.revision;
                        assert!(person.rename("Alex M. Murphy").is_ok());
                        assert!(vault.save(root, &person, expected).is_ok());
                        let saved = fs::read_to_string(&path);
                        assert!(saved.is_ok());
                        if let Ok(saved) = saved {
                            assert!(saved.contains("x-example-field: retained"));
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn lists_people_in_stable_order() {
        let directory = tempdir();
        assert!(directory.is_ok());
        if let Ok(directory) = directory {
            let root = directory.path();
            let vault = MarkdownVault::new();
            assert!(InitialiseWorkspace::new(vault.clone()).execute(
                root,
                "People",
                WorkspaceProfile::Personal,
                BuildProfile::Airgap,
                "en-IE",
            ).is_ok());
            let create = CreatePerson::new(vault.clone());
            assert!(create.execute(root, "Zara Example", None).is_ok());
            assert!(create.execute(root, "Alex Example", None).is_ok());
            let people = ListPeople::new(vault).execute(root, false);
            assert!(people.is_ok());
            if let Ok(people) = people {
                let names: Vec<&str> = people.iter().map(|person| person.display_name.as_str()).collect();
                assert_eq!(names, vec!["Alex Example", "Zara Example"]);
            }
        }
    }
}
