//! Customisation bounded context.
//!
//! Owns classified settings entries, the settings-only bundle review, diff
//! preview with explicit conflict choices, and revisioned apply and rollback
//! semantics. Canonical storage formats, user interfaces, and the exact
//! serialized bundle layout remain outside this crate; the bundle format here
//! is provisional until the owning A0 G2c milestone reconciles it.

#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use liaison_shared_kernel::Revision;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{self, Display, Formatter};
use thiserror::Error;
use uuid::Uuid;

/// Provisional bundle format label. Not a canonical format: the owning A0
/// G2c milestone defines the canonical layout and may replace this entirely.
pub const PROVISIONAL_BUNDLE_FORMAT: &str = "settings-bundle-0.1-provisional";

/// The settings classes a bundle may carry, from LRM-WS-013.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingsClass {
    Layouts,
    Fields,
    Packs,
    Templates,
    Views,
    Appearance,
    Policies,
}

impl SettingsClass {
    #[must_use]
    pub fn from_segment(segment: &str) -> Option<Self> {
        match segment {
            "layouts" => Some(Self::Layouts),
            "fields" => Some(Self::Fields),
            "packs" => Some(Self::Packs),
            "templates" => Some(Self::Templates),
            "views" => Some(Self::Views),
            "appearance" => Some(Self::Appearance),
            "policies" => Some(Self::Policies),
            _ => None,
        }
    }
}

/// A namespaced settings key: two or more dot-separated lowercase segments.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SettingKey(String);

impl SettingKey {
    pub fn parse(value: impl Into<String>) -> Result<Self, CustomisationError> {
        let value = value.into();
        let segments: Vec<&str> = value.split('.').collect();
        if segments.len() < 2 || !segments.iter().all(|segment| valid_segment(segment)) {
            return Err(CustomisationError::InvalidKey(value));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The known settings class of the first segment, if any. `None` means
    /// an unknown class, which is preserved when its content is safe.
    #[must_use]
    pub fn class(&self) -> Option<SettingsClass> {
        self.0
            .split('.')
            .next()
            .and_then(SettingsClass::from_segment)
    }

    fn has_secret_like_segment(&self) -> bool {
        const FORBIDDEN: [&str; 6] = [
            "secret",
            "token",
            "password",
            "credential",
            "apikey",
            "keychain",
        ];
        self.0
            .split('.')
            .any(|segment| FORBIDDEN.iter().any(|term| segment.contains(term)))
    }
}

impl Display for SettingKey {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

fn valid_segment(value: &str) -> bool {
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    first.is_ascii_lowercase()
        && characters.all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
}

/// A classified settings value. There is no variant for binary blobs,
/// records, or file contents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum SettingValue {
    Text(String),
    Number(i64),
    Flag(bool),
    List(Vec<String>),
}

/// Why an entry was rejected. Detection is a deterministic deny floor:
/// rejection is final, but passing it is not an approval of arbitrary
/// content — the owning milestone may tighten it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnsafeContent {
    RecordIdentifier,
    SecretLike,
    AbsolutePath,
}

fn unsafe_text(text: &str) -> Option<UnsafeContent> {
    if contains_uuid_token(text) {
        return Some(UnsafeContent::RecordIdentifier);
    }
    if looks_like_absolute_path(text) {
        return Some(UnsafeContent::AbsolutePath);
    }
    if looks_secret_like(text) {
        return Some(UnsafeContent::SecretLike);
    }
    None
}

fn contains_uuid_token(text: &str) -> bool {
    text.split(|character: char| !(character.is_ascii_alphanumeric() || character == '-'))
        .filter(|token| token.len() == 36 || token.len() == 32)
        .any(|token| Uuid::try_parse(token).is_ok())
}

fn looks_like_absolute_path(text: &str) -> bool {
    const MARKERS: [&str; 6] = ["/Users/", "/home/", "file://", ":\\", "~/", "\\\\"];
    text.starts_with('/') || MARKERS.iter().any(|marker| text.contains(marker))
}

fn looks_secret_like(text: &str) -> bool {
    const PREFIXES: [&str; 6] = [
        "sk-",
        "ghp_",
        "github_pat_",
        "AKIA",
        "Bearer ",
        "-----BEGIN",
    ];
    if PREFIXES.iter().any(|prefix| text.contains(prefix)) {
        return true;
    }
    text.split_whitespace().any(|token| {
        token.len() >= 32
            && token
                .chars()
                .all(|character| character.is_ascii_alphanumeric() || "+/=_-".contains(character))
            && token.chars().any(|character| character.is_ascii_digit())
            && token
                .chars()
                .any(|character| character.is_ascii_alphabetic())
    })
}

impl SettingValue {
    /// The first unsafe finding in this value, if any.
    #[must_use]
    pub fn unsafe_content(&self) -> Option<UnsafeContent> {
        match self {
            Self::Text(text) => unsafe_text(text),
            Self::Number(_) | Self::Flag(_) => None,
            Self::List(items) => items.iter().find_map(|item| unsafe_text(item)),
        }
    }
}

/// A settings-only bundle in the provisional format.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsBundle {
    pub format: String,
    pub entries: BTreeMap<SettingKey, SettingValue>,
}

