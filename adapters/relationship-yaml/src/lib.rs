//! Open YAML persistence for explicit relationship intent.

#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use liaison_relationships::{
    ContactCadence, RelationshipError, RelationshipProfile, RelationshipRepository,
    RelationshipTier, RelationshipUpdate,
};
use liaison_shared_kernel::{PersonId, Revision};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};
use tempfile::NamedTempFile;

const FORMAT: &str = "liaison-relationship";
const SCHEMA_VERSION: u32 = 1;
const WORKSPACE_MANIFEST: &str = ".liaison/workspace.yaml";

#[derive(Clone, Debug, Default)]
pub struct RelationshipYamlStore;

impl RelationshipYamlStore {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    fn directory(workspace: &Path) -> PathBuf {
        workspace.join("relationships")
    }

    fn path(workspace: &Path, person_id: PersonId) -> PathBuf {
        Self::directory(workspace).join(format!("{person_id}.yaml"))
    }

    fn ensure_workspace(workspace: &Path) -> Result<(), RelationshipError> {
        if workspace.join(WORKSPACE_MANIFEST).is_file() {
            Ok(())
        } else {
            Err(RelationshipError::Storage(
                "workspace manifest does not exist".to_owned(),
            ))
        }
    }
}

impl RelationshipRepository for RelationshipYamlStore {
    fn load(
        &self,
        workspace: &Path,
        person_id: PersonId,
    ) -> Result<RelationshipProfile, RelationshipError> {
        Self::ensure_workspace(workspace)?;
        read_document(&Self::path(workspace, person_id))?.into_domain()
    }

    fn list(&self, workspace: &Path) -> Result<Vec<RelationshipProfile>, RelationshipError> {
        Self::ensure_workspace(workspace)?;
        let directory = Self::directory(workspace);
        if !directory.exists() {
            return Ok(Vec::new());
        }

        let mut profiles = Vec::new();
        for entry in fs::read_dir(&directory).map_err(storage)? {
            let entry = entry.map_err(storage)?;
            let file_type = entry.file_type().map_err(storage)?;
            if file_type.is_symlink() {
                return Err(RelationshipError::Storage(format!(
                    "symbolic links are not allowed in relationship storage: {}",
                    entry.path().display()
                )));
            }
            if !file_type.is_file()
                || entry.path().extension().and_then(|value| value.to_str()) != Some("yaml")
            {
                continue;
            }
            profiles.push(read_document(&entry.path())?.into_domain()?);
        }
        profiles.sort_by_key(|profile| profile.person_id.to_string());
        Ok(profiles)
    }

