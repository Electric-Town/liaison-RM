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
    fs::{self},
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
        let mut matches = Vec::new();
        for entry in WalkDir::new(&people)
            .min_depth(1)
            .max_depth(1)
            .sort_by_file_name()
        {
            let Ok(entry) = entry else {
                // Workspace Health owns unreadable-entry diagnostics. A bad
                // sibling must not make a healthy target unreachable.
                continue;
            };
            if !entry.file_type().is_file()
                || entry.path().extension().and_then(|value| value.to_str()) != Some("md")
            {
                continue;
            }
            let Ok(parsed) = read_person_document(entry.path()) else {
                // Health reports malformed records with their path and repair
                // guidance. A damaged sibling must not make a healthy Person
                // unreachable.
                continue;
            };
            let Ok(person) = parsed.document.into_domain() else {
                continue;
            };
            if person.id == id {
                matches.push(entry.path().to_path_buf());
            }
        }
        match matches.as_slice() {
            [] => Err(PeopleError::NotFound),
            [path] => Ok(path.clone()),
            _ => Err(PeopleError::Storage(
                "duplicate person identity requires repair".to_owned(),
            )),
        }
    }
}

impl WorkspaceStore for MarkdownVault {
    fn initialise(&self, root: &Path, manifest: &WorkspaceManifest) -> Result<(), WorkspaceError> {
        let manifest_path = Self::manifest_path(root);
        if manifest_path.exists() {
            return Err(WorkspaceError::AlreadyExists);
        }
        if root.exists() {
            if !root.is_dir() {
                return Err(WorkspaceError::InitialiseTargetNotEmpty);
            }
            let mut entries = fs::read_dir(root).map_err(storage_workspace)?;
            if entries
                .next()
                .transpose()
                .map_err(storage_workspace)?
                .is_some()
            {
                return Err(WorkspaceError::InitialiseTargetNotEmpty);
            }
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
        for required in [
            "people",
            "organisations",
            "relationships",
            "interactions",
            "events",
        ] {
            if !root.join(required).is_dir() {
                findings.push(ValidationFinding {
                    code: "workspace.missing-directory".to_owned(),
                    severity: FindingSeverity::Error,
                    path: required.to_owned(),
                    message: format!("required workspace directory is missing: {required}"),
                    recovery: format!(
                        "create the directory after taking a workspace backup: {required}"
                    ),
                });
            }
        }
        let people = Self::people_path(root);
        if people.is_dir() {
            let mut identities = BTreeMap::<String, String>::new();
            for entry in WalkDir::new(&people)
                .min_depth(1)
                .max_depth(1)
                .sort_by_file_name()
            {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(error) => {
                        let path = error
                            .path()
                            .and_then(|path| path.strip_prefix(root).ok())
                            .map_or_else(|| "people".to_owned(), |path| path.display().to_string());
                        findings.push(ValidationFinding {
                            code: "people.unreadable-entry".to_owned(),
                            severity: FindingSeverity::Error,
                            path,
                            message: "a Person directory entry could not be read".to_owned(),
                            recovery: "preserve the workspace, correct local file access, and run Health again; Liaison will not delete the entry automatically".to_owned(),
                        });
                        continue;
                    }
                };
                if entry.path().extension().and_then(|value| value.to_str()) != Some("md") {
                    continue;
                }
                let relative = entry
                    .path()
                    .strip_prefix(root)
                    .unwrap_or(entry.path())
                    .display()
                    .to_string();
                if !entry.file_type().is_file() {
                    findings.push(ValidationFinding {
                        code: "people.invalid-record".to_owned(),
                        severity: FindingSeverity::Error,
                        path: relative,
                        message: "person record is not a regular file".to_owned(),
                        recovery: "preserve the entry and replace it with a reviewed readable Markdown record; Liaison will not follow or delete it automatically".to_owned(),
                    });
                    continue;
                }
                match read_person_document(entry.path())
                    .and_then(|parsed| parsed.document.into_domain())
                {
                    Ok(person) => {
                        let identity = person.id.to_string();
                        if let Some(first_path) = identities.insert(identity, relative.clone()) {
                            findings.push(ValidationFinding {
                                code: "people.duplicate-id".to_owned(),
                                severity: FindingSeverity::Error,
                                path: relative,
                                message: "person identity appears in more than one record".to_owned(),
                                recovery: format!(
                                    "preserve both files and resolve the duplicate identity with the first record at {first_path}; Liaison will not merge or delete either file automatically"
                                ),
                            });
                        }
                    }
                    Err(error) => findings.push(ValidationFinding {
                        code: "people.invalid-record".to_owned(),
                        severity: FindingSeverity::Error,
                        path: relative,
                        message: safe_people_validation_message(&error).to_owned(),
                        recovery: "inspect the file, preserve a copy, and repair its front matter; Liaison will not delete it automatically".to_owned(),
                    }),
                }
            }
        }
        Ok(findings)
    }
}

