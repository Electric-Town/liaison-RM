//! Workspace bounded context.
//!
//! Owns workspace identity, manifest invariants, lifecycle use cases, and the
//! repository port required by storage adapters.

#![allow(clippy::module_name_repetitions, clippy::missing_errors_doc)]

use chrono::{DateTime, Utc};
pub use liaison_shared_kernel::WorkspaceId;
use liaison_shared_kernel::WorkspaceSessionId;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    error::Error as StdError,
    fmt,
    path::Path,
    sync::{Condvar, Mutex},
};
use thiserror::Error;

pub const WORKSPACE_FORMAT: &str = "liaison-workspace";
pub const CURRENT_SCHEMA_VERSION: u32 = 1;
pub const WRITER_DIAGNOSTIC_FORMAT: &str = "liaison-workspace-writer-diagnostic";
pub const WRITER_DIAGNOSTIC_SCHEMA_VERSION: u32 = 1;
const DEFAULT_WORKSPACE_MODULE: &str = "people";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BuildProfile {
    Airgap,
    ConnectedLocal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkspaceProfile {
    Personal,
    Family,
    Team,
    Workplace,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceManifest {
    pub format: String,
    pub schema_version: u32,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub profile: WorkspaceProfile,
    pub build_profile: BuildProfile,
    pub default_locale: String,
    #[serde(default = "default_enabled_modules")]
    pub enabled_modules: Vec<String>,
}

fn default_enabled_modules() -> Vec<String> {
    vec![DEFAULT_WORKSPACE_MODULE.to_owned()]
}

impl WorkspaceManifest {
    pub fn new(
        workspace_id: WorkspaceId,
        name: impl Into<String>,
        profile: WorkspaceProfile,
        build_profile: BuildProfile,
        default_locale: impl Into<String>,
    ) -> Result<Self, WorkspaceError> {
        let name = name.into();
        let name = normalise_required(&name, "workspace name")?;
        let default_locale = default_locale.into();
        let default_locale = normalise_required(&default_locale, "default locale")?;
        Ok(Self {
            format: WORKSPACE_FORMAT.to_owned(),
            schema_version: CURRENT_SCHEMA_VERSION,
            workspace_id,
            name,
            profile,
            build_profile,
            default_locale,
            enabled_modules: default_enabled_modules(),
        })
    }

    pub fn validate(&self) -> Result<(), WorkspaceError> {
        if self.format != WORKSPACE_FORMAT {
            return Err(WorkspaceError::UnexpectedFormat(self.format.clone()));
        }
        if self.schema_version != CURRENT_SCHEMA_VERSION {
            return Err(WorkspaceError::UnsupportedSchema {
                found: self.schema_version,
                supported: CURRENT_SCHEMA_VERSION,
            });
        }
        normalise_required(&self.name, "workspace name")?;
        normalise_required(&self.default_locale, "default locale")?;
        validate_enabled_modules(&self.enabled_modules)?;
        Ok(())
    }
}

fn validate_enabled_modules(modules: &[String]) -> Result<(), WorkspaceError> {
    if modules.is_empty() {
        return Err(WorkspaceError::RequiredField("enabled modules"));
    }
    let mut unique = BTreeSet::new();
    for module in modules {
        let normalized = module.trim();
        let valid = normalized == module
            && !normalized.is_empty()
            && normalized.len() <= 128
            && normalized
                .chars()
                .next()
                .is_some_and(|character| character.is_ascii_lowercase())
            && normalized.split(['.', '-']).all(|segment| {
                !segment.is_empty()
                    && segment.chars().all(|character| {
                        character.is_ascii_lowercase() || character.is_ascii_digit()
                    })
            });
        if !valid || !unique.insert(normalized) {
            return Err(WorkspaceError::InvalidField("enabled modules"));
        }
    }
    if !unique.contains(DEFAULT_WORKSPACE_MODULE) {
        return Err(WorkspaceError::InvalidField("enabled modules"));
    }
    Ok(())
}

fn normalise_required(value: &str, field: &'static str) -> Result<String, WorkspaceError> {
    let value = value.trim().to_owned();
    if value.is_empty() {
        Err(WorkspaceError::RequiredField(field))
    } else {
        Ok(value)
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum WorkspaceError {
    #[error("{0} is required")]
    RequiredField(&'static str),
    #[error("{0} has an invalid value")]
    InvalidField(&'static str),
    #[error("unexpected workspace format: {0}")]
    UnexpectedFormat(String),
    #[error("workspace schema {found} is not supported; this build supports {supported}")]
    UnsupportedSchema { found: u32, supported: u32 },
    #[error("workspace already exists")]
    AlreadyExists,
    #[error("workspace initialisation target is not empty")]
    InitialiseTargetNotEmpty,
    #[error("workspace does not exist")]
    NotFound,
    #[error("workspace identity changed after the session opened")]
    SessionIdentityChanged {
        expected: WorkspaceId,
        found: WorkspaceId,
    },
    #[error("workspace schema changed after the session opened")]
    SessionSchemaChanged { expected: u32, found: u32 },
    #[error("workspace storage error: {0}")]
    Storage(String),
}

/// Best-effort information written only after the operating-system lock has
/// been acquired. This value is never proof of authority: it may be absent,
/// stale, malformed, or unrelated to the live holder.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriterDiagnostic {
    format: String,
    schema_version: u32,
    session_id: WorkspaceSessionId,
    process_id: u32,
    acquired_at: DateTime<Utc>,
}

impl WriterDiagnostic {
    #[must_use]
    pub fn new(
        session_id: WorkspaceSessionId,
        process_id: u32,
        acquired_at: DateTime<Utc>,
    ) -> Self {
        Self {
            format: WRITER_DIAGNOSTIC_FORMAT.to_owned(),
            schema_version: WRITER_DIAGNOSTIC_SCHEMA_VERSION,
            session_id,
            process_id,
            acquired_at,
        }
    }

    #[must_use]
    pub fn is_current_format(&self) -> bool {
        self.format == WRITER_DIAGNOSTIC_FORMAT
            && self.schema_version == WRITER_DIAGNOSTIC_SCHEMA_VERSION
    }

    #[must_use]
    pub const fn session_id(&self) -> WorkspaceSessionId {
        self.session_id
    }

    #[must_use]
    pub const fn process_id(&self) -> u32 {
        self.process_id
    }

    #[must_use]
    pub const fn acquired_at(&self) -> DateTime<Utc> {
        self.acquired_at
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkspaceAuthorityPathIssue {
    RootMustBeAbsolute,
    RootIsNotDirectory,
    ControlDirectoryMissing,
    ControlDirectoryIsSymlink,
    ControlDirectoryIsNotDirectory,
    ControlDirectoryWasReplaced,
    LockFileIsSymlink,
    LockPathIsNotFile,
    LockFileWasReplaced,
    IdentityRegistryRootMustBeAbsolute,
    IdentityRegistryRootIsSymlink,
    IdentityRegistryRootIsNotDirectory,
    IdentityRegistryRootWasReplaced,
    IdentityRegistryPermissionsUnsafe,
    IdentityRegistryOwnerMismatch,
    IdentityLockFileIsSymlink,
    IdentityLockPathIsNotFile,
    IdentityLockFileWasReplaced,
    IdentityLockFileHasUnexpectedData,
    IdentityLockPermissionsUnsafe,
    IdentityLockOwnerMismatch,
    IdentityLockLinkCountUnsafe,
}

impl fmt::Display for WorkspaceAuthorityPathIssue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::RootMustBeAbsolute => "the workspace root must be absolute",
            Self::RootIsNotDirectory => "the workspace root is not a directory",
            Self::ControlDirectoryMissing => "the workspace control directory is missing",
            Self::ControlDirectoryIsSymlink => {
                "the workspace control directory must not be a symbolic link"
            }
            Self::ControlDirectoryIsNotDirectory => "the workspace control path is not a directory",
            Self::ControlDirectoryWasReplaced => {
                "the workspace control directory changed after it was bound"
            }
            Self::LockFileIsSymlink => "the writer-lock path must not be a symbolic link",
            Self::LockPathIsNotFile => "the writer-lock path is not a regular file",
            Self::LockFileWasReplaced => "the writer-lock file changed while it was being opened",
            Self::IdentityRegistryRootMustBeAbsolute => {
                "the per-user writer-authority registry must be absolute"
            }
            Self::IdentityRegistryRootIsSymlink => {
                "the per-user writer-authority registry must not be a symbolic link or reparse point"
            }
            Self::IdentityRegistryRootIsNotDirectory => {
                "the per-user writer-authority registry is not a directory"
            }
            Self::IdentityRegistryRootWasReplaced => {
                "the per-user writer-authority registry changed after it was bound"
            }
            Self::IdentityRegistryPermissionsUnsafe => {
                "the per-user writer-authority registry permissions are unsafe"
            }
            Self::IdentityRegistryOwnerMismatch => {
                "the per-user writer-authority registry has a different owner"
            }
            Self::IdentityLockFileIsSymlink => {
                "the identity writer-lock path must not be a symbolic link or reparse point"
            }
            Self::IdentityLockPathIsNotFile => {
                "the identity writer-lock path is not a regular file"
            }
            Self::IdentityLockFileWasReplaced => {
                "the identity writer-lock file changed while it was being opened"
            }
            Self::IdentityLockFileHasUnexpectedData => {
                "the identity writer-lock file contains unexpected data"
            }
            Self::IdentityLockPermissionsUnsafe => {
                "the identity writer-lock file permissions are unsafe"
            }
            Self::IdentityLockOwnerMismatch => {
                "the identity writer-lock file has a different owner"
            }
            Self::IdentityLockLinkCountUnsafe => {
                "the identity writer-lock file has an unsafe link count"
            }
        })
    }
}

impl WorkspaceAuthorityPathIssue {
    #[must_use]
    pub const fn is_identity_authority(self) -> bool {
        matches!(
            self,
            Self::IdentityRegistryRootMustBeAbsolute
                | Self::IdentityRegistryRootIsSymlink
                | Self::IdentityRegistryRootIsNotDirectory
                | Self::IdentityRegistryRootWasReplaced
                | Self::IdentityRegistryPermissionsUnsafe
                | Self::IdentityRegistryOwnerMismatch
                | Self::IdentityLockFileIsSymlink
                | Self::IdentityLockPathIsNotFile
                | Self::IdentityLockFileWasReplaced
                | Self::IdentityLockFileHasUnexpectedData
                | Self::IdentityLockPermissionsUnsafe
                | Self::IdentityLockOwnerMismatch
                | Self::IdentityLockLinkCountUnsafe
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkspaceAuthorityOperation {
    ResolveRoot,
    InspectControlDirectory,
    OpenLockFile,
    AcquireWriterLock,
    ResolveIdentityRegistry,
    InspectIdentityRegistry,
    OpenIdentityLockFile,
    AcquireIdentityLock,
}

impl fmt::Display for WorkspaceAuthorityOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::ResolveRoot => "resolving the workspace root",
            Self::InspectControlDirectory => "inspecting the workspace control directory",
            Self::OpenLockFile => "opening the workspace writer-lock file",
            Self::AcquireWriterLock => "acquiring the workspace writer lock",
            Self::ResolveIdentityRegistry => "resolving the per-user writer-authority registry",
            Self::InspectIdentityRegistry => "inspecting the per-user writer-authority registry",
            Self::OpenIdentityLockFile => "opening the workspace-identity writer lock",
            Self::AcquireIdentityLock => "acquiring the workspace-identity writer lock",
        })
    }
}

impl WorkspaceAuthorityOperation {
    #[must_use]
    pub const fn is_identity_authority(self) -> bool {
        matches!(
            self,
            Self::ResolveIdentityRegistry
                | Self::InspectIdentityRegistry
                | Self::OpenIdentityLockFile
                | Self::AcquireIdentityLock
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkspaceAuthorityFailureKind {
    NotFound,
    PermissionDenied,
    ReadOnlyFilesystem,
    ResourceBusy,
    ResourceExhausted,
    Unsupported,
    InvalidData,
    Unexpected,
}

impl fmt::Display for WorkspaceAuthorityFailureKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::NotFound => "not found",
            Self::PermissionDenied => "permission denied",
            Self::ReadOnlyFilesystem => "read-only filesystem",
            Self::ResourceBusy => "resource busy",
            Self::ResourceExhausted => "resource exhausted",
            Self::Unsupported => "unsupported filesystem operation",
            Self::InvalidData => "invalid filesystem data",
            Self::Unexpected => "unexpected filesystem failure",
        })
    }
}

#[derive(Debug, Error)]
pub enum WorkspaceSessionError {
    #[error(transparent)]
    Workspace(#[from] WorkspaceError),
    #[error("another workspace writer is active")]
    WriterAlreadyActive {
        /// Untrusted diagnostics observed only after the OS rejected the lock.
        observed_diagnostic: Option<WriterDiagnostic>,
    },
    #[error("another Liaison writer is active for this workspace identity")]
    IdentityWriterAlreadyActive,
    #[error("unsafe workspace authority path: {issue}")]
    UnsafeAuthorityPath { issue: WorkspaceAuthorityPathIssue },
    #[error("workspace writer authority failed while {operation}: {failure}")]
    AuthorityUnavailable {
        operation: WorkspaceAuthorityOperation,
        failure: WorkspaceAuthorityFailureKind,
        #[source]
        source: Box<dyn StdError + Send + Sync>,
    },
    #[error("workspace session is quiescing")]
    Quiescing,
    #[error("workspace session is closed")]
    Closed,
    #[error("workspace session state is unavailable")]
    StateUnavailable,
}

/// The exclusive OS authority held for this value's lifetime. Implementations
/// release authority on drop; process termination is released by OS handle
/// cleanup, never PID age or sidecar deletion.
pub trait WorkspaceWriterAuthority: fmt::Debug + Send + Sync {
    fn diagnostic(&self) -> &WriterDiagnostic;
    fn diagnostic_published(&self) -> bool;
    fn verify_authority(&self) -> Result<(), WorkspaceSessionError>;
}

/// Adapter-bound writer authority. No raw path crosses this port.
pub trait WorkspaceWriterAuthorityPort: fmt::Debug + Send + Sync {
    fn acquire_writer(
        &self,
        workspace_id: WorkspaceId,
        diagnostic: WriterDiagnostic,
    ) -> Result<Box<dyn WorkspaceWriterAuthority>, WorkspaceSessionError>;
}

/// Manifest and Health access bound to one adapter-owned workspace root.
pub trait BoundWorkspaceStore: fmt::Debug + Send + Sync {
    fn load_manifest(&self) -> Result<WorkspaceManifest, WorkspaceError>;
    fn validate_layout(&self) -> Result<Vec<ValidationFinding>, WorkspaceError>;
}

/// One composition-time binding for writer authority and all session-bound
/// repositories. Consuming the binding prevents application code from pairing
/// a live authority with a later arbitrary path.
pub trait BoundWorkspaceSessionPort:
    BoundWorkspaceStore + WorkspaceWriterAuthorityPort + Sized
{
    type Repositories: BoundWorkspaceStore;

    fn into_repositories(self) -> Self::Repositories;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkspaceRecoveryState {
    UnavailableUntilRecoverableOperations,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkspaceKeyState {
    UnavailableUntilWorkspaceSecurity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkspaceProjectionState {
    UnavailableUntilDirectoryProjection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SessionPhase {
    Open,
    Quiescing,
    Closed,
}

#[derive(Debug)]
struct SessionLifecycle {
    phase: SessionPhase,
    in_flight: usize,
}

/// Arc-owned, non-cloneable Workspace capability aggregate.
///
/// The aggregate owns one OS writer authority, identity/schema, path-free
/// repositories, explicit not-yet-available capability states, and a
/// quiescence barrier. Callers may share it with `Arc`; they cannot clone the
/// authority-bearing aggregate itself.
pub struct WorkspaceSession<R>
where
    R: BoundWorkspaceStore,
{
    manifest: WorkspaceManifest,
    repositories: R,
    writer_authority: Mutex<Option<Box<dyn WorkspaceWriterAuthority>>>,
    lifecycle: Mutex<SessionLifecycle>,
    lifecycle_changed: Condvar,
    recovery_state: WorkspaceRecoveryState,
    key_state: WorkspaceKeyState,
    projection_state: WorkspaceProjectionState,
}

impl<R> fmt::Debug for WorkspaceSession<R>
where
    R: BoundWorkspaceStore,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkspaceSession")
            .field("manifest", &self.manifest)
            .field("repositories", &self.repositories)
            .field("recovery_state", &self.recovery_state)
            .field("key_state", &self.key_state)
            .field("projection_state", &self.projection_state)
            .finish_non_exhaustive()
    }
}

impl<R> WorkspaceSession<R>
where
    R: BoundWorkspaceStore,
{
    #[must_use]
    pub const fn workspace_id(&self) -> WorkspaceId {
        self.manifest.workspace_id
    }

    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.manifest.schema_version
    }

    #[must_use]
    pub const fn manifest(&self) -> &WorkspaceManifest {
        &self.manifest
    }

    #[must_use]
    pub const fn recovery_state(&self) -> WorkspaceRecoveryState {
        self.recovery_state
    }

    #[must_use]
    pub const fn key_state(&self) -> WorkspaceKeyState {
        self.key_state
    }

    #[must_use]
    pub const fn projection_state(&self) -> WorkspaceProjectionState {
        self.projection_state
    }

    pub fn session_id(&self) -> Result<WorkspaceSessionId, WorkspaceSessionError> {
        let authority = self
            .writer_authority
            .lock()
            .map_err(|_| WorkspaceSessionError::StateUnavailable)?;
        authority
            .as_ref()
            .map(|authority| authority.diagnostic().session_id())
            .ok_or(WorkspaceSessionError::Closed)
    }

    /// Starts one unit of session work. Once close begins, new work is
    /// rejected while previously issued guards are allowed to drain.
    pub fn begin_work(&self) -> Result<WorkspaceWorkGuard<'_, R>, WorkspaceSessionError> {
        let mut lifecycle = self
            .lifecycle
            .lock()
            .map_err(|_| WorkspaceSessionError::StateUnavailable)?;
        match lifecycle.phase {
            SessionPhase::Open => {
                {
                    let authority = self
                        .writer_authority
                        .lock()
                        .map_err(|_| WorkspaceSessionError::StateUnavailable)?;
                    authority
                        .as_ref()
                        .ok_or(WorkspaceSessionError::Closed)?
                        .verify_authority()?;
                }
                lifecycle.in_flight = lifecycle
                    .in_flight
                    .checked_add(1)
                    .ok_or(WorkspaceSessionError::StateUnavailable)?;
                Ok(WorkspaceWorkGuard {
                    session: self,
                    active: true,
                })
            }
            SessionPhase::Quiescing => Err(WorkspaceSessionError::Quiescing),
            SessionPhase::Closed => Err(WorkspaceSessionError::Closed),
        }
    }

    /// Rejects new work, drains issued work guards, and then releases the OS
    /// writer authority. Concurrent or repeated close calls are idempotent.
    pub fn close(&self) -> Result<(), WorkspaceSessionError> {
        let mut lifecycle = self
            .lifecycle
            .lock()
            .map_err(|_| WorkspaceSessionError::StateUnavailable)?;
        if lifecycle.phase == SessionPhase::Closed {
            return Ok(());
        }
        if lifecycle.phase == SessionPhase::Open {
            lifecycle.phase = SessionPhase::Quiescing;
            self.lifecycle_changed.notify_all();
        }
        while lifecycle.in_flight != 0 {
            lifecycle = self
                .lifecycle_changed
                .wait(lifecycle)
                .map_err(|_| WorkspaceSessionError::StateUnavailable)?;
        }
        lifecycle.phase = SessionPhase::Closed;
        self.lifecycle_changed.notify_all();
        drop(lifecycle);

        let authority = match self.writer_authority.lock() {
            Ok(mut authority) => authority.take(),
            Err(poisoned) => poisoned.into_inner().take(),
        };
        drop(authority);
        Ok(())
    }
}

impl<R> Drop for WorkspaceSession<R>
where
    R: BoundWorkspaceStore,
{
    fn drop(&mut self) {
        let authority = match self.writer_authority.get_mut() {
            Ok(authority) => authority.take(),
            Err(poisoned) => poisoned.into_inner().take(),
        };
        drop(authority);
    }
}

#[derive(Debug)]
pub struct WorkspaceWorkGuard<'session, R>
where
    R: BoundWorkspaceStore,
{
    session: &'session WorkspaceSession<R>,
    active: bool,
}

impl<R> WorkspaceWorkGuard<'_, R>
where
    R: BoundWorkspaceStore,
{
    /// The only public route to session-bound repositories. Holding this
    /// borrowed guard proves the session is open and prevents close from
    /// releasing writer authority until the operation finishes.
    #[must_use]
    pub const fn repositories(&self) -> &R {
        &self.session.repositories
    }

    pub fn verify_identity(&self) -> Result<(), WorkspaceError> {
        let manifest = self.session.repositories.load_manifest()?;
        manifest.validate()?;
        if manifest.workspace_id != self.session.manifest.workspace_id {
            return Err(WorkspaceError::SessionIdentityChanged {
                expected: self.session.manifest.workspace_id,
                found: manifest.workspace_id,
            });
        }
        if manifest.schema_version != self.session.manifest.schema_version {
            return Err(WorkspaceError::SessionSchemaChanged {
                expected: self.session.manifest.schema_version,
                found: manifest.schema_version,
            });
        }
        Ok(())
    }

    pub fn validate_current_layout(&self) -> Result<WorkspaceValidationReport, WorkspaceError> {
        let manifest = self.session.repositories.load_manifest()?;
        manifest.validate()?;
        if manifest.workspace_id != self.session.manifest.workspace_id {
            return Err(WorkspaceError::SessionIdentityChanged {
                expected: self.session.manifest.workspace_id,
                found: manifest.workspace_id,
            });
        }
        if manifest.schema_version != self.session.manifest.schema_version {
            return Err(WorkspaceError::SessionSchemaChanged {
                expected: self.session.manifest.schema_version,
                found: manifest.schema_version,
            });
        }
        Ok(WorkspaceValidationReport {
            workspace_id: manifest.workspace_id,
            schema_version: manifest.schema_version,
            findings: self.session.repositories.validate_layout()?,
        })
    }
}

impl<R> Drop for WorkspaceWorkGuard<'_, R>
where
    R: BoundWorkspaceStore,
{
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let mut lifecycle = match self.session.lifecycle.lock() {
            Ok(lifecycle) => lifecycle,
            Err(poisoned) => poisoned.into_inner(),
        };
        if lifecycle.in_flight > 0 {
            lifecycle.in_flight -= 1;
        }
        self.active = false;
        if lifecycle.in_flight == 0 {
            self.session.lifecycle_changed.notify_all();
        }
    }
}

#[derive(Debug)]
pub struct OpenWorkspaceSession<B> {
    binding: B,
}

impl<B> OpenWorkspaceSession<B>
where
    B: BoundWorkspaceSessionPort,
{
    #[must_use]
    pub const fn new(binding: B) -> Self {
        Self { binding }
    }

    pub fn execute(
        self,
        diagnostic: WriterDiagnostic,
    ) -> Result<WorkspaceSession<B::Repositories>, WorkspaceSessionError> {
        let manifest = self.binding.load_manifest()?;
        manifest.validate()?;
        let writer_authority = self
            .binding
            .acquire_writer(manifest.workspace_id, diagnostic)?;
        let current_manifest = self.binding.load_manifest()?;
        current_manifest.validate()?;
        if current_manifest.workspace_id != manifest.workspace_id {
            return Err(WorkspaceError::SessionIdentityChanged {
                expected: manifest.workspace_id,
                found: current_manifest.workspace_id,
            }
            .into());
        }
        if current_manifest.schema_version != manifest.schema_version {
            return Err(WorkspaceError::SessionSchemaChanged {
                expected: manifest.schema_version,
                found: current_manifest.schema_version,
            }
            .into());
        }
        let repositories = self.binding.into_repositories();
        Ok(WorkspaceSession {
            manifest: current_manifest,
            repositories,
            writer_authority: Mutex::new(Some(writer_authority)),
            lifecycle: Mutex::new(SessionLifecycle {
                phase: SessionPhase::Open,
                in_flight: 0,
            }),
            lifecycle_changed: Condvar::new(),
            recovery_state: WorkspaceRecoveryState::UnavailableUntilRecoverableOperations,
            key_state: WorkspaceKeyState::UnavailableUntilWorkspaceSecurity,
            projection_state: WorkspaceProjectionState::UnavailableUntilDirectoryProjection,
        })
    }
}

pub trait WorkspaceStore: Send + Sync {
    fn initialise(&self, root: &Path, manifest: &WorkspaceManifest) -> Result<(), WorkspaceError>;

    fn load(&self, root: &Path) -> Result<WorkspaceManifest, WorkspaceError>;

    fn validate_layout(&self, root: &Path) -> Result<Vec<ValidationFinding>, WorkspaceError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidationFinding {
    pub code: String,
    pub severity: FindingSeverity,
    pub path: String,
    pub message: String,
    pub recovery: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FindingSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug)]
pub struct InitialiseWorkspace<S> {
    store: S,
}

impl<S> InitialiseWorkspace<S>
where
    S: WorkspaceStore,
{
    #[must_use]
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub fn execute(
        &self,
        root: &Path,
        workspace_id: WorkspaceId,
        name: impl Into<String>,
        profile: WorkspaceProfile,
        build_profile: BuildProfile,
        locale: impl Into<String>,
    ) -> Result<WorkspaceManifest, WorkspaceError> {
        let manifest = WorkspaceManifest::new(workspace_id, name, profile, build_profile, locale)?;
        self.store.initialise(root, &manifest)?;
        Ok(manifest)
    }
}

#[derive(Debug)]
pub struct ValidateWorkspace<S> {
    store: S,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceValidationReport {
    pub workspace_id: WorkspaceId,
    pub schema_version: u32,
    pub findings: Vec<ValidationFinding>,
}

impl WorkspaceValidationReport {
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Error)
    }
}

