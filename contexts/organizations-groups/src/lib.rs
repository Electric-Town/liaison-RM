//! Organizations and Groups bounded context.
//!
//! Owns organizations, groups, locations, and dated memberships without
//! collapsing them into mutable fields on a Person record.

#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use chrono::NaiveDate;
use liaison_shared_kernel::PersonId;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;
use uuid::Uuid;

macro_rules! identifier {
    ($name:ident, $kind:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            #[must_use]
            pub fn new() -> Self {
                Self(Uuid::now_v7())
            }

            pub fn parse(value: &str) -> Result<Self, DirectoryError> {
                Uuid::parse_str(value)
                    .map(Self)
                    .map_err(|_| DirectoryError::InvalidIdentifier {
                        kind: $kind,
                        value: value.to_owned(),
                    })
            }

            #[must_use]
            pub const fn as_uuid(self) -> Uuid {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

identifier!(OrganizationId, "organization");
identifier!(GroupId, "group");
identifier!(LocationId, "location");
identifier!(MembershipId, "membership");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationKind {
    Company,
    Nonprofit,
    School,
    Government,
    Community,
    Club,
    Religious,
    Vendor,
    Client,
    ServiceProvider,
    Informal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Organization {
    id: OrganizationId,
    name: String,
    kind: OrganizationKind,
    archived: bool,
}

impl Organization {
    pub fn new(
        id: OrganizationId,
        name: impl Into<String>,
        kind: OrganizationKind,
    ) -> Result<Self, DirectoryError> {
        Ok(Self {
            id,
            name: required(name, "organization name")?,
            kind,
            archived: false,
        })
    }

    #[must_use]
    pub const fn id(&self) -> OrganizationId {
        self.id
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn kind(&self) -> OrganizationKind {
        self.kind
    }

    #[must_use]
    pub const fn is_archived(&self) -> bool {
        self.archived
    }

    pub fn archive(&mut self) {
        self.archived = true;
    }

    pub fn restore(&mut self) {
        self.archived = false;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupKind {
    Static,
    Snapshot,
    Household,
    Team,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Group {
    id: GroupId,
    name: String,
    kind: GroupKind,
    archived: bool,
}

impl Group {
    pub fn new(
        id: GroupId,
        name: impl Into<String>,
        kind: GroupKind,
    ) -> Result<Self, DirectoryError> {
        Ok(Self {
            id,
            name: required(name, "group name")?,
            kind,
            archived: false,
        })
    }

    #[must_use]
    pub const fn id(&self) -> GroupId {
        self.id
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn kind(&self) -> GroupKind {
        self.kind
    }

    #[must_use]
    pub const fn is_archived(&self) -> bool {
        self.archived
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "id", rename_all = "snake_case")]
pub enum MembershipTarget {
    Organization(OrganizationId),
    Group(GroupId),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Membership {
    id: MembershipId,
    person_id: PersonId,
    target: MembershipTarget,
    role: String,
    department: Option<String>,
    cost_centre: Option<String>,
    location_id: Option<LocationId>,
    started_on: NaiveDate,
    ended_on: Option<NaiveDate>,
    primary: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MembershipSpec {
    pub id: MembershipId,
    pub person_id: PersonId,
    pub target: MembershipTarget,
    pub role: String,
    pub department: Option<String>,
    pub cost_centre: Option<String>,
    pub location_id: Option<LocationId>,
    pub started_on: NaiveDate,
    pub ended_on: Option<NaiveDate>,
    pub primary: bool,
}

impl Membership {
    pub fn new(spec: MembershipSpec) -> Result<Self, DirectoryError> {
        if spec.ended_on.is_some_and(|ended| ended < spec.started_on) {
            return Err(DirectoryError::InvalidMembershipDates);
        }
        Ok(Self {
            id: spec.id,
            person_id: spec.person_id,
            target: spec.target,
            role: required(spec.role, "membership role")?,
            department: optional(spec.department),
            cost_centre: optional(spec.cost_centre),
            location_id: spec.location_id,
            started_on: spec.started_on,
            ended_on: spec.ended_on,
            primary: spec.primary,
        })
    }

    #[must_use]
    pub const fn id(&self) -> MembershipId {
        self.id
    }

    #[must_use]
    pub const fn person_id(&self) -> PersonId {
        self.person_id
    }

    #[must_use]
    pub const fn target(&self) -> MembershipTarget {
        self.target
    }

    #[must_use]
    pub fn role(&self) -> &str {
        &self.role
    }

    #[must_use]
    pub const fn started_on(&self) -> NaiveDate {
        self.started_on
    }

    #[must_use]
    pub const fn ended_on(&self) -> Option<NaiveDate> {
        self.ended_on
    }

    #[must_use]
    pub const fn is_primary(&self) -> bool {
        self.primary
    }

    #[must_use]
    pub fn is_effective_on(&self, date: NaiveDate) -> bool {
        self.started_on <= date && self.ended_on.is_none_or(|ended| date <= ended)
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DirectoryError {
    #[error("invalid {kind} identifier: {value}")]
    InvalidIdentifier { kind: &'static str, value: String },
    #[error("{0} is required")]
    RequiredField(&'static str),
    #[error("membership end date cannot be before its start date")]
    InvalidMembershipDates,
}

fn required(value: impl Into<String>, field: &'static str) -> Result<String, DirectoryError> {
    let value = value.into().trim().to_owned();
    if value.is_empty() {
        Err(DirectoryError::RequiredField(field))
    } else {
        Ok(value)
    }
}

fn optional(value: Option<String>) -> Option<String> {
    value
        .map(|item| item.trim().to_owned())
        .filter(|item| !item.is_empty())
}

#[cfg(test)]
mod tests {
    use super::{
        Group, GroupId, GroupKind, Membership, MembershipId, MembershipSpec, MembershipTarget,
        Organization, OrganizationId, OrganizationKind,
    };
    use chrono::NaiveDate;
    use liaison_shared_kernel::PersonId;

    fn date(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(year, month, day)
    }

    #[test]
    fn organizations_and_groups_require_names_but_not_shared_identity() {
        assert!(Organization::new(
            OrganizationId::new(),
            "Acme",
            OrganizationKind::Company
        )
        .is_ok());
        assert!(Group::new(GroupId::new(), "", GroupKind::Static).is_err());
    }

    #[test]
    fn membership_supports_concurrent_history_and_as_of_queries() {
        let started = date(2024, 4, 1);
        let ended = date(2025, 5, 1);
        let current = date(2025, 1, 1);
        let after = date(2026, 1, 1);
        assert!(started.is_some() && ended.is_some() && current.is_some() && after.is_some());
        let (Some(started), Some(ended), Some(current), Some(after)) =
            (started, ended, current, after)
        else {
            return;
        };
        let membership = Membership::new(MembershipSpec {
            id: MembershipId::new(),
            person_id: PersonId::new(),
            target: MembershipTarget::Organization(OrganizationId::new()),
            role: "Executive Assistant".to_owned(),
            department: Some("Leadership".to_owned()),
            cost_centre: None,
            location_id: None,
            started_on: started,
            ended_on: Some(ended),
            primary: true,
        });
        assert!(membership.is_ok());
        let Ok(membership) = membership else {
            return;
        };
        assert!(membership.is_effective_on(current));
        assert!(!membership.is_effective_on(after));
    }

    #[test]
    fn membership_rejects_reversed_dates() {
        let started = date(2026, 1, 1);
        let ended = date(2025, 1, 1);
        assert!(started.is_some() && ended.is_some());
        let (Some(started), Some(ended)) = (started, ended) else {
            return;
        };
        let result = Membership::new(MembershipSpec {
            id: MembershipId::new(),
            person_id: PersonId::new(),
            target: MembershipTarget::Group(GroupId::new()),
            role: "Member".to_owned(),
            department: None,
            cost_centre: None,
            location_id: None,
            started_on: started,
            ended_on: Some(ended),
            primary: false,
        });
        assert!(result.is_err());
    }
}
