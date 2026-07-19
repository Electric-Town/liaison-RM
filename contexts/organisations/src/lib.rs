//! Organisations and Groups bounded context.
//!
//! Owns organisations, groups, locations, and effective-dated memberships
//! with provenance, without collapsing them into mutable fields on a Person
//! record. Adapted from the preserved `agent/r4-organizations-groups-domain`
//! branch per its recorded disposition, renamed to the context map's
//! language, and extended with locations, membership provenance, and as-of
//! snapshot queries.

#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use chrono::NaiveDate;
use liaison_shared_kernel::PersonId;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;
use uuid::Uuid;

macro_rules! identifier {
    ($name:ident, $kind:literal) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            #[must_use]
            pub fn new() -> Self {
                Self(Uuid::now_v7())
            }

            pub fn parse(value: &str) -> Result<Self, OrganisationsError> {
                Uuid::parse_str(value).map(Self).map_err(|_| {
                    OrganisationsError::InvalidIdentifier {
                        kind: $kind,
                        value: value.to_owned(),
                    }
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

identifier!(OrganisationId, "organisation");
identifier!(GroupId, "group");
identifier!(LocationId, "location");
identifier!(MembershipId, "membership");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganisationKind {
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
pub struct Organisation {
    id: OrganisationId,
    name: String,
    kind: OrganisationKind,
    archived: bool,
}

impl Organisation {
    pub fn new(
        id: OrganisationId,
        name: impl Into<String>,
        kind: OrganisationKind,
    ) -> Result<Self, OrganisationsError> {
        Ok(Self {
            id,
            name: required(name, "organisation name")?,
            kind,
            archived: false,
        })
    }

    #[must_use]
    pub const fn id(&self) -> OrganisationId {
        self.id
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn kind(&self) -> OrganisationKind {
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

/// A stable place record: an office, site, campus, or venue. Filtering by
/// location never relies on an unvalidated free-text tag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    id: LocationId,
    name: String,
    organisation_id: Option<OrganisationId>,
    archived: bool,
}

impl Location {
    pub fn new(
        id: LocationId,
        name: impl Into<String>,
        organisation_id: Option<OrganisationId>,
    ) -> Result<Self, OrganisationsError> {
        Ok(Self {
            id,
            name: required(name, "location name")?,
            organisation_id,
            archived: false,
        })
    }

    #[must_use]
    pub const fn id(&self) -> LocationId {
        self.id
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn organisation_id(&self) -> Option<OrganisationId> {
        self.organisation_id
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
    ) -> Result<Self, OrganisationsError> {
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
    Organisation(OrganisationId),
    Group(GroupId),
}

/// Where a membership record came from (LRM-OR-002): a non-empty source
/// label such as an import job, a manual entry, or a directory sync.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MembershipSource(String);

impl MembershipSource {
    pub fn parse(value: impl Into<String>) -> Result<Self, OrganisationsError> {
        let value = value.into().trim().to_owned();
        if value.is_empty() {
            return Err(OrganisationsError::RequiredField("membership source"));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An effective-dated membership with provenance. A person moving
/// departments gets a new membership; history is never overwritten, so
/// reports retain the applicable snapshot.
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
    source: MembershipSource,
    recorded_on: NaiveDate,
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
    pub source: MembershipSource,
    pub recorded_on: NaiveDate,
}

impl Membership {
    pub fn new(spec: MembershipSpec) -> Result<Self, OrganisationsError> {
        if spec.ended_on.is_some_and(|ended| ended < spec.started_on) {
            return Err(OrganisationsError::InvalidMembershipDates);
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
            source: spec.source,
            recorded_on: spec.recorded_on,
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
    pub fn department(&self) -> Option<&str> {
        self.department.as_deref()
    }

    #[must_use]
    pub fn cost_centre(&self) -> Option<&str> {
        self.cost_centre.as_deref()
    }

    #[must_use]
    pub const fn location_id(&self) -> Option<LocationId> {
        self.location_id
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
    pub fn source(&self) -> &MembershipSource {
        &self.source
    }

    #[must_use]
    pub const fn recorded_on(&self) -> NaiveDate {
        self.recorded_on
    }

    #[must_use]
    pub fn is_effective_on(&self, date: NaiveDate) -> bool {
        self.started_on <= date && self.ended_on.is_none_or(|ended| date <= ended)
    }
}

/// The membership snapshot for one person at a date. Historical reports use
/// this so a later department move never rewrites what was true at the time.
#[must_use]
pub fn memberships_as_of(
    memberships: &[Membership],
    person: PersonId,
    date: NaiveDate,
) -> Vec<&Membership> {
    memberships
        .iter()
        .filter(|membership| membership.person_id() == person && membership.is_effective_on(date))
        .collect()
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum OrganisationsError {
    #[error("invalid {kind} identifier: {value}")]
    InvalidIdentifier { kind: &'static str, value: String },
    #[error("{0} is required")]
    RequiredField(&'static str),
    #[error("membership end date cannot be before its start date")]
    InvalidMembershipDates,
}

fn required(value: impl Into<String>, field: &'static str) -> Result<String, OrganisationsError> {
    let value = value.into().trim().to_owned();
    if value.is_empty() {
        Err(OrganisationsError::RequiredField(field))
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
        Group, GroupId, GroupKind, Location, LocationId, Membership, MembershipId,
        MembershipSource, MembershipSpec, MembershipTarget, Organisation, OrganisationId,
        OrganisationKind, memberships_as_of,
    };
    use chrono::NaiveDate;
    use liaison_shared_kernel::PersonId;

    fn date(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(year, month, day)
    }

    fn source() -> Option<MembershipSource> {
        MembershipSource::parse("directory import 2026-07").ok()
    }

    fn spec(
        person: PersonId,
        target: MembershipTarget,
        role: &str,
        department: Option<&str>,
        started: NaiveDate,
        ended: Option<NaiveDate>,
    ) -> Option<MembershipSpec> {
        Some(MembershipSpec {
            id: MembershipId::new(),
            person_id: person,
            target,
            role: role.to_owned(),
            department: department.map(ToOwned::to_owned),
            cost_centre: None,
            location_id: None,
            started_on: started,
            ended_on: ended,
            primary: true,
            source: source()?,
            recorded_on: started,
        })
    }

    #[test]
    fn organisations_groups_and_locations_require_names() {
        assert!(
            Organisation::new(
                OrganisationId::new(),
                "Electric Town",
                OrganisationKind::Company
            )
            .is_ok()
        );
        assert!(Group::new(GroupId::new(), "", GroupKind::Static).is_err());
        assert!(Location::new(LocationId::new(), "  ", None).is_err());
        let location = Location::new(LocationId::new(), "Dublin office", None);
        assert!(location.is_ok());
    }

    #[test]
    fn membership_records_provenance_and_rejects_reversed_dates() {
        let (Some(started), Some(ended)) = (date(2026, 1, 1), date(2025, 1, 1)) else {
            return;
        };
        let Some(reversed) = spec(
            PersonId::new(),
            MembershipTarget::Group(GroupId::new()),
            "Member",
            None,
            started,
            Some(ended),
        ) else {
            return;
        };
        assert!(Membership::new(reversed).is_err());

        let Some(valid) = spec(
            PersonId::new(),
            MembershipTarget::Organisation(OrganisationId::new()),
            "Executive Assistant",
            Some("Leadership"),
            ended,
            None,
        ) else {
            return;
        };
        let membership = Membership::new(valid);
        assert!(membership.is_ok());
        let Ok(membership) = membership else {
            return;
        };
        assert_eq!(membership.source().as_str(), "directory import 2026-07");
        assert_eq!(membership.department(), Some("Leadership"));
    }

    #[test]
    fn a_department_move_keeps_the_historical_snapshot_intact() {
        let person = PersonId::new();
        let organisation = MembershipTarget::Organisation(OrganisationId::new());
        let (Some(first_start), Some(first_end), Some(second_start)) =
            (date(2024, 4, 1), date(2025, 5, 1), date(2025, 5, 2))
        else {
            return;
        };
        let (Some(reception), Some(leadership)) = (
            spec(
                person,
                organisation,
                "Receptionist",
                Some("Reception"),
                first_start,
                Some(first_end),
            ),
            spec(
                person,
                organisation,
                "Executive Assistant",
                Some("Leadership"),
                second_start,
                None,
            ),
        ) else {
            return;
        };
        let (Ok(reception), Ok(leadership)) =
            (Membership::new(reception), Membership::new(leadership))
        else {
            return;
        };
        let history = vec![reception, leadership];
        let (Some(during_first), Some(during_second)) = (date(2025, 1, 1), date(2026, 7, 1)) else {
            return;
        };
        let first_snapshot = memberships_as_of(&history, person, during_first);
        assert_eq!(first_snapshot.len(), 1);
        assert_eq!(first_snapshot[0].department(), Some("Reception"));
        let second_snapshot = memberships_as_of(&history, person, during_second);
        assert_eq!(second_snapshot.len(), 1);
        assert_eq!(second_snapshot[0].department(), Some("Leadership"));
        let other_person = memberships_as_of(&history, PersonId::new(), during_second);
        assert!(other_person.is_empty());
    }

    #[test]
    fn concurrent_memberships_appear_together_in_a_snapshot() {
        let person = PersonId::new();
        let (Some(start), Some(query)) = (date(2025, 1, 1), date(2025, 6, 1)) else {
            return;
        };
        let (Some(employment), Some(team)) = (
            spec(
                person,
                MembershipTarget::Organisation(OrganisationId::new()),
                "Engineer",
                Some("Platform"),
                start,
                None,
            ),
            spec(
                person,
                MembershipTarget::Group(GroupId::new()),
                "Member",
                None,
                start,
                None,
            ),
        ) else {
            return;
        };
        let (Ok(employment), Ok(team)) = (Membership::new(employment), Membership::new(team))
        else {
            return;
        };
        let history = vec![employment, team];
        assert_eq!(memberships_as_of(&history, person, query).len(), 2);
    }
}
