//! People bounded context.
//!
//! Owns person profile identity and invariants. Persistence formats and address
//! book provider models remain outside this crate.

#![allow(clippy::module_name_repetitions, clippy::missing_errors_doc)]

use chrono::NaiveDate;
use liaison_shared_kernel::{PersonId, Revision};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersonProfile {
    pub id: PersonId,
    pub revision: Revision,
    pub display_name: String,
    pub aliases: Vec<String>,
    pub emails: Vec<EmailAddress>,
    pub phones: Vec<PhoneNumber>,
    pub birthday: Option<PartialDate>,
    pub archived: bool,
}

impl PersonProfile {
    pub fn create(id: PersonId, display_name: impl Into<String>) -> Result<Self, PeopleError> {
        let display_name = display_name.into();
        let display_name = required(&display_name, "display name")?;
        Ok(Self {
            id,
            revision: Revision::INITIAL,
            display_name,
            aliases: Vec::new(),
            emails: Vec::new(),
            phones: Vec::new(),
            birthday: None,
            archived: false,
        })
    }

    /// Creates the initial persisted profile, including an optional primary
    /// email, without representing the supplied creation fields as later
    /// revisions.
    pub fn create_with_primary_email(
        id: PersonId,
        display_name: impl Into<String>,
        primary_email: Option<String>,
    ) -> Result<Self, PeopleError> {
        let mut person = Self::create(id, display_name)?;
        if let Some(primary_email) = primary_email {
            person
                .emails
                .push(EmailAddress::new(primary_email, "primary")?);
        }
        Ok(person)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn rehydrate(
        id: PersonId,
        revision: Revision,
        display_name: impl Into<String>,
        aliases: Vec<String>,
        emails: Vec<EmailAddress>,
        phones: Vec<PhoneNumber>,
        birthday: Option<PartialDate>,
        archived: bool,
    ) -> Result<Self, PeopleError> {
        let display_name = display_name.into();
        let display_name = required(&display_name, "display name")?;
        let emails = emails
            .into_iter()
            .map(|email| EmailAddress::new(email.value, email.label))
            .collect::<Result<Vec<_>, _>>()?;
        let phones = phones
            .into_iter()
            .map(|phone| PhoneNumber::new(phone.value, phone.label))
            .collect::<Result<Vec<_>, _>>()?;
        let birthday = match birthday {
            Some(PartialDate::MonthDay { month, day }) => Some(PartialDate::month_day(month, day)?),
            value => value,
        };
        Ok(Self {
            id,
            revision,
            display_name,
            aliases,
            emails,
            phones,
            birthday,
            archived,
        })
    }

    pub fn rename(&mut self, display_name: impl Into<String>) -> Result<(), PeopleError> {
        let display_name = display_name.into();
        self.display_name = required(&display_name, "display name")?;
        self.bump_revision()
    }

    pub fn add_email(
        &mut self,
        value: impl Into<String>,
        label: impl Into<String>,
    ) -> Result<(), PeopleError> {
        let email = EmailAddress::new(value, label)?;
        if !self.emails.contains(&email) {
            self.emails.push(email);
            self.bump_revision()?;
        }
        Ok(())
    }

    pub fn add_phone(
        &mut self,
        value: impl Into<String>,
        label: impl Into<String>,
    ) -> Result<(), PeopleError> {
        let phone = PhoneNumber::new(value, label)?;
        if !self.phones.contains(&phone) {
            self.phones.push(phone);
            self.bump_revision()?;
        }
        Ok(())
    }

    pub fn set_birthday(&mut self, birthday: Option<PartialDate>) -> Result<(), PeopleError> {
        if self.birthday != birthday {
            self.birthday = birthday;
            self.bump_revision()?;
        }
        Ok(())
    }

    pub fn archive(&mut self) -> Result<(), PeopleError> {
        if !self.archived {
            self.archived = true;
            self.bump_revision()?;
        }
        Ok(())
    }

    pub fn restore(&mut self) -> Result<(), PeopleError> {
        if self.archived {
            self.archived = false;
            self.bump_revision()?;
        }
        Ok(())
    }

    fn bump_revision(&mut self) -> Result<(), PeopleError> {
        self.revision = self
            .revision
            .next()
            .map_err(|_| PeopleError::RevisionOverflow)?;
        Ok(())
    }
}

fn required(value: &str, field: &'static str) -> Result<String, PeopleError> {
    let value = value.trim().to_owned();
    if value.is_empty() {
        Err(PeopleError::RequiredField(field))
    } else {
        Ok(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailAddress {
    pub value: String,
    pub label: String,
}

impl EmailAddress {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Result<Self, PeopleError> {
        let value = value.into();
        let value = required(&value, "email")?.to_lowercase();
        let label = label.into();
        let label = required(&label, "email label")?;
        let mut parts = value.split('@');
        let local = parts.next().unwrap_or_default();
        let domain = parts.next().unwrap_or_default();
        if local.is_empty() || domain.is_empty() || parts.next().is_some() || !domain.contains('.')
        {
            return Err(PeopleError::InvalidEmail(value));
        }
        Ok(Self { value, label })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneNumber {
    pub value: String,
    pub label: String,
}

impl PhoneNumber {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Result<Self, PeopleError> {
        let value = value.into();
        let value = required(&value, "phone")?;
        let label = label.into();
        let label = required(&label, "phone label")?;
        if !value.chars().any(|character| character.is_ascii_digit()) {
            return Err(PeopleError::InvalidPhone(value));
        }
        Ok(Self { value, label })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "precision", rename_all = "kebab-case")]
pub enum PartialDate {
    Full { date: NaiveDate },
    MonthDay { month: u8, day: u8 },
}

impl PartialDate {
    pub fn month_day(month: u8, day: u8) -> Result<Self, PeopleError> {
        let validation_year = 2000;
        if NaiveDate::from_ymd_opt(validation_year, u32::from(month), u32::from(day)).is_none() {
            return Err(PeopleError::InvalidPartialDate { month, day });
        }
        Ok(Self::MonthDay { month, day })
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum PeopleError {
    #[error("{0} is required")]
    RequiredField(&'static str),
    #[error("invalid email address: {0}")]
    InvalidEmail(String),
    #[error("invalid phone number: {0}")]
    InvalidPhone(String),
    #[error("invalid month and day: {month:02}-{day:02}")]
    InvalidPartialDate { month: u8, day: u8 },
    #[error("person already exists")]
    AlreadyExists,
    #[error("person does not exist")]
    NotFound,
    #[error("person revision precondition failed; expected {expected}, found {found}")]
    RevisionConflict { expected: u64, found: u64 },
    #[error("person revision overflowed")]
    RevisionOverflow,
    #[error("people storage error: {0}")]
    Storage(String),
}

/// Person persistence already bound to the root owned by a `WorkspaceSession`.
/// Application commands use this port after opening; no later raw path can be
/// substituted for the writer-authoritative workspace.
pub trait PersonRepository: std::fmt::Debug + Send + Sync {
    fn create(&self, person: &PersonProfile) -> Result<(), PeopleError>;
    fn list(&self, include_archived: bool) -> Result<Vec<PersonProfile>, PeopleError>;
    fn find(&self, id: PersonId) -> Result<PersonProfile, PeopleError>;
    fn save(&self, person: &PersonProfile, expected_revision: Revision) -> Result<(), PeopleError>;
}

#[derive(Debug)]
pub struct CreatePerson<'repository, R> {
    repository: &'repository R,
}

impl<'repository, R> CreatePerson<'repository, R>
where
    R: PersonRepository,
{
    #[must_use]
    pub const fn new(repository: &'repository R) -> Self {
        Self { repository }
    }

    pub fn execute(
        &self,
        person_id: PersonId,
        display_name: impl Into<String>,
        email: Option<String>,
    ) -> Result<PersonProfile, PeopleError> {
        let person = PersonProfile::create_with_primary_email(person_id, display_name, email)?;
        self.repository.create(&person)?;
        Ok(person)
    }
}

#[derive(Debug)]
pub struct ListPeople<'repository, R> {
    repository: &'repository R,
}

impl<'repository, R> ListPeople<'repository, R>
where
    R: PersonRepository,
{
    #[must_use]
    pub const fn new(repository: &'repository R) -> Self {
        Self { repository }
    }

    pub fn execute(&self, include_archived: bool) -> Result<Vec<PersonProfile>, PeopleError> {
        self.repository.list(include_archived)
    }
}

#[cfg(test)]
mod tests {
    use super::{EmailAddress, PartialDate, PeopleError, PersonProfile, PhoneNumber};
    use liaison_shared_kernel::{PersonId, Revision};

    #[test]
    fn profile_requires_a_name() {
        let result = PersonProfile::create(PersonId::new(), "   ");
        assert_eq!(result, Err(PeopleError::RequiredField("display name")));
    }

    #[test]
    fn unknown_birth_year_does_not_require_an_age() {
        let date = PartialDate::month_day(2, 29);
        assert!(matches!(
            date,
            Ok(PartialDate::MonthDay { month: 2, day: 29 })
        ));
    }

    #[test]
    fn changing_profile_increments_revision() {
        let result = PersonProfile::create(PersonId::new(), "Alex Murphy");
        assert!(result.is_ok());
        if let Ok(mut person) = result {
            let before = person.revision.get();
            assert!(person.add_email("alex@example.test", "work").is_ok());
            assert_eq!(person.revision.get(), before + 1);
        }
    }

    #[test]
    fn creation_fields_share_the_initial_revision() {
        let result = PersonProfile::create_with_primary_email(
            PersonId::new(),
            "Alex Murphy",
            Some("ALEX@example.test".to_owned()),
        );
        assert!(result.is_ok());
        if let Ok(person) = result {
            assert_eq!(person.revision.get(), 1);
            assert_eq!(person.emails.len(), 1);
            assert_eq!(person.emails[0].value, "alex@example.test");
            assert_eq!(person.emails[0].label, "primary");
        }
    }

    #[test]
    fn rehydration_rechecks_serialized_contact_invariants() {
        let result = PersonProfile::rehydrate(
            PersonId::new(),
            Revision::INITIAL,
            "Alex Murphy",
            Vec::new(),
            vec![EmailAddress {
                value: "not-an-email".to_owned(),
                label: "primary".to_owned(),
            }],
            Vec::new(),
            None,
            false,
        );
        assert!(matches!(result, Err(PeopleError::InvalidEmail(_))));
    }

    #[test]
    fn rehydration_rechecks_serialized_phone_and_partial_date_invariants() {
        let invalid_phone = PersonProfile::rehydrate(
            PersonId::new(),
            Revision::INITIAL,
            "Alex Murphy",
            Vec::new(),
            Vec::new(),
            vec![PhoneNumber {
                value: "no digits".to_owned(),
                label: "mobile".to_owned(),
            }],
            None,
            false,
        );
        assert!(matches!(invalid_phone, Err(PeopleError::InvalidPhone(_))));

        let invalid_date = PersonProfile::rehydrate(
            PersonId::new(),
            Revision::INITIAL,
            "Alex Murphy",
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Some(PartialDate::MonthDay { month: 2, day: 30 }),
            false,
        );
        assert_eq!(
            invalid_date,
            Err(PeopleError::InvalidPartialDate { month: 2, day: 30 })
        );
    }
}
