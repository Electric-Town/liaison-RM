//! People bounded context.
//!
//! Owns person profile identity and invariants. Persistence formats and address
//! book provider models remain outside this crate.

#![allow(clippy::module_name_repetitions, clippy::missing_errors_doc)]

use chrono::NaiveDate;
use liaison_shared_kernel::{PersonId, Revision};
use serde::{Deserialize, Serialize};
use std::path::Path;
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
    pub fn create(display_name: impl Into<String>) -> Result<Self, PeopleError> {
        let display_name = required(display_name.into(), "display name")?;
        Ok(Self {
            id: PersonId::new(),
            revision: Revision::INITIAL,
            display_name,
            aliases: Vec::new(),
            emails: Vec::new(),
            phones: Vec::new(),
            birthday: None,
            archived: false,
        })
    }

    pub fn rename(&mut self, display_name: impl Into<String>) -> Result<(), PeopleError> {
        self.display_name = required(display_name.into(), "display name")?;
        self.bump_revision()
    }

    pub fn add_email(&mut self, value: impl Into<String>, label: impl Into<String>) -> Result<(), PeopleError> {
        let email = EmailAddress::new(value, label)?;
        if !self.emails.contains(&email) {
            self.emails.push(email);
            self.bump_revision()?;
        }
        Ok(())
    }

    pub fn add_phone(&mut self, value: impl Into<String>, label: impl Into<String>) -> Result<(), PeopleError> {
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
        self.revision = self.revision.next().map_err(|_| PeopleError::RevisionOverflow)?;
        Ok(())
    }
}

fn required(value: String, field: &'static str) -> Result<String, PeopleError> {
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
        let value = required(value.into(), "email")?.to_lowercase();
        let label = required(label.into(), "email label")?;
        let mut parts = value.split('@');
        let local = parts.next().unwrap_or_default();
        let domain = parts.next().unwrap_or_default();
        if local.is_empty() || domain.is_empty() || parts.next().is_some() || !domain.contains('.') {
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
        let value = required(value.into(), "phone")?;
        let label = required(label.into(), "phone label")?;
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

pub trait PersonRepository: Send + Sync {
    fn create(&self, workspace: &Path, person: &PersonProfile) -> Result<(), PeopleError>;
    fn list(&self, workspace: &Path, include_archived: bool) -> Result<Vec<PersonProfile>, PeopleError>;
    fn find(&self, workspace: &Path, id: PersonId) -> Result<PersonProfile, PeopleError>;
    fn save(
        &self,
        workspace: &Path,
        person: &PersonProfile,
        expected_revision: Revision,
    ) -> Result<(), PeopleError>;
}

#[derive(Debug)]
pub struct CreatePerson<R> {
    repository: R,
}

impl<R> CreatePerson<R>
where
    R: PersonRepository,
{
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn execute(
        &self,
        workspace: &Path,
        display_name: impl Into<String>,
        email: Option<String>,
    ) -> Result<PersonProfile, PeopleError> {
        let mut person = PersonProfile::create(display_name)?;
        if let Some(email) = email {
            person.add_email(email, "primary")?;
        }
        self.repository.create(workspace, &person)?;
        Ok(person)
    }
}

#[derive(Debug)]
pub struct ListPeople<R> {
    repository: R,
}

impl<R> ListPeople<R>
where
    R: PersonRepository,
{
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn execute(&self, workspace: &Path, include_archived: bool) -> Result<Vec<PersonProfile>, PeopleError> {
        self.repository.list(workspace, include_archived)
    }
}

#[cfg(test)]
mod tests {
    use super::{PartialDate, PeopleError, PersonProfile};

    #[test]
    fn profile_requires_a_name() {
        let result = PersonProfile::create("   ");
        assert_eq!(result, Err(PeopleError::RequiredField("display name")));
    }

    #[test]
    fn unknown_birth_year_does_not_require_an_age() {
        let date = PartialDate::month_day(2, 29);
        assert!(matches!(date, Ok(PartialDate::MonthDay { month: 2, day: 29 })));
    }

    #[test]
    fn changing_profile_increments_revision() {
        let result = PersonProfile::create("Alex Murphy");
        assert!(result.is_ok());
        if let Ok(mut person) = result {
            let before = person.revision.get();
            assert!(person.add_email("alex@example.test", "work").is_ok());
            assert_eq!(person.revision.get(), before + 1);
        }
    }
}
