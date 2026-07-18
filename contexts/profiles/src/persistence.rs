use crate::{FieldId, FieldValue, ProfileError, ProfileSnapshot};
use liaison_shared_kernel::{PersonId, Revision};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileRecord {
    person_id: PersonId,
    revision: Revision,
    snapshot: ProfileSnapshot,
}

impl ProfileRecord {
    #[must_use]
    pub fn new(person_id: PersonId) -> Self {
        Self {
            person_id,
            revision: Revision::INITIAL,
            snapshot: ProfileSnapshot::new(),
        }
    }

    #[must_use]
    pub const fn person_id(&self) -> PersonId {
        self.person_id
    }

    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    #[must_use]
    pub const fn snapshot(&self) -> &ProfileSnapshot {
        &self.snapshot
    }

    #[must_use]
    pub fn into_snapshot(self) -> ProfileSnapshot {
        self.snapshot
    }

    pub fn set_value(&mut self, value: FieldValue) -> Result<(), ProfilePersistenceError> {
        if value.is_sealed() {
            return Err(ProfilePersistenceError::SealedValueUnsupported(
                value.field_id().clone(),
            ));
        }
        self.snapshot.set(value);
        self.revision = self
            .revision
            .next()
            .map_err(|_| ProfilePersistenceError::RevisionOverflow)?;
        Ok(())
    }

    pub fn from_parts(
        person_id: PersonId,
        revision: Revision,
        snapshot: ProfileSnapshot,
    ) -> Result<Self, ProfilePersistenceError> {
        if let Some(value) = snapshot.values().values().find(|value| value.is_sealed()) {
            return Err(ProfilePersistenceError::SealedValueUnsupported(
                value.field_id().clone(),
            ));
        }
        Ok(Self {
            person_id,
            revision,
            snapshot,
        })
    }
}

pub trait ProfileRepository {
    fn load(
        &self,
        workspace: &Path,
        person_id: PersonId,
    ) -> Result<ProfileRecord, ProfilePersistenceError>;

    fn save(
        &self,
        workspace: &Path,
        record: &ProfileRecord,
        expected_revision: Option<Revision>,
    ) -> Result<(), ProfilePersistenceError>;
}

#[derive(Debug)]
pub struct LoadProfile<Repository> {
    repository: Repository,
}

impl<Repository> LoadProfile<Repository>
where
    Repository: ProfileRepository,
{
    #[must_use]
    pub const fn new(repository: Repository) -> Self {
        Self { repository }
    }

    pub fn execute(
        &self,
        workspace: &Path,
        person_id: PersonId,
    ) -> Result<ProfileRecord, ProfilePersistenceError> {
        self.repository.load(workspace, person_id)
    }
}

#[derive(Debug)]
pub struct SaveProfile<Repository> {
    repository: Repository,
}

impl<Repository> SaveProfile<Repository>
where
    Repository: ProfileRepository,
{
    #[must_use]
    pub const fn new(repository: Repository) -> Self {
        Self { repository }
    }

    pub fn execute(
        &self,
        workspace: &Path,
        record: &ProfileRecord,
        expected_revision: Option<Revision>,
    ) -> Result<(), ProfilePersistenceError> {
        self.repository.save(workspace, record, expected_revision)
    }
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum ProfilePersistenceError {
    #[error("profile record was not found")]
    NotFound,
    #[error("profile record already exists")]
    AlreadyExists,
    #[error("profile revision conflict: expected {expected}, found {found}")]
    RevisionConflict { expected: u64, found: u64 },
    #[error("profile revision overflowed")]
    RevisionOverflow,
    #[error("sealed field {0} requires an approved sealed-store adapter")]
    SealedValueUnsupported(FieldId),
    #[error("profile storage error: {0}")]
    Storage(String),
    #[error(transparent)]
    Domain(#[from] ProfileError),
}

#[cfg(test)]
mod tests {
    use super::{ProfilePersistenceError, ProfileRecord};
    use crate::{
        Classification, FieldDefinition, FieldDefinitionSpec, FieldId, FieldType, FieldValue,
        InformationState,
    };
    use liaison_shared_kernel::PersonId;
    use std::collections::BTreeSet;

    fn definition(
        id: &str,
        classification: Classification,
        sealed_by_default: bool,
    ) -> Result<FieldDefinition, crate::ProfileError> {
        FieldDefinition::new(FieldDefinitionSpec {
            id: FieldId::parse(id)?,
            label: "Fixture field".to_owned(),
            field_type: FieldType::ShortText,
            classification,
            required_for: BTreeSet::new(),
            stale_after_days: None,
            sealed_by_default,
        })
    }

    #[test]
    fn setting_a_value_advances_the_record_revision() {
        let definition = definition("identity.preferred_channel", Classification::Private, false);
        assert!(definition.is_ok());
        let Ok(definition) = definition else {
            return;
        };
        let value = FieldValue::new(
            &definition,
            InformationState::Known,
            Some("email".to_owned()),
            false,
        );
        assert!(value.is_ok());
        let Ok(value) = value else {
            return;
        };
        let mut record = ProfileRecord::new(PersonId::new());
        assert!(record.set_value(value).is_ok());
        assert_eq!(record.revision().get(), 2);
    }

    #[test]
    fn sealed_values_fail_closed_without_a_sealed_store() {
        let definition = definition("travel.accessibility", Classification::Sensitive, true);
        assert!(definition.is_ok());
        let Ok(definition) = definition else {
            return;
        };
        let value = FieldValue::new(
            &definition,
            InformationState::Verified,
            Some("Step-free route".to_owned()),
            true,
        );
        assert!(value.is_ok());
        let Ok(value) = value else {
            return;
        };
        let mut record = ProfileRecord::new(PersonId::new());
        assert!(matches!(
            record.set_value(value),
            Err(ProfilePersistenceError::SealedValueUnsupported(_))
        ));
    }
}
