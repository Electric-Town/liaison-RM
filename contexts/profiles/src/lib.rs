//! Identity and Profiles bounded context.
//!
//! Owns Topic Packs, stable Field Definitions, explicit information states,
//! profile values, Purpose Definitions, and purpose-specific readiness.

#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FieldId(String);

impl FieldId {
    pub fn parse(value: impl Into<String>) -> Result<Self, ProfileError> {
        let value = value.into();
        if !valid_namespaced_id(&value) {
            return Err(ProfileError::InvalidIdentifier {
                kind: "field",
                value,
            });
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for FieldId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TopicPackId(String);

impl TopicPackId {
    pub fn parse(value: impl Into<String>) -> Result<Self, ProfileError> {
        let value = value.into();
        if !valid_simple_id(&value) {
            return Err(ProfileError::InvalidIdentifier {
                kind: "topic pack",
                value,
            });
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for TopicPackId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PurposeId(String);

impl PurposeId {
    pub fn parse(value: impl Into<String>) -> Result<Self, ProfileError> {
        let value = value.into();
        if !valid_simple_id(&value) {
            return Err(ProfileError::InvalidIdentifier {
                kind: "purpose",
                value,
            });
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for PurposeId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    ShortText,
    LongText,
    Markdown,
    Date,
    PartialDate,
    RecurringDate,
    Enum,
    MultiSelect,
    Boolean,
    Number,
    Measurement,
    Address,
    Location,
    EntityReference,
    EntityReferenceList,
    ResourceReference,
    ResourceReferenceList,
    Sealed,
    Calculated,
    List,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Classification {
    Public,
    Shared,
    Private,
    Sensitive,
    Secret,
}

impl Classification {
    #[must_use]
    pub const fn requires_sealed_storage(self) -> bool {
        matches!(self, Self::Sensitive | Self::Secret)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InformationState {
    Known,
    Verified,
    Unverified,
    Unknown,
    NotApplicable,
    Declined,
    Stale,
    Conflicting,
    NeedsClarification,
    Derived,
}

impl InformationState {
    #[must_use]
    pub const fn requires_payload(self) -> bool {
        matches!(
            self,
            Self::Known | Self::Verified | Self::Unverified | Self::Derived
        )
    }

    #[must_use]
    pub const fn forbids_payload(self) -> bool {
        matches!(self, Self::Unknown | Self::NotApplicable | Self::Declined)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDefinitionSpec {
    pub id: FieldId,
    pub label: String,
    pub field_type: FieldType,
    pub classification: Classification,
    pub required_for: BTreeSet<PurposeId>,
    pub stale_after_days: Option<u32>,
    pub sealed_by_default: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldDefinition {
    id: FieldId,
    label: String,
    field_type: FieldType,
    classification: Classification,
    required_for: BTreeSet<PurposeId>,
    stale_after_days: Option<u32>,
    sealed_by_default: bool,
}

impl FieldDefinition {
    pub fn new(spec: FieldDefinitionSpec) -> Result<Self, ProfileError> {
        let label = normalized_required(spec.label, "field label")?;
        if spec.classification.requires_sealed_storage() && !spec.sealed_by_default {
            return Err(ProfileError::SensitiveFieldIsNotSealed(spec.id));
        }
        if spec.stale_after_days == Some(0) {
            return Err(ProfileError::InvalidStaleness);
        }
        Ok(Self {
            id: spec.id,
            label,
            field_type: spec.field_type,
            classification: spec.classification,
            required_for: spec.required_for,
            stale_after_days: spec.stale_after_days,
            sealed_by_default: spec.sealed_by_default,
        })
    }

    #[must_use]
    pub fn id(&self) -> &FieldId {
        &self.id
    }

    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub const fn field_type(&self) -> FieldType {
        self.field_type
    }

    #[must_use]
    pub const fn classification(&self) -> Classification {
        self.classification
    }

    #[must_use]
    pub fn required_for(&self) -> &BTreeSet<PurposeId> {
        &self.required_for
    }

    #[must_use]
    pub const fn stale_after_days(&self) -> Option<u32> {
        self.stale_after_days
    }

    #[must_use]
    pub const fn sealed_by_default(&self) -> bool {
        self.sealed_by_default
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopicPack {
    id: TopicPackId,
    version: u32,
    label: String,
    fields: Vec<FieldDefinition>,
}

impl TopicPack {
    pub fn new(
        id: TopicPackId,
        version: u32,
        label: impl Into<String>,
        fields: Vec<FieldDefinition>,
    ) -> Result<Self, ProfileError> {
        if version == 0 {
            return Err(ProfileError::InvalidVersion);
        }
        let label = normalized_required(label, "Topic Pack label")?;
        if fields.is_empty() {
            return Err(ProfileError::TopicPackHasNoFields(id));
        }
        let field_ids = fields
            .iter()
            .map(|field| field.id().clone())
            .collect::<BTreeSet<_>>();
        if field_ids.len() != fields.len() {
            return Err(ProfileError::DuplicateFieldId);
        }
        Ok(Self {
            id,
            version,
            label,
            fields,
        })
    }

    #[must_use]
    pub fn id(&self) -> &TopicPackId {
        &self.id
    }

    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version
    }

    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub fn fields(&self) -> &[FieldDefinition] {
        &self.fields
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldValue {
    field_id: FieldId,
    state: InformationState,
    payload: Option<String>,
    sealed: bool,
}

impl FieldValue {
    pub fn new(
        definition: &FieldDefinition,
        state: InformationState,
        payload: Option<String>,
        sealed: bool,
    ) -> Result<Self, ProfileError> {
        let payload = payload
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty());
        if state.requires_payload() && payload.is_none() {
            return Err(ProfileError::StateRequiresValue(state));
        }
        if state.forbids_payload() && payload.is_some() {
            return Err(ProfileError::StateForbidsValue(state));
        }
        if definition.classification().requires_sealed_storage() && !sealed {
            return Err(ProfileError::SensitiveValueIsNotSealed(
                definition.id().clone(),
            ));
        }
        Ok(Self {
            field_id: definition.id().clone(),
            state,
            payload,
            sealed,
        })
    }

    #[must_use]
    pub fn field_id(&self) -> &FieldId {
        &self.field_id
    }

    #[must_use]
    pub const fn state(&self) -> InformationState {
        self.state
    }

    #[must_use]
    pub fn payload(&self) -> Option<&str> {
        self.payload.as_deref()
    }

    #[must_use]
    pub const fn is_sealed(&self) -> bool {
        self.sealed
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ProfileSnapshot {
    values: BTreeMap<FieldId, FieldValue>,
}

impl ProfileSnapshot {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    pub fn set(&mut self, value: FieldValue) {
        self.values.insert(value.field_id().clone(), value);
    }

    #[must_use]
    pub fn get(&self, field_id: &FieldId) -> Option<&FieldValue> {
        self.values.get(field_id)
    }

    #[must_use]
    pub fn values(&self) -> &BTreeMap<FieldId, FieldValue> {
        &self.values
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PurposeDefinition {
    id: PurposeId,
    version: u32,
    required_fields: BTreeSet<FieldId>,
    acceptable_states: BTreeSet<InformationState>,
}

impl PurposeDefinition {
    pub fn new(
        id: PurposeId,
        version: u32,
        required_fields: BTreeSet<FieldId>,
        acceptable_states: BTreeSet<InformationState>,
    ) -> Result<Self, ProfileError> {
        if version == 0 {
            return Err(ProfileError::InvalidVersion);
        }
        if required_fields.is_empty() {
            return Err(ProfileError::PurposeHasNoRequiredFields(id));
        }
        if acceptable_states.is_empty() {
            return Err(ProfileError::PurposeHasNoAcceptableStates(id));
        }
        Ok(Self {
            id,
            version,
            required_fields,
            acceptable_states,
        })
    }

    #[must_use]
    pub fn id(&self) -> &PurposeId {
        &self.id
    }

    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version
    }

    #[must_use]
    pub fn required_fields(&self) -> &BTreeSet<FieldId> {
        &self.required_fields
    }

    #[must_use]
    pub fn acceptable_states(&self) -> &BTreeSet<InformationState> {
        &self.acceptable_states
    }

    #[must_use]
    pub fn calculate_readiness(&self, profile: &ProfileSnapshot) -> ProfileReadiness {
        let mut gaps = Vec::new();
        let mut satisfied_count = 0;
        for field_id in &self.required_fields {
            match profile.get(field_id) {
                None => gaps.push(ReadinessGap {
                    field_id: field_id.clone(),
                    reason: ReadinessGapReason::Missing,
                }),
                Some(value) if self.acceptable_states.contains(&value.state()) => {
                    satisfied_count += 1;
                }
                Some(value) => gaps.push(ReadinessGap {
                    field_id: field_id.clone(),
                    reason: ReadinessGapReason::StateNotAccepted(value.state()),
                }),
            }
        }
        ProfileReadiness {
            purpose_id: self.id.clone(),
            purpose_version: self.version,
            required_count: self.required_fields.len(),
            satisfied_count,
            gaps,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileReadiness {
    pub purpose_id: PurposeId,
    pub purpose_version: u32,
    pub required_count: usize,
    pub satisfied_count: usize,
    pub gaps: Vec<ReadinessGap>,
}

impl ProfileReadiness {
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        self.required_count == self.satisfied_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessGap {
    pub field_id: FieldId,
    pub reason: ReadinessGapReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessGapReason {
    Missing,
    StateNotAccepted(InformationState),
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ProfileError {
    #[error("invalid {kind} identifier: {value}")]
    InvalidIdentifier { kind: &'static str, value: String },
    #[error("{0} is required")]
    RequiredField(&'static str),
    #[error("version must be at least one")]
    InvalidVersion,
    #[error("stale-after days must be greater than zero")]
    InvalidStaleness,
    #[error("sensitive field {0} must be sealed by default")]
    SensitiveFieldIsNotSealed(FieldId),
    #[error("sensitive value {0} must use sealed storage")]
    SensitiveValueIsNotSealed(FieldId),
    #[error("information state {0:?} requires a value")]
    StateRequiresValue(InformationState),
    #[error("information state {0:?} must not contain a value")]
    StateForbidsValue(InformationState),
    #[error("Topic Pack {0} must contain at least one field")]
    TopicPackHasNoFields(TopicPackId),
    #[error("Topic Pack contains duplicate field IDs")]
    DuplicateFieldId,
    #[error("purpose {0} must define at least one required field")]
    PurposeHasNoRequiredFields(PurposeId),
    #[error("purpose {0} must define at least one acceptable information state")]
    PurposeHasNoAcceptableStates(PurposeId),
}

fn normalized_required(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, ProfileError> {
    let value = value.into().trim().to_owned();
    if value.is_empty() {
        Err(ProfileError::RequiredField(field))
    } else {
        Ok(value)
    }
}

fn valid_namespaced_id(value: &str) -> bool {
    value.contains('.')
        && value
            .split('.')
            .all(|segment| valid_id_segment(segment, true))
}

fn valid_simple_id(value: &str) -> bool {
    value
        .split(['.', '-'])
        .all(|segment| valid_id_segment(segment, false))
}

fn valid_id_segment(value: &str, allow_underscore: bool) -> bool {
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    first.is_ascii_lowercase()
        && characters.all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || (allow_underscore && character == '_')
        })
}

#[cfg(test)]
mod tests {
    use super::{
        Classification, FieldDefinition, FieldDefinitionSpec, FieldId, FieldType, FieldValue,
        InformationState, ProfileError, ProfileSnapshot, PurposeDefinition, PurposeId, TopicPack,
        TopicPackId,
    };
    use std::collections::BTreeSet;

    fn field(
        id: &str,
        classification: Classification,
        sealed_by_default: bool,
    ) -> Result<FieldDefinition, ProfileError> {
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
    fn field_ids_are_stable_and_namespaced() {
        assert!(FieldId::parse("travel.seat_preference").is_ok());
        assert!(FieldId::parse("seat_preference").is_err());
        assert!(FieldId::parse("Travel.seat").is_err());
    }

    #[test]
    fn sensitive_fields_and_values_require_sealing() {
        let definition = field("travel.accessibility", Classification::Sensitive, false);
        assert!(matches!(
            definition,
            Err(ProfileError::SensitiveFieldIsNotSealed(_))
        ));

        let definition = field("travel.accessibility", Classification::Sensitive, true);
        assert!(definition.is_ok());
        let Ok(definition) = definition else {
            return;
        };
        let value = FieldValue::new(
            &definition,
            InformationState::Verified,
            Some("Step-free route".to_owned()),
            false,
        );
        assert!(matches!(
            value,
            Err(ProfileError::SensitiveValueIsNotSealed(_))
        ));
    }

    #[test]
    fn unknown_declined_and_not_applicable_are_not_empty_known_values() {
        let definition = field("identity.preferred_channel", Classification::Private, false);
        assert!(definition.is_ok());
        let Ok(definition) = definition else {
            return;
        };
        assert!(
            FieldValue::new(&definition, InformationState::Known, None, false).is_err()
        );
        assert!(FieldValue::new(
            &definition,
            InformationState::Unknown,
            Some("email".to_owned()),
            false
        )
        .is_err());
        assert!(
            FieldValue::new(&definition, InformationState::Declined, None, false).is_ok()
        );
    }

    #[test]
    fn readiness_is_calculated_for_one_named_purpose() {
        let role_id = FieldId::parse("professional.current_role");
        let channel_id = FieldId::parse("identity.preferred_channel");
        assert!(role_id.is_ok());
        assert!(channel_id.is_ok());
        let (Ok(role_id), Ok(channel_id)) = (role_id, channel_id) else {
            return;
        };
        let role = field("professional.current_role", Classification::Private, false);
        let channel = field("identity.preferred_channel", Classification::Private, false);
        assert!(role.is_ok());
        assert!(channel.is_ok());
        let (Ok(role), Ok(channel)) = (role, channel) else {
            return;
        };

        let mut profile = ProfileSnapshot::new();
        let channel_value = FieldValue::new(
            &channel,
            InformationState::Verified,
            Some("email".to_owned()),
            false,
        );
        assert!(channel_value.is_ok());
        let Ok(channel_value) = channel_value else {
            return;
        };
        profile.set(channel_value);

        let purpose = PurposeDefinition::new(
            PurposeId::parse("meeting_brief").unwrap_or_else(|error| {
                unreachable!("fixture purpose ID should be valid: {error}")
            }),
            1,
            BTreeSet::from([role_id.clone(), channel_id]),
            BTreeSet::from([InformationState::Verified]),
        );
        assert!(purpose.is_ok());
        let Ok(purpose) = purpose else {
            return;
        };
        let readiness = purpose.calculate_readiness(&profile);
        assert!(!readiness.is_ready());
        assert_eq!(readiness.satisfied_count, 1);
        assert_eq!(readiness.gaps.len(), 1);
        assert_eq!(readiness.gaps[0].field_id, role_id);
    }

    #[test]
    fn topic_pack_rejects_duplicate_fields() {
        let first = field("pets.names", Classification::Private, false);
        assert!(first.is_ok());
        let Ok(first) = first else {
            return;
        };
        let pack = TopicPack::new(
            TopicPackId::parse("pets").unwrap_or_else(|error| {
                unreachable!("fixture pack ID should be valid: {error}")
            }),
            1,
            "Pets",
            vec![first.clone(), first],
        );
        assert_eq!(pack, Err(ProfileError::DuplicateFieldId));
    }
}