impl SettingsBundle {
    #[must_use]
    pub fn new(entries: BTreeMap<SettingKey, SettingValue>) -> Self {
        Self {
            format: PROVISIONAL_BUNDLE_FORMAT.to_owned(),
            entries,
        }
    }
}

/// The review outcome for one bundle entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum EntryFinding {
    Accepted { known_class: bool },
    Rejected { reason: UnsafeContent },
}

impl EntryFinding {
    #[must_use]
    pub const fn is_accepted(self) -> bool {
        matches!(self, Self::Accepted { .. })
    }
}

/// A complete adversarial review of one bundle. Rejected entries are listed,
/// never silently dropped; accepted entries are the only content later steps
/// can see.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleReview {
    findings: BTreeMap<SettingKey, EntryFinding>,
    accepted: BTreeMap<SettingKey, SettingValue>,
}

impl BundleReview {
    #[must_use]
    pub fn review(bundle: &SettingsBundle) -> Self {
        let mut findings = BTreeMap::new();
        let mut accepted = BTreeMap::new();
        for (key, value) in &bundle.entries {
            let finding = if key.has_secret_like_segment() {
                EntryFinding::Rejected {
                    reason: UnsafeContent::SecretLike,
                }
            } else if let Some(reason) = value.unsafe_content() {
                EntryFinding::Rejected { reason }
            } else {
                EntryFinding::Accepted {
                    known_class: key.class().is_some(),
                }
            };
            if finding.is_accepted() {
                accepted.insert(key.clone(), value.clone());
            }
            findings.insert(key.clone(), finding);
        }
        Self { findings, accepted }
    }

    #[must_use]
    pub fn findings(&self) -> &BTreeMap<SettingKey, EntryFinding> {
        &self.findings
    }

    #[must_use]
    pub fn accepted_entries(&self) -> &BTreeMap<SettingKey, SettingValue> {
        &self.accepted
    }

    #[must_use]
    pub fn rejected(&self) -> Vec<(&SettingKey, UnsafeContent)> {
        self.findings
            .iter()
            .filter_map(|(key, finding)| match finding {
                EntryFinding::Rejected { reason } => Some((key, *reason)),
                EntryFinding::Accepted { .. } => None,
            })
            .collect()
    }

    #[must_use]
    pub fn has_rejections(&self) -> bool {
        !self.rejected().is_empty()
    }

    /// The bundle with every rejected entry removed. The review itself keeps
    /// the rejection list, so redaction is visible, not silent.
    #[must_use]
    pub fn redacted_bundle(&self) -> SettingsBundle {
        SettingsBundle::new(self.accepted.clone())
    }
}

/// One previewed difference between current settings and a reviewed bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DiffEntry {
    Added {
        new: SettingValue,
    },
    Changed {
        current: SettingValue,
        new: SettingValue,
    },
    UnknownPreserved {
        new: SettingValue,
    },
}

/// A dry-run preview. It is built only from a review's accepted entries, so
/// rejected content structurally cannot be previewed or applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffPreview {
    entries: BTreeMap<SettingKey, DiffEntry>,
}