    fn save(
        &self,
        workspace: &Path,
        relationship: &RelationshipProfile,
        expected_revision: Option<Revision>,
    ) -> Result<(), RelationshipError> {
        Self::ensure_workspace(workspace)?;
        let path = Self::path(workspace, relationship.person_id);
        fs::create_dir_all(Self::directory(workspace)).map_err(storage)?;

        match expected_revision {
            None => {
                if path.exists() {
                    return Err(RelationshipError::AlreadyExists);
                }
                let document = RelationshipDocument::from_domain(
                    relationship,
                    BTreeMap::new(),
                );
                write_new(&path, &render(&document)?)
            }
            Some(expected) => {
                let existing = read_document(&path)?;
                if existing.revision != expected {
                    return Err(RelationshipError::RevisionConflict {
                        expected: expected.get(),
                        found: existing.revision.get(),
                    });
                }
                let next = expected
                    .next()
                    .map_err(|_| RelationshipError::RevisionOverflow)?;
                if relationship.revision != expected && relationship.revision != next {
                    return Err(RelationshipError::RevisionConflict {
                        expected: next.get(),
                        found: relationship.revision.get(),
                    });
                }
                let document = RelationshipDocument::from_domain(
                    relationship,
                    existing.extra,
                );
                replace(&path, &render(&document)?)
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RelationshipDocument {
    format: String,
    schema_version: u32,
    person_id: PersonId,
    revision: Revision,
    relationship_type: Option<String>,
    tier: RelationshipTier,
    cadence: ContactCadence,
    last_contacted: Option<chrono::NaiveDate>,
    next_contact_due: Option<chrono::NaiveDate>,
    reason_to_contact: Option<String>,
    last_meaningful_topic: Option<String>,
    #[serde(default)]
    circles: BTreeSet<String>,
    paused_until: Option<chrono::NaiveDate>,
    #[serde(default)]
    do_not_contact: bool,
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl RelationshipDocument {
    fn from_domain(
        relationship: &RelationshipProfile,
        extra: BTreeMap<String, Value>,
    ) -> Self {
        Self {
            format: FORMAT.to_owned(),
            schema_version: SCHEMA_VERSION,
            person_id: relationship.person_id,
            revision: relationship.revision,
            relationship_type: relationship.relationship_type.clone(),
            tier: relationship.tier,
            cadence: relationship.cadence,
            last_contacted: relationship.last_contacted,
            next_contact_due: relationship.next_contact_due,
            reason_to_contact: relationship.reason_to_contact.clone(),
            last_meaningful_topic: relationship.last_meaningful_topic.clone(),
            circles: relationship.circles.clone(),
            paused_until: relationship.paused_until,
            do_not_contact: relationship.do_not_contact,
            extra,
        }
    }

    fn into_domain(self) -> Result<RelationshipProfile, RelationshipError> {
        if self.format != FORMAT {
            return Err(RelationshipError::Storage(format!(
                "unexpected relationship format: {}",
                self.format
            )));
        }
        if self.schema_version != SCHEMA_VERSION {
            return Err(RelationshipError::Storage(format!(
                "unsupported relationship schema: {}",
                self.schema_version
            )));
        }
        if self.revision.get() == 0 {
            return Err(RelationshipError::Storage(
                "relationship revision must be positive".to_owned(),
            ));
        }
        let mut profile = RelationshipProfile::create(
            self.person_id,
            RelationshipUpdate {
                relationship_type: self.relationship_type,
                tier: self.tier,
                cadence: self.cadence,
                last_contacted: self.last_contacted,
                next_contact_due: self.next_contact_due,
                reason_to_contact: self.reason_to_contact,
                last_meaningful_topic: self.last_meaningful_topic,
                circles: self.circles,
                paused_until: self.paused_until,
                do_not_contact: self.do_not_contact,
            },
        )?;
        profile.revision = self.revision;
        Ok(profile)
    }
}

fn read_document(path: &Path) -> Result<RelationshipDocument, RelationshipError> {
    let text = fs::read_to_string(path).map_err(|error| {
        if error.kind() == io::ErrorKind::NotFound {
            RelationshipError::NotFound
        } else {
            storage(error)
        }
    })?;
    serde_yaml::from_str(&text).map_err(storage)
}

fn render(document: &RelationshipDocument) -> Result<Vec<u8>, RelationshipError> {
    serde_yaml::to_string(document)
        .map(String::into_bytes)
        .map_err(storage)
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), RelationshipError> {
    let parent = path.parent().ok_or_else(|| {
        RelationshipError::Storage("relationship path has no parent".to_owned())
    })?;
    let mut temporary = NamedTempFile::new_in(parent).map_err(storage)?;
    temporary.write_all(bytes).map_err(storage)?;
    temporary.as_file().sync_all().map_err(storage)?;
    temporary
        .persist_noclobber(path)
        .map(|_| ())
        .map_err(|error| {
            if error.error.kind() == io::ErrorKind::AlreadyExists {
                RelationshipError::AlreadyExists
            } else {
                storage(error.error)
            }
        })
}

fn replace(path: &Path, bytes: &[u8]) -> Result<(), RelationshipError> {
    let parent = path.parent().ok_or_else(|| {
        RelationshipError::Storage("relationship path has no parent".to_owned())
    })?;
    let mut temporary = NamedTempFile::new_in(parent).map_err(storage)?;
    temporary.write_all(bytes).map_err(storage)?;
    temporary.as_file().sync_all().map_err(storage)?;
    temporary
        .persist(path)
        .map(|_| ())
        .map_err(|error| storage(error.error))
}

fn storage(error: impl std::fmt::Display) -> RelationshipError {
    RelationshipError::Storage(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::RelationshipYamlStore;
    use liaison_relationships::{
        ContactCadence, RelationshipProfile, RelationshipRepository,
        RelationshipTier, RelationshipUpdate,
    };
    use liaison_shared_kernel::PersonId;
    use std::{collections::BTreeSet, fs};
    use tempfile::tempdir;

    fn workspace() -> Result<tempfile::TempDir, Box<dyn std::error::Error>> {
        let directory = tempdir()?;
        fs::create_dir_all(directory.path().join(".liaison"))?;
        fs::write(
            directory.path().join(".liaison/workspace.yaml"),
            "format: liaison-workspace\n",
        )?;
        fs::create_dir_all(directory.path().join("relationships"))?;
        Ok(directory)
    }

    #[test]
    fn round_trips_readable_relationship_intent() -> Result<(), Box<dyn std::error::Error>> {
        let directory = workspace()?;
        let store = RelationshipYamlStore::new();
        let profile = RelationshipProfile::create(
            PersonId::new(),
            RelationshipUpdate {
                relationship_type: Some("friend".to_owned()),
                tier: RelationshipTier::Core,
                cadence: ContactCadence::Quarterly,
                circles: BTreeSet::from(["Close friends".to_owned()]),
                ..RelationshipUpdate::default()
            },
        )?;
        store.save(directory.path(), &profile, None)?;
        let loaded = store.load(directory.path(), profile.person_id)?;
        assert_eq!(loaded, profile);
        let text = fs::read_to_string(
            directory
                .path()
                .join("relationships")
                .join(format!("{}.yaml", profile.person_id)),
        )?;
        assert!(text.contains("format: liaison-relationship"));
        assert!(text.contains("Close friends"));
        Ok(())
    }

    #[test]
    fn preserves_unknown_keys_on_update() -> Result<(), Box<dyn std::error::Error>> {
        let directory = workspace()?;
        let store = RelationshipYamlStore::new();
        let mut profile = RelationshipProfile::create(
            PersonId::new(),
            RelationshipUpdate::default(),
        )?;
        store.save(directory.path(), &profile, None)?;
        let path = directory
            .path()
            .join("relationships")
            .join(format!("{}.yaml", profile.person_id));
        let original = fs::read_to_string(&path)?;
        fs::write(&path, format!("{original}x-example: retained\n"))?;
        let expected = profile.revision;
        profile.apply(RelationshipUpdate {
            relationship_type: Some("mentor".to_owned()),
            ..RelationshipUpdate::default()
        })?;
        store.save(directory.path(), &profile, Some(expected))?;
        assert!(fs::read_to_string(path)?.contains("x-example: retained"));
        Ok(())
    }

    #[test]
    fn rejects_stale_revision() -> Result<(), Box<dyn std::error::Error>> {
        let directory = workspace()?;
        let store = RelationshipYamlStore::new();
        let mut profile = RelationshipProfile::create(
            PersonId::new(),
            RelationshipUpdate::default(),
        )?;
        store.save(directory.path(), &profile, None)?;
        let stale = profile.revision;
        profile.apply(RelationshipUpdate {
            relationship_type: Some("friend".to_owned()),
            ..RelationshipUpdate::default()
        })?;
        store.save(directory.path(), &profile, Some(stale))?;
        assert!(store.save(directory.path(), &profile, Some(stale)).is_err());
        Ok(())
    }
}
