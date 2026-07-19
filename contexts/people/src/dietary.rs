//! Structured dietary source facts.
//!
//! People owns dietary information as four orthogonal source axes —
//! availability, freshness, conflict, and disclosure — plus the constrained
//! operational instruction and the separately classified detailed note.
//! Events consumes an authorised operational view derived here; it never
//! receives the detailed note, and readiness outcomes are derived downstream
//! without rewriting these source facts.

use chrono::NaiveDate;
use liaison_shared_kernel::Revision;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use thiserror::Error;

/// The distinguished dietary kinds from LRM-PE-008.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DietaryKind {
    Allergy,
    Intolerance,
    MedicalRestriction,
    ReligiousRestriction,
    EthicalPreference,
    Dislike,
    PositivePreference,
    Other,
}

impl DietaryKind {
    pub const ALL: [Self; 8] = [
        Self::Allergy,
        Self::Intolerance,
        Self::MedicalRestriction,
        Self::ReligiousRestriction,
        Self::EthicalPreference,
        Self::Dislike,
        Self::PositivePreference,
        Self::Other,
    ];
}

/// A constrained, single-line catering instruction. This is the only dietary
/// text that can reach an operational view; there is no dedicated field for
/// diagnosis, treatment, or medical history anywhere in this module.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OperationalInstruction(String);

impl OperationalInstruction {
    pub const MAXIMUM_CHARACTERS: usize = 200;

