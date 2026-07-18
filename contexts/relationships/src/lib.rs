//! Relationships bounded context.
//!
//! Owns explicit relationship intent, cadence, circles, boundaries, and
//! explainable maintenance status. Interaction evidence remains in the
//! Interactions and Commitments context.

#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use chrono::{Days, NaiveDate};
use liaison_shared_kernel::{PersonId, Revision};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipTier {
    Core,
    Active,
    #[default]
    Warm,
    Loose,
    Paused,
    Archive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(tag = "kind", content = "days", rename_all = "snake_case")]
pub enum ContactCadence {
    #[default]
    None,
    Monthly,
    Quarterly,
    TwiceYearly,
    Yearly,
    Custom(u16),
}

impl ContactCadence {
    pub fn validate(self) -> Result<Self, RelationshipError> {
        if let Self::Custom(days) = self
            && !(1..=3_650).contains(&days)
        {
            return Err(RelationshipError::InvalidCustomCadence(days));
        }
        Ok(self)
    }

    #[must_use]
    pub const fn interval_days(self) -> Option<u64> {
        match self {
            Self::None => None,
            Self::Monthly => Some(30),
            Self::Quarterly => Some(91),
            Self::TwiceYearly => Some(183),
            Self::Yearly => Some(365),
            Self::Custom(days) => Some(u64::from(days)),
        }
    }