impl<S> ValidateWorkspace<S>
where
    S: WorkspaceStore,
{
    #[must_use]
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub fn execute(&self, root: &Path) -> Result<WorkspaceValidationReport, WorkspaceError> {
        let manifest = self.store.load(root)?;
        manifest.validate()?;
        let findings = self.store.validate_layout(root)?;
        Ok(WorkspaceValidationReport {
            workspace_id: manifest.workspace_id,
            schema_version: manifest.schema_version,
            findings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BoundWorkspaceSessionPort, BoundWorkspaceStore, BuildProfile, OpenWorkspaceSession,
        ValidationFinding, WorkspaceError, WorkspaceKeyState, WorkspaceManifest, WorkspaceProfile,
        WorkspaceProjectionState, WorkspaceRecoveryState, WorkspaceSessionError,
        WorkspaceWriterAuthority, WorkspaceWriterAuthorityPort, WriterDiagnostic,
    };
    use chrono::{DateTime, Utc};
    use liaison_shared_kernel::{WorkspaceId, WorkspaceSessionId};
    use std::{
        collections::VecDeque,
        sync::{
            Arc, Mutex,
            atomic::{AtomicBool, Ordering},
            mpsc,
        },
        thread,
        time::Duration,
    };
    use uuid::Uuid;

    #[derive(Debug)]
    struct MemoryRepositories {
        manifest: WorkspaceManifest,
    }

    impl BoundWorkspaceStore for MemoryRepositories {
        fn load_manifest(&self) -> Result<WorkspaceManifest, WorkspaceError> {
            Ok(self.manifest.clone())
        }

        fn validate_layout(&self) -> Result<Vec<ValidationFinding>, WorkspaceError> {
            Ok(Vec::new())
        }
    }

    #[derive(Debug)]
    struct TrackingBinding {
        repositories: MemoryRepositories,
        authority_acquired: Arc<AtomicBool>,
        authority_dropped: Arc<AtomicBool>,
    }

    impl BoundWorkspaceStore for TrackingBinding {
        fn load_manifest(&self) -> Result<WorkspaceManifest, WorkspaceError> {
            self.repositories.load_manifest()
        }

        fn validate_layout(&self) -> Result<Vec<ValidationFinding>, WorkspaceError> {
            self.repositories.validate_layout()
        }
    }

    impl WorkspaceWriterAuthorityPort for TrackingBinding {
        fn acquire_writer(
            &self,
            _workspace_id: WorkspaceId,
            diagnostic: WriterDiagnostic,
        ) -> Result<Box<dyn WorkspaceWriterAuthority>, WorkspaceSessionError> {
            self.authority_acquired.store(true, Ordering::SeqCst);
            Ok(Box::new(TrackingAuthority {
                diagnostic,
                dropped: Arc::clone(&self.authority_dropped),
            }))
        }
    }

    impl BoundWorkspaceSessionPort for TrackingBinding {
        type Repositories = MemoryRepositories;

        fn into_repositories(self) -> Self::Repositories {
            self.repositories
        }
    }

    #[derive(Debug)]
    struct TrackingAuthority {
        diagnostic: WriterDiagnostic,
        dropped: Arc<AtomicBool>,
    }

    impl WorkspaceWriterAuthority for TrackingAuthority {
        fn diagnostic(&self) -> &WriterDiagnostic {
            &self.diagnostic
        }

        fn diagnostic_published(&self) -> bool {
            true
        }

        fn verify_authority(&self) -> Result<(), WorkspaceSessionError> {
            Ok(())
        }
    }

    impl Drop for TrackingAuthority {
        fn drop(&mut self) {
            self.dropped.store(true, Ordering::SeqCst);
        }
    }

    fn manifest() -> WorkspaceManifest {
        WorkspaceManifest::new(
            WorkspaceId::from_uuid(Uuid::from_u128(1)),
            "People",
            WorkspaceProfile::Personal,
            BuildProfile::Airgap,
            "en-IE",
        )
        .unwrap_or_else(|error| unreachable!("test manifest must be valid: {error}"))
    }

    fn diagnostic() -> WriterDiagnostic {
        WriterDiagnostic::new(
            WorkspaceSessionId::from_uuid(Uuid::from_u128(2)),
            42,
            DateTime::<Utc>::UNIX_EPOCH,
        )
    }

    fn tracking_session(
        manifest: WorkspaceManifest,
        acquired: &Arc<AtomicBool>,
        dropped: &Arc<AtomicBool>,
    ) -> Result<super::WorkspaceSession<MemoryRepositories>, WorkspaceSessionError> {
        OpenWorkspaceSession::new(TrackingBinding {
            repositories: MemoryRepositories { manifest },
            authority_acquired: Arc::clone(acquired),
            authority_dropped: Arc::clone(dropped),
        })
        .execute(diagnostic())
    }

    #[test]
    fn manifest_rejects_blank_name() {
        let result = WorkspaceManifest::new(
            WorkspaceId::new(),
            "  ",
            WorkspaceProfile::Personal,
            BuildProfile::Airgap,
            "en-IE",
        );
        assert_eq!(result, Err(WorkspaceError::RequiredField("workspace name")));
    }

    #[test]
    fn manifest_uses_current_format_and_schema() {
        let result = WorkspaceManifest::new(
            WorkspaceId::new(),
            "People",
            WorkspaceProfile::Personal,
            BuildProfile::Airgap,
            "en-IE",
        );
        assert!(result.is_ok());
        let manifest = result.unwrap_or_else(|error| unreachable!("unexpected error: {error}"));
        assert_eq!(manifest.format, "liaison-workspace");
        assert_eq!(manifest.schema_version, 1);
        assert_eq!(manifest.enabled_modules, vec!["people"]);
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn manifest_requires_stable_module_identifiers_and_people() {
        let mut manifest = manifest();
        manifest.enabled_modules = Vec::new();
        assert_eq!(
            manifest.validate(),
            Err(WorkspaceError::RequiredField("enabled modules"))
        );

        manifest.enabled_modules = vec!["people".to_owned(), "people".to_owned()];
        assert_eq!(
            manifest.validate(),
            Err(WorkspaceError::InvalidField("enabled modules"))
        );

        manifest.enabled_modules = vec!["people\nprivate".to_owned()];
        assert_eq!(
            manifest.validate(),
            Err(WorkspaceError::InvalidField("enabled modules"))
        );

        manifest.enabled_modules = vec!["réseau".to_owned()];
        assert_eq!(
            manifest.validate(),
            Err(WorkspaceError::InvalidField("enabled modules"))
        );

        manifest.enabled_modules = vec!["people..private".to_owned()];
        assert_eq!(
            manifest.validate(),
            Err(WorkspaceError::InvalidField("enabled modules"))
        );

        manifest.enabled_modules = vec!["events".to_owned()];
        assert_eq!(
            manifest.validate(),
            Err(WorkspaceError::InvalidField("enabled modules"))
        );
    }

    #[test]
    fn session_owns_authority_repositories_and_honest_unavailable_states() {
        let dropped = Arc::new(AtomicBool::new(false));
        let acquired = Arc::new(AtomicBool::new(false));
        let expected_manifest = manifest();
        let session = tracking_session(expected_manifest.clone(), &acquired, &dropped);
        assert!(session.is_ok());
        let Ok(session) = session else {
            return;
        };
        assert_eq!(session.workspace_id(), expected_manifest.workspace_id);
        assert_eq!(session.schema_version(), expected_manifest.schema_version);
        assert!(matches!(
            session.session_id(),
            Ok(found) if found == diagnostic().session_id()
        ));
        assert_eq!(
            session.recovery_state(),
            WorkspaceRecoveryState::UnavailableUntilRecoverableOperations
        );
        assert_eq!(
            session.key_state(),
            WorkspaceKeyState::UnavailableUntilWorkspaceSecurity
        );
        assert_eq!(
            session.projection_state(),
            WorkspaceProjectionState::UnavailableUntilDirectoryProjection
        );
        {
            let work = session.begin_work();
            assert!(work.is_ok());
            if let Ok(work) = work {
                assert!(work.verify_identity().is_ok());
                assert!(work.validate_current_layout().is_ok());
            }
        }
        assert!(!dropped.load(Ordering::SeqCst));
        assert!(acquired.load(Ordering::SeqCst));
        drop(session);
        assert!(dropped.load(Ordering::SeqCst));
    }

    #[test]
    fn close_rejects_new_work_drains_in_flight_and_releases_authority() {
        let dropped = Arc::new(AtomicBool::new(false));
        let acquired = Arc::new(AtomicBool::new(false));
        let session = tracking_session(manifest(), &acquired, &dropped);
        assert!(session.is_ok());
        let Ok(session) = session else {
            return;
        };
        let session = Arc::new(session);
        let work = session.begin_work();
        assert!(work.is_ok());
        let Ok(work) = work else {
            return;
        };
        let closing_session = Arc::clone(&session);
        let (done_sender, done_receiver) = mpsc::sync_channel(1);
        let closer = thread::spawn(move || {
            let result = closing_session.close();
            let _ = done_sender.send(result);
        });

        let mut observed_quiescence = false;
        for _ in 0..10_000 {
            match session.begin_work() {
                Err(WorkspaceSessionError::Quiescing | WorkspaceSessionError::Closed) => {
                    observed_quiescence = true;
                    break;
                }
                Ok(extra_work) => drop(extra_work),
                Err(_) => break,
            }
            thread::yield_now();
        }
        assert!(observed_quiescence);
        assert!(done_receiver.try_recv().is_err());
        assert!(!dropped.load(Ordering::SeqCst));

        drop(work);
        let close_result = done_receiver.recv_timeout(Duration::from_secs(2));
        assert!(matches!(close_result, Ok(Ok(()))));
        assert!(closer.join().is_ok());
        assert!(dropped.load(Ordering::SeqCst));
        assert!(matches!(
            session.begin_work(),
            Err(WorkspaceSessionError::Closed)
        ));
        assert!(session.close().is_ok());
    }

    #[test]
    fn invalid_schema_is_rejected_before_authority_is_acquired() {
        let dropped = Arc::new(AtomicBool::new(false));
        let acquired = Arc::new(AtomicBool::new(false));
        let mut invalid = manifest();
        invalid.schema_version += 1;
        let result = tracking_session(invalid, &acquired, &dropped);
        assert!(matches!(
            result,
            Err(WorkspaceSessionError::Workspace(
                WorkspaceError::UnsupportedSchema { .. }
            ))
        ));
        assert!(!acquired.load(Ordering::SeqCst));
        assert!(!dropped.load(Ordering::SeqCst));
    }

    #[derive(Debug)]
    struct SequencedBinding {
        manifests: Arc<Mutex<VecDeque<WorkspaceManifest>>>,
        repositories: MemoryRepositories,
        acquired_workspace_id: Arc<Mutex<Option<WorkspaceId>>>,
        authority_dropped: Arc<AtomicBool>,
    }

    impl BoundWorkspaceStore for SequencedBinding {
        fn load_manifest(&self) -> Result<WorkspaceManifest, WorkspaceError> {
            self.manifests
                .lock()
                .map_err(|_| WorkspaceError::Storage("test manifest queue unavailable".into()))?
                .pop_front()
                .ok_or_else(|| WorkspaceError::Storage("test manifest queue exhausted".into()))
        }

        fn validate_layout(&self) -> Result<Vec<ValidationFinding>, WorkspaceError> {
            Ok(Vec::new())
        }
    }

    impl WorkspaceWriterAuthorityPort for SequencedBinding {
        fn acquire_writer(
            &self,
            workspace_id: WorkspaceId,
            diagnostic: WriterDiagnostic,
        ) -> Result<Box<dyn WorkspaceWriterAuthority>, WorkspaceSessionError> {
            *self
                .acquired_workspace_id
                .lock()
                .map_err(|_| WorkspaceSessionError::StateUnavailable)? = Some(workspace_id);
            Ok(Box::new(TrackingAuthority {
                diagnostic,
                dropped: Arc::clone(&self.authority_dropped),
            }))
        }
    }

    impl BoundWorkspaceSessionPort for SequencedBinding {
        type Repositories = MemoryRepositories;

        fn into_repositories(self) -> Self::Repositories {
            self.repositories
        }
    }

    #[test]
    fn manifest_identity_change_after_acquisition_releases_authority() {
        let before = manifest();
        let mut after = before.clone();
        after.workspace_id = WorkspaceId::from_uuid(Uuid::from_u128(3));
        let acquired_workspace_id = Arc::new(Mutex::new(None));
        let authority_dropped = Arc::new(AtomicBool::new(false));
        let result = OpenWorkspaceSession::new(SequencedBinding {
            manifests: Arc::new(Mutex::new(VecDeque::from([before.clone(), after.clone()]))),
            repositories: MemoryRepositories {
                manifest: after.clone(),
            },
            acquired_workspace_id: Arc::clone(&acquired_workspace_id),
            authority_dropped: Arc::clone(&authority_dropped),
        })
        .execute(diagnostic());

        assert!(matches!(
            result,
            Err(WorkspaceSessionError::Workspace(
                WorkspaceError::SessionIdentityChanged { expected, found }
            )) if expected == before.workspace_id && found == after.workspace_id
        ));
        assert_eq!(
            acquired_workspace_id.lock().ok().and_then(|value| *value),
            Some(before.workspace_id)
        );
        assert!(authority_dropped.load(Ordering::SeqCst));
    }
}
