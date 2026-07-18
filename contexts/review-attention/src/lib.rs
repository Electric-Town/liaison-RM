//! Review and Attention bounded context.
//!
//! Owns explainable review reasons, hard suppressions, capacity-bounded
//! reason-only policies, and deterministic queue construction.

#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use chrono::NaiveDate;
use liaison_shared_kernel::PersonId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PolicyId(String);

impl PolicyId {
    pub fn parse(value: impl Into<String>) -> Result<Self, ReviewError> {
        let value = value.into();
        if !valid_policy_id(&value) {
            return Err(ReviewError::InvalidPolicyId(value));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for PolicyId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewReason {
    OverdueRelativeToOwnCadence,
    OpenCommitment,
    UpcomingImportantDate,
    SelectedProfileReadinessGap,
    StaleRequiredContext,
    ManualPin,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "until", rename_all = "snake_case")]
pub enum Suppression {
    Archived,
    DoNotContact,
    RelationshipEnded,
    PausedUntil(NaiveDate),
    SnoozedUntil(NaiveDate),
    Excluded,
}

impl Suppression {
    #[must_use]
    pub fn is_active(&self, as_of: NaiveDate) -> bool {
        match self {
            Self::Archived | Self::DoNotContact | Self::RelationshipEnded | Self::Excluded => true,
            Self::PausedUntil(date) | Self::SnoozedUntil(date) => *date >= as_of,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasonOnlyPolicy {
    id: PolicyId,
    version: u32,
    daily_capacity: usize,
    reason_order: Vec<ReviewReason>,
}

impl ReasonOnlyPolicy {
    pub fn new(
        id: PolicyId,
        version: u32,
        daily_capacity: usize,
        reason_order: Vec<ReviewReason>,
    ) -> Result<Self, ReviewError> {
        if version == 0 {
            return Err(ReviewError::InvalidVersion);
        }
        if !(1..=50).contains(&daily_capacity) {
            return Err(ReviewError::InvalidCapacity(daily_capacity));
        }
        if reason_order.is_empty() {
            return Err(ReviewError::NoReviewReasons);
        }
        let unique = reason_order.iter().copied().collect::<BTreeSet<_>>();
        if unique.len() != reason_order.len() {
            return Err(ReviewError::DuplicateReviewReason);
        }
        Ok(Self {
            id,
            version,
            daily_capacity,
            reason_order,
        })
    }

    #[must_use]
    pub fn id(&self) -> &PolicyId {
        &self.id
    }

    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version
    }

    #[must_use]
    pub const fn daily_capacity(&self) -> usize {
        self.daily_capacity
    }

    #[must_use]
    pub fn reason_order(&self) -> &[ReviewReason] {
        &self.reason_order
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewCandidate {
    person_id: PersonId,
    reasons: BTreeSet<ReviewReason>,
    suppressions: Vec<Suppression>,
}

impl ReviewCandidate {
    #[must_use]
    pub fn new(
        person_id: PersonId,
        reasons: BTreeSet<ReviewReason>,
        suppressions: Vec<Suppression>,
    ) -> Self {
        Self {
            person_id,
            reasons,
            suppressions,
        }
    }

    #[must_use]
    pub const fn person_id(&self) -> PersonId {
        self.person_id
    }

    #[must_use]
    pub fn reasons(&self) -> &BTreeSet<ReviewReason> {
        &self.reasons
    }

    #[must_use]
    pub fn suppressions(&self) -> &[Suppression] {
        &self.suppressions
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewQueueItem {
    pub person_id: PersonId,
    pub reasons: Vec<ReviewReason>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewQueue {
    pub policy_id: PolicyId,
    pub policy_version: u32,
    pub as_of: NaiveDate,
    pub capacity: usize,
    pub items: Vec<ReviewQueueItem>,
}

impl ReviewQueue {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct BuildReasonOnlyQueue {
    policy: ReasonOnlyPolicy,
}

impl BuildReasonOnlyQueue {
    #[must_use]
    pub const fn new(policy: ReasonOnlyPolicy) -> Self {
        Self { policy }
    }

    #[must_use]
    pub fn execute(&self, candidates: Vec<ReviewCandidate>, as_of: NaiveDate) -> ReviewQueue {
        let mut selected = candidates
            .into_iter()
            .filter_map(|candidate| self.select(candidate, as_of))
            .collect::<Vec<_>>();
        selected.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
        selected.truncate(self.policy.daily_capacity());
        ReviewQueue {
            policy_id: self.policy.id().clone(),
            policy_version: self.policy.version(),
            as_of,
            capacity: self.policy.daily_capacity(),
            items: selected.into_iter().map(|(_, _, item)| item).collect(),
        }
    }

    fn select(
        &self,
        candidate: ReviewCandidate,
        as_of: NaiveDate,
    ) -> Option<(usize, String, ReviewQueueItem)> {
        if candidate
            .suppressions()
            .iter()
            .any(|suppression| suppression.is_active(as_of))
        {
            return None;
        }
        let reasons = self
            .policy
            .reason_order()
            .iter()
            .filter(|reason| candidate.reasons().contains(reason))
            .copied()
            .collect::<Vec<_>>();
        let primary_reason = reasons.first()?;
        let primary_index = self
            .policy
            .reason_order()
            .iter()
            .position(|reason| reason == primary_reason)?;
        let person_id = candidate.person_id();
        Some((
            primary_index,
            person_id.to_string(),
            ReviewQueueItem { person_id, reasons },
        ))
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ReviewError {
    #[error("invalid review policy ID: {0}")]
    InvalidPolicyId(String),
    #[error("policy version must be at least one")]
    InvalidVersion,
    #[error("daily capacity must be between 1 and 50; found {0}")]
    InvalidCapacity(usize),
    #[error("reason-only policy must contain at least one reason")]
    NoReviewReasons,
    #[error("reason-only policy cannot repeat a review reason")]
    DuplicateReviewReason,
}

fn valid_policy_id(value: &str) -> bool {
    let mut segments = value.split('-');
    let Some(first) = segments.next() else {
        return false;
    };
    valid_segment(first) && segments.all(valid_segment)
}

fn valid_segment(value: &str) -> bool {
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    first.is_ascii_lowercase()
        && characters.all(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::{
        BuildReasonOnlyQueue, PolicyId, ReasonOnlyPolicy, ReviewCandidate, ReviewReason,
        Suppression,
    };
    use chrono::NaiveDate;
    use liaison_shared_kernel::PersonId;
    use std::collections::BTreeSet;

    fn date(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(year, month, day)
    }

    fn policy(capacity: usize) -> Result<ReasonOnlyPolicy, super::ReviewError> {
        ReasonOnlyPolicy::new(
            PolicyId::parse("calm-daily-review")?,
            1,
            capacity,
            vec![
                ReviewReason::OpenCommitment,
                ReviewReason::UpcomingImportantDate,
                ReviewReason::OverdueRelativeToOwnCadence,
                ReviewReason::SelectedProfileReadinessGap,
                ReviewReason::ManualPin,
            ],
        )
    }

    #[test]
    fn hard_suppressions_apply_before_queue_ordering() {
        let as_of = date(2026, 7, 18);
        assert!(as_of.is_some());
        let Some(as_of) = as_of else {
            return;
        };
        let active = ReviewCandidate::new(
            PersonId::new(),
            BTreeSet::from([ReviewReason::OpenCommitment]),
            vec![],
        );
        let archived = ReviewCandidate::new(
            PersonId::new(),
            BTreeSet::from([ReviewReason::OpenCommitment]),
            vec![Suppression::Archived],
        );
        let future_pause = ReviewCandidate::new(
            PersonId::new(),
            BTreeSet::from([ReviewReason::UpcomingImportantDate]),
            vec![Suppression::PausedUntil(date(2026, 7, 25).unwrap_or(as_of))],
        );
        let policy = policy(5);
        assert!(policy.is_ok());
        let Ok(policy) = policy else {
            return;
        };
        let queue = BuildReasonOnlyQueue::new(policy)
            .execute(vec![active.clone(), archived, future_pause], as_of);
        assert_eq!(queue.items.len(), 1);
        assert_eq!(queue.items[0].person_id, active.person_id());
    }

    #[test]
    fn expired_snooze_does_not_hide_a_candidate() {
        let as_of = date(2026, 7, 18);
        let expired = date(2026, 7, 17);
        assert!(as_of.is_some());
        assert!(expired.is_some());
        let (Some(as_of), Some(expired)) = (as_of, expired) else {
            return;
        };
        let candidate = ReviewCandidate::new(
            PersonId::new(),
            BTreeSet::from([ReviewReason::ManualPin]),
            vec![Suppression::SnoozedUntil(expired)],
        );
        let policy = policy(5);
        assert!(policy.is_ok());
        let Ok(policy) = policy else {
            return;
        };
        let queue = BuildReasonOnlyQueue::new(policy).execute(vec![candidate], as_of);
        assert_eq!(queue.items.len(), 1);
    }

    #[test]
    fn policy_capacity_bounds_the_session() {
        let as_of = date(2026, 7, 18);
        assert!(as_of.is_some());
        let Some(as_of) = as_of else {
            return;
        };
        let candidates = (0..4)
            .map(|_| {
                ReviewCandidate::new(
                    PersonId::new(),
                    BTreeSet::from([ReviewReason::OpenCommitment]),
                    vec![],
                )
            })
            .collect();
        let policy = policy(2);
        assert!(policy.is_ok());
        let Ok(policy) = policy else {
            return;
        };
        let queue = BuildReasonOnlyQueue::new(policy).execute(candidates, as_of);
        assert_eq!(queue.capacity, 2);
        assert_eq!(queue.items.len(), 2);
    }

    #[test]
    fn queue_preserves_visible_reason_order_without_a_score() {
        let as_of = date(2026, 7, 18);
        assert!(as_of.is_some());
        let Some(as_of) = as_of else {
            return;
        };
        let candidate = ReviewCandidate::new(
            PersonId::new(),
            BTreeSet::from([
                ReviewReason::UpcomingImportantDate,
                ReviewReason::OpenCommitment,
            ]),
            vec![],
        );
        let policy = policy(5);
        assert!(policy.is_ok());
        let Ok(policy) = policy else {
            return;
        };
        let queue = BuildReasonOnlyQueue::new(policy).execute(vec![candidate], as_of);
        assert_eq!(
            queue.items[0].reasons,
            vec![
                ReviewReason::OpenCommitment,
                ReviewReason::UpcomingImportantDate
            ]
        );
    }

    #[test]
    fn candidate_without_policy_reasons_is_not_invented_into_the_queue() {
        let as_of = date(2026, 7, 18);
        assert!(as_of.is_some());
        let Some(as_of) = as_of else {
            return;
        };
        let candidate = ReviewCandidate::new(PersonId::new(), BTreeSet::new(), vec![]);
        let policy = policy(5);
        assert!(policy.is_ok());
        let Ok(policy) = policy else {
            return;
        };
        let queue = BuildReasonOnlyQueue::new(policy).execute(vec![candidate], as_of);
        assert!(queue.is_empty());
    }
}
