use crate::{FindingSeverity, WorkspaceError, WorkspaceManifest, WorkspaceStore};
use liaison_shared_kernel::WorkspaceId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;
use thiserror::Error;

pub const BACKUP_FORMAT: &str = "liaison-workspace-backup";
pub const BACKUP_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupFile {
    pub path: String,
    pub size_bytes: u64,
    pub sha256: String,
}

impl BackupFile {
    pub fn new(
        path: impl Into<String>,
        size_bytes: u64,
        sha256: impl Into<String>,
    ) -> Result<Self, BackupError> {
        let file = Self {
            path: path.into(),
            size_bytes,
            sha256: sha256.into(),
        };
        file.validate()?;
        Ok(file)
    }

    pub fn validate(&self) -> Result<(), BackupError> {
        validate_relative_path(&self.path)?;
        if self.sha256.len() != 64
            || !self
                .sha256
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(BackupError::InvalidDigest(self.path.clone()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupManifest {
    pub format: String,
    pub format_version: u32,
    pub workspace_id: WorkspaceId,
    pub workspace_schema_version: u32,
    pub directories: Vec<String>,
    pub files: Vec<BackupFile>,
}

impl BackupManifest {
    pub fn new(
        workspace: &WorkspaceManifest,
        mut directories: Vec<String>,
        mut files: Vec<BackupFile>,
    ) -> Result<Self, BackupError> {
        directories.sort();
        files.sort_by(|left, right| left.path.cmp(&right.path));
        let manifest = Self {
            format: BACKUP_FORMAT.to_owned(),
            format_version: BACKUP_FORMAT_VERSION,
            workspace_id: workspace.workspace_id,
            workspace_schema_version: workspace.schema_version,
            directories,
            files,
        };
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<(), BackupError> {
        if self.format != BACKUP_FORMAT {
            return Err(BackupError::UnexpectedFormat(self.format.clone()));
        }
        if self.format_version != BACKUP_FORMAT_VERSION {
            return Err(BackupError::UnsupportedFormatVersion {
                found: self.format_version,
                supported: BACKUP_FORMAT_VERSION,
            });
        }
        if self.files.is_empty() {
            return Err(BackupError::EmptySnapshot);
        }

        validate_sorted_unique_paths(&self.directories)?;

        let mut previous: Option<&str> = None;
        let mut file_paths = BTreeSet::new();
        for file in &self.files {
            file.validate()?;
            if !file_paths.insert(file.path.as_str()) {
                return Err(BackupError::DuplicatePath(file.path.clone()));
            }
            if previous.is_some_and(|value| value >= file.path.as_str()) {
                return Err(BackupError::UnsortedManifest);
            }
            previous = Some(file.path.as_str());
        }

        if let Some(path) = self
            .directories
            .iter()
            .find(|path| file_paths.contains(path.as_str()))
        {
            return Err(BackupError::PathKindConflict(path.clone()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BackupVerificationReport {
    pub workspace_id: WorkspaceId,
    pub workspace_schema_version: u32,
    pub directories_checked: usize,
    pub files_checked: usize,
    pub total_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RestoreReport {
    pub workspace_id: WorkspaceId,
    pub workspace_schema_version: u32,
    pub target: String,
    pub directories_restored: usize,
    pub files_restored: usize,
    pub total_bytes: u64,
}

pub trait WorkspaceBackupStore: Send + Sync {
    fn create_snapshot(
        &self,
        workspace: &Path,
        destination: &Path,
        workspace_manifest: &WorkspaceManifest,
    ) -> Result<BackupManifest, BackupError>;

    fn verify_snapshot(&self, snapshot: &Path) -> Result<BackupVerificationReport, BackupError>;

    fn stage_restore(&self, snapshot: &Path, target: &Path) -> Result<RestoreReport, BackupError>;

    fn finalize_restore(&self, target: &Path) -> Result<(), BackupError>;

    fn discard_restore(&self, target: &Path) -> Result<(), BackupError>;
}

#[derive(Debug)]
pub struct CreateWorkspaceBackup<Workspace, Backup> {
    workspace_store: Workspace,
    backup_store: Backup,
}

impl<Workspace, Backup> CreateWorkspaceBackup<Workspace, Backup>
where
    Workspace: WorkspaceStore,
    Backup: WorkspaceBackupStore,
{
    #[must_use]
    pub const fn new(workspace_store: Workspace, backup_store: Backup) -> Self {
        Self {
            workspace_store,
            backup_store,
        }
    }

    pub fn execute(
        &self,
        workspace: &Path,
        destination: &Path,
    ) -> Result<BackupManifest, BackupError> {
        let manifest = self.workspace_store.load(workspace)?;
        manifest.validate()?;
        let findings = self.workspace_store.validate_layout(workspace)?;
        if findings
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Error)
        {
            return Err(BackupError::WorkspaceInvalid(
                "workspace validation contains an error finding".to_owned(),
            ));
        }
        self.backup_store
            .create_snapshot(workspace, destination, &manifest)
    }
}

#[derive(Debug)]
pub struct VerifyWorkspaceBackup<Backup> {
    backup_store: Backup,
}

impl<Backup> VerifyWorkspaceBackup<Backup>
where
    Backup: WorkspaceBackupStore,
{
    #[must_use]
    pub const fn new(backup_store: Backup) -> Self {
        Self { backup_store }
    }

    pub fn execute(&self, snapshot: &Path) -> Result<BackupVerificationReport, BackupError> {
        self.backup_store.verify_snapshot(snapshot)
    }
}

#[derive(Debug)]
pub struct RestoreWorkspaceBackup<Workspace, Backup> {
    workspace_store: Workspace,
    backup_store: Backup,
}

impl<Workspace, Backup> RestoreWorkspaceBackup<Workspace, Backup>
where
    Workspace: WorkspaceStore,
    Backup: WorkspaceBackupStore,
{
    #[must_use]
    pub const fn new(workspace_store: Workspace, backup_store: Backup) -> Self {
        Self {
            workspace_store,
            backup_store,
        }
    }

    pub fn execute(&self, snapshot: &Path, target: &Path) -> Result<RestoreReport, BackupError> {
        let verified = self.backup_store.verify_snapshot(snapshot)?;
        let staged = self.backup_store.stage_restore(snapshot, target)?;

        let validation = (|| -> Result<(), BackupError> {
            let manifest = self.workspace_store.load(target)?;
            manifest.validate()?;
            if manifest.workspace_id != verified.workspace_id {
                return Err(BackupError::WorkspaceIdentityMismatch {
                    expected: verified.workspace_id,
                    found: manifest.workspace_id,
                });
            }
            if manifest.schema_version != verified.workspace_schema_version {
                return Err(BackupError::WorkspaceSchemaMismatch {
                    expected: verified.workspace_schema_version,
                    found: manifest.schema_version,
                });
            }
            let findings = self.workspace_store.validate_layout(target)?;
            if let Some(finding) = findings
                .iter()
                .find(|finding| finding.severity == FindingSeverity::Error)
            {
                return Err(BackupError::WorkspaceInvalid(format!(
                    "{}: {}",
                    finding.path, finding.message
                )));
            }
            Ok(())
        })();

        match validation {
            Ok(()) => {
                self.backup_store.finalize_restore(target)?;
                Ok(staged)
            }
            Err(error) => match self.backup_store.discard_restore(target) {
                Ok(()) => Err(error),
                Err(cleanup_error) => Err(BackupError::CleanupFailed {
                    validation: error.to_string(),
                    cleanup: cleanup_error.to_string(),
                }),
            },
        }
    }
}

fn validate_sorted_unique_paths(paths: &[String]) -> Result<(), BackupError> {
    let mut previous: Option<&str> = None;
    let mut unique = BTreeSet::new();
    for path in paths {
        validate_relative_path(path)?;
        if !unique.insert(path.as_str()) {
            return Err(BackupError::DuplicatePath(path.clone()));
        }
        if previous.is_some_and(|value| value >= path.as_str()) {
            return Err(BackupError::UnsortedManifest);
        }
        previous = Some(path.as_str());
    }
    Ok(())
}

fn validate_relative_path(path: &str) -> Result<(), BackupError> {
    if path.is_empty()
        || path.len() > 2_048
        || path.starts_with('/')
        || path.ends_with('/')
        || path.contains('\\')
        || path.chars().any(char::is_control)
        || path
            .split('/')
            .any(|segment| segment.is_empty() || segment == "." || segment == "..")
    {
        return Err(BackupError::UnsafePath(path.to_owned()));
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum BackupError {
    #[error("unexpected backup format: {0}")]
    UnexpectedFormat(String),
    #[error("backup format version {found} is not supported; this build supports {supported}")]
    UnsupportedFormatVersion { found: u32, supported: u32 },
    #[error("backup snapshot contains no files")]
    EmptySnapshot,
    #[error("backup manifest contains duplicate path: {0}")]
    DuplicatePath(String),
    #[error("backup manifest paths are not strictly sorted")]
    UnsortedManifest,
    #[error("backup path is declared as both file and directory: {0}")]
    PathKindConflict(String),
    #[error("unsafe backup path: {0}")]
    UnsafePath(String),
    #[error("invalid SHA-256 digest for backup path: {0}")]
    InvalidDigest(String),
    #[error("backup destination already exists: {0}")]
    DestinationExists(String),
    #[error("backup destination must be outside the source workspace: {0}")]
    DestinationInsideWorkspace(String),
    #[error("restore target already exists: {0}")]
    RestoreTargetExists(String),
    #[error("restore target must be outside the backup snapshot: {0}")]
    RestoreTargetInsideSnapshot(String),
    #[error("symbolic links are not permitted in backup or restore paths: {0}")]
    SymbolicLink(String),
    #[error("backup manifest is missing: {0}")]
    ManifestMissing(String),
    #[error("backup contains undeclared or missing payload path: {0}")]
    PayloadMismatch(String),
    #[error("checksum mismatch for {path}; expected {expected}, found {found}")]
    ChecksumMismatch {
        path: String,
        expected: String,
        found: String,
    },
    #[error("restored workspace identity mismatch; expected {expected}, found {found}")]
    WorkspaceIdentityMismatch {
        expected: WorkspaceId,
        found: WorkspaceId,
    },
    #[error("restored workspace schema mismatch; expected {expected}, found {found}")]
    WorkspaceSchemaMismatch { expected: u32, found: u32 },
    #[error("workspace is not eligible for backup or restore activation: {0}")]
    WorkspaceInvalid(String),
    #[error("restore cleanup failed after validation error '{validation}': {cleanup}")]
    CleanupFailed { validation: String, cleanup: String },
    #[error("backup storage error: {0}")]
    Storage(String),
    #[error(transparent)]
    Workspace(#[from] WorkspaceError),
}

#[cfg(test)]
mod tests {
    use super::{BACKUP_FORMAT, BackupFile, BackupManifest};
    use crate::{BuildProfile, WorkspaceManifest, WorkspaceProfile};

    #[test]
    fn manifest_sorts_layout_and_rejects_traversal() {
        let workspace = WorkspaceManifest::new(
            "Relationships",
            WorkspaceProfile::Personal,
            BuildProfile::Airgap,
            "en-IE",
        )
        .unwrap_or_else(|error| unreachable!("unexpected workspace error: {error}"));
        let digest = "0".repeat(64);
        let files = vec![
            BackupFile::new("people/b.md", 1, &digest)
                .unwrap_or_else(|error| unreachable!("unexpected backup error: {error}")),
            BackupFile::new(".liaison/workspace.yaml", 2, &digest)
                .unwrap_or_else(|error| unreachable!("unexpected backup error: {error}")),
        ];
        let directories = vec!["people".to_owned(), ".liaison".to_owned()];
        let manifest = BackupManifest::new(&workspace, directories, files)
            .unwrap_or_else(|error| unreachable!("unexpected backup error: {error}"));
        assert_eq!(manifest.format, BACKUP_FORMAT);
        assert_eq!(manifest.directories[0], ".liaison");
        assert_eq!(manifest.files[0].path, ".liaison/workspace.yaml");
        assert!(BackupFile::new("../secret", 1, digest).is_err());
    }
}
