//! Recoverable canonical-operation contracts.
//!
//! The Workspace context owns the operation vocabulary and invariants. A
//! filesystem adapter owns staging, flushing, publication, and recovery.

use chrono::{DateTime, Utc};
use liaison_shared_kernel::{OperationId, WorkspaceId};
use serde::{Deserialize, Serialize};
use std::{error::Error as StdError, fmt};

pub const OPERATION_FORMAT: &str = "liaison-canonical-operation";
pub const OPERATION_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CanonicalPath(String);

impl CanonicalPath {
    pub fn parse(value: impl Into<String>) -> Result<Self, OperationContractError> {
        let value = value.into();
        if value.is_empty()
            || value.len() > 1_024
            || value.starts_with('/')
            || value.ends_with('/')
            || value.contains('\\')
            || value.chars().any(char::is_control)
        {
            return Err(OperationContractError::UnsafePath(value));
        }
        if value
            .split('/')
            .any(|segment| segment.is_empty() || segment == "." || segment == "..")
        {
            return Err(OperationContractError::UnsafePath(value));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CanonicalPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CanonicalDigest(String);

impl CanonicalDigest {
    pub fn parse(value: impl Into<String>) -> Result<Self, OperationContractError> {
        let value = value.into();
        if value.len() != 64
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(OperationContractError::InvalidDigest(value));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CanonicalDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum CanonicalPrecondition {
    Absent,
    ExactDigest {
        digest: CanonicalDigest,
        #[serde(skip_serializing_if = "Option::is_none")]
        expected_revision: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        revision_format: Option<String>,
    },
}

impl CanonicalPrecondition {
    pub fn exact_digest(
        digest: CanonicalDigest,
        expected_revision: Option<u64>,
        revision_format: Option<String>,
    ) -> Result<Self, OperationContractError> {
        if expected_revision.is_some_and(|revision| revision == 0) {
            return Err(OperationContractError::InvalidRevision);
        }
        if revision_format
            .as_ref()
            .is_some_and(|format| format.trim().is_empty())
        {
            return Err(OperationContractError::InvalidRevisionFormat);
        }
        if expected_revision.is_some() != revision_format.is_some() {
            return Err(OperationContractError::IncompleteRevisionPrecondition);
        }
        Ok(Self::ExactDigest {
            digest,
            expected_revision,
            revision_format,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalOperationTarget {
    pub ordinal: u32,
    pub path: CanonicalPath,
    pub content_digest: CanonicalDigest,
    pub size_bytes: u64,
    pub precondition: CanonicalPrecondition,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalOperationManifest {
    pub format: String,
    pub schema_version: u32,
    pub operation_id: OperationId,
    pub workspace_id: WorkspaceId,
    pub started_at: DateTime<Utc>,
    pub targets: Vec<CanonicalOperationTarget>,
}

impl CanonicalOperationManifest {
    pub fn new(
        operation_id: OperationId,
        workspace_id: WorkspaceId,
        started_at: DateTime<Utc>,
        mut targets: Vec<CanonicalOperationTarget>,
    ) -> Result<Self, OperationContractError> {
        if targets.is_empty() {
            return Err(OperationContractError::EmptyOperation);
        }
        targets.sort_by(|left, right| left.path.cmp(&right.path));
        for (index, target) in targets.iter_mut().enumerate() {
            target.ordinal =
                u32::try_from(index).map_err(|_| OperationContractError::TooManyTargets)?;
        }
        if targets
            .windows(2)
            .any(|window| window[0].path == window[1].path)
        {
            return Err(OperationContractError::DuplicateTarget);
        }
        Ok(Self {
            format: OPERATION_FORMAT.to_owned(),
            schema_version: OPERATION_SCHEMA_VERSION,
            operation_id,
            workspace_id,
            started_at,
            targets,
        })
    }

    pub fn validate(&self) -> Result<(), OperationContractError> {
        if self.format != OPERATION_FORMAT {
            return Err(OperationContractError::UnexpectedFormat(
                self.format.clone(),
            ));
        }
        if self.schema_version != OPERATION_SCHEMA_VERSION {
            return Err(OperationContractError::UnsupportedSchema {
                found: self.schema_version,
                supported: OPERATION_SCHEMA_VERSION,
            });
        }
        if self.targets.is_empty() {
            return Err(OperationContractError::EmptyOperation);
        }
        for (index, target) in self.targets.iter().enumerate() {
            if usize::try_from(target.ordinal).ok() != Some(index) {
                return Err(OperationContractError::InvalidOrdinal);
            }
        }
        if self
            .targets
            .windows(2)
            .any(|window| window[0].path >= window[1].path)
        {
            return Err(OperationContractError::TargetsNotStrictlyOrdered);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OperationPhase {
    Staged,
    CommitDecided,
    Publishing,
    Complete,
    DiscardedBeforeCommit,
    RecoveryConflict,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationReceipt {
    pub operation_id: OperationId,
    pub workspace_id: WorkspaceId,
    pub completed_at: DateTime<Utc>,
    pub published_targets: u32,
    pub phase: OperationPhase,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OperationRecoveryReport {
    pub discarded_before_commit: u32,
    pub rolled_forward: u32,
    pub already_complete: u32,
}

impl OperationRecoveryReport {
    #[must_use]
    pub const fn total_examined(&self) -> u32 {
        self.discarded_before_commit
            .saturating_add(self.rolled_forward)
            .saturating_add(self.already_complete)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalWrite {
    pub path: CanonicalPath,
    pub content: Vec<u8>,
    pub precondition: CanonicalPrecondition,
}

impl CanonicalWrite {
    pub fn new(
        path: CanonicalPath,
        content: Vec<u8>,
        precondition: CanonicalPrecondition,
    ) -> Result<Self, OperationContractError> {
        Ok(Self {
            path,
            content,
            precondition,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationContext {
    pub operation_id: OperationId,
    pub started_at: DateTime<Utc>,
}

impl OperationContext {
    #[must_use]
    pub const fn new(operation_id: OperationId, started_at: DateTime<Utc>) -> Self {
        Self {
            operation_id,
            started_at,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultPoint {
    None,
    AfterStaging,
    AfterCommitDecision,
    AfterPublishedTargets(u32),
    BeforeComplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoverableOperationErrorKind {
    Contract,
    Storage,
    Precondition,
    RecoveryConflict,
    FaultInjected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoverableOperationError {
    pub kind: RecoverableOperationErrorKind,
    pub operation_id: Option<OperationId>,
    pub path: Option<CanonicalPath>,
    pub message: String,
}

impl RecoverableOperationError {
    #[must_use]
    pub fn new(kind: RecoverableOperationErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            operation_id: None,
            path: None,
            message: message.into(),
        }
    }

    #[must_use]
    pub const fn with_operation(mut self, operation_id: OperationId) -> Self {
        self.operation_id = Some(operation_id);
        self
    }

    #[must_use]
    pub fn with_path(mut self, path: CanonicalPath) -> Self {
        self.path = Some(path);
        self
    }
}

impl fmt::Display for RecoverableOperationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl StdError for RecoverableOperationError {}

pub trait RecoverableOperationStore: fmt::Debug + Send + Sync {
    fn execute_operation(
        &self,
        workspace_id: WorkspaceId,
        context: OperationContext,
        writes: Vec<CanonicalWrite>,
    ) -> Result<OperationReceipt, RecoverableOperationError>;

    fn recover_operations(
        &self,
        workspace_id: WorkspaceId,
    ) -> Result<OperationRecoveryReport, RecoverableOperationError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationContractError {
    UnsafePath(String),
    InvalidDigest(String),
    InvalidRevision,
    InvalidRevisionFormat,
    IncompleteRevisionPrecondition,
    EmptyOperation,
    TooManyTargets,
    DuplicateTarget,
    InvalidOrdinal,
    TargetsNotStrictlyOrdered,
    UnexpectedFormat(String),
    UnsupportedSchema { found: u32, supported: u32 },
}

impl fmt::Display for OperationContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsafePath(path) => write!(formatter, "unsafe canonical path: {path}"),
            Self::InvalidDigest(value) => write!(formatter, "invalid SHA-256 digest: {value}"),
            Self::InvalidRevision => formatter.write_str("expected revision must be positive"),
            Self::InvalidRevisionFormat => formatter.write_str("revision format must not be blank"),
            Self::IncompleteRevisionPrecondition => formatter.write_str(
                "revision and revision format must either both be present or both be absent",
            ),
            Self::EmptyOperation => {
                formatter.write_str("operation must contain at least one target")
            }
            Self::TooManyTargets => formatter.write_str("operation contains too many targets"),
            Self::DuplicateTarget => formatter.write_str("operation contains a duplicate target"),
            Self::InvalidOrdinal => formatter.write_str("operation target ordinal is invalid"),
            Self::TargetsNotStrictlyOrdered => {
                formatter.write_str("operation targets are not strictly ordered")
            }
            Self::UnexpectedFormat(value) => {
                write!(formatter, "unexpected operation format: {value}")
            }
            Self::UnsupportedSchema { found, supported } => write!(
                formatter,
                "operation schema {found} is unsupported; this build supports {supported}"
            ),
        }
    }
}

impl StdError for OperationContractError {}

#[cfg(test)]
mod tests {
    use super::{
        CanonicalDigest, CanonicalOperationManifest, CanonicalOperationTarget, CanonicalPath,
        CanonicalPrecondition,
    };
    use chrono::{DateTime, Utc};
    use liaison_shared_kernel::{OperationId, WorkspaceId};
    use uuid::Uuid;

    #[test]
    fn canonical_paths_reject_escape_and_platform_specific_separators() {
        assert!(CanonicalPath::parse("people/person.md").is_ok());
        assert!(CanonicalPath::parse("../outside").is_err());
        assert!(CanonicalPath::parse("people\\person.md").is_err());
        assert!(CanonicalPath::parse("people//person.md").is_err());
    }

    #[test]
    fn manifest_sorts_targets_and_assigns_ordinals() {
        let digest = CanonicalDigest::parse("a".repeat(64));
        assert!(digest.is_ok());
        let Ok(digest) = digest else {
            return;
        };
        let first = CanonicalOperationTarget {
            ordinal: 99,
            path: CanonicalPath::parse("people/z.md")
                .unwrap_or_else(|error| unreachable!("test path must be valid: {error}")),
            content_digest: digest.clone(),
            size_bytes: 1,
            precondition: CanonicalPrecondition::Absent,
        };
        let second = CanonicalOperationTarget {
            ordinal: 99,
            path: CanonicalPath::parse("people/a.md")
                .unwrap_or_else(|error| unreachable!("test path must be valid: {error}")),
            content_digest: digest,
            size_bytes: 1,
            precondition: CanonicalPrecondition::Absent,
        };
        let manifest = CanonicalOperationManifest::new(
            OperationId::from_uuid(Uuid::from_u128(1)),
            WorkspaceId::from_uuid(Uuid::from_u128(2)),
            DateTime::<Utc>::UNIX_EPOCH,
            vec![first, second],
        );
        assert!(manifest.is_ok());
        let Ok(manifest) = manifest else {
            return;
        };
        assert_eq!(manifest.targets[0].path.as_str(), "people/a.md");
        assert_eq!(manifest.targets[0].ordinal, 0);
        assert_eq!(manifest.targets[1].ordinal, 1);
        assert!(manifest.validate().is_ok());
    }
}
