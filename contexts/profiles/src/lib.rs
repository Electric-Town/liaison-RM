//! Identity and Profiles bounded context.
//!
//! Owns Topic Packs, stable Field Definitions, explicit information states,
//! profile values, Purpose Definitions, and purpose-specific readiness.

#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter};
use thiserror::Error;

/// Field-id namespaces reserved for canonical identity and contact facts.
/// A user-defined custom field cannot shadow, retype, or replace them.
pub const RESERVED_FIELD_NAMESPACES: [&str; 4] = ["identity", "contact", "name", "dietary"];

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

impl FieldType {
    /// Selection types constrain a value to an explicit option set.
    #[must_use]
    pub const fn is_selection(self) -> bool {
        matches!(self, Self::Enum | Self::MultiSelect)
    }

    /// A calculated field is derived and read-only; a user never sets it.
    #[must_use]
    pub const fn is_user_writable(self) -> bool {
        !matches!(self, Self::Calculated)
    }

    /// Validates one non-empty payload against this type and, for selection
    /// types, the field's options. The payload is already trimmed and known
    /// non-empty when this runs.
    pub fn validate_payload(
        self,
        payload: &str,
        options: Option<&FieldOptions>,
    ) -> Result<(), ProfileError> {
        let invalid = |reason: &'static str| Err(ProfileError::InvalidValue { kind: self, reason });
        match self {
            Self::ShortText => {
                if payload
                    .chars()
                    .any(|character| character == '\n' || character == '\r')
                {
                    return invalid("short text must be a single line");
                }
                Ok(())
            }
            Self::Number | Self::Measurement => {
                let number = payload.split_whitespace().next().unwrap_or(payload);
                if number.parse::<f64>().is_err() {
                    return invalid("must begin with a number");
                }
                Ok(())
            }
            Self::Boolean => match payload {
                "true" | "false" => Ok(()),
                _ => invalid("must be exactly true or false"),
            },
            Self::Date => match NaiveDate::parse_from_str(payload, "%Y-%m-%d") {
                Ok(_) => Ok(()),
                Err(_) => invalid("must be a real ISO date, YYYY-MM-DD"),
            },
            Self::PartialDate | Self::RecurringDate => {
                if parse_partial_date(payload) {
                    Ok(())
                } else {
                    invalid("must be YYYY-MM-DD or MM-DD with real month and day")
                }
            }
            Self::Enum => {
                let options = options.ok_or(ProfileError::OptionsRequired(self))?;
                if options.contains(payload) {
                    Ok(())
                } else {
                    invalid("value is not one of the field's options")
                }
            }
            Self::MultiSelect => {
                let options = options.ok_or(ProfileError::OptionsRequired(self))?;
                let mut seen = BTreeSet::new();
                let mut any = false;
                for line in payload
                    .lines()
                    .map(str::trim)
                    .filter(|line| !line.is_empty())
                {
                    any = true;
                    if !options.contains(line) {
                        return invalid("a selected value is not one of the field's options");
                    }
                    if !seen.insert(line.to_owned()) {
                        return invalid("a value is selected more than once");
                    }
                }
                if any {
                    Ok(())
                } else {
                    invalid("select at least one option")
                }
            }
            // Free-form and reference types accept any non-empty payload; the
            // reference targets themselves are resolved by their owning context.
            Self::LongText
            | Self::Markdown
            | Self::Address
            | Self::Location
            | Self::EntityReference
            | Self::EntityReferenceList
            | Self::ResourceReference
            | Self::ResourceReferenceList
            | Self::List
            | Self::Sealed
            | Self::Calculated => Ok(()),
        }
    }
}

/// The explicit, ordered option set for a selection field. Options are
/// non-empty, unique, and single-line so a value can be matched exactly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldOptions {
    values: Vec<String>,
}

impl FieldOptions {
    pub fn new(values: impl IntoIterator<Item = impl Into<String>>) -> Result<Self, ProfileError> {
        let mut cleaned = Vec::new();
        let mut seen = BTreeSet::new();
        for value in values {
            let value = value.into().trim().to_owned();
            if value.is_empty() {
                return Err(ProfileError::EmptyOption);
            }
            if value.contains('\n') || value.contains('\r') {
                return Err(ProfileError::MultilineOption);
            }
            if !seen.insert(value.clone()) {
                return Err(ProfileError::DuplicateOption(value));
            }
            cleaned.push(value);
        }
        if cleaned.is_empty() {
            return Err(ProfileError::NoOptions);
        }
        Ok(Self { values: cleaned })
    }

    #[must_use]
    pub fn contains(&self, value: &str) -> bool {
        self.values.iter().any(|option| option == value)
    }