impl PersonRepository for MarkdownVault {
    fn create(&self, workspace: &Path, person: &PersonProfile) -> Result<(), PeopleError> {
        MarkdownVault::ensure_workspace(workspace)
            .map_err(|error| PeopleError::Storage(error.to_string()))?;
        match MarkdownVault::find_person_path(workspace, person.id) {
            Ok(_) => return Err(PeopleError::AlreadyExists),
            Err(PeopleError::NotFound) => {}
            Err(error) => return Err(error),
        }
        let filename = format!("{}--{}.md", slug(&person.display_name), person.id);
        let path = Self::people_path(workspace).join(filename);
        let document = PersonDocument::from_domain(person, BTreeMap::new());
        let rendered = render_person(&document, &default_person_body(person))?;
        atomic_create(&path, rendered.as_bytes()).map_err(storage_people)
    }

    fn list(
        &self,
        workspace: &Path,
        include_archived: bool,
    ) -> Result<Vec<PersonProfile>, PeopleError> {
        MarkdownVault::ensure_workspace(workspace)
            .map_err(|error| PeopleError::Storage(error.to_string()))?;
        let people = Self::people_path(workspace);
        let mut by_identity = BTreeMap::<String, Vec<PersonProfile>>::new();
        for entry in WalkDir::new(&people)
            .min_depth(1)
            .max_depth(1)
            .sort_by_file_name()
        {
            let Ok(entry) = entry else {
                continue;
            };
            if !entry.file_type().is_file()
                || entry.path().extension().and_then(|value| value.to_str()) != Some("md")
            {
                continue;
            }
            let Ok(parsed) = read_person_document(entry.path()) else {
                // Validation owns the diagnostic view. Listing remains useful
                // for healthy records while a malformed sibling is repaired.
                continue;
            };
            let Ok(person) = parsed.document.into_domain() else {
                continue;
            };
            if include_archived || !person.archived {
                by_identity
                    .entry(person.id.to_string())
                    .or_default()
                    .push(person);
            }
        }
        let mut result = by_identity
            .into_values()
            .filter_map(|mut people| (people.len() == 1).then(|| people.remove(0)))
            .collect::<Vec<_>>();
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
        let rendered = render_person(&document, &existing.body)?;
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
        PersonProfile::rehydrate(
            self.id,
            self.revision,
            self.display_name,
            self.aliases,
            self.emails,
            self.phones,
            self.birthday,
            self.archived,
        )
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
        return Err(PeopleError::Storage(
            "person revision must be positive".to_owned(),
        ));
    }
    Ok(ParsedPerson {
        document,
        body: body.to_owned(),
    })
}

fn safe_people_validation_message(error: &PeopleError) -> &'static str {
    match error {
        PeopleError::RequiredField(_) => "person record is missing a required value",
        PeopleError::InvalidEmail(_) => "person record contains an invalid email address",
        PeopleError::InvalidPhone(_) => "person record contains an invalid phone number",
        PeopleError::InvalidPartialDate { .. } => "person record contains an invalid partial date",
        PeopleError::AlreadyExists => "person record duplicates an existing identity",
        PeopleError::NotFound => "person record could not be found",
        PeopleError::RevisionConflict { .. } => {
            "person record revision conflicts with the expected version"
        }
        PeopleError::RevisionOverflow => "person record revision is outside the supported range",
        PeopleError::Storage(_) => "person record format or schema is invalid",
    }
}

