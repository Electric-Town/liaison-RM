//! Connections bounded context.
//!
//! Owns provider-neutral capability declarations, connection grants, provider
//! registration, and object-store semantics. Provider SDKs and network clients
//! remain in adapters.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use chrono::{DateTime, Utc};
use liaison_shared_kernel::{MemberId, Revision};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fmt};
use thiserror::Error;
use uuid::Uuid;

pub const OBJECT_STORE_CONTRACT_V1: &str = "object-store@1";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProviderId(String);

impl ProviderId {
    pub fn parse(value: impl Into<String>) -> Result<Self, ConnectionsError> {
        let value = normalise_identifier(value.into(), "provider id")?;
        if value
            .chars()
            .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-')
        {
            Ok(Self(value))
        } else {
            Err(ConnectionsError::InvalidProviderId(value))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProviderId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ContractId(String);

impl ContractId {
    pub fn parse(value: impl Into<String>) -> Result<Self, ConnectionsError> {
        let value = normalise_identifier(value.into(), "contract id")?;
        let Some((name, version)) = value.rsplit_once('@') else {
            return Err(ConnectionsError::InvalidContractId(value));
        };
        if name.is_empty()
            || version.parse::<u32>().ok().filter(|number| *number > 0).is_none()
            || !name
                .chars()
                .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-')
        {
            return Err(ConnectionsError::InvalidContractId(value));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ContractId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractClaim {
    pub contract: ContractId,
    pub operations: BTreeSet<String>,
}

impl ContractClaim {
    pub fn new(
        contract: ContractId,
        operations: impl IntoIterator<Item = String>,
    ) -> Result<Self, ConnectionsError> {
        let operations = operations
            .into_iter()
            .map(|operation| normalise_identifier(operation, "operation"))
            .collect::<Result<BTreeSet<_>, _>>()?;
        if operations.is_empty() {
            return Err(ConnectionsError::RequiredField("contract operations"));
        }
        Ok(Self {
            contract,
            operations,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SafeMode {
    Backup,
    Restore,
    SingleWriterPublication,
    ImmutableTransport,
    MultiWriterSynchronisation,
    ContactsImport,
    ContactsSynchronisation,
    CalendarImport,
    EmailMetadataImport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConsistencyModel {
    LocalFilesystem,
    StrongReadAfterWrite,
    Eventual,
    ProviderDeclared,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLimits {
    pub maximum_object_bytes: Option<u64>,
    pub maximum_page_size: Option<u32>,
    pub requests_per_minute: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConformanceReference {
    pub report_path: String,
    pub tested_at: DateTime<Utc>,
    pub contract_versions: BTreeSet<ContractId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderDescriptor {
    pub provider_id: ProviderId,
    pub provider_version: String,
    pub display_name: String,
    pub contracts: Vec<ContractClaim>,
    pub safe_modes: BTreeSet<SafeMode>,
    pub configuration_schema: String,
    pub secret_slots: BTreeSet<String>,
    pub destinations: BTreeSet<String>,
    pub consistency: ConsistencyModel,
    pub limits: ProviderLimits,
    pub conformance: Option<ConformanceReference>,
}

impl ProviderDescriptor {
    pub fn validate(&self) -> Result<(), ConnectionsError> {
        normalise_required(self.provider_version.clone(), "provider version")?;
        normalise_required(self.display_name.clone(), "display name")?;
        normalise_required(self.configuration_schema.clone(), "configuration schema")?;
        if self.contracts.is_empty() {
            return Err(ConnectionsError::RequiredField("provider contracts"));
        }
        let unique_contracts = self
            .contracts
            .iter()
            .map(|claim| claim.contract.clone())
            .collect::<BTreeSet<_>>();
        if unique_contracts.len() != self.contracts.len() {
            return Err(ConnectionsError::DuplicateContractClaim);
        }
        for destination in &self.destinations {
            normalise_required(destination.clone(), "destination")?;
        }
        for slot in &self.secret_slots {
            normalise_identifier(slot.clone(), "secret slot")?;
        }
        Ok(())
    }

    pub fn supports_contract(&self, contract: &ContractId) -> bool {
        self.contracts
            .iter()
            .any(|claim| &claim.contract == contract)
    }

    pub fn advertises(&self, mode: SafeMode) -> bool {
        self.safe_modes.contains(&mode)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ConnectionId(Uuid);

impl ConnectionId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for ConnectionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ConnectionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GrantId(Uuid);

impl GrantId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for GrantId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConnectionState {
    Draft,
    Configured,
    Tested,
    Active,
    Suspended,
    Revoked,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectionGrant {
    pub id: GrantId,
    pub revision: Revision,
    pub connection_id: ConnectionId,
    pub provider_id: ProviderId,
    pub purpose: String,
    pub endpoint: Option<String>,
    pub operations: BTreeSet<String>,
    pub data_classes: BTreeSet<String>,
    pub record_scope: BTreeSet<String>,
    pub schedule: Option<String>,
    pub retention: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub approved_by: MemberId,
    pub revoked_at: Option<DateTime<Utc>>,
}

impl ConnectionGrant {
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        connection_id: ConnectionId,
        provider_id: ProviderId,
        purpose: impl Into<String>,
        endpoint: Option<String>,
        operations: impl IntoIterator<Item = String>,
        data_classes: impl IntoIterator<Item = String>,
        record_scope: impl IntoIterator<Item = String>,
        approved_by: MemberId,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<Self, ConnectionsError> {
        let purpose = normalise_required(purpose.into(), "grant purpose")?;
        let operations = normalise_set(operations, "grant operation")?;
        let data_classes = normalise_set(data_classes, "data class")?;
        let record_scope = normalise_set(record_scope, "record scope")?;
        if operations.is_empty() {
            return Err(ConnectionsError::RequiredField("grant operations"));
        }
        if data_classes.is_empty() {
            return Err(ConnectionsError::RequiredField("grant data classes"));
        }
        if record_scope.is_empty() {
            return Err(ConnectionsError::RequiredField("grant record scope"));
        }
        if let Some(endpoint) = endpoint.as_ref() {
            normalise_required(endpoint.clone(), "grant endpoint")?;
        }
        Ok(Self {
            id: GrantId::new(),
            revision: Revision::INITIAL,
            connection_id,
            provider_id,
            purpose,
            endpoint,
            operations,
            data_classes,
            record_scope,
            schedule: None,
            retention: None,
            expires_at,
            approved_by,
            revoked_at: None,
        })
    }

    pub fn permits(
        &self,
        operation: &str,
        data_class: &str,
        scope: &str,
        now: DateTime<Utc>,
    ) -> bool {
        self.revoked_at.is_none()
            && self.expires_at.is_none_or(|expiry| now < expiry)
            && self.operations.contains(operation)
            && self.data_classes.contains(data_class)
            && self.record_scope.contains(scope)
    }

    pub fn revoke(&mut self, now: DateTime<Utc>) -> Result<(), ConnectionsError> {
        if self.revoked_at.is_none() {
            self.revoked_at = Some(now);
            self.revision = self
                .revision
                .next()
                .map_err(|_| ConnectionsError::RevisionOverflow)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObjectKey(String);

impl ObjectKey {
    pub fn parse(value: impl Into<String>) -> Result<Self, ObjectStoreError> {
        let value = value.into();
        if value.is_empty()
            || value.len() > 1024
            || value.starts_with('/')
            || value.starts_with('\\')
            || value.contains('\\')
            || value.contains('\0')
            || value.contains(':')
            || value
                .split('/')
                .any(|segment| segment.is_empty() || segment == "." || segment == "..")
        {
            return Err(ObjectStoreError::InvalidKey(value));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ObjectKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Sha256Digest(String);

impl Sha256Digest {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let digest = Sha256::digest(bytes);
        let mut encoded = String::with_capacity(64);
        const HEX: &[u8; 16] = b"0123456789abcdef";
        for byte in digest {
            encoded.push(char::from(HEX[usize::from(byte >> 4)]));
            encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
        Self(encoded)
    }

    pub fn parse(value: impl Into<String>) -> Result<Self, ObjectStoreError> {
        let value = value.into().to_ascii_lowercase();
        if value.len() != 64 || !value.chars().all(|character| character.is_ascii_hexdigit()) {
            return Err(ObjectStoreError::InvalidDigest(value));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Sha256Digest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectMetadata {
    pub key: ObjectKey,
    pub size: u64,
    pub digest: Sha256Digest,
    pub revision: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PutOutcome {
    Created,
    AlreadyPresent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListPage {
    pub objects: Vec<ObjectMetadata>,
    pub next_cursor: Option<String>,
}

pub trait ObjectStore: Send + Sync {
    fn put_immutable(
        &self,
        key: &ObjectKey,
        bytes: &[u8],
        expected_digest: &Sha256Digest,
    ) -> Result<PutOutcome, ObjectStoreError>;

    fn get(&self, key: &ObjectKey) -> Result<Vec<u8>, ObjectStoreError>;

    fn head(&self, key: &ObjectKey) -> Result<ObjectMetadata, ObjectStoreError>;

    fn list(
        &self,
        prefix: Option<&str>,
        cursor: Option<&str>,
        limit: u32,
    ) -> Result<ListPage, ObjectStoreError>;

    fn delete_if_digest(
        &self,
        key: &ObjectKey,
        expected_digest: &Sha256Digest,
    ) -> Result<(), ObjectStoreError>;

    fn replace_manifest_if_revision(
        &self,
        key: &ObjectKey,
        expected_revision: Option<&str>,
        bytes: &[u8],
    ) -> Result<ObjectMetadata, ObjectStoreError>;
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ObjectStoreError {
    #[error("invalid object key: {0}")]
    InvalidKey(String),
    #[error("invalid SHA-256 digest: {0}")]
    InvalidDigest(String),
    #[error("object does not exist: {0}")]
    NotFound(ObjectKey),
    #[error("immutable object already exists with different content: {0}")]
    ImmutableConflict(ObjectKey),
    #[error("object digest mismatch for {key}; expected {expected}, found {found}")]
    DigestMismatch {
        key: ObjectKey,
        expected: Sha256Digest,
        found: Sha256Digest,
    },
    #[error("manifest revision precondition failed for {key}; expected {expected:?}, found {found:?}")]
    RevisionConflict {
        key: ObjectKey,
        expected: Option<String>,
        found: Option<String>,
    },
    #[error("object-store path contains a symbolic link: {0}")]
    SymbolicLink(String),
    #[error("object-store operation exceeds provider limit")]
    LimitExceeded,
    #[error("object-store storage error: {0}")]
    Storage(String),
}

pub trait ProviderRegistry: Send + Sync {
    fn register(&self, descriptor: ProviderDescriptor) -> Result<(), ConnectionsError>;
    fn list(&self) -> Result<Vec<ProviderDescriptor>, ConnectionsError>;
    fn find(&self, provider_id: &ProviderId) -> Result<ProviderDescriptor, ConnectionsError>;
}

#[derive(Debug)]
pub struct RegisterProvider<R> {
    registry: R,
}

impl<R> RegisterProvider<R>
where
    R: ProviderRegistry,
{
    pub const fn new(registry: R) -> Self {
        Self { registry }
    }

    pub fn execute(&self, descriptor: ProviderDescriptor) -> Result<(), ConnectionsError> {
        descriptor.validate()?;
        self.registry.register(descriptor)
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ConnectionsError {
    #[error("{0} is required")]
    RequiredField(&'static str),
    #[error("invalid provider id: {0}")]
    InvalidProviderId(String),
    #[error("invalid contract id: {0}")]
    InvalidContractId(String),
    #[error("provider descriptor contains the same contract more than once")]
    DuplicateContractClaim,
    #[error("provider is already registered: {0}")]
    ProviderAlreadyRegistered(ProviderId),
    #[error("provider is not registered: {0}")]
    ProviderNotFound(ProviderId),
    #[error("grant revision overflowed")]
    RevisionOverflow,
    #[error("connections storage error: {0}")]
    Storage(String),
}

fn normalise_required(value: String, field: &'static str) -> Result<String, ConnectionsError> {
    let value = value.trim().to_owned();
    if value.is_empty() {
        Err(ConnectionsError::RequiredField(field))
    } else {
        Ok(value)
    }
}

fn normalise_identifier(value: String, field: &'static str) -> Result<String, ConnectionsError> {
    let value = normalise_required(value, field)?;
    if value.contains(char::is_whitespace) {
        Err(ConnectionsError::RequiredField(field))
    } else {
        Ok(value)
    }
}

fn normalise_set(
    values: impl IntoIterator<Item = String>,
    field: &'static str,
) -> Result<BTreeSet<String>, ConnectionsError> {
    values
        .into_iter()
        .map(|value| normalise_identifier(value, field))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        ConnectionGrant, ConnectionId, ContractId, ObjectKey, ProviderId, Sha256Digest,
    };
    use chrono::{Duration, Utc};
    use liaison_shared_kernel::MemberId;

    #[test]
    fn provider_and_contract_ids_are_constrained() {
        assert!(ProviderId::parse("local-folder").is_ok());
        assert!(ProviderId::parse("Local Folder").is_err());
        assert!(ContractId::parse("object-store@1").is_ok());
        assert!(ContractId::parse("object-store").is_err());
    }

    #[test]
    fn object_keys_reject_traversal_and_platform_paths() {
        for value in ["", "/root", "../secret", "a/../secret", "C:/secret", "a\\b"] {
            assert!(ObjectKey::parse(value).is_err());
        }
        assert!(ObjectKey::parse("objects/ab/cd").is_ok());
    }

    #[test]
    fn digest_is_stable_and_validated() {
        let digest = Sha256Digest::from_bytes(b"liaison");
        assert_eq!(digest.as_str().len(), 64);
        assert_eq!(Sha256Digest::parse(digest.to_string()), Ok(digest));
    }

    #[test]
    fn grant_enforces_scope_expiry_and_revocation() {
        let now = Utc::now();
        let grant = ConnectionGrant::create(
            ConnectionId::new(),
            ProviderId::parse("local-folder").unwrap_or_else(|error| {
                unreachable!("static provider id should be valid: {error}")
            }),
            "encrypted backup",
            None,
            ["put-immutable".to_owned()],
            ["encrypted-backup".to_owned()],
            ["workspace:example".to_owned()],
            MemberId::new(),
            Some(now + Duration::hours(1)),
        );
        assert!(grant.is_ok());
        if let Ok(mut grant) = grant {
            assert!(grant.permits(
                "put-immutable",
                "encrypted-backup",
                "workspace:example",
                now
            ));
            assert!(grant.revoke(now).is_ok());
            assert!(!grant.permits(
                "put-immutable",
                "encrypted-backup",
                "workspace:example",
                now
            ));
        }
    }
}