    pub fn parse(value: impl Into<String>) -> Result<Self, DietaryError> {
        let value = value.into().trim().to_owned();
        if value.is_empty() {
            return Err(DietaryError::InvalidInstruction("must not be empty"));
        }
        if value.chars().any(char::is_control) {
            return Err(DietaryError::InvalidInstruction(
                "must be a single line without control characters",
            ));
        }
        if value.chars().count() > Self::MAXIMUM_CHARACTERS {
            return Err(DietaryError::InvalidInstruction(
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

/// A detailed note under a stricter classification and its own disclosure
/// policy. It is a distinct type so operational views cannot carry it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DetailedNote(String);

impl DetailedNote {
    pub fn parse(value: impl Into<String>) -> Result<Self, DietaryError> {
        let value = value.into().trim().to_owned();
        if value.is_empty() {
            return Err(DietaryError::InvalidNote("must not be empty"));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Orthogonal availability axis. An absent fact means unknown; verified none
/// exists only as an explicit, dated verification and never as a default.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum Availability {
    Provided { instruction: OperationalInstruction },
    VerifiedNone { verified_on: NaiveDate },
    Pending,
    Declined,
    Unreachable,
    Unknown,
}

impl Availability {
    #[must_use]
    pub const fn instruction(&self) -> Option<&OperationalInstruction> {
        match self {
            Self::Provided { instruction } => Some(instruction),
            _ => None,
        }
    }
}

/// Orthogonal freshness axis, derived from the recorded review date and
/// exposed alongside the other axes rather than collapsed into them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Freshness {
    Fresh,
    Stale,
}

/// Orthogonal conflict axis between recorded sources. The recording layer
/// sets it; derivation never rewrites it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictState {
    Consistent,
    Conflicting,
}

/// Orthogonal disclosure axis for the catering purpose. There is no
/// recipient, account, or catering-role grant concept here or anywhere in
/// B0; the trusted owner derives briefs downstream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CateringDisclosure {
    Allowed,
    Excluded,
}

/// A legacy mutually exclusive coverage value. Migration input only: it maps
/// onto the four axes while the original value and its source stay recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LegacyCoverage {
    VerifiedNone,
    Provided,
    Pending,
    Stale,
    Declined,
    Unreachable,
    ExcludedFromCatering,
    Unknown,
}

impl LegacyCoverage {
    pub const ALL: [Self; 8] = [
        Self::VerifiedNone,
        Self::Provided,
        Self::Pending,
        Self::Stale,
        Self::Declined,
        Self::Unreachable,
        Self::ExcludedFromCatering,
        Self::Unknown,
    ];
}

/// Provenance for a recorded fact or migrated legacy value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FactSource(String);

impl FactSource {
    pub fn parse(value: impl Into<String>) -> Result<Self, DietaryError> {
        let value = value.into().trim().to_owned();
        if value.is_empty() {
            return Err(DietaryError::InvalidSource);
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The retained record of a migrated legacy value (LRM-PE-009).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegacyMigration {
    pub value: LegacyCoverage,
    pub source: FactSource,
    pub migrated_on: NaiveDate,
}

/// One recorded dietary source fact: a kind plus the four orthogonal axes,
/// provenance, review dates, and the separately classified detailed note.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DietaryFact {
    kind: DietaryKind,
    availability: Availability,
    conflict: ConflictState,
    disclosure: CateringDisclosure,
    source: FactSource,
    recorded_on: NaiveDate,
    review_due: Option<NaiveDate>,
    note: Option<DetailedNote>,
    legacy: Option<LegacyMigration>,
}

/// Inputs for recording a fact. Availability carries its own evidence:
/// verified none cannot be expressed without a verification date.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DietaryFactSpec {
    pub kind: DietaryKind,
    pub availability: Availability,
    pub conflict: ConflictState,
    pub disclosure: CateringDisclosure,
    pub source: FactSource,
    pub recorded_on: NaiveDate,
    pub review_due: Option<NaiveDate>,
    pub note: Option<DetailedNote>,
}

impl DietaryFact {
    pub fn record(spec: DietaryFactSpec) -> Result<Self, DietaryError> {
        if let Availability::VerifiedNone { verified_on } = &spec.availability
            && *verified_on > spec.recorded_on
        {
            return Err(DietaryError::VerificationInTheFuture);
        }
        Ok(Self {
            kind: spec.kind,
            availability: spec.availability,
            conflict: spec.conflict,
            disclosure: spec.disclosure,
            source: spec.source,
            recorded_on: spec.recorded_on,
            review_due: spec.review_due,
            note: spec.note,
            legacy: None,
        })
    }

    /// Migrates one legacy mutually exclusive coverage value onto the axes.
    /// The legacy value and its source stay recorded on the fact.
    pub fn from_legacy(
        kind: DietaryKind,
        value: LegacyCoverage,
        instruction: Option<OperationalInstruction>,
        source: FactSource,
        migrated_on: NaiveDate,
    ) -> Result<Self, DietaryError> {
        let availability = match (value, instruction) {
            (LegacyCoverage::Provided, Some(instruction)) => Availability::Provided { instruction },
            (LegacyCoverage::Provided, None) => {
                return Err(DietaryError::LegacyProvidedWithoutInstruction);
            }
            (LegacyCoverage::VerifiedNone, _) => Availability::VerifiedNone {
                verified_on: migrated_on,
            },
            (LegacyCoverage::Pending, _) => Availability::Pending,
            (LegacyCoverage::Declined, _) => Availability::Declined,
            (LegacyCoverage::Unreachable, _) => Availability::Unreachable,
            (
                LegacyCoverage::Stale
                | LegacyCoverage::Unknown
                | LegacyCoverage::ExcludedFromCatering,
                _,
            ) => Availability::Unknown,
        };
        let disclosure = if value == LegacyCoverage::ExcludedFromCatering {
            CateringDisclosure::Excluded
        } else {
            CateringDisclosure::Allowed
        };
        let review_due = if value == LegacyCoverage::Stale {
            Some(migrated_on)
        } else {
            None
        };
        Ok(Self {
            kind,
            availability,
            conflict: ConflictState::Consistent,
            disclosure,
            source: source.clone(),
            recorded_on: migrated_on,
            review_due,
            note: None,
            legacy: Some(LegacyMigration {
                value,
                source,
                migrated_on,
            }),
        })
    }

    #[must_use]
    pub const fn kind(&self) -> DietaryKind {
        self.kind
    }

    #[must_use]
    pub const fn availability(&self) -> &Availability {
        &self.availability
    }

    #[must_use]
    pub const fn conflict(&self) -> ConflictState {
        self.conflict
    }

    #[must_use]
    pub const fn disclosure(&self) -> CateringDisclosure {
        self.disclosure
    }

    #[must_use]
    pub fn source(&self) -> &FactSource {
        &self.source
    }

    #[must_use]
    pub const fn recorded_on(&self) -> NaiveDate {
        self.recorded_on
    }

    #[must_use]
    pub const fn review_due(&self) -> Option<NaiveDate> {
        self.review_due
    }

    #[must_use]
    pub const fn detailed_note(&self) -> Option<&DetailedNote> {
        self.note.as_ref()
    }

    #[must_use]
    pub const fn legacy(&self) -> Option<&LegacyMigration> {
        self.legacy.as_ref()
    }

    /// Marks this fact as conflicting with another active source. The
    /// recording layer calls this; derivation never does.
    pub const fn mark_conflicting(&mut self) {
        self.conflict = ConflictState::Conflicting;
    }

    /// The freshness axis at a date: stale when the recorded review date has
    /// passed. Facts without a review date do not silently expire.
    #[must_use]
    pub fn freshness(&self, as_of: NaiveDate) -> Freshness {
        match self.review_due {
            Some(due) if due < as_of => Freshness::Stale,
            _ => Freshness::Fresh,
        }
    }

    /// The authorised operational view for the trusted owner's catering
    /// workflow. It carries the constrained instruction only when one is
    /// provided; the detailed note is structurally absent from the type.
    #[must_use]
    pub fn operational_view(&self, as_of: NaiveDate, revision: Revision) -> DietaryOperationalView {
        DietaryOperationalView {
            availability: self.availability.clone(),
            freshness: self.freshness(as_of),
            conflict: self.conflict,
            disclosure: self.disclosure,
            profile_revision: revision,
        }
    }
}

/// The authorised per-person dietary view Events consumes. There is no field
/// for the detailed note, a diagnosis, or any free source text, and no
/// recipient ever receives this view directly: B0 recipients get immutable
/// least-disclosure brief bytes derived downstream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DietaryOperationalView {
    pub availability: Availability,
    pub freshness: Freshness,
    pub conflict: ConflictState,
    pub disclosure: CateringDisclosure,
    pub profile_revision: Revision,
}

impl DietaryOperationalView {
    /// The single blessed representation of "nothing recorded": unknown
    /// availability, fresh, consistent, catering allowed. An empty record is
    /// never verified none.
    #[must_use]
    pub const fn unknown(revision: Revision) -> Self {
        Self {
            availability: Availability::Unknown,
            freshness: Freshness::Fresh,
            conflict: ConflictState::Consistent,
            disclosure: CateringDisclosure::Allowed,
            profile_revision: revision,
        }
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DietaryError {
    #[error("invalid operational instruction: {0}")]
    InvalidInstruction(&'static str),
    #[error("invalid detailed note: {0}")]
    InvalidNote(&'static str),
    #[error("a dietary fact requires a non-empty source")]
    InvalidSource,
    #[error("a verified-none record cannot be dated after it was recorded")]
    VerificationInTheFuture,
    #[error("a legacy provided value requires its operational instruction")]
    LegacyProvidedWithoutInstruction,
}

#[cfg(test)]
mod tests {
    use super::{
        Availability, CateringDisclosure, ConflictState, DetailedNote, DietaryFact,
        DietaryFactSpec, DietaryKind, DietaryOperationalView, FactSource, Freshness,
        LegacyCoverage, OperationalInstruction,
    };
    use chrono::NaiveDate;
    use liaison_shared_kernel::Revision;

    fn day(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 7, day).unwrap_or_default()
    }

    fn source() -> Option<FactSource> {
        FactSource::parse("self-reported at onboarding").ok()
    }

    fn instruction(text: &str) -> Option<OperationalInstruction> {
        OperationalInstruction::parse(text).ok()
    }

    fn spec(kind: DietaryKind, availability: Availability) -> Option<DietaryFactSpec> {
        Some(DietaryFactSpec {
            kind,
            availability,
            conflict: ConflictState::Consistent,
            disclosure: CateringDisclosure::Allowed,
            source: source()?,
            recorded_on: day(10),
            review_due: None,
            note: None,
        })
    }

    #[test]
    fn every_kind_validates_independently_and_can_carry_an_instruction() {
        for kind in DietaryKind::ALL {
            let Some(text) = instruction("no shellfish; separate preparation") else {
                return;
            };
            let Some(spec) = spec(kind, Availability::Provided { instruction: text }) else {
                return;
            };
            let fact = DietaryFact::record(spec);
            assert!(fact.is_ok(), "{kind:?} must validate independently");
            let Ok(fact) = fact else {
                return;
            };
            assert!(fact.availability().instruction().is_some());
            assert_eq!(fact.kind(), kind);
        }
    }

    #[test]
    fn an_absent_record_is_unknown_and_never_verified_none() {
        let view = DietaryOperationalView::unknown(Revision::INITIAL);
        assert_eq!(view.availability, Availability::Unknown);
        assert!(view.availability.instruction().is_none());
        assert!(
            !matches!(view.availability, Availability::VerifiedNone { .. }),
            "empty must never read as verified none"
        );
    }

    #[test]
    fn verified_none_requires_a_dated_verification_and_rejects_future_dates() {
        let Some(mut ok) = spec(
            DietaryKind::Allergy,
            Availability::VerifiedNone {
                verified_on: day(9),
            },
        ) else {
            return;
        };
        assert!(DietaryFact::record(ok.clone()).is_ok());
        ok.availability = Availability::VerifiedNone {
            verified_on: day(11),
        };
        assert!(
            DietaryFact::record(ok).is_err(),
            "verification cannot postdate the record"
        );
    }

    #[test]
    fn the_four_axes_stay_orthogonal_across_every_combination() {
        let availabilities = [
            Availability::Provided {
                instruction: match instruction("halal; no alcohol in preparation") {
                    Some(text) => text,
                    None => return,
                },
            },
            Availability::VerifiedNone {
                verified_on: day(9),
            },
            Availability::Pending,
            Availability::Declined,
            Availability::Unreachable,
            Availability::Unknown,
        ];
        for availability in availabilities {
            for conflict in [ConflictState::Consistent, ConflictState::Conflicting] {
                for disclosure in [CateringDisclosure::Allowed, CateringDisclosure::Excluded] {
                    for review_due in [None, Some(day(12)), Some(day(1))] {
                        let Some(mut fact_spec) = spec(DietaryKind::Other, availability.clone())
                        else {
                            return;
                        };
                        fact_spec.conflict = conflict;
                        fact_spec.disclosure = disclosure;
                        fact_spec.review_due = review_due;
                        let fact = DietaryFact::record(fact_spec);
                        assert!(fact.is_ok(), "no axis combination may be rejected");
                        let Ok(fact) = fact else {
                            return;
                        };
                        assert_eq!(fact.conflict(), conflict);
                        assert_eq!(fact.disclosure(), disclosure);
                        let expected = match review_due {
                            Some(due) if due < day(10) => Freshness::Stale,
                            _ => Freshness::Fresh,
                        };
                        assert_eq!(fact.freshness(day(10)), expected);
                    }
                }
            }
        }
    }

    #[test]
    fn legacy_values_map_onto_axes_while_keeping_value_and_provenance() {
        for value in LegacyCoverage::ALL {
            let migrated = DietaryFact::from_legacy(
                DietaryKind::Allergy,
                value,
                instruction("no gluten"),
                match source() {
                    Some(source) => source,
                    None => return,
                },
                day(10),
            );
            assert!(migrated.is_ok(), "{value:?} must migrate");
            let Ok(migrated) = migrated else {
                return;
            };
            assert!(
                migrated.legacy().is_some(),
                "legacy value must stay recorded"
            );
            let Some(legacy) = migrated.legacy() else {
                return;
            };
            assert_eq!(legacy.value, value);
            assert_eq!(legacy.source.as_str(), "self-reported at onboarding");
            match value {
                LegacyCoverage::ExcludedFromCatering => {
                    assert_eq!(migrated.disclosure(), CateringDisclosure::Excluded);
                }
                LegacyCoverage::Stale => {
                    assert_eq!(migrated.freshness(day(11)), Freshness::Stale);
                }
                LegacyCoverage::VerifiedNone => {
                    assert!(matches!(
                        migrated.availability(),
                        Availability::VerifiedNone { .. }
                    ));
                }
                _ => {}
            }
        }
        assert!(
            DietaryFact::from_legacy(
                DietaryKind::Allergy,
                LegacyCoverage::Provided,
                None,
                match source() {
                    Some(source) => source,
                    None => return,
                },
                day(10),
            )
            .is_err(),
            "legacy provided without its instruction is not silently invented"
        );
    }

    #[test]
    fn the_operational_view_carries_the_instruction_but_never_the_note() {
        let Some(text) = instruction("no nuts; separate preparation") else {
            return;
        };
        let Some(mut fact_spec) = spec(
            DietaryKind::Allergy,
            Availability::Provided { instruction: text },
        ) else {
            return;
        };
        fact_spec.note =
            DetailedNote::parse("anaphylaxis history; carries an epinephrine pen").ok();
        let fact = DietaryFact::record(fact_spec);
        assert!(fact.is_ok());
        let Ok(fact) = fact else {
            return;
        };
        assert!(fact.detailed_note().is_some());
        let view = fact.operational_view(day(11), Revision::INITIAL);
        assert_eq!(
            view.availability.instruction().map(ToString::to_string),
            Some("no nuts; separate preparation".to_owned())
        );
        let serialised = serde_json::to_string(&view).unwrap_or_default();
        assert!(
            !serialised.contains("anaphylaxis"),
            "the detailed note is structurally absent from the view"
        );
    }

    #[test]
    fn instructions_stay_single_line_and_bounded() {
        assert!(OperationalInstruction::parse("vegetarian; no fish stock").is_ok());
        assert!(OperationalInstruction::parse("   ").is_err());
        assert!(OperationalInstruction::parse("line one\nline two").is_err());
        let long = "x".repeat(OperationalInstruction::MAXIMUM_CHARACTERS + 1);
        assert!(OperationalInstruction::parse(long).is_err());
    }
}
