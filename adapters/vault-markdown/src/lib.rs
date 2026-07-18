//! Markdown/YAML adapter for the open Liaison workspace.
//!
//! This crate implements context-owned repository ports. It translates
//! between versioned file documents and domain types; file documents are not
//! re-exported as domain models.

#![allow(clippy::module_name_repetitions, clippy::missing_errors_doc)]

use cap_fs_ext::{DirExt, FollowSymlinks, OpenOptionsFollowExt};
use cap_std::{
    ambient_authority,
    fs::{Dir, OpenOptions},
};
use liaison_people::{PeopleError, PersonProfile, PersonRepository};
use liaison_shared_kernel::{PersonId, Revision};
use liaison_workspace::{
    BoundWorkspaceStore, FindingSeverity, ValidationFinding, WorkspaceError, WorkspaceManifest,
    WorkspaceStore, WorkspaceValidationReport,
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

/// Markdown repositories bound once to the root retained by a
/// `WorkspaceSession`. The root is deliberately private and no path-taking
/// repository method is exposed on this capability.
#[derive(Debug)]
pub struct BoundMarkdownVault {
    root: Dir,
}

impl MarkdownVault {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn bind_directory(&self, root: Dir) -> BoundMarkdownVault {
        BoundMarkdownVault { root }
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

    fn read_manifest(root: &Path) -> Result<WorkspaceManifest, WorkspaceError> {
        let path = Self::manifest_path(root);
        let text = fs::read_to_string(path).map_err(|error| {
            if error.kind() == io::ErrorKind::NotFound {
                WorkspaceError::NotFound
            } else {
                storage_workspace(error)
            }
        })?;
        serde_yaml::from_str(&text).map_err(storage_workspace)
    }

    /// Read-only Health deliberately does not acquire writer authority. A
    /// parseable newer-schema manifest is reported as a finding while safe
    /// layout checks still run; malformed manifests remain a typed read error.
    pub fn inspect_health(&self, root: &Path) -> Result<WorkspaceValidationReport, WorkspaceError> {
        let root = Dir::open_ambient_dir(root, ambient_authority()).map_err(|error| {
            if error.kind() == io::ErrorKind::NotFound {
                WorkspaceError::NotFound
            } else {
                storage_workspace(error)
            }
        })?;
        let manifest = bound_read_manifest_unvalidated(&root)?;
        let mut findings = Vec::new();
        if let Err(error) = manifest.validate() {
            findings.push(manifest_validation_finding(&error));
        }
        findings.extend(bound_validate_layout(&root)?);
        Ok(WorkspaceValidationReport {
            workspace_id: manifest.workspace_id,
            schema_version: manifest.schema_version,
            findings,
        })
    }

    #[cfg(test)]
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
        let manifest = Self::read_manifest(root)?;
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
                            .map_or_else(|| "people".to_owned(), portable_workspace_path);
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
                    .map_or_else(|_| "people".to_owned(), portable_workspace_path);
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

fn portable_workspace_path(path: &Path) -> String {
    path.iter()
        .map(|component| component.to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

impl BoundWorkspaceStore for BoundMarkdownVault {
    fn load_manifest(&self) -> Result<WorkspaceManifest, WorkspaceError> {
        bound_load_manifest(&self.root)
    }

    fn validate_layout(&self) -> Result<Vec<ValidationFinding>, WorkspaceError> {
        bound_validate_layout(&self.root)
    }
}

impl PersonRepository for BoundMarkdownVault {
    fn create(&self, person: &PersonProfile) -> Result<(), PeopleError> {
        bound_create_person(&self.root, person)
    }

    fn list(&self, include_archived: bool) -> Result<Vec<PersonProfile>, PeopleError> {
        bound_list_people(&self.root, include_archived)
    }

    fn find(&self, id: PersonId) -> Result<PersonProfile, PeopleError> {
        let (_, parsed) = bound_find_person(&self.root, id)?;
        parsed.document.into_domain()
    }

    fn save(&self, person: &PersonProfile, expected_revision: Revision) -> Result<(), PeopleError> {
        bound_save_person(&self.root, person, expected_revision)
    }
}

fn nofollow_read_options() -> OpenOptions {
    let mut options = OpenOptions::new();
    options.read(true).follow(FollowSymlinks::No);
    options
}

fn bound_read_text(directory: &Dir, path: &Path) -> io::Result<String> {
    let mut file = directory.open_with(path, &nofollow_read_options())?;
    if !file.metadata()?.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "record is not a regular file",
        ));
    }
    let mut text = String::new();
    std::io::Read::read_to_string(&mut file, &mut text)?;
    Ok(text)
}

fn bound_load_manifest(root: &Dir) -> Result<WorkspaceManifest, WorkspaceError> {
    let manifest = bound_read_manifest_unvalidated(root)?;
    manifest.validate()?;
    Ok(manifest)
}

fn bound_read_manifest_unvalidated(root: &Dir) -> Result<WorkspaceManifest, WorkspaceError> {
    let text = bound_read_text(root, Path::new(MANIFEST_PATH)).map_err(|error| {
        if error.kind() == io::ErrorKind::NotFound {
            WorkspaceError::NotFound
        } else {
            storage_workspace(error)
        }
    })?;
    serde_yaml::from_str(&text).map_err(storage_workspace)
}

fn bound_people_directory(root: &Dir) -> Result<Dir, PeopleError> {
    root.open_dir_nofollow("people").map_err(storage_people)
}

fn bound_person_entries(people: &Dir) -> Result<Vec<cap_std::fs::DirEntry>, PeopleError> {
    let mut entries = people
        .entries()
        .map_err(storage_people)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(storage_people)?;
    entries.sort_by_key(cap_std::fs::DirEntry::file_name);
    Ok(entries)
}

fn bound_read_person_entry(entry: &cap_std::fs::DirEntry) -> Result<ParsedPerson, PeopleError> {
    let mut file = entry
        .open_with(&nofollow_read_options())
        .map_err(storage_people)?;
    if !file.metadata().map_err(storage_people)?.is_file() {
        return Err(PeopleError::Storage(
            "person record is not a regular file".to_owned(),
        ));
    }
    let mut text = String::new();
    std::io::Read::read_to_string(&mut file, &mut text).map_err(storage_people)?;
    parse_person_document(&text)
}

fn bound_list_people(
    root: &Dir,
    include_archived: bool,
) -> Result<Vec<PersonProfile>, PeopleError> {
    bound_load_manifest(root).map_err(|error| PeopleError::Storage(error.to_string()))?;
    let people = bound_people_directory(root)?;
    let mut by_identity = BTreeMap::<String, Vec<PersonProfile>>::new();
    for entry in bound_person_entries(&people)? {
        let name = PathBuf::from(entry.file_name());
        if name.extension().and_then(|value| value.to_str()) != Some("md") {
            continue;
        }
        let Ok(parsed) = bound_read_person_entry(&entry) else {
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

fn bound_find_person(root: &Dir, id: PersonId) -> Result<(PathBuf, ParsedPerson), PeopleError> {
    let people = bound_people_directory(root)?;
    let mut found = Vec::new();
    for entry in bound_person_entries(&people)? {
        let name = PathBuf::from(entry.file_name());
        if name.extension().and_then(|value| value.to_str()) != Some("md") {
            continue;
        }
        let Ok(parsed) = bound_read_person_entry(&entry) else {
            continue;
        };
        if parsed.document.id == id {
            found.push((name, parsed));
        }
    }
    match found.len() {
        0 => Err(PeopleError::NotFound),
        1 => found.pop().ok_or(PeopleError::NotFound),
        _ => Err(PeopleError::Storage(
            "duplicate person identity requires repair".to_owned(),
        )),
    }
}

fn bound_create_person(root: &Dir, person: &PersonProfile) -> Result<(), PeopleError> {
    bound_load_manifest(root).map_err(|error| PeopleError::Storage(error.to_string()))?;
    match bound_find_person(root, person.id) {
        Ok(_) => return Err(PeopleError::AlreadyExists),
        Err(PeopleError::NotFound) => {}
        Err(error) => return Err(error),
    }
    let people = bound_people_directory(root)?;
    let filename = format!("{}--{}.md", slug(&person.display_name), person.id);
    let temporary = format!(".person-{}.tmp", person.id);
    let document = PersonDocument::from_domain(person, BTreeMap::new());
    let rendered = render_person(&document, &default_person_body(person))?;
    let mut options = OpenOptions::new();
    options
        .write(true)
        .create_new(true)
        .follow(FollowSymlinks::No);
    let mut file = people
        .open_with(&temporary, &options)
        .map_err(storage_people)?;
    if let Err(error) = file
        .write_all(rendered.as_bytes())
        .and_then(|()| file.sync_all())
    {
        drop(file);
        let _ = people.remove_file(&temporary);
        return Err(storage_people(error));
    }
    drop(file);
    if let Err(error) = people.hard_link(&temporary, &people, &filename) {
        let _ = people.remove_file(&temporary);
        if error.kind() == io::ErrorKind::AlreadyExists {
            return Err(PeopleError::AlreadyExists);
        }
        return Err(storage_people(error));
    }
    // Publication is complete once the no-clobber hard link exists. A crash
    // before this point leaves only an ignored staging file; cleanup failure
    // after publication must not report a false failed creation.
    let _ = people.remove_file(&temporary);
    Ok(())
}

fn bound_save_person(
    root: &Dir,
    person: &PersonProfile,
    expected_revision: Revision,
) -> Result<(), PeopleError> {
    let (filename, existing) = bound_find_person(root, person.id)?;
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
    let people = bound_people_directory(root)?;
    let temporary = format!(".person-{}-{}.tmp", person.id, person.revision.get());
    let document = PersonDocument::from_domain(person, existing.document.extra);
    let rendered = render_person(&document, &existing.body)?;
    let mut options = OpenOptions::new();
    options
        .write(true)
        .create_new(true)
        .follow(FollowSymlinks::No);
    let mut file = people
        .open_with(&temporary, &options)
        .map_err(storage_people)?;
    if let Err(error) = file
        .write_all(rendered.as_bytes())
        .and_then(|()| file.sync_all())
    {
        drop(file);
        let _ = people.remove_file(&temporary);
        return Err(storage_people(error));
    }
    drop(file);
    people
        .rename(&temporary, &people, &filename)
        .map_err(storage_people)
}

fn bound_validate_layout(root: &Dir) -> Result<Vec<ValidationFinding>, WorkspaceError> {
    let _manifest = bound_read_text(root, Path::new(MANIFEST_PATH)).map_err(storage_workspace)?;
    let mut findings = Vec::new();
    for required in [
        "people",
        "organisations",
        "relationships",
        "interactions",
        "events",
    ] {
        let valid = root
            .symlink_metadata(required)
            .is_ok_and(|metadata| metadata.is_dir() && !metadata.file_type().is_symlink());
        if !valid {
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
    let Ok(people) = root.open_dir_nofollow("people") else {
        return Ok(findings);
    };
    let Ok(entries) = people.entries() else {
        findings.push(ValidationFinding {
            code: "people.unreadable-entry".to_owned(),
            severity: FindingSeverity::Error,
            path: "people".to_owned(),
            message: "the Person directory could not be read".to_owned(),
            recovery: "preserve the workspace, correct local file access, and run Health again"
                .to_owned(),
        });
        return Ok(findings);
    };
    let mut identities = BTreeMap::<String, String>::new();
    for entry in entries {
        let Ok(entry) = entry else {
            findings.push(ValidationFinding {
                code: "people.unreadable-entry".to_owned(),
                severity: FindingSeverity::Error,
                path: "people".to_owned(),
                message: "a Person directory entry could not be read".to_owned(),
                recovery: "preserve the workspace, correct local file access, and run Health again"
                    .to_owned(),
            });
            continue;
        };
        let name = PathBuf::from(entry.file_name());
        if name.extension().and_then(|value| value.to_str()) != Some("md") {
            continue;
        }
        let relative = format!("people/{}", name.to_string_lossy());
        match bound_read_person_entry(&entry).and_then(|parsed| parsed.document.into_domain()) {
            Ok(person) => {
                if let Some(first_path) = identities.insert(person.id.to_string(), relative.clone()) {
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
    Ok(findings)
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
    parse_person_document(&text)
}

fn parse_person_document(text: &str) -> Result<ParsedPerson, PeopleError> {
    let (front_matter, body) = split_front_matter(text)?;
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

fn manifest_validation_finding(error: &WorkspaceError) -> ValidationFinding {
    match error {
        WorkspaceError::UnsupportedSchema { found, supported } => ValidationFinding {
            code: "workspace.unsupported-schema".to_owned(),
            severity: FindingSeverity::Error,
            path: MANIFEST_PATH.to_owned(),
            message: format!(
                "workspace schema {found} is newer than this build supports ({supported})"
            ),
            recovery: "keep the workspace read-only and open it with a compatible Liaison build; do not rewrite the manifest manually".to_owned(),
        },
        WorkspaceError::UnexpectedFormat(_) => ValidationFinding {
            code: "workspace.unexpected-format".to_owned(),
            severity: FindingSeverity::Error,
            path: MANIFEST_PATH.to_owned(),
            message: "workspace manifest format is not supported".to_owned(),
            recovery: "keep the workspace read-only and restore a verified Liaison manifest"
                .to_owned(),
        },
        WorkspaceError::RequiredField(_) => ValidationFinding {
            code: "workspace.invalid-manifest".to_owned(),
            severity: FindingSeverity::Error,
            path: MANIFEST_PATH.to_owned(),
            message: "workspace manifest is missing a required value".to_owned(),
            recovery: "preserve the workspace and repair the manifest from a verified copy"
                .to_owned(),
        },
        _ => ValidationFinding {
            code: "workspace.invalid-manifest".to_owned(),
            severity: FindingSeverity::Error,
            path: MANIFEST_PATH.to_owned(),
            message: "workspace manifest is invalid".to_owned(),
            recovery: "preserve the workspace and inspect the manifest before making changes"
                .to_owned(),
        },
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
    use super::{BoundMarkdownVault, MarkdownVault, slug};
    use cap_std::{ambient_authority, fs::Dir};
    use liaison_people::{CreatePerson, ListPeople, PersonProfile, PersonRepository};
    use liaison_shared_kernel::{PersonId, WorkspaceId};
    use liaison_workspace::{
        BuildProfile, InitialiseWorkspace, ValidateWorkspace, WorkspaceError, WorkspaceProfile,
    };
    use std::{fs, path::Path};
    use tempfile::tempdir;

    fn bound_vault(vault: &MarkdownVault, root: &Path) -> BoundMarkdownVault {
        let directory = Dir::open_ambient_dir(root, ambient_authority())
            .unwrap_or_else(|error| unreachable!("test workspace must open: {error}"));
        vault.bind_directory(directory)
    }

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

            let bound = bound_vault(&vault, root);
            let create_person = CreatePerson::new(&bound);
            let person = create_person.execute(
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
            let bound = bound_vault(&vault, root);
            let create = CreatePerson::new(&bound);
            let person = create.execute(PersonId::new(), "Alex Murphy", None);
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
                        assert!(bound.save(&person, expected).is_ok());
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
            let bound = bound_vault(&vault, root);
            let create = CreatePerson::new(&bound);
            assert!(
                create
                    .execute(PersonId::new(), "Zara Example", None)
                    .is_ok()
            );
            assert!(
                create
                    .execute(PersonId::new(), "Alex Example", None)
                    .is_ok()
            );
            let people = ListPeople::new(&bound).execute(false);
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
            let bound = bound_vault(&vault, root);
            let created = CreatePerson::new(&bound).execute(
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

            let listed = ListPeople::new(&bound).execute(false);
            assert!(listed.is_ok());
            if let Ok(listed) = listed {
                assert_eq!(listed.len(), 1);
                assert_eq!(listed[0].id, created.id);
            }

            let found = bound.find(created.id);
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
                CreatePerson::new(&bound_vault(&vault, root))
                    .execute(PersonId::new(), "Healthy Person", None)
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
            let bound = bound_vault(&vault, root);
            let listed = ListPeople::new(&bound).execute(false);
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
            let bound = bound_vault(&vault, root);
            let listed = ListPeople::new(&bound).execute(false);
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
            let bound = bound_vault(&vault, root);
            let created =
                CreatePerson::new(&bound).execute(PersonId::new(), "Duplicate Person", None);
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
            let listed = ListPeople::new(&bound).execute(false);
            assert!(listed.is_ok());
            if let Ok(listed) = listed {
                assert!(listed.is_empty());
            }
            assert!(bound.find(created.id).is_err());
            let colliding = PersonProfile::create(created.id, "Third Duplicate");
            assert!(colliding.is_ok());
            if let Ok(colliding) = colliding {
                assert!(matches!(
                    bound.create(&colliding),
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
                CreatePerson::new(&bound_vault(&vault, root))
                    .execute(PersonId::new(), "Healthy Person", None)
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
            let bound = bound_vault(&vault, root);
            let listed = ListPeople::new(&bound).execute(false);
            assert!(listed.is_ok());
            if let Ok(listed) = listed {
                assert_eq!(listed.len(), 1);
                assert_eq!(listed[0].display_name, "Healthy Person");
            }
        }
    }

    #[cfg(unix)]
    #[test]
    fn read_only_health_never_follows_a_manifest_symlink() {
        use std::os::unix::fs::symlink;

        let directory = tempdir();
        assert!(directory.is_ok());
        let Ok(directory) = directory else {
            return;
        };
        let root = directory.path().join("workspace");
        let vault = MarkdownVault::new();
        assert!(
            InitialiseWorkspace::new(vault.clone())
                .execute(
                    &root,
                    WorkspaceId::new(),
                    "People",
                    WorkspaceProfile::Personal,
                    BuildProfile::Airgap,
                    "en-IE",
                )
                .is_ok()
        );
        let manifest = root.join(".liaison/workspace.yaml");
        let outside = directory.path().join("outside-private.yaml");
        assert!(fs::rename(&manifest, &outside).is_ok());
        assert!(symlink(&outside, &manifest).is_ok());

        assert!(matches!(
            vault.inspect_health(&root),
            Err(WorkspaceError::Storage(_))
        ));
    }

    #[test]
    fn bound_create_stages_and_never_clobbers_an_existing_final_record() {
        let directory = tempdir();
        assert!(directory.is_ok());
        let Ok(directory) = directory else {
            return;
        };
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
        let person = PersonProfile::create(PersonId::new(), "External Record");
        assert!(person.is_ok());
        let Ok(person) = person else {
            return;
        };
        let filename = format!("{}--{}.md", slug(&person.display_name), person.id);
        let final_path = root.join("people").join(filename);
        assert!(fs::write(&final_path, "external edit\n").is_ok());
        let bound = bound_vault(&vault, root);

        assert!(matches!(
            bound.create(&person),
            Err(liaison_people::PeopleError::AlreadyExists)
        ));
        assert_eq!(
            fs::read_to_string(&final_path).as_deref().ok(),
            Some("external edit\n")
        );
        assert!(
            !root
                .join("people")
                .join(format!(".person-{}.tmp", person.id))
                .exists()
        );
    }
}