impl DiffPreview {
    #[must_use]
    pub fn between(state: &SettingsState, review: &BundleReview) -> Self {
        let mut entries = BTreeMap::new();
        for (key, new) in review.accepted_entries() {
            match state.entries.get(key) {
                Some(current) if current == new => {}
                Some(current) => {
                    entries.insert(
                        key.clone(),
                        DiffEntry::Changed {
                            current: current.clone(),
                            new: new.clone(),
                        },
                    );
                }
                None if key.class().is_some() => {
                    entries.insert(key.clone(), DiffEntry::Added { new: new.clone() });
                }
                None => {
                    entries.insert(
                        key.clone(),
                        DiffEntry::UnknownPreserved { new: new.clone() },
                    );
                }
            }
        }
        Self { entries }
    }

    #[must_use]
    pub fn entries(&self) -> &BTreeMap<SettingKey, DiffEntry> {
        &self.entries
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// The user's explicit decision for one changed entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictChoice {
    KeepCurrent,
    TakeBundle,
}

/// An apply plan derived from a preview. Every changed entry needs an
/// explicit conflict choice before a plan exists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyPlan {
    take: BTreeMap<SettingKey, SettingValue>,
}

impl ApplyPlan {
    pub fn from_preview(
        preview: &DiffPreview,
        choices: &BTreeMap<SettingKey, ConflictChoice>,
    ) -> Result<Self, CustomisationError> {
        let mut take = BTreeMap::new();
        for (key, entry) in preview.entries() {
            match entry {
                DiffEntry::Added { new } | DiffEntry::UnknownPreserved { new } => {
                    take.insert(key.clone(), new.clone());
                }
                DiffEntry::Changed { new, .. } => match choices.get(key) {
                    Some(ConflictChoice::TakeBundle) => {
                        take.insert(key.clone(), new.clone());
                    }
                    Some(ConflictChoice::KeepCurrent) => {}
                    None => {
                        return Err(CustomisationError::MissingConflictChoice(key.clone()));
                    }
                },
            }
        }
        Ok(Self { take })
    }

    #[must_use]
    pub fn take_entries(&self) -> &BTreeMap<SettingKey, SettingValue> {
        &self.take
    }
}

/// The current settings of a workspace as classified entries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsState {
    pub revision: Revision,
    pub entries: BTreeMap<SettingKey, SettingValue>,
}

impl SettingsState {
    #[must_use]
    pub fn initial() -> Self {
        Self {
            revision: Revision::INITIAL,
            entries: BTreeMap::new(),
        }
    }

    /// Applies a plan, returning the next state and an exact rollback point.
    /// The receiver is untouched, so a failed import can never leave partial
    /// mutation behind.
    pub fn apply(&self, plan: &ApplyPlan) -> Result<AppliedImport, CustomisationError> {
        let revision = self
            .revision
            .next()
            .map_err(|_| CustomisationError::RevisionOverflow)?;
        let mut entries = self.entries.clone();
        for (key, value) in plan.take_entries() {
            entries.insert(key.clone(), value.clone());
        }
        Ok(AppliedImport {
            state: Self { revision, entries },
            rollback: RollbackPoint {
                previous: self.clone(),
            },
        })
    }

    /// Exports the current settings as a bundle in the provisional format.
    /// State only ever gains entries through reviewed plans, so an export
    /// round-trips classified settings and preserved unknown safe keys only.
    #[must_use]
    pub fn export_bundle(&self) -> SettingsBundle {
        SettingsBundle::new(self.entries.clone())
    }
}

/// A completed apply with its exact rollback point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppliedImport {
    pub state: SettingsState,
    pub rollback: RollbackPoint,
}

/// The exact pre-apply state, restorable verbatim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollbackPoint {
    previous: SettingsState,
}

