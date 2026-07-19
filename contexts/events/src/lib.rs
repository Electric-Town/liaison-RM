//! Events bounded context.
//!
//! Owns event identity, the attendee lifecycle, immutable cohort revisions,
//! ordered fail-closed dietary readiness derivation, and least-disclosure
//! catering-brief content evidence. Dietary source authoring, persistence
//! formats, and delivery transports remain outside this crate.

#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use chrono::NaiveDate;
use liaison_shared_kernel::{MemberId, PersonId, Revision};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt::{self, Display, Formatter, Write as _};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EventId(Uuid);

impl EventId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    #[must_use]
    pub const fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for EventId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// A constrained, single-line catering instruction.
///
/// This is the only dietary text the Events context accepts. There is no
/// field capable of carrying a diagnosis, medical history, or diagnostic
/// narrative.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OperationalInstruction(String);

impl OperationalInstruction {
    pub const MAXIMUM_CHARACTERS: usize = 200;

    pub fn parse(value: impl Into<String>) -> Result<Self, EventsError> {
        let value = value.into().trim().to_owned();
        if value.is_empty() {
            return Err(EventsError::InvalidInstruction("must not be empty"));
        }
        if value.chars().any(char::is_control) {
            return Err(EventsError::InvalidInstruction(
                "must be a single line without control characters",
            ));
        }
        if value.chars().count() > Self::MAXIMUM_CHARACTERS {
            return Err(EventsError::InvalidInstruction(
                "must not exceed 200 characters",
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for OperationalInstruction {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Orthogonal availability fact: what dietary information exists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", content = "instruction", rename_all = "snake_case")]
pub enum Availability {
    Provided(OperationalInstruction),
    VerifiedNone,
    Pending,
    Declined,
    Unreachable,
    Unknown,
}

impl Availability {
    #[must_use]
    pub const fn class(&self) -> AvailabilityClass {
        match self {
            Self::Provided(_) => AvailabilityClass::Provided,
            Self::VerifiedNone => AvailabilityClass::VerifiedNone,
            Self::Pending => AvailabilityClass::Pending,
            Self::Declined => AvailabilityClass::Declined,
            Self::Unreachable => AvailabilityClass::Unreachable,
            Self::Unknown => AvailabilityClass::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityClass {
    Provided,
    VerifiedNone,
    Pending,
    Declined,
    Unreachable,
    Unknown,
}

impl AvailabilityClass {
    pub const ALL: [Self; 6] = [
        Self::Provided,
        Self::VerifiedNone,
        Self::Pending,
        Self::Declined,
        Self::Unreachable,
        Self::Unknown,
    ];
}

/// Orthogonal freshness fact, derived upstream and stored separately.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Freshness {
    Fresh,
    Stale,
}

impl Freshness {
    pub const ALL: [Self; 2] = [Self::Fresh, Self::Stale];
}

/// Orthogonal conflict fact between recorded sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictState {
    Consistent,
    Conflicting,
}

impl ConflictState {
    pub const ALL: [Self; 2] = [Self::Consistent, Self::Conflicting];
}

/// Orthogonal disclosure fact for the catering purpose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisclosureState {
    Allowed,
    ExcludedFromCatering,
}

impl DisclosureState {
    pub const ALL: [Self; 2] = [Self::Allowed, Self::ExcludedFromCatering];
}

/// The authorised dietary view the People context supplies for one person.
///
/// Availability, freshness, conflict, and disclosure stay separate source
/// facts; readiness derivation reads them and never writes them back.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DietaryOperationalView {
    pub availability: Availability,
    pub freshness: Freshness,
    pub conflict: ConflictState,
    pub disclosure: DisclosureState,
    pub profile_revision: Revision,
}

/// Exactly one explicit readiness outcome per attendee.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DietaryOutcome {
    VerifiedNone,
    Provided,
    Pending,
    Declined,
    Unreachable,
    ExcludedFromCatering,
    Conflicting,
    Stale,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadinessPolicyId(String);

impl ReadinessPolicyId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventsError> {
        let value = value.into();
        if !valid_label_id(&value) {
            return Err(EventsError::InvalidPolicyId(value));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ReadinessPolicyId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// One condition of an ordered readiness rule. `None` matches any value.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FactPattern {
    pub availability: Option<AvailabilityClass>,
    pub freshness: Option<Freshness>,
    pub conflict: Option<ConflictState>,
    pub disclosure: Option<DisclosureState>,
}

impl FactPattern {
    #[must_use]
    pub fn matches(&self, view: &DietaryOperationalView) -> bool {
        self.availability
            .is_none_or(|wanted| wanted == view.availability.class())
            && self.freshness.is_none_or(|wanted| wanted == view.freshness)
            && self.conflict.is_none_or(|wanted| wanted == view.conflict)
            && self
                .disclosure
                .is_none_or(|wanted| wanted == view.disclosure)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessRule {
    pub when: FactPattern,
    pub outcome: DietaryOutcome,
}

/// An ordered, versioned, fail-closed readiness decision table.
///
/// The first matching rule wins. A view no rule matches derives `Unknown`.
/// Construction rejects rules that could invent a `VerifiedNone` or
/// `Provided` outcome from any other availability fact, so unknown can never
/// become verified none.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessPolicy {
    id: ReadinessPolicyId,
    version: u32,
    rules: Vec<ReadinessRule>,
}

impl ReadinessPolicy {
    pub fn new(
        id: ReadinessPolicyId,
        version: u32,
        rules: Vec<ReadinessRule>,
    ) -> Result<Self, EventsError> {
        if version == 0 {
            return Err(EventsError::InvalidPolicyVersion);
        }
        if rules.is_empty() {
            return Err(EventsError::EmptyPolicy);
        }
        for (index, rule) in rules.iter().enumerate() {
            if rule.outcome == DietaryOutcome::VerifiedNone
                && rule.when.availability != Some(AvailabilityClass::VerifiedNone)
            {
                return Err(EventsError::UnsafeRule {
                    index,
                    reason: "verified none requires a verified none availability condition",
                });
            }
            if rule.outcome == DietaryOutcome::Provided
                && rule.when.availability != Some(AvailabilityClass::Provided)
            {
                return Err(EventsError::UnsafeRule {
                    index,
                    reason: "provided requires a provided availability condition",
                });
            }
        }
        Ok(Self { id, version, rules })
    }

    /// The initial B0 table: exclusion, then conflict, then staleness, then
    /// the availability states, with everything else failing closed.
    pub fn baseline() -> Result<Self, EventsError> {
        let availability = |class: AvailabilityClass| FactPattern {
            availability: Some(class),
            ..FactPattern::default()
        };
        Self::new(
            ReadinessPolicyId::parse("b0-baseline")?,
            1,
            vec![
                ReadinessRule {
                    when: FactPattern {
                        disclosure: Some(DisclosureState::ExcludedFromCatering),
                        ..FactPattern::default()
                    },
                    outcome: DietaryOutcome::ExcludedFromCatering,
                },
                ReadinessRule {
                    when: FactPattern {
                        conflict: Some(ConflictState::Conflicting),
                        ..FactPattern::default()
                    },
                    outcome: DietaryOutcome::Conflicting,
                },
                ReadinessRule {
                    when: FactPattern {
                        availability: Some(AvailabilityClass::Provided),
                        freshness: Some(Freshness::Stale),
                        ..FactPattern::default()
                    },
                    outcome: DietaryOutcome::Stale,
                },
                ReadinessRule {
                    when: FactPattern {
                        availability: Some(AvailabilityClass::VerifiedNone),
                        freshness: Some(Freshness::Stale),
                        ..FactPattern::default()
                    },
                    outcome: DietaryOutcome::Stale,
                },
                ReadinessRule {
                    when: availability(AvailabilityClass::Provided),
                    outcome: DietaryOutcome::Provided,
                },
                ReadinessRule {
                    when: availability(AvailabilityClass::VerifiedNone),
                    outcome: DietaryOutcome::VerifiedNone,
                },
                ReadinessRule {
                    when: availability(AvailabilityClass::Pending),
                    outcome: DietaryOutcome::Pending,
                },
                ReadinessRule {
                    when: availability(AvailabilityClass::Declined),
                    outcome: DietaryOutcome::Declined,
                },
                ReadinessRule {
                    when: availability(AvailabilityClass::Unreachable),
                    outcome: DietaryOutcome::Unreachable,
                },
            ],
        )
    }

    #[must_use]
    pub fn id(&self) -> &ReadinessPolicyId {
        &self.id
    }

    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version
    }

    #[must_use]
    pub fn rules(&self) -> &[ReadinessRule] {
        &self.rules
    }

    /// Derives exactly one outcome without writing to the source facts.
    #[must_use]
    pub fn derive(&self, view: &DietaryOperationalView) -> DerivedDietaryOutcome {
        for (index, rule) in self.rules.iter().enumerate() {
            if rule.when.matches(view) {
                return DerivedDietaryOutcome {
                    outcome: rule.outcome,
                    rule_index: Some(index),
                    policy_version: self.version,
                };
            }
        }
        DerivedDietaryOutcome {
            outcome: DietaryOutcome::Unknown,
            rule_index: None,
            policy_version: self.version,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedDietaryOutcome {
    pub outcome: DietaryOutcome,
    pub rule_index: Option<usize>,
    pub policy_version: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RowId(u32);

impl RowId {
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl Display for RowId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "row {}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AttendeeIdentity {
    Resolved { person: PersonId },
    Unresolved { source_label: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttendeeOrigin {
    Selected,
    WalkIn,
}

/// Recorded participation states from LRM-EV-002.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Participation {
    Invited,
    Confirmed,
    Attended,
    Declined,
    Cancelled,
    NoShow,
    Unknown,
}

impl Participation {
    /// Forward transitions an operator may record without a correction.
    #[must_use]
    pub fn may_progress_to(self, next: Self) -> bool {
        match self {
            Self::Unknown => next != Self::Unknown,
            Self::Invited => {
                matches!(next, Self::Confirmed | Self::Declined | Self::Cancelled)
            }
            Self::Confirmed => matches!(
                next,
                Self::Attended | Self::NoShow | Self::Cancelled | Self::Declined
            ),
            Self::Attended | Self::Declined | Self::Cancelled | Self::NoShow => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum RowStatus {
    Active,
    Removed { on: NaiveDate, reason: String },
    SupersededAsDuplicateOf { row: RowId, on: NaiveDate },
}

impl RowStatus {
    #[must_use]
    pub const fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}

/// One attendee row. Rows are append-only; corrections supersede rather than
/// erase, so historical and corrected rows stay inspectable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttendeeRow {
    pub row: RowId,
    pub identity: AttendeeIdentity,
    pub origin: AttendeeOrigin,
    pub participation: Participation,
    pub status: RowStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "change", rename_all = "snake_case")]
pub enum CorrectionChange {
    ParticipationChanged {
        from: Participation,
        to: Participation,
    },
    Removed {
        reason: String,
    },
    MarkedDuplicateOf {
        row: RowId,
    },
    IdentityResolved {
        person: PersonId,
    },
    WalkInAdded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Correction {
    pub sequence: u32,
    pub row: RowId,
    pub on: NaiveDate,
    pub change: CorrectionChange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CohortRevision(u32);

impl CohortRevision {
    pub const INITIAL: Self = Self(1);

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }

    const fn next(self) -> Result<Self, EventsError> {
        match self.0.checked_add(1) {
            Some(value) => Ok(Self(value)),
            None => Err(EventsError::RevisionOverflow),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum EventStatus {
    Planned,
    Cancelled { on: NaiveDate },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityResolution {
    Resolved,
    MergedAsDuplicate { of: RowId },
}

/// The Event aggregate: details, status, and the cohort lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    id: EventId,
    revision: Revision,
    name: String,
    date: NaiveDate,
    status: EventStatus,
    cohort_revision: CohortRevision,
    finalized_on: Option<NaiveDate>,
    next_row: u32,
    next_correction: u32,
    rows: Vec<AttendeeRow>,
    corrections: Vec<Correction>,
}

impl Event {
    pub fn create(name: impl Into<String>, date: NaiveDate) -> Result<Self, EventsError> {
        let name = name.into().trim().to_owned();
        if name.is_empty() {
            return Err(EventsError::RequiredField("event name"));
        }
        Ok(Self {
            id: EventId::new(),
            revision: Revision::INITIAL,
            name,
            date,
            status: EventStatus::Planned,
            cohort_revision: CohortRevision::INITIAL,
            finalized_on: None,
            next_row: 1,
            next_correction: 1,
            rows: Vec::new(),
            corrections: Vec::new(),
        })
    }

    #[must_use]
    pub const fn id(&self) -> EventId {
        self.id
    }

    #[must_use]
    pub const fn revision(&self) -> Revision {
        self.revision
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn date(&self) -> NaiveDate {
        self.date
    }

    #[must_use]
    pub const fn status(&self) -> &EventStatus {
        &self.status
    }

    #[must_use]
    pub const fn cohort_revision(&self) -> CohortRevision {
        self.cohort_revision
    }

    #[must_use]
    pub const fn finalized_on(&self) -> Option<NaiveDate> {
        self.finalized_on
    }

    #[must_use]
    pub fn rows(&self) -> &[AttendeeRow] {
        &self.rows
    }

    #[must_use]
    pub fn corrections(&self) -> &[Correction] {
        &self.corrections
    }

    pub fn active_rows(&self) -> impl Iterator<Item = &AttendeeRow> {
        self.rows.iter().filter(|row| row.status.is_active())
    }

    /// The active, deduplicated denominator every readiness calculation and
    /// count must reconcile against.
    #[must_use]
    pub fn active_denominator(&self) -> usize {
        self.active_rows().count()
    }

    pub fn cancel(&mut self, on: NaiveDate) -> Result<(), EventsError> {
        self.require_planned()?;
        self.status = EventStatus::Cancelled { on };
        self.bump()
    }

    /// Adds a person to the draft cohort selection.
    pub fn add_selected_person(&mut self, person: PersonId) -> Result<RowId, EventsError> {
        self.require_planned()?;
        if self.finalized_on.is_some() {
            return Err(EventsError::CohortAlreadyFinalized);
        }
        self.require_person_not_active(person)?;
        self.push_row(
            AttendeeIdentity::Resolved { person },
            AttendeeOrigin::Selected,
            Participation::Invited,
        )
    }

    /// Adds an identity the import source could not resolve to a person.
    pub fn add_unresolved(
        &mut self,
        source_label: impl Into<String>,
    ) -> Result<RowId, EventsError> {
        self.require_planned()?;
        if self.finalized_on.is_some() {
            return Err(EventsError::CohortAlreadyFinalized);
        }
        let source_label = source_label.into().trim().to_owned();
        if source_label.is_empty() {
            return Err(EventsError::RequiredField("unresolved source label"));
        }
        self.push_row(
            AttendeeIdentity::Unresolved { source_label },
            AttendeeOrigin::Selected,
            Participation::Unknown,
        )
    }

    /// Freezes the cohort selection. Later changes are superseding
    /// corrections, walk-ins, or identity work — never silent edits.
    pub fn finalize_cohort(&mut self, on: NaiveDate) -> Result<(), EventsError> {
        self.require_planned()?;
        if self.finalized_on.is_some() {
            return Err(EventsError::CohortAlreadyFinalized);
        }
        self.finalized_on = Some(on);
        self.bump()
    }

    /// Records a walk-in after finalization; walk-ins have attended.
    pub fn add_walk_in_person(
        &mut self,
        person: PersonId,
        on: NaiveDate,
    ) -> Result<RowId, EventsError> {
        self.require_planned()?;
        self.require_finalized()?;
        self.require_person_not_active(person)?;
        let row = self.push_row(
            AttendeeIdentity::Resolved { person },
            AttendeeOrigin::WalkIn,
            Participation::Attended,
        )?;
        self.push_correction(row, on, CorrectionChange::WalkInAdded)?;
        Ok(row)
    }

    /// Records a forward participation transition after finalization.
    pub fn record_participation(
        &mut self,
        row: RowId,
        to: Participation,
        on: NaiveDate,
    ) -> Result<(), EventsError> {
        self.require_planned()?;
        self.require_finalized()?;
        let from = self.active_row(row)?.participation;
        if !from.may_progress_to(to) {
            return Err(EventsError::InvalidTransition { from, to });
        }
        self.set_participation(row, from, to, on)
    }

    /// Supersedes a recorded participation state with a correction. Any
    /// change is allowed, and the previous value stays in the history.
    pub fn correct_participation(
        &mut self,
        row: RowId,
        to: Participation,
        on: NaiveDate,
    ) -> Result<(), EventsError> {
        self.require_planned()?;
        self.require_finalized()?;
        let from = self.active_row(row)?.participation;
        if from == to {
            return Ok(());
        }
        self.set_participation(row, from, to, on)
    }

    /// Removes an attendee row. The row and its history stay inspectable.
    pub fn remove_attendee(
        &mut self,
        row: RowId,
        reason: impl Into<String>,
        on: NaiveDate,
    ) -> Result<(), EventsError> {
        self.require_planned()?;
        let reason = reason.into().trim().to_owned();
        if reason.is_empty() {
            return Err(EventsError::RequiredField("removal reason"));
        }
        self.active_row(row)?;
        self.set_row_status(
            row,
            RowStatus::Removed {
                on,
                reason: reason.clone(),
            },
        )?;
        self.push_correction(row, on, CorrectionChange::Removed { reason })?;
        self.bump()
    }

    /// Resolves an unresolved identity. Resolving to a person who is already
    /// active supersedes this row as a duplicate instead of double counting.
    pub fn resolve_identity(
        &mut self,
        row: RowId,
        person: PersonId,
        on: NaiveDate,
    ) -> Result<IdentityResolution, EventsError> {
        self.require_planned()?;
        let existing = self.active_person_row(person);
        let target = self.active_row(row)?;
        if matches!(target.identity, AttendeeIdentity::Resolved { .. }) {
            return Err(EventsError::IdentityAlreadyResolved(row));
        }
        if let Some(duplicate_of) = existing {
            self.set_row_status(
                row,
                RowStatus::SupersededAsDuplicateOf {
                    row: duplicate_of,
                    on,
                },
            )?;
            self.push_correction(
                row,
                on,
                CorrectionChange::MarkedDuplicateOf { row: duplicate_of },
            )?;
            self.bump()?;
            return Ok(IdentityResolution::MergedAsDuplicate { of: duplicate_of });
        }
        self.set_row_identity(row, AttendeeIdentity::Resolved { person })?;
        self.push_correction(row, on, CorrectionChange::IdentityResolved { person })?;
        self.bump()?;
        Ok(IdentityResolution::Resolved)
    }

    /// Marks one active row a duplicate of another active row.
    pub fn mark_duplicate(
        &mut self,
        row: RowId,
        of: RowId,
        on: NaiveDate,
    ) -> Result<(), EventsError> {
        self.require_planned()?;
        if row == of {
            return Err(EventsError::RowNotActive(row));
        }
        self.active_row(of)?;
        self.active_row(row)?;
        self.set_row_status(row, RowStatus::SupersededAsDuplicateOf { row: of, on })?;
        self.push_correction(row, on, CorrectionChange::MarkedDuplicateOf { row: of })?;
        self.bump()
    }

    fn set_participation(
        &mut self,
        row: RowId,
        from: Participation,
        to: Participation,
        on: NaiveDate,
    ) -> Result<(), EventsError> {
        let entry = self
            .rows
            .iter_mut()
            .find(|entry| entry.row == row)
            .ok_or(EventsError::RowNotFound(row))?;
        entry.participation = to;
        self.push_correction(row, on, CorrectionChange::ParticipationChanged { from, to })?;
        self.bump()
    }

    fn set_row_status(&mut self, row: RowId, status: RowStatus) -> Result<(), EventsError> {
        let entry = self
            .rows
            .iter_mut()
            .find(|entry| entry.row == row)
            .ok_or(EventsError::RowNotFound(row))?;
        entry.status = status;
        Ok(())
    }

    fn set_row_identity(
        &mut self,
        row: RowId,
        identity: AttendeeIdentity,
    ) -> Result<(), EventsError> {
        let entry = self
            .rows
            .iter_mut()
            .find(|entry| entry.row == row)
            .ok_or(EventsError::RowNotFound(row))?;
        entry.identity = identity;
        Ok(())
    }

    fn push_row(
        &mut self,
        identity: AttendeeIdentity,
        origin: AttendeeOrigin,
        participation: Participation,
    ) -> Result<RowId, EventsError> {
        let row = RowId(self.next_row);
        self.next_row = self
            .next_row
            .checked_add(1)
            .ok_or(EventsError::RevisionOverflow)?;
        self.rows.push(AttendeeRow {
            row,
            identity,
            origin,
            participation,
            status: RowStatus::Active,
        });
        self.bump()?;
        Ok(row)
    }

    fn push_correction(
        &mut self,
        row: RowId,
        on: NaiveDate,
        change: CorrectionChange,
    ) -> Result<(), EventsError> {
        let sequence = self.next_correction;
        self.next_correction = sequence
            .checked_add(1)
            .ok_or(EventsError::RevisionOverflow)?;
        self.corrections.push(Correction {
            sequence,
            row,
            on,
            change,
        });
        Ok(())
    }

    fn active_row(&self, row: RowId) -> Result<&AttendeeRow, EventsError> {
        let entry = self
            .rows
            .iter()
            .find(|entry| entry.row == row)
            .ok_or(EventsError::RowNotFound(row))?;
        if entry.status.is_active() {
            Ok(entry)
        } else {
            Err(EventsError::RowNotActive(row))
        }
    }

    fn active_person_row(&self, person: PersonId) -> Option<RowId> {
        self.active_rows()
            .find(|row| matches!(row.identity, AttendeeIdentity::Resolved { person: existing } if existing == person))
            .map(|row| row.row)
    }

    fn require_person_not_active(&self, person: PersonId) -> Result<(), EventsError> {
        if self.active_person_row(person).is_some() {
            return Err(EventsError::DuplicateActivePerson(person));
        }
        Ok(())
    }

    fn require_planned(&self) -> Result<(), EventsError> {
        match self.status {
            EventStatus::Planned => Ok(()),
            EventStatus::Cancelled { .. } => Err(EventsError::EventCancelled),
        }
    }

    fn require_finalized(&self) -> Result<(), EventsError> {
        if self.finalized_on.is_none() {
            return Err(EventsError::CohortNotFinalized);
        }
        Ok(())
    }

    fn bump(&mut self) -> Result<(), EventsError> {
        self.revision = self
            .revision
            .next()
            .map_err(|_| EventsError::RevisionOverflow)?;
        self.cohort_revision = self.cohort_revision.next()?;
        Ok(())
    }
}

/// Exactly one outcome per active attendee row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AttendeeOutcome {
    Dietary {
        outcome: DietaryOutcome,
        rule_index: Option<usize>,
        instruction: Option<OperationalInstruction>,
        profile_revision: Option<Revision>,
    },
    UnresolvedIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttendeeReadiness {
    pub row: RowId,
    pub outcome: AttendeeOutcome,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessTotals {
    pub denominator: usize,
    pub verified_none: usize,
    pub provided: usize,
    pub pending: usize,
    pub declined: usize,
    pub unreachable: usize,
    pub excluded_from_catering: usize,
    pub conflicting: usize,
    pub stale: usize,
    pub unknown: usize,
    pub unresolved_identities: usize,
}

impl ReadinessTotals {
    /// True when every category plus unresolved identities reconciles
    /// exactly to the active deduplicated denominator.
    #[must_use]
    pub const fn reconciles(&self) -> bool {
        self.denominator
            == self.verified_none
                + self.provided
                + self.pending
                + self.declined
                + self.unreachable
                + self.excluded_from_catering
                + self.conflicting
                + self.stale
                + self.unknown
                + self.unresolved_identities
    }
}

/// A readiness calculation over one finalized cohort revision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessAssessment {
    pub event_id: EventId,
    pub cohort_revision: CohortRevision,
    pub policy_id: ReadinessPolicyId,
    pub policy_version: u32,
    pub as_of: NaiveDate,
    pub entries: Vec<AttendeeReadiness>,
    pub totals: ReadinessTotals,
    pub input_revisions: BTreeMap<PersonId, Revision>,
}

impl ReadinessAssessment {
    /// Derives one outcome for every active attendee. A resolved attendee
    /// without a supplied view fails closed to `Unknown`; an unresolved
    /// identity is counted, never silently dropped.
    pub fn assess(
        event: &Event,
        policy: &ReadinessPolicy,
        views: &BTreeMap<PersonId, DietaryOperationalView>,
        as_of: NaiveDate,
    ) -> Result<Self, EventsError> {
        event.require_planned()?;
        event.require_finalized()?;
        let mut entries = Vec::new();
        let mut totals = ReadinessTotals::default();
        let mut input_revisions = BTreeMap::new();
        for row in event.active_rows() {
            totals.denominator += 1;
            let outcome = match &row.identity {
                AttendeeIdentity::Unresolved { .. } => {
                    totals.unresolved_identities += 1;
                    AttendeeOutcome::UnresolvedIdentity
                }
                AttendeeIdentity::Resolved { person } => {
                    if let Some(view) = views.get(person) {
                        let derived = policy.derive(view);
                        Self::count(&mut totals, derived.outcome);
                        input_revisions.insert(*person, view.profile_revision);
                        let instruction = match (&view.availability, derived.outcome) {
                            (Availability::Provided(instruction), DietaryOutcome::Provided) => {
                                Some(instruction.clone())
                            }
                            _ => None,
                        };
                        AttendeeOutcome::Dietary {
                            outcome: derived.outcome,
                            rule_index: derived.rule_index,
                            instruction,
                            profile_revision: Some(view.profile_revision),
                        }
                    } else {
                        Self::count(&mut totals, DietaryOutcome::Unknown);
                        AttendeeOutcome::Dietary {
                            outcome: DietaryOutcome::Unknown,
                            rule_index: None,
                            instruction: None,
                            profile_revision: None,
                        }
                    }
                }
            };
            entries.push(AttendeeReadiness {
                row: row.row,
                outcome,
            });
        }
        let assessment = Self {
            event_id: event.id(),
            cohort_revision: event.cohort_revision(),
            policy_id: policy.id().clone(),
            policy_version: policy.version(),
            as_of,
            entries,
            totals,
            input_revisions,
        };
        if assessment.verify_reconciliation() {
            Ok(assessment)
        } else {
            Err(EventsError::TotalsDoNotReconcile)
        }
    }

    /// Re-checks that entries and totals reconcile exactly.
    #[must_use]
    pub fn verify_reconciliation(&self) -> bool {
        self.totals.reconciles() && self.entries.len() == self.totals.denominator
    }

    const fn count(totals: &mut ReadinessTotals, outcome: DietaryOutcome) {
        match outcome {
            DietaryOutcome::VerifiedNone => totals.verified_none += 1,
            DietaryOutcome::Provided => totals.provided += 1,
            DietaryOutcome::Pending => totals.pending += 1,
            DietaryOutcome::Declined => totals.declined += 1,
            DietaryOutcome::Unreachable => totals.unreachable += 1,
            DietaryOutcome::ExcludedFromCatering => totals.excluded_from_catering += 1,
            DietaryOutcome::Conflicting => totals.conflicting += 1,
            DietaryOutcome::Stale => totals.stale += 1,
            DietaryOutcome::Unknown => totals.unknown += 1,
        }
    }
}

fn valid_label_id(value: &str) -> bool {
    let mut segments = value.split('-');
    let Some(first) = segments.next() else {
        return false;
    };
    valid_label_segment(first) && segments.all(valid_label_segment)
}

fn valid_label_segment(value: &str) -> bool {
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    first.is_ascii_lowercase()
        && characters.all(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
}

/// A non-empty single-line label such as a recipient class or purpose.
fn required_line(value: impl Into<String>, field: &'static str) -> Result<String, EventsError> {
    let value = value.into().trim().to_owned();
    if value.is_empty() {
        return Err(EventsError::RequiredField(field));
    }
    if value.chars().any(char::is_control) {
        return Err(EventsError::InvalidInstruction(
            "must be a single line without control characters",
        ));
    }
    Ok(value)
}

/// How a brief may identify attendees. Names are structurally absent; an
/// opaque event-local token appears only under an explicitly named policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum IdentifierDisclosure {
    NamesAbsent,
    OpaqueAttendeeIdentifiers { approved_policy: String },
}

/// Least-disclosure catering-brief content: counts and grouped operational
/// instructions for a named recipient and purpose. There is no field for a
/// person's name, diagnosis, private note, or hidden row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BriefContent {
    pub event_id: EventId,
    pub cohort_revision: CohortRevision,
    pub policy_id: ReadinessPolicyId,
    pub policy_version: u32,
    pub as_of: NaiveDate,
    pub recipient: String,
    pub purpose: String,
    pub expires_on: NaiveDate,
    pub identifiers: IdentifierDisclosure,
    pub totals: ReadinessTotals,
    pub instruction_counts: BTreeMap<OperationalInstruction, usize>,
    pub instruction_tokens: BTreeMap<OperationalInstruction, Vec<String>>,
    pub input_revisions: BTreeMap<PersonId, Revision>,
}

impl BriefContent {
    pub fn from_assessment(
        assessment: &ReadinessAssessment,
        recipient: impl Into<String>,
        purpose: impl Into<String>,
        expires_on: NaiveDate,
        identifiers: IdentifierDisclosure,
    ) -> Result<Self, EventsError> {
        if let IdentifierDisclosure::OpaqueAttendeeIdentifiers { approved_policy } = &identifiers {
            required_line(approved_policy.clone(), "approved identifier policy")?;
        }
        let recipient = required_line(recipient, "brief recipient")?;
        let purpose = required_line(purpose, "brief purpose")?;
        let mut instruction_counts: BTreeMap<OperationalInstruction, usize> = BTreeMap::new();
        let mut instruction_tokens: BTreeMap<OperationalInstruction, Vec<String>> = BTreeMap::new();
        for entry in &assessment.entries {
            if let AttendeeOutcome::Dietary {
                instruction: Some(instruction),
                ..
            } = &entry.outcome
            {
                *instruction_counts.entry(instruction.clone()).or_default() += 1;
                if matches!(
                    identifiers,
                    IdentifierDisclosure::OpaqueAttendeeIdentifiers { .. }
                ) {
                    instruction_tokens
                        .entry(instruction.clone())
                        .or_default()
                        .push(format!("attendee-{}", entry.row.get()));
                }
            }
        }
        for tokens in instruction_tokens.values_mut() {
            tokens.sort();
        }
        Ok(Self {
            event_id: assessment.event_id,
            cohort_revision: assessment.cohort_revision,
            policy_id: assessment.policy_id.clone(),
            policy_version: assessment.policy_version,
            as_of: assessment.as_of,
            recipient,
            purpose,
            expires_on,
            identifiers,
            totals: assessment.totals,
            instruction_counts,
            instruction_tokens,
            input_revisions: assessment.input_revisions.clone(),
        })
    }

    /// The deterministic delivered payload. Preview and sealing render the
    /// same bytes; person identifiers and revisions never enter them.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut text = String::new();
        let _ = writeln!(text, "liaison-catering-brief v1");
        let _ = writeln!(text, "event: {}", self.event_id);
        let _ = writeln!(text, "cohort-revision: {}", self.cohort_revision.get());
        let _ = writeln!(text, "policy: {} v{}", self.policy_id, self.policy_version);
        let _ = writeln!(text, "as-of: {}", self.as_of);
        let _ = writeln!(text, "recipient: {}", self.recipient);
        let _ = writeln!(text, "purpose: {}", self.purpose);
        let _ = writeln!(text, "expires-on: {}", self.expires_on);
        match &self.identifiers {
            IdentifierDisclosure::NamesAbsent => {
                let _ = writeln!(text, "identifiers: names-absent");
            }
            IdentifierDisclosure::OpaqueAttendeeIdentifiers { approved_policy } => {
                let _ = writeln!(
                    text,
                    "identifiers: opaque-attendee-tokens (approved-policy: {approved_policy})"
                );
            }
        }
        let totals = &self.totals;
        let _ = writeln!(text, "denominator: {}", totals.denominator);
        let _ = writeln!(text, "verified-none: {}", totals.verified_none);
        let _ = writeln!(text, "provided: {}", totals.provided);
        let _ = writeln!(text, "pending: {}", totals.pending);
        let _ = writeln!(text, "declined: {}", totals.declined);
        let _ = writeln!(text, "unreachable: {}", totals.unreachable);
        let _ = writeln!(
            text,
            "excluded-from-catering: {}",
            totals.excluded_from_catering
        );
        let _ = writeln!(text, "conflicting: {}", totals.conflicting);
        let _ = writeln!(text, "stale: {}", totals.stale);
        let _ = writeln!(text, "unknown: {}", totals.unknown);
        let _ = writeln!(
            text,
            "unresolved-identities: {}",
            totals.unresolved_identities
        );
        let _ = writeln!(text, "instructions:");
        for (instruction, count) in &self.instruction_counts {
            match self.instruction_tokens.get(instruction) {
                Some(tokens) if !tokens.is_empty() => {
                    let _ = writeln!(
                        text,
                        "- {count} x {instruction} (tokens: {})",
                        tokens.join(", ")
                    );
                }
                _ => {
                    let _ = writeln!(text, "- {count} x {instruction}");
                }
            }
        }
        text.into_bytes()
    }

    #[must_use]
    pub fn checksum(&self) -> String {
        let digest = Sha256::digest(self.canonical_bytes());
        let mut text = String::with_capacity(digest.len() * 2);
        for byte in digest {
            let _ = write!(text, "{byte:02x}");
        }
        text
    }
}

/// A sealed immutable internal brief. Content bytes are frozen at sealing;
/// later source changes mark the brief stale without rewriting history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SealedBrief {
    content: BriefContent,
    bytes: Vec<u8>,
    checksum: String,
    creator: MemberId,
    sealed_on: NaiveDate,
    stale: bool,
}

impl SealedBrief {
    #[must_use]
    pub fn seal(content: BriefContent, creator: MemberId, sealed_on: NaiveDate) -> Self {
        let bytes = content.canonical_bytes();
        let checksum = content.checksum();
        Self {
            content,
            bytes,
            checksum,
            creator,
            sealed_on,
            stale: false,
        }
    }

    #[must_use]
    pub const fn content(&self) -> &BriefContent {
        &self.content
    }

    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[must_use]
    pub fn checksum(&self) -> &str {
        &self.checksum
    }

    #[must_use]
    pub const fn creator(&self) -> MemberId {
        self.creator
    }

    #[must_use]
    pub const fn sealed_on(&self) -> NaiveDate {
        self.sealed_on
    }

    #[must_use]
    pub const fn is_stale(&self) -> bool {
        self.stale
    }

    /// True when the preview bytes shown before generation match the sealed
    /// bytes exactly.
    #[must_use]
    pub fn matches_preview(&self, preview: &[u8]) -> bool {
        self.bytes == preview
    }

    /// True when any recorded input revision differs from the supplied
    /// current revisions, meaning the brief no longer reflects its sources.
    #[must_use]
    pub fn is_stale_against(&self, current: &BTreeMap<PersonId, Revision>) -> bool {
        self.content
            .input_revisions
            .iter()
            .any(|(person, revision)| current.get(person) != Some(revision))
    }

    /// Marks the brief stale. Its content, bytes, and checksum stay frozen.
    pub const fn mark_stale(&mut self) {
        self.stale = true;
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum EventsError {
    #[error("required field is missing: {0}")]
    RequiredField(&'static str),
    #[error("invalid operational instruction: {0}")]
    InvalidInstruction(&'static str),
    #[error("invalid readiness policy ID: {0}")]
    InvalidPolicyId(String),
    #[error("readiness policy version must be at least one")]
    InvalidPolicyVersion,
    #[error("readiness policy must contain at least one rule")]
    EmptyPolicy,
    #[error("unsafe readiness rule at index {index}: {reason}")]
    UnsafeRule { index: usize, reason: &'static str },
    #[error("event is cancelled")]
    EventCancelled,
    #[error("cohort is already finalized")]
    CohortAlreadyFinalized,
    #[error("cohort is not finalized")]
    CohortNotFinalized,
    #[error("attendee {0} does not exist")]
    RowNotFound(RowId),
    #[error("attendee {0} is not active")]
    RowNotActive(RowId),
    #[error("person {0} is already an active attendee")]
    DuplicateActivePerson(PersonId),
    #[error("attendee {0} already has a resolved identity")]
    IdentityAlreadyResolved(RowId),
    #[error("invalid participation transition from {from:?} to {to:?}")]
    InvalidTransition {
        from: Participation,
        to: Participation,
    },
    #[error("readiness totals do not reconcile to the active denominator")]
    TotalsDoNotReconcile,
    #[error("revision overflowed")]
    RevisionOverflow,
}

#[cfg(test)]
mod tests {
    use super::{
        Availability, AvailabilityClass, BriefContent, ConflictState, DietaryOperationalView,
        DietaryOutcome, DisclosureState, Event, EventsError, FactPattern, Freshness,
        IdentifierDisclosure, IdentityResolution, OperationalInstruction, Participation,
        ReadinessAssessment, ReadinessPolicy, ReadinessPolicyId, ReadinessRule, SealedBrief,
    };
    use chrono::NaiveDate;
    use liaison_shared_kernel::{MemberId, PersonId, Revision};
    use std::collections::BTreeMap;

    fn day(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 7, day).unwrap_or_default()
    }

    fn instruction(value: &str) -> Option<OperationalInstruction> {
        OperationalInstruction::parse(value).ok()
    }

    fn availability_for(class: AvailabilityClass) -> Option<Availability> {
        Some(match class {
            AvailabilityClass::Provided => Availability::Provided(instruction("no gluten")?),
            AvailabilityClass::VerifiedNone => Availability::VerifiedNone,
            AvailabilityClass::Pending => Availability::Pending,
            AvailabilityClass::Declined => Availability::Declined,
            AvailabilityClass::Unreachable => Availability::Unreachable,
            AvailabilityClass::Unknown => Availability::Unknown,
        })
    }

    fn view(
        availability: Availability,
        freshness: Freshness,
        conflict: ConflictState,
        disclosure: DisclosureState,
    ) -> DietaryOperationalView {
        DietaryOperationalView {
            availability,
            freshness,
            conflict,
            disclosure,
            profile_revision: Revision::INITIAL,
        }
    }

    fn provided_view(text: &str) -> Option<DietaryOperationalView> {
        Some(view(
            Availability::Provided(instruction(text)?),
            Freshness::Fresh,
            ConflictState::Consistent,
            DisclosureState::Allowed,
        ))
    }

    #[test]
    fn baseline_covers_every_fact_combination_with_exactly_one_safe_outcome() {
        let policy = ReadinessPolicy::baseline();
        assert!(policy.is_ok());
        let Ok(policy) = policy else {
            return;
        };
        let mut fail_closed_hits = 0;
        for class in AvailabilityClass::ALL {
            for freshness in Freshness::ALL {
                for conflict in ConflictState::ALL {
                    for disclosure in DisclosureState::ALL {
                        let Some(availability) = availability_for(class) else {
                            return;
                        };
                        let facts = view(availability, freshness, conflict, disclosure);
                        let before = facts.clone();
                        let derived = policy.derive(&facts);
                        assert_eq!(facts, before, "derivation must not overwrite source facts");
                        assert_eq!(derived.policy_version, policy.version());
                        if derived.rule_index.is_none() {
                            assert_eq!(derived.outcome, DietaryOutcome::Unknown);
                            fail_closed_hits += 1;
                        }
                        if derived.outcome == DietaryOutcome::VerifiedNone {
                            assert_eq!(class, AvailabilityClass::VerifiedNone);
                            assert_eq!(freshness, Freshness::Fresh);
                            assert_eq!(conflict, ConflictState::Consistent);
                            assert_eq!(disclosure, DisclosureState::Allowed);
                        }
                        if derived.outcome == DietaryOutcome::Provided {
                            assert_eq!(class, AvailabilityClass::Provided);
                            assert_eq!(freshness, Freshness::Fresh);
                        }
                        if disclosure == DisclosureState::ExcludedFromCatering {
                            assert_eq!(derived.outcome, DietaryOutcome::ExcludedFromCatering);
                        } else if conflict == ConflictState::Conflicting {
                            assert_eq!(derived.outcome, DietaryOutcome::Conflicting);
                        }
                    }
                }
            }
        }
        assert!(
            fail_closed_hits > 0,
            "the fail-closed default must be exercised"
        );
    }

    #[test]
    fn stale_beats_provided_and_verified_none_in_the_baseline() {
        let policy = ReadinessPolicy::baseline();
        assert!(policy.is_ok());
        let Ok(policy) = policy else {
            return;
        };
        let Some(provided) = instruction("no dairy") else {
            return;
        };
        let stale_provided = view(
            Availability::Provided(provided),
            Freshness::Stale,
            ConflictState::Consistent,
            DisclosureState::Allowed,
        );
        let stale_none = view(
            Availability::VerifiedNone,
            Freshness::Stale,
            ConflictState::Consistent,
            DisclosureState::Allowed,
        );
        assert_eq!(
            policy.derive(&stale_provided).outcome,
            DietaryOutcome::Stale
        );
        assert_eq!(policy.derive(&stale_none).outcome, DietaryOutcome::Stale);
    }

    #[test]
    fn a_rule_cannot_invent_verified_none_or_provided_from_other_availability() {
        let id = ReadinessPolicyId::parse("custom");
        assert!(id.is_ok());
        let Ok(id) = id else {
            return;
        };
        let unsafe_none = ReadinessPolicy::new(
            id.clone(),
            1,
            vec![ReadinessRule {
                when: FactPattern {
                    availability: Some(AvailabilityClass::Unknown),
                    ..FactPattern::default()
                },
                outcome: DietaryOutcome::VerifiedNone,
            }],
        );
        assert_eq!(
            unsafe_none,
            Err(EventsError::UnsafeRule {
                index: 0,
                reason: "verified none requires a verified none availability condition",
            })
        );
        let unsafe_provided = ReadinessPolicy::new(
            id,
            1,
            vec![ReadinessRule {
                when: FactPattern::default(),
                outcome: DietaryOutcome::Provided,
            }],
        );
        assert_eq!(
            unsafe_provided,
            Err(EventsError::UnsafeRule {
                index: 0,
                reason: "provided requires a provided availability condition",
            })
        );
    }

    #[test]
    fn forward_participation_follows_the_matrix_and_rejects_regressions() {
        let event = Event::create("summer lunch", day(30));
        assert!(event.is_ok());
        let Ok(mut event) = event else {
            return;
        };
        let row = event.add_selected_person(PersonId::new());
        assert!(row.is_ok());
        let Ok(row) = row else {
            return;
        };
        assert!(event.finalize_cohort(day(1)).is_ok());
        assert!(
            event
                .record_participation(row, Participation::Confirmed, day(2))
                .is_ok()
        );
        assert!(
            event
                .record_participation(row, Participation::Attended, day(30))
                .is_ok()
        );
        assert_eq!(
            event.record_participation(row, Participation::Invited, day(30)),
            Err(EventsError::InvalidTransition {
                from: Participation::Attended,
                to: Participation::Invited,
            })
        );
    }

    #[test]
    fn corrections_supersede_participation_and_preserve_history() {
        let event = Event::create("summer lunch", day(30));
        assert!(event.is_ok());
        let Ok(mut event) = event else {
            return;
        };
        let row = event.add_selected_person(PersonId::new());
        assert!(row.is_ok());
        let Ok(row) = row else {
            return;
        };
        assert!(event.finalize_cohort(day(1)).is_ok());
        assert!(
            event
                .record_participation(row, Participation::Confirmed, day(2))
                .is_ok()
        );
        assert!(
            event
                .record_participation(row, Participation::NoShow, day(30))
                .is_ok()
        );
        let corrections_before = event.corrections().len();
        assert!(
            event
                .correct_participation(row, Participation::Attended, day(31))
                .is_ok()
        );
        assert_eq!(event.corrections().len(), corrections_before + 1);
        let history: Vec<_> = event
            .corrections()
            .iter()
            .filter(|correction| correction.row == row)
            .collect();
        assert!(history.len() >= 3, "every change stays inspectable");
    }

    #[test]
    fn participation_requires_a_finalized_cohort() {
        let event = Event::create("summer lunch", day(30));
        assert!(event.is_ok());
        let Ok(mut event) = event else {
            return;
        };
        let row = event.add_selected_person(PersonId::new());
        assert!(row.is_ok());
        let Ok(row) = row else {
            return;
        };
        assert_eq!(
            event.record_participation(row, Participation::Confirmed, day(2)),
            Err(EventsError::CohortNotFinalized)
        );
    }

    #[test]
    fn selection_closes_at_finalization_and_walk_ins_open_after_it() {
        let event = Event::create("summer lunch", day(30));
        assert!(event.is_ok());
        let Ok(mut event) = event else {
            return;
        };
        let walk_in_before = event.add_walk_in_person(PersonId::new(), day(30));
        assert_eq!(walk_in_before, Err(EventsError::CohortNotFinalized));
        assert!(event.add_selected_person(PersonId::new()).is_ok());
        assert!(event.finalize_cohort(day(1)).is_ok());
        assert_eq!(
            event.add_selected_person(PersonId::new()),
            Err(EventsError::CohortAlreadyFinalized)
        );
        let walk_in = event.add_walk_in_person(PersonId::new(), day(30));
        assert!(walk_in.is_ok());
        assert_eq!(event.active_denominator(), 2);
    }

    #[test]
    fn duplicate_and_removal_corrections_reconcile_the_active_denominator() {
        let event = Event::create("summer lunch", day(30));
        assert!(event.is_ok());
        let Ok(mut event) = event else {
            return;
        };
        let person = PersonId::new();
        let first = event.add_selected_person(person);
        assert!(first.is_ok());
        let Ok(first) = first else {
            return;
        };
        assert_eq!(
            event.add_selected_person(person),
            Err(EventsError::DuplicateActivePerson(person))
        );
        let badge = event.add_unresolved("badge 0417");
        assert!(badge.is_ok());
        let Ok(badge) = badge else {
            return;
        };
        let other = event.add_selected_person(PersonId::new());
        assert!(other.is_ok());
        let Ok(other) = other else {
            return;
        };
        assert!(event.finalize_cohort(day(1)).is_ok());
        assert_eq!(event.active_denominator(), 3);
        let merged = event.resolve_identity(badge, person, day(2));
        assert_eq!(
            merged,
            Ok(IdentityResolution::MergedAsDuplicate { of: first })
        );
        assert_eq!(event.active_denominator(), 2);
        assert!(
            event
                .remove_attendee(other, "left the company", day(3))
                .is_ok()
        );
        assert_eq!(event.active_denominator(), 1);
        assert_eq!(event.rows().len(), 3, "history keeps every row inspectable");
    }

    #[test]
    fn resolving_a_new_identity_keeps_the_row_and_records_the_person() {
        let event = Event::create("summer lunch", day(30));
        assert!(event.is_ok());
        let Ok(mut event) = event else {
            return;
        };
        let badge = event.add_unresolved("badge 0911");
        assert!(badge.is_ok());
        let Ok(badge) = badge else {
            return;
        };
        let person = PersonId::new();
        assert_eq!(
            event.resolve_identity(badge, person, day(2)),
            Ok(IdentityResolution::Resolved)
        );
        assert_eq!(
            event.resolve_identity(badge, person, day(3)),
            Err(EventsError::IdentityAlreadyResolved(badge))
        );
        assert_eq!(event.active_denominator(), 1);
    }

    #[test]
    fn a_cancelled_event_refuses_further_cohort_work_and_assessment() {
        let event = Event::create("summer lunch", day(30));
        assert!(event.is_ok());
        let Ok(mut event) = event else {
            return;
        };
        assert!(event.add_selected_person(PersonId::new()).is_ok());
        assert!(event.finalize_cohort(day(1)).is_ok());
        assert!(event.cancel(day(2)).is_ok());
        assert_eq!(
            event.add_walk_in_person(PersonId::new(), day(30)),
            Err(EventsError::EventCancelled)
        );
        let policy = ReadinessPolicy::baseline();
        assert!(policy.is_ok());
        let Ok(policy) = policy else {
            return;
        };
        let assessment = ReadinessAssessment::assess(&event, &policy, &BTreeMap::new(), day(3));
        assert_eq!(assessment, Err(EventsError::EventCancelled));
    }

    fn assessed_event() -> Option<(Event, ReadinessAssessment, PersonId)> {
        let mut event = Event::create("summer lunch", day(30)).ok()?;
        let with_instruction = PersonId::new();
        let verified_none = PersonId::new();
        let no_view = PersonId::new();
        event.add_selected_person(with_instruction).ok()?;
        event.add_selected_person(verified_none).ok()?;
        event.add_selected_person(no_view).ok()?;
        event.add_unresolved("badge 0417").ok()?;
        event.finalize_cohort(day(1)).ok()?;
        let mut views = BTreeMap::new();
        views.insert(
            with_instruction,
            provided_view("no nuts; separate preparation")?,
        );
        views.insert(
            verified_none,
            view(
                Availability::VerifiedNone,
                Freshness::Fresh,
                ConflictState::Consistent,
                DisclosureState::Allowed,
            ),
        );
        let policy = ReadinessPolicy::baseline().ok()?;
        let assessment = ReadinessAssessment::assess(&event, &policy, &views, day(2)).ok()?;
        Some((event, assessment, with_instruction))
    }

    #[test]
    fn assessment_accounts_for_every_active_attendee_exactly_once() {
        let Some((event, assessment, _)) = assessed_event() else {
            return;
        };
        assert!(assessment.verify_reconciliation());
        assert_eq!(assessment.totals.denominator, event.active_denominator());
        assert_eq!(assessment.totals.provided, 1);
        assert_eq!(assessment.totals.verified_none, 1);
        assert_eq!(assessment.totals.unknown, 1, "a missing view fails closed");
        assert_eq!(assessment.totals.unresolved_identities, 1);
        assert_eq!(assessment.cohort_revision, event.cohort_revision());
    }

    #[test]
    fn assessment_requires_a_finalized_cohort() {
        let event = Event::create("summer lunch", day(30));
        assert!(event.is_ok());
        let Ok(event) = event else {
            return;
        };
        let policy = ReadinessPolicy::baseline();
        assert!(policy.is_ok());
        let Ok(policy) = policy else {
            return;
        };
        assert_eq!(
            ReadinessAssessment::assess(&event, &policy, &BTreeMap::new(), day(2)),
            Err(EventsError::CohortNotFinalized)
        );
    }

    #[test]
    fn brief_bytes_are_deterministic_and_match_between_preview_and_sealing() {
        let Some((_, assessment, _)) = assessed_event() else {
            return;
        };
        let content = BriefContent::from_assessment(
            &assessment,
            "campus caterer",
            "summer lunch catering",
            day(31),
            IdentifierDisclosure::NamesAbsent,
        );
        assert!(content.is_ok());
        let Ok(content) = content else {
            return;
        };
        let preview = content.canonical_bytes();
        assert_eq!(preview, content.canonical_bytes());
        let sealed = SealedBrief::seal(content, MemberId::new(), day(3));
        assert!(sealed.matches_preview(&preview));
        assert_eq!(sealed.checksum(), sealed.content().checksum());
    }

    #[test]
    fn brief_contains_no_person_identifier_without_an_approved_policy() {
        let Some((_, assessment, person)) = assessed_event() else {
            return;
        };
        let content = BriefContent::from_assessment(
            &assessment,
            "campus caterer",
            "summer lunch catering",
            day(31),
            IdentifierDisclosure::NamesAbsent,
        );
        assert!(content.is_ok());
        let Ok(content) = content else {
            return;
        };
        let text = String::from_utf8(content.canonical_bytes()).unwrap_or_default();
        assert!(!text.contains(&person.to_string()));
        assert!(!text.contains("attendee-"));
        assert!(text.contains("- 1 x no nuts; separate preparation"));
    }

    #[test]
    fn opaque_identifiers_require_an_explicitly_named_policy_and_stay_event_local() {
        let Some((_, assessment, person)) = assessed_event() else {
            return;
        };
        let missing_policy = BriefContent::from_assessment(
            &assessment,
            "campus caterer",
            "summer lunch catering",
            day(31),
            IdentifierDisclosure::OpaqueAttendeeIdentifiers {
                approved_policy: "  ".into(),
            },
        );
        assert_eq!(
            missing_policy,
            Err(EventsError::RequiredField("approved identifier policy"))
        );
        let content = BriefContent::from_assessment(
            &assessment,
            "campus caterer",
            "summer lunch catering",
            day(31),
            IdentifierDisclosure::OpaqueAttendeeIdentifiers {
                approved_policy: "allergen table cards".into(),
            },
        );
        assert!(content.is_ok());
        let Ok(content) = content else {
            return;
        };
        let text = String::from_utf8(content.canonical_bytes()).unwrap_or_default();
        assert!(text.contains("tokens: attendee-"));
        assert!(
            !text.contains(&person.to_string()),
            "tokens are event-local rows"
        );
    }

    #[test]
    fn a_source_change_marks_the_brief_stale_without_rewriting_its_bytes() {
        let Some((_, assessment, person)) = assessed_event() else {
            return;
        };
        let content = BriefContent::from_assessment(
            &assessment,
            "campus caterer",
            "summer lunch catering",
            day(31),
            IdentifierDisclosure::NamesAbsent,
        );
        assert!(content.is_ok());
        let Ok(content) = content else {
            return;
        };
        let mut sealed = SealedBrief::seal(content, MemberId::new(), day(3));
        let bytes_before = sealed.bytes().to_vec();
        let checksum_before = sealed.checksum().to_owned();
        let mut current = assessment.input_revisions.clone();
        assert!(!sealed.is_stale_against(&current));
        let bumped = Revision::INITIAL.next();
        assert!(bumped.is_ok());
        let Ok(bumped) = bumped else {
            return;
        };
        current.insert(person, bumped);
        assert!(sealed.is_stale_against(&current));
        sealed.mark_stale();
        assert!(sealed.is_stale());
        assert_eq!(sealed.bytes(), bytes_before.as_slice());
        assert_eq!(sealed.checksum(), checksum_before);
    }

    #[test]
    fn operational_instructions_stay_single_line_and_bounded() {
        assert!(OperationalInstruction::parse("no gluten; separate fryer").is_ok());
        assert_eq!(
            OperationalInstruction::parse("   "),
            Err(EventsError::InvalidInstruction("must not be empty"))
        );
        assert_eq!(
            OperationalInstruction::parse("no gluten\ncoeliac diagnosis"),
            Err(EventsError::InvalidInstruction(
                "must be a single line without control characters"
            ))
        );
        let long = "x".repeat(OperationalInstruction::MAXIMUM_CHARACTERS + 1);
        assert_eq!(
            OperationalInstruction::parse(long),
            Err(EventsError::InvalidInstruction(
                "must not exceed 200 characters"
            ))
        );
    }
}