    #[must_use]
    pub fn values(&self) -> &[String] {
        &self.values
    }
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
    pub options: Option<FieldOptions>,
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
    options: Option<FieldOptions>,
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
        // Options belong to selection types and to no others.
        if spec.field_type.is_selection() && spec.options.is_none() {
            return Err(ProfileError::OptionsRequired(spec.field_type));
        }
        if !spec.field_type.is_selection() && spec.options.is_some() {
            return Err(ProfileError::OptionsNotAllowed(spec.field_type));
        }
        Ok(Self {
            id: spec.id,
            label,
            field_type: spec.field_type,
            classification: spec.classification,
            required_for: spec.required_for,
            stale_after_days: spec.stale_after_days,
            sealed_by_default: spec.sealed_by_default,
            options: spec.options,
        })
    }

    /// A user-defined field. Everything `new` checks, plus the rule that a
    /// custom field cannot occupy a reserved canonical namespace, so it can
    /// never shadow, retype, or replace identity, contact, name, or the
    /// constrained dietary facts.
    pub fn new_custom(spec: FieldDefinitionSpec) -> Result<Self, ProfileError> {
        let namespace = spec.id.as_str().split('.').next().unwrap_or_default();
        if RESERVED_FIELD_NAMESPACES.contains(&namespace) {
            return Err(ProfileError::ReservedFieldNamespace(spec.id));
        }
        Self::new(spec)
    }

    #[must_use]
    pub const fn options(&self) -> Option<&FieldOptions> {
        self.options.as_ref()
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
        if !definition.field_type().is_user_writable() && payload.is_some() {
            return Err(ProfileError::CalculatedFieldIsReadOnly(
                definition.id().clone(),
            ));
        }
        if let Some(value) = payload.as_deref() {
            definition
                .field_type()
                .validate_payload(value, definition.options())?;
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
    #[error("value for a {kind:?} field is invalid: {reason}")]
    InvalidValue {
        kind: FieldType,
        reason: &'static str,
    },
    #[error("a {0:?} field requires an option set")]
    OptionsRequired(FieldType),
    #[error("a {0:?} field must not define an option set")]
    OptionsNotAllowed(FieldType),
    #[error("an option must not be empty")]
    EmptyOption,
    #[error("an option must be a single line")]
    MultilineOption,
    #[error("duplicate option: {0}")]
    DuplicateOption(String),
    #[error("a selection field must define at least one option")]
    NoOptions,
    #[error("a calculated field {0} is read-only and cannot be set by a user")]
    CalculatedFieldIsReadOnly(FieldId),
    #[error("custom field {0} uses a reserved canonical namespace")]
    ReservedFieldNamespace(FieldId),
}

/// Accepts `YYYY-MM-DD` or `MM-DD` (year unknown), rejecting impossible
/// month/day combinations without inventing a year.
fn parse_partial_date(value: &str) -> bool {
    if NaiveDate::parse_from_str(value, "%Y-%m-%d").is_ok() {
        return true;
    }
    let parts: Vec<&str> = value.split('-').filter(|part| !part.is_empty()).collect();
    if parts.len() != 2 {
        return false;
    }
    let (Ok(month), Ok(day)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) else {
        return false;
    };
    // A leap year makes 29 February valid when the year is unknown.
    NaiveDate::from_ymd_opt(2024, month, day).is_some()
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
        Classification, FieldDefinition, FieldDefinitionSpec, FieldId, FieldOptions, FieldType,
        FieldValue, InformationState, ProfileError, ProfileSnapshot, PurposeDefinition, PurposeId,
        TopicPack, TopicPackId,
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
            options: None,
        })
    }

    fn typed_field(
        id: &str,
        field_type: FieldType,
        options: Option<FieldOptions>,
    ) -> Result<FieldDefinition, ProfileError> {
        FieldDefinition::new(FieldDefinitionSpec {
            id: FieldId::parse(id)?,
            label: "Typed fixture".to_owned(),
            field_type,
            classification: Classification::Public,
            required_for: BTreeSet::new(),
            stale_after_days: None,
            sealed_by_default: false,
            options,
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
        assert!(FieldValue::new(&definition, InformationState::Known, None, false).is_err());
        assert!(
            FieldValue::new(
                &definition,
                InformationState::Unknown,
                Some("email".to_owned()),
                false
            )
            .is_err()
        );
        assert!(FieldValue::new(&definition, InformationState::Declined, None, false).is_ok());
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
        let (Ok(_role), Ok(channel)) = (role, channel) else {
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
            {
                let parsed = PurposeId::parse("meeting-brief");
                assert!(parsed.is_ok());
                let Ok(value) = parsed else {
                    return;
                };
                value
            },
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
            {
                let parsed = TopicPackId::parse("pets");
                assert!(parsed.is_ok());
                let Ok(value) = parsed else {
                    return;
                };
                value
            },
            1,
            "Pets",
            vec![first.clone(), first],
        );
        assert_eq!(pack, Err(ProfileError::DuplicateFieldId));
    }

    fn value(definition: &FieldDefinition, payload: &str) -> Result<FieldValue, ProfileError> {
        FieldValue::new(
            definition,
            InformationState::Known,
            Some(payload.to_owned()),
            false,
        )
    }

    #[test]
    fn typed_values_are_validated_against_their_field_type() {
        let Ok(number) = typed_field("health.resting_rate", FieldType::Number, None) else {
            return;
        };
        assert!(value(&number, "58").is_ok());
        assert!(value(&number, "58 bpm").is_ok());
        assert!(matches!(
            value(&number, "fast"),
            Err(ProfileError::InvalidValue { .. })
        ));

        let Ok(flag) = typed_field("travel.window_seat", FieldType::Boolean, None) else {
            return;
        };
        assert!(value(&flag, "true").is_ok());
        assert!(matches!(
            value(&flag, "yes"),
            Err(ProfileError::InvalidValue { .. })
        ));

        let Ok(date) = typed_field("dates.joined_on", FieldType::Date, None) else {
            return;
        };
        assert!(
            value(&date, "2026-02-29").is_err(),
            "2026 is not a leap year"
        );
        assert!(value(&date, "2024-02-29").is_ok());
        assert!(matches!(
            value(&date, "the fifth"),
            Err(ProfileError::InvalidValue { .. })
        ));
    }

    #[test]
    fn a_partial_date_accepts_a_month_and_day_without_a_year() {
        let Ok(birthday) = typed_field("dates.birthday", FieldType::PartialDate, None) else {
            return;
        };
        assert!(value(&birthday, "08-14").is_ok());
        assert!(value(&birthday, "02-29").is_ok());
        assert!(value(&birthday, "1990-08-14").is_ok());
        assert!(matches!(
            value(&birthday, "13-40"),
            Err(ProfileError::InvalidValue { .. })
        ));
    }

    #[test]
    fn selection_fields_require_options_and_constrain_the_value() {
        // A non-selection type must not carry options.
        assert!(matches!(
            typed_field(
                "notes.free",
                FieldType::ShortText,
                FieldOptions::new(["a"]).ok(),
            ),
            Err(ProfileError::OptionsNotAllowed(_))
        ));
        // A selection type must carry options.
        assert!(matches!(
            typed_field("food.style", FieldType::Enum, None),
            Err(ProfileError::OptionsRequired(_))
        ));

        let Ok(options) = FieldOptions::new(["vegetarian", "vegan", "omnivore"]) else {
            return;
        };
        let Ok(style) = typed_field("food.style", FieldType::Enum, Some(options)) else {
            return;
        };
        assert!(value(&style, "vegan").is_ok());
        assert!(matches!(
            value(&style, "pescatarian"),
            Err(ProfileError::InvalidValue { .. })
        ));

        let Ok(multi_options) = FieldOptions::new(["nuts", "gluten", "dairy"]) else {
            return;
        };
        let Ok(avoids) = typed_field("food.avoids", FieldType::MultiSelect, Some(multi_options))
        else {
            return;
        };
        assert!(value(&avoids, "nuts\ndairy").is_ok());
        assert!(matches!(
            value(&avoids, "nuts\nshellfish"),
            Err(ProfileError::InvalidValue { .. })
        ));
        assert!(matches!(
            value(&avoids, "nuts\nnuts"),
            Err(ProfileError::InvalidValue { .. })
        ));
    }

    #[test]
    fn field_options_reject_empty_and_duplicate_values() {
        assert!(matches!(
            FieldOptions::new([" "]),
            Err(ProfileError::EmptyOption)
        ));
        assert!(matches!(
            FieldOptions::new(["a", "a"]),
            Err(ProfileError::DuplicateOption(_))
        ));
        assert!(matches!(
            FieldOptions::new(Vec::<String>::new()),
            Err(ProfileError::NoOptions)
        ));
    }

    #[test]
    fn a_custom_field_cannot_shadow_a_reserved_canonical_namespace() {
        let reserved = FieldDefinition::new_custom(FieldDefinitionSpec {
            id: match FieldId::parse("contact.email") {
                Ok(id) => id,
                Err(_) => return,
            },
            label: "My email".to_owned(),
            field_type: FieldType::ShortText,
            classification: Classification::Public,
            required_for: BTreeSet::new(),
            stale_after_days: None,
            sealed_by_default: false,
            options: None,
        });
        assert!(matches!(
            reserved,
            Err(ProfileError::ReservedFieldNamespace(_))
        ));

        let allowed = FieldDefinition::new_custom(FieldDefinitionSpec {
            id: match FieldId::parse("hobbies.instrument") {
                Ok(id) => id,
                Err(_) => return,
            },
            label: "Instrument".to_owned(),
            field_type: FieldType::ShortText,
            classification: Classification::Public,
            required_for: BTreeSet::new(),
            stale_after_days: None,
            sealed_by_default: false,
            options: None,
        });
        assert!(allowed.is_ok());
    }

    #[test]
    fn a_calculated_field_rejects_a_user_supplied_value() {
        let Ok(derived) = typed_field("stats.age", FieldType::Calculated, None) else {
            return;
        };
        assert!(matches!(
            value(&derived, "34"),
            Err(ProfileError::CalculatedFieldIsReadOnly(_))
        ));
    }
}