    #[must_use]
    pub fn due_after(self, date: NaiveDate) -> Option<NaiveDate> {
        self.interval_days()
            .and_then(|days| date.checked_add_days(Days::new(days)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaintenanceStatus {
    NoCadence,
    OnTrack,
    DueSoon,
    DueToday,
    Overdue,
    Paused,
    DoNotContact,
    Archived,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationshipUpdate {
    pub relationship_type: Option<String>,
    pub tier: RelationshipTier,
    pub cadence: ContactCadence,
    pub last_contacted: Option<NaiveDate>,
    pub next_contact_due: Option<NaiveDate>,
    pub reason_to_contact: Option<String>,
    pub last_meaningful_topic: Option<String>,
    pub circles: BTreeSet<String>,
    pub paused_until: Option<NaiveDate>,
    pub do_not_contact: bool,
}

impl Default for RelationshipUpdate {
    fn default() -> Self {
        Self {
            relationship_type: None,
            tier: RelationshipTier::Warm,
            cadence: ContactCadence::None,
            last_contacted: None,
            next_contact_due: None,
            reason_to_contact: None,
            last_meaningful_topic: None,
            circles: BTreeSet::new(),
            paused_until: None,
            do_not_contact: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationshipProfile {
    pub person_id: PersonId,
    pub revision: Revision,
    pub relationship_type: Option<String>,
    pub tier: RelationshipTier,
    pub cadence: ContactCadence,
    pub last_contacted: Option<NaiveDate>,
    pub next_contact_due: Option<NaiveDate>,
    pub reason_to_contact: Option<String>,
    pub last_meaningful_topic: Option<String>,
    pub circles: BTreeSet<String>,
    pub paused_until: Option<NaiveDate>,
    pub do_not_contact: bool,
}

impl RelationshipProfile {
    pub fn create(
        person_id: PersonId,
        update: RelationshipUpdate,
    ) -> Result<Self, RelationshipError> {
        let normalized = normalize_update(update)?;
        Ok(Self {
            person_id,
            revision: Revision::INITIAL,
            relationship_type: normalized.relationship_type,
            tier: normalized.tier,
            cadence: normalized.cadence,
            last_contacted: normalized.last_contacted,
            next_contact_due: normalized.next_contact_due,
            reason_to_contact: normalized.reason_to_contact,
            last_meaningful_topic: normalized.last_meaningful_topic,
            circles: normalized.circles,
            paused_until: normalized.paused_until,
            do_not_contact: normalized.do_not_contact,
        })
    }

    pub fn apply(&mut self, update: RelationshipUpdate) -> Result<(), RelationshipError> {
        let normalized = normalize_update(update)?;
        let changed = self.relationship_type != normalized.relationship_type
            || self.tier != normalized.tier
            || self.cadence != normalized.cadence
            || self.last_contacted != normalized.last_contacted
            || self.next_contact_due != normalized.next_contact_due
            || self.reason_to_contact != normalized.reason_to_contact
            || self.last_meaningful_topic != normalized.last_meaningful_topic
            || self.circles != normalized.circles
            || self.paused_until != normalized.paused_until
            || self.do_not_contact != normalized.do_not_contact;

        if changed {
            self.relationship_type = normalized.relationship_type;
            self.tier = normalized.tier;
            self.cadence = normalized.cadence;
            self.last_contacted = normalized.last_contacted;
            self.next_contact_due = normalized.next_contact_due;
            self.reason_to_contact = normalized.reason_to_contact;
            self.last_meaningful_topic = normalized.last_meaningful_topic;
            self.circles = normalized.circles;
            self.paused_until = normalized.paused_until;
            self.do_not_contact = normalized.do_not_contact;
            self.revision = self
                .revision
                .next()
                .map_err(|_| RelationshipError::RevisionOverflow)?;
        }
        Ok(())
    }

    #[must_use]
    pub fn effective_due_date(&self) -> Option<NaiveDate> {
        self.next_contact_due.or_else(|| {
            self.last_contacted
                .and_then(|last| self.cadence.due_after(last))
        })
    }

    #[must_use]
    pub fn maintenance_status(
        &self,
        as_of: NaiveDate,
        person_archived: bool,
    ) -> MaintenanceStatus {
        if person_archived || self.tier == RelationshipTier::Archive {
            return MaintenanceStatus::Archived;
        }
        if self.do_not_contact {
            return MaintenanceStatus::DoNotContact;
        }
        if self.tier == RelationshipTier::Paused
            || self.paused_until.is_some_and(|until| until >= as_of)
        {
            return MaintenanceStatus::Paused;
        }
        let Some(due) = self.effective_due_date() else {
            return MaintenanceStatus::NoCadence;
        };
        if due < as_of {
            MaintenanceStatus::Overdue
        } else if due == as_of {
            MaintenanceStatus::DueToday
        } else if as_of
            .checked_add_days(Days::new(14))
            .is_some_and(|window| due <= window)
        {
            MaintenanceStatus::DueSoon
        } else {
            MaintenanceStatus::OnTrack
        }
    }
}

fn normalize_update(update: RelationshipUpdate) -> Result<RelationshipUpdate, RelationshipError> {
    let circles = update
        .circles
        .into_iter()
        .map(|value| normalize_required(value, "circle"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    Ok(RelationshipUpdate {
        relationship_type: normalize_optional(update.relationship_type),
        tier: update.tier,
        cadence: update.cadence.validate()?,
        last_contacted: update.last_contacted,
        next_contact_due: update.next_contact_due,
        reason_to_contact: normalize_optional(update.reason_to_contact),
        last_meaningful_topic: normalize_optional(update.last_meaningful_topic),
        circles,
        paused_until: update.paused_until,
        do_not_contact: update.do_not_contact,
    })
}

fn normalize_required(value: String, field: &'static str) -> Result<String, RelationshipError> {
    let value = value.trim().to_owned();
    if value.is_empty() {
        Err(RelationshipError::RequiredField(field))
    } else if value.chars().count() > 120 {
        Err(RelationshipError::FieldTooLong(field))
    } else {
        Ok(value)
    }
}

fn normalize_optional(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let value = value.trim().to_owned();
        (!value.is_empty()).then_some(value)
    })
}

pub trait RelationshipRepository: Send + Sync {
    fn load(
        &self,
        workspace: &Path,
        person_id: PersonId,
    ) -> Result<RelationshipProfile, RelationshipError>;
    fn list(&self, workspace: &Path) -> Result<Vec<RelationshipProfile>, RelationshipError>;
    fn save(
        &self,
        workspace: &Path,
        relationship: &RelationshipProfile,
        expected_revision: Option<Revision>,
    ) -> Result<(), RelationshipError>;
}

#[derive(Debug)]
pub struct GetRelationship<R> {
    repository: R,
}

impl<R> GetRelationship<R>
where
    R: RelationshipRepository,
{
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn execute(
        &self,
        workspace: &Path,
        person_id: PersonId,
    ) -> Result<Option<RelationshipProfile>, RelationshipError> {
        match self.repository.load(workspace, person_id) {
            Ok(value) => Ok(Some(value)),
            Err(RelationshipError::NotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }
}

#[derive(Debug)]
pub struct ListRelationships<R> {
    repository: R,
}

impl<R> ListRelationships<R>
where
    R: RelationshipRepository,
{
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn execute(&self, workspace: &Path) -> Result<Vec<RelationshipProfile>, RelationshipError> {
        self.repository.list(workspace)
    }
}

#[derive(Debug)]
pub struct SaveRelationship<R> {
    repository: R,
}

impl<R> SaveRelationship<R>
where
    R: RelationshipRepository,
{
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn execute(
        &self,
        workspace: &Path,
        person_id: PersonId,
        expected_revision: Option<Revision>,
        update: RelationshipUpdate,
    ) -> Result<RelationshipProfile, RelationshipError> {
        match expected_revision {
            None => {
                match self.repository.load(workspace, person_id) {
                    Ok(_) => return Err(RelationshipError::AlreadyExists),
                    Err(RelationshipError::NotFound) => {}
                    Err(error) => return Err(error),
                }
                let relationship = RelationshipProfile::create(person_id, update)?;
                self.repository.save(workspace, &relationship, None)?;
                Ok(relationship)
            }
            Some(expected) => {
                let mut relationship = self.repository.load(workspace, person_id)?;
                if relationship.revision != expected {
                    return Err(RelationshipError::RevisionConflict {
                        expected: expected.get(),
                        found: relationship.revision.get(),
                    });
                }
                relationship.apply(update)?;
                self.repository
                    .save(workspace, &relationship, Some(expected))?;
                Ok(relationship)
            }
        }
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum RelationshipError {
    #[error("{0} is required")]
    RequiredField(&'static str),
    #[error("{0} is longer than 120 characters")]
    FieldTooLong(&'static str),
    #[error("custom cadence must be between 1 and 3650 days; found {0}")]
    InvalidCustomCadence(u16),
    #[error("relationship intent already exists")]
    AlreadyExists,
    #[error("relationship intent does not exist")]
    NotFound,
    #[error("relationship revision precondition failed; expected {expected}, found {found}")]
    RevisionConflict { expected: u64, found: u64 },
    #[error("relationship revision overflowed")]
    RevisionOverflow,
    #[error("relationship storage error: {0}")]
    Storage(String),
}

#[cfg(test)]
mod tests {
    use super::{
        ContactCadence, MaintenanceStatus, RelationshipProfile, RelationshipTier,
        RelationshipUpdate,
    };
    use chrono::NaiveDate;
    use liaison_shared_kernel::PersonId;
    use std::collections::BTreeSet;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap_or(NaiveDate::MIN)
    }

    #[test]
    fn overdue_status_is_explainable_from_own_cadence() {
        let relationship = RelationshipProfile::create(
            PersonId::new(),
            RelationshipUpdate {
                cadence: ContactCadence::Monthly,
                last_contacted: Some(date(2026, 5, 1)),
                ..RelationshipUpdate::default()
            },
        );
        assert!(relationship.is_ok());
        let Ok(relationship) = relationship else {
            return;
        };
        assert_eq!(
            relationship.maintenance_status(date(2026, 7, 18), false),
            MaintenanceStatus::Overdue
        );
    }

    #[test]
    fn hard_boundaries_override_due_dates() {
        let relationship = RelationshipProfile::create(
            PersonId::new(),
            RelationshipUpdate {
                cadence: ContactCadence::Monthly,
                last_contacted: Some(date(2026, 1, 1)),
                do_not_contact: true,
                ..RelationshipUpdate::default()
            },
        );
        assert!(relationship.is_ok());
        let Ok(relationship) = relationship else {
            return;
        };
        assert_eq!(
            relationship.maintenance_status(date(2026, 7, 18), false),
            MaintenanceStatus::DoNotContact
        );
    }

    #[test]
    fn circle_names_are_trimmed_and_deduplicated() {
        let relationship = RelationshipProfile::create(
            PersonId::new(),
            RelationshipUpdate {
                tier: RelationshipTier::Core,
                circles: BTreeSet::from([" Family ".to_owned(), "Family".to_owned()]),
                ..RelationshipUpdate::default()
            },
        );
        assert!(relationship.is_ok());
        let Ok(relationship) = relationship else {
            return;
        };
        assert_eq!(relationship.circles, BTreeSet::from(["Family".to_owned()]));
    }
}