impl RollbackPoint {
    /// Restores the prior settings revision and entries exactly.
    #[must_use]
    pub fn restore(&self) -> SettingsState {
        self.previous.clone()
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CustomisationError {
    #[error("invalid settings key: {0}")]
    InvalidKey(String),
    #[error("changed entry {0} requires an explicit conflict choice")]
    MissingConflictChoice(SettingKey),
    #[error("settings revision overflowed")]
    RevisionOverflow,
}

#[cfg(test)]
mod tests {
    use super::{
        ApplyPlan, BundleReview, ConflictChoice, DiffEntry, DiffPreview, EntryFinding,
        PROVISIONAL_BUNDLE_FORMAT, SettingKey, SettingValue, SettingsBundle, SettingsState,
        UnsafeContent,
    };
    use std::collections::BTreeMap;

    fn key(value: &str) -> Option<SettingKey> {
        SettingKey::parse(value).ok()
    }

    fn text(value: &str) -> SettingValue {
        SettingValue::Text(value.to_owned())
    }

    fn bundle(entries: Vec<(&str, SettingValue)>) -> Option<SettingsBundle> {
        let mut map = BTreeMap::new();
        for (name, value) in entries {
            map.insert(key(name)?, value);
        }
        Some(SettingsBundle::new(map))
    }

    fn applied_state(entries: Vec<(&str, SettingValue)>) -> Option<SettingsState> {
        let bundle = bundle(entries)?;
        let review = BundleReview::review(&bundle);
        if review.has_rejections() {
            return None;
        }
        let state = SettingsState::initial();
        let preview = DiffPreview::between(&state, &review);
        let plan = ApplyPlan::from_preview(&preview, &BTreeMap::new()).ok()?;
        Some(state.apply(&plan).ok()?.state)
    }

    #[test]
    fn adversarial_content_is_rejected_and_listed_never_silently_dropped() {
        let Some(adversarial) = bundle(vec![
            (
                "views.recent-person",
                text("018f6a2b-9c3d-7e4f-8a1b-2c3d4e5f6a7b"),
            ),
            (
                "appearance.wallpaper",
                text("/Users/someone/Pictures/team.png"),
            ),
            ("policies.sync-token", text("harmless label")),
            ("templates.greeting", text("sk-abc123def456")),
            ("appearance.theme", text("dark")),
        ]) else {
            return;
        };
        let review = BundleReview::review(&adversarial);
        assert!(review.has_rejections());
        let rejected: BTreeMap<String, UnsafeContent> = review
            .rejected()
            .into_iter()
            .map(|(entry_key, reason)| (entry_key.to_string(), reason))
            .collect();
        assert_eq!(rejected.len(), 4);
        assert_eq!(
            rejected.get("views.recent-person"),
            Some(&UnsafeContent::RecordIdentifier)
        );
        assert_eq!(
            rejected.get("appearance.wallpaper"),
            Some(&UnsafeContent::AbsolutePath)
        );
        assert_eq!(
            rejected.get("policies.sync-token"),
            Some(&UnsafeContent::SecretLike),
            "a secret-like key name is rejected regardless of its value"
        );
        assert_eq!(
            rejected.get("templates.greeting"),
            Some(&UnsafeContent::SecretLike)
        );
        let redacted = review.redacted_bundle();
        assert_eq!(redacted.entries.len(), 1);
        assert!(
            review
                .accepted_entries()
                .keys()
                .any(|k| k.as_str() == "appearance.theme")
        );
    }

    #[test]
    fn unknown_safe_keys_are_preserved_through_preview_apply_and_export() {
        let Some(incoming) = bundle(vec![
            ("future-module.density", text("compact")),
            ("appearance.theme", text("high-contrast")),
        ]) else {
            return;
        };
        let review = BundleReview::review(&incoming);
        assert!(!review.has_rejections());
        assert_eq!(
            review
                .findings()
                .iter()
                .find(|(k, _)| k.as_str() == "future-module.density")
                .map(|(_, f)| *f),
            Some(EntryFinding::Accepted { known_class: false })
        );
        let state = SettingsState::initial();
        let preview = DiffPreview::between(&state, &review);
        assert!(matches!(
            preview
                .entries()
                .iter()
                .find(|(k, _)| k.as_str() == "future-module.density")
                .map(|(_, e)| e),
            Some(DiffEntry::UnknownPreserved { .. })
        ));
        let plan = ApplyPlan::from_preview(&preview, &BTreeMap::new());
        assert!(plan.is_ok());
        let Ok(plan) = plan else {
            return;
        };
        let applied = state.apply(&plan);
        assert!(applied.is_ok());
        let Ok(applied) = applied else {
            return;
        };
        let exported = applied.state.export_bundle();
        assert!(
            exported
                .entries
                .keys()
                .any(|k| k.as_str() == "future-module.density")
        );
        assert_eq!(exported.format, PROVISIONAL_BUNDLE_FORMAT);
    }

    #[test]
    fn changed_entries_require_an_explicit_conflict_choice() {
        let Some(current) = applied_state(vec![("appearance.theme", text("light"))]) else {
            return;
        };
        let Some(incoming) = bundle(vec![("appearance.theme", text("dark"))]) else {
            return;
        };
        let review = BundleReview::review(&incoming);
        let preview = DiffPreview::between(&current, &review);
        let missing = ApplyPlan::from_preview(&preview, &BTreeMap::new());
        assert!(
            missing.is_err(),
            "a changed entry without a choice must not apply"
        );

        let Some(theme_key) = key("appearance.theme") else {
            return;
        };
        let keep = BTreeMap::from([(theme_key.clone(), ConflictChoice::KeepCurrent)]);
        let keep_plan = ApplyPlan::from_preview(&preview, &keep);
        assert!(keep_plan.is_ok());
        let Ok(keep_plan) = keep_plan else {
            return;
        };
        assert!(keep_plan.take_entries().is_empty());

        let take = BTreeMap::from([(theme_key.clone(), ConflictChoice::TakeBundle)]);
        let take_plan = ApplyPlan::from_preview(&preview, &take);
        assert!(take_plan.is_ok());
        let Ok(take_plan) = take_plan else {
            return;
        };
        let applied = current.apply(&take_plan);
        assert!(applied.is_ok());
        let Ok(applied) = applied else {
            return;
        };
        assert_eq!(applied.state.entries.get(&theme_key), Some(&text("dark")));
    }

    #[test]
    fn apply_never_mutates_the_prior_state_and_rollback_restores_it_exactly() {
        let Some(current) = applied_state(vec![
            ("appearance.theme", text("light")),
            ("views.default", text("directory")),
        ]) else {
            return;
        };
        let before = current.clone();
        let Some(incoming) = bundle(vec![("views.default", text("events"))]) else {
            return;
        };
        let review = BundleReview::review(&incoming);
        let preview = DiffPreview::between(&current, &review);
        let Some(views_key) = key("views.default") else {
            return;
        };
        let choices = BTreeMap::from([(views_key, ConflictChoice::TakeBundle)]);
        let plan = ApplyPlan::from_preview(&preview, &choices);
        assert!(plan.is_ok());
        let Ok(plan) = plan else {
            return;
        };
        let applied = current.apply(&plan);
        assert!(applied.is_ok());
        let Ok(applied) = applied else {
            return;
        };
        assert_eq!(
            current, before,
            "apply is pure; no partial mutation is possible"
        );
        assert!(applied.state.revision > before.revision);
        assert_eq!(applied.rollback.restore(), before);
    }

    #[test]
    fn an_export_of_reviewed_state_contains_no_unsafe_literals() {
        let Some(state) = applied_state(vec![
            ("appearance.theme", text("dark")),
            ("layouts.profile", text("two-column")),
            ("policies.review", SettingValue::Flag(true)),
            ("fields.retention-days", SettingValue::Number(30)),
        ]) else {
            return;
        };
        let exported = state.export_bundle();
        assert!(
            exported
                .entries
                .values()
                .all(|value| value.unsafe_content().is_none())
        );
    }

    #[test]
    fn keys_are_validated_and_numbers_and_flags_are_safe() {
        assert!(SettingKey::parse("appearance.theme").is_ok());
        assert!(
            SettingKey::parse("appearance").is_err(),
            "one segment is not namespaced"
        );
        assert!(SettingKey::parse("Appearance.Theme").is_err());
        assert!(SettingKey::parse("appearance..theme").is_err());
        assert_eq!(SettingValue::Number(9000).unsafe_content(), None);
        assert_eq!(SettingValue::Flag(false).unsafe_content(), None);
        assert_eq!(
            SettingValue::List(vec!["window".into(), "aisle".into()]).unsafe_content(),
            None
        );
        assert_eq!(
            SettingValue::List(vec!["ok".into(), "/Users/someone/cache".into()]).unsafe_content(),
            Some(UnsafeContent::AbsolutePath)
        );
    }
}