fn split_front_matter(text: &str) -> Result<(&str, &str), PeopleError> {
    let mut lines = text.split_inclusive('\n');
    if lines.next().map(str::trim_end) != Some("---") {
        return Err(PeopleError::Storage(
            "Markdown record is missing opening front matter delimiter".to_owned(),
        ));
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
    Err(PeopleError::Storage(
        "Markdown record is missing closing front matter delimiter".to_owned(),
    ))
}

fn render_person(document: &PersonDocument, body: &str) -> Result<String, PeopleError> {
    let yaml = serde_yaml::to_string(document).map_err(storage_people)?;
    Ok(format!("---\n{yaml}---\n{}", body.trim_start_matches('\n')))
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
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "target already exists",
        ));
    }
    write_temp_and_persist(path, bytes)
}

fn atomic_replace(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "target has no parent"))?;
    let mut temporary = NamedTempFile::new_in(parent)?;
    temporary.write_all(bytes)?;
    temporary.as_file().sync_all()?;
    temporary
        .persist(path)
        .map(|_| ())
        .map_err(|error| error.error)
}

fn write_temp_and_persist(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "target has no parent"))?;
    fs::create_dir_all(parent)?;
    let mut temporary = NamedTempFile::new_in(parent)?;
    temporary.write_all(bytes)?;
    temporary.as_file().sync_all()?;
    temporary
        .persist_noclobber(path)
        .map(|_| ())
        .map_err(|error| error.error)
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
    use liaison_people::{CreatePerson, ListPeople, PersonProfile, PersonRepository};
    use liaison_shared_kernel::{PersonId, WorkspaceId};
    use liaison_workspace::{
        BuildProfile, InitialiseWorkspace, ValidateWorkspace, WorkspaceError, WorkspaceProfile,
    };
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
                WorkspaceId::new(),
                "People",
                WorkspaceProfile::Personal,
                BuildProfile::Airgap,
                "en-IE",
            );
            assert!(created.is_ok());

            let create_person = CreatePerson::new(vault.clone());
            let person = create_person.execute(
                root,
                PersonId::new(),
                "Alex Murphy",
                Some("alex@example.test".to_owned()),
            );
            assert!(person.is_ok());
            if let Ok(person) = &person {
                assert_eq!(person.revision.get(), 1);
            }

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
            assert!(
                initialise
                    .execute(
                        root,
                        WorkspaceId::new(),
                        "People",
                        WorkspaceProfile::Personal,
                        BuildProfile::Airgap,
                        "en-IE",
                    )
                    .is_ok()
            );
            let create = CreatePerson::new(vault.clone());
            let person = create.execute(root, PersonId::new(), "Alex Murphy", None);
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
            assert!(
                InitialiseWorkspace::new(vault.clone())
                    .execute(
                        root,
                        WorkspaceId::new(),
                        "People",
                        WorkspaceProfile::Personal,
                        BuildProfile::Airgap,
                        "en-IE",
                    )
                    .is_ok()
            );
            let create = CreatePerson::new(vault.clone());
            assert!(
                create
                    .execute(root, PersonId::new(), "Zara Example", None)
                    .is_ok()
            );
            assert!(
                create
                    .execute(root, PersonId::new(), "Alex Example", None)
                    .is_ok()
            );
            let people = ListPeople::new(vault).execute(root, false);
            assert!(people.is_ok());
            if let Ok(people) = people {
                let names: Vec<&str> = people
                    .iter()
                    .map(|person| person.display_name.as_str())
                    .collect();
                assert_eq!(names, vec!["Alex Example", "Zara Example"]);
            }
        }
    }

    #[test]
    fn malformed_sibling_does_not_hide_healthy_people() {
        let directory = tempdir();
        assert!(directory.is_ok());
        if let Ok(directory) = directory {
            let root = directory.path();
            let vault = MarkdownVault::new();
            assert!(
                InitialiseWorkspace::new(vault.clone())
                    .execute(
                        root,
                        WorkspaceId::new(),
                        "People",
                        WorkspaceProfile::Personal,
                        BuildProfile::Airgap,
                        "en-IE",
                    )
                    .is_ok()
            );
            let created = CreatePerson::new(vault.clone()).execute(
                root,
                PersonId::new(),
                "Alex Example",
                Some("alex@example.test".to_owned()),
            );
            assert!(created.is_ok());
            let Ok(created) = created else {
                return;
            };
            assert!(
                fs::write(
                    root.join("people/000-malformed.md"),
                    "# Missing YAML front matter\n",
                )
                .is_ok()
            );

            let listed = ListPeople::new(vault.clone()).execute(root, false);
            assert!(listed.is_ok());
            if let Ok(listed) = listed {
                assert_eq!(listed.len(), 1);
                assert_eq!(listed[0].id, created.id);
            }

            let found = vault.find(root, created.id);
            assert!(found.is_ok());
            if let Ok(found) = found {
                assert_eq!(found.display_name, "Alex Example");
                assert_eq!(found.revision.get(), 1);
            }
        }
    }

    #[test]
    fn initialisation_refuses_a_nonempty_directory_without_touching_user_files() {
        let directory = tempdir();
        assert!(directory.is_ok());
        if let Ok(directory) = directory {
            let root = directory.path().join("existing");
            assert!(fs::create_dir(&root).is_ok());
            let user_file = root.join("user-notes.txt");
            assert!(fs::write(&user_file, "keep me").is_ok());

            let result = InitialiseWorkspace::new(MarkdownVault::new()).execute(
                &root,
                WorkspaceId::new(),
                "People",
                WorkspaceProfile::Personal,
                BuildProfile::ConnectedLocal,
                "en-IE",
            );

            assert_eq!(result, Err(WorkspaceError::InitialiseTargetNotEmpty));
            assert_eq!(
                fs::read_to_string(user_file).as_deref().ok(),
                Some("keep me")
            );
            assert!(!root.join(".liaison/workspace.yaml").exists());
        }
    }

    #[test]
    fn semantic_corruption_is_reported_without_hiding_healthy_people() {
        let directory = tempdir();
        assert!(directory.is_ok());
        if let Ok(directory) = directory {
            let root = directory.path();
            let vault = MarkdownVault::new();
            assert!(
                InitialiseWorkspace::new(vault.clone())
                    .execute(
                        root,
                        WorkspaceId::new(),
                        "People",
                        WorkspaceProfile::Personal,
                        BuildProfile::ConnectedLocal,
                        "en-IE",
                    )
                    .is_ok()
            );
            assert!(
                CreatePerson::new(vault.clone())
                    .execute(root, PersonId::new(), "Healthy Person", None)
                    .is_ok()
            );
            let invalid_id = PersonId::new();
            let invalid = format!(
                "---\nformat: wrong-format\nschema_version: 1\nid: {invalid_id}\nrevision: 1\ndisplay_name: Invalid Person\n---\n# Invalid Person\n"
            );
            assert!(fs::write(root.join("people/invalid.md"), invalid).is_ok());

            let report = ValidateWorkspace::new(vault.clone()).execute(root);
            assert!(report.is_ok());
            if let Ok(report) = report {
                assert!(!report.is_valid());
                assert!(report.findings.iter().any(|finding| {
                    finding.code == "people.invalid-record"
                        && finding.message == "person record format or schema is invalid"
                }));
            }
            let listed = ListPeople::new(vault).execute(root, false);
            assert!(listed.is_ok());
            if let Ok(listed) = listed {
                assert_eq!(listed.len(), 1);
                assert_eq!(listed[0].display_name, "Healthy Person");
            }
        }
    }

    #[test]
    fn invalid_serialized_email_is_redacted_from_health_and_not_loaded() {
        let directory = tempdir();
        assert!(directory.is_ok());
        if let Ok(directory) = directory {
            let root = directory.path();
            let vault = MarkdownVault::new();
            assert!(
                InitialiseWorkspace::new(vault.clone())
                    .execute(
                        root,
                        WorkspaceId::new(),
                        "People",
                        WorkspaceProfile::Personal,
                        BuildProfile::ConnectedLocal,
                        "en-IE",
                    )
                    .is_ok()
            );
            let invalid_id = PersonId::new();
            let private_input = "private-invalid-address";
            let invalid = format!(
                "---\nformat: liaison-person\nschema_version: 1\nid: {invalid_id}\nrevision: 1\ndisplay_name: Invalid Email\nemails:\n  - value: {private_input}\n    label: primary\n---\n# Invalid Email\n"
            );
            assert!(fs::write(root.join("people/invalid-email.md"), invalid).is_ok());

            let report = ValidateWorkspace::new(vault.clone()).execute(root);
            assert!(report.is_ok());
            if let Ok(report) = report {
                assert!(!report.is_valid());
                assert!(report.findings.iter().all(|finding| {
                    !finding.message.contains(private_input)
                        && !finding.recovery.contains(private_input)
                }));
            }
            let listed = ListPeople::new(vault).execute(root, false);
            assert!(matches!(listed, Ok(people) if people.is_empty()));
        }
    }

    #[test]
    fn duplicate_person_identity_is_reported_and_not_returned_ambiguously() {
        let directory = tempdir();
        assert!(directory.is_ok());
        if let Ok(directory) = directory {
            let root = directory.path();
            let vault = MarkdownVault::new();
            assert!(
                InitialiseWorkspace::new(vault.clone())
                    .execute(
                        root,
                        WorkspaceId::new(),
                        "People",
                        WorkspaceProfile::Personal,
                        BuildProfile::ConnectedLocal,
                        "en-IE",
                    )
                    .is_ok()
            );
            let created = CreatePerson::new(vault.clone()).execute(
                root,
                PersonId::new(),
                "Duplicate Person",
                None,
            );
            assert!(created.is_ok());
            let Ok(created) = created else {
                return;
            };
            let original = MarkdownVault::find_person_path(root, created.id);
            assert!(original.is_ok());
            let Ok(original) = original else {
                return;
            };
            assert!(fs::copy(&original, root.join("people/zzz-duplicate-copy.md")).is_ok());

            let report = ValidateWorkspace::new(vault.clone()).execute(root);
            assert!(report.is_ok());
            if let Ok(report) = report {
                assert!(report.findings.iter().any(|finding| {
                    finding.code == "people.duplicate-id"
                        && finding.path == "people/zzz-duplicate-copy.md"
                }));
            }
            let listed = ListPeople::new(vault.clone()).execute(root, false);
            assert!(listed.is_ok());
            if let Ok(listed) = listed {
                assert!(listed.is_empty());
            }
            assert!(vault.find(root, created.id).is_err());
            let colliding = PersonProfile::create(created.id, "Third Duplicate");
            assert!(colliding.is_ok());
            if let Ok(colliding) = colliding {
                assert!(matches!(
                    vault.create(root, &colliding),
                    Err(liaison_people::PeopleError::Storage(_))
                ));
                let record_count = fs::read_dir(root.join("people"))
                    .map(|entries| entries.filter_map(Result::ok).count());
                assert_eq!(record_count.ok(), Some(2));
            }
        }
    }

    #[test]
    fn non_regular_markdown_entry_is_a_health_finding_without_hiding_people() {
        let directory = tempdir();
        assert!(directory.is_ok());
        if let Ok(directory) = directory {
            let root = directory.path();
            let vault = MarkdownVault::new();
            assert!(
                InitialiseWorkspace::new(vault.clone())
                    .execute(
                        root,
                        WorkspaceId::new(),
                        "People",
                        WorkspaceProfile::Personal,
                        BuildProfile::ConnectedLocal,
                        "en-IE",
                    )
                    .is_ok()
            );
            assert!(
                CreatePerson::new(vault.clone())
                    .execute(root, PersonId::new(), "Healthy Person", None)
                    .is_ok()
            );
            assert!(fs::create_dir(root.join("people/not-a-record.md")).is_ok());

            let report = ValidateWorkspace::new(vault.clone()).execute(root);
            assert!(report.is_ok());
            if let Ok(report) = report {
                assert!(report.findings.iter().any(|finding| {
                    finding.code == "people.invalid-record"
                        && finding.path == "people/not-a-record.md"
                        && finding.message == "person record is not a regular file"
                }));
            }
            let listed = ListPeople::new(vault).execute(root, false);
            assert!(listed.is_ok());
            if let Ok(listed) = listed {
                assert_eq!(listed.len(), 1);
                assert_eq!(listed[0].display_name, "Healthy Person");
            }
        }
    }
}
