//! Local operating-system writer authority for Workspace Sessions.
//!
//! The live file lock is authority. The JSON sidecar is diagnostics only and
//! is ignored when deciding whether a writer may proceed.

#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

mod identity_registry;

use cap_fs_ext::{DirExt, FollowSymlinks, OpenOptionsFollowExt};
use cap_std::{
    ambient_authority,
    fs::{Dir, OpenOptions},
};
use identity_registry::{IdentityRegistry, IdentityWriterAuthority};
use liaison_workspace::{
    WorkspaceAuthorityFailureKind, WorkspaceAuthorityOperation, WorkspaceAuthorityPathIssue,
    WorkspaceId, WorkspaceSessionError, WorkspaceWriterAuthority, WorkspaceWriterAuthorityPort,
    WriterDiagnostic,
};
use std::{
    fs::{File, TryLockError},
    io::{self, Read, Write},
    path::Path,
};

const CONTROL_DIRECTORY: &str = ".liaison";
const LOCK_FILE_NAME: &str = "workspace-writer.lock";
const DIAGNOSTIC_FILE_NAME: &str = "workspace-writer.json";
const PRIVATE_FILE_MODE: u32 = 0o600;
const DIAGNOSTIC_MAX_BYTES: u64 = 16 * 1024;

#[derive(Debug)]
pub struct LocalWorkspaceSessionAuthority {
    root_directory: Dir,
    control_directory: Dir,
    identity_registry: IdentityRegistry,
}

impl LocalWorkspaceSessionAuthority {
    #[cfg(any(unix, windows))]
    pub fn bind(root: &Path) -> Result<Self, WorkspaceSessionError> {
        let registry_root = identity_registry::default_registry_path()?;
        Self::bind_with_registry(root, &registry_root)
    }

    /// Binds the workspace to an explicit process-shared identity registry.
    ///
    /// Production callers use [`Self::bind`]. This seam supports isolated
    /// host composition and deterministic tests; every cooperating Liaison
    /// process for one user must resolve the same registry root.
    #[cfg(any(unix, windows))]
    pub fn bind_with_registry(
        root: &Path,
        registry_root: &Path,
    ) -> Result<Self, WorkspaceSessionError> {
        if !root.is_absolute() {
            return Err(unsafe_path(WorkspaceAuthorityPathIssue::RootMustBeAbsolute));
        }
        let root_directory = match Dir::open_ambient_dir(root, ambient_authority()) {
            Ok(directory) => directory,
            Err(error) if error.kind() == io::ErrorKind::NotADirectory => {
                return Err(unsafe_path(WorkspaceAuthorityPathIssue::RootIsNotDirectory));
            }
            Err(error) => {
                return Err(authority_unavailable(
                    WorkspaceAuthorityOperation::ResolveRoot,
                    error,
                ));
            }
        };
        if !root_directory
            .dir_metadata()
            .map_err(|error| {
                authority_unavailable(WorkspaceAuthorityOperation::ResolveRoot, error)
            })?
            .is_dir()
        {
            return Err(unsafe_path(WorkspaceAuthorityPathIssue::RootIsNotDirectory));
        }
        let control_directory = open_control_directory(&root_directory)?;
        let identity_registry = IdentityRegistry::bind(registry_root)?;
        Ok(Self {
            root_directory,
            control_directory,
            identity_registry,
        })
    }

    #[cfg(not(any(unix, windows)))]
    pub fn bind(_root: &Path) -> Result<Self, WorkspaceSessionError> {
        Err(authority_unavailable(
            WorkspaceAuthorityOperation::ResolveRoot,
            io::Error::new(
                io::ErrorKind::Unsupported,
                "workspace writer authority is implemented only for Unix and Windows",
            ),
        ))
    }

    fn verify_control_binding(&self) -> Result<(), WorkspaceSessionError> {
        verify_control_binding(&self.root_directory, &self.control_directory)
    }

    /// Clones the already-opened workspace root capability so session-bound
    /// repositories and writer authority derive from the same retained root,
    /// even if its ambient path is later renamed or replaced.
    pub fn try_clone_root_directory(&self) -> Result<Dir, WorkspaceSessionError> {
        self.root_directory
            .try_clone()
            .map_err(|error| authority_unavailable(WorkspaceAuthorityOperation::ResolveRoot, error))
    }
}

impl WorkspaceWriterAuthorityPort for LocalWorkspaceSessionAuthority {
    fn acquire_writer(
        &self,
        workspace_id: WorkspaceId,
        diagnostic: WriterDiagnostic,
    ) -> Result<Box<dyn WorkspaceWriterAuthority>, WorkspaceSessionError> {
        self.verify_control_binding()?;
        let retained_root_directory = self.root_directory.try_clone().map_err(|error| {
            authority_unavailable(WorkspaceAuthorityOperation::ResolveRoot, error)
        })?;
        let retained_control_directory = self.control_directory.try_clone().map_err(|error| {
            authority_unavailable(WorkspaceAuthorityOperation::InspectControlDirectory, error)
        })?;
        let lock_file = open_lock_file(&self.control_directory)?;

        match lock_file.try_lock() {
            Ok(()) => {}
            Err(TryLockError::WouldBlock) => {
                return Err(WorkspaceSessionError::WriterAlreadyActive {
                    observed_diagnostic: read_diagnostic(&self.control_directory),
                });
            }
            Err(TryLockError::Error(error)) => {
                return Err(authority_unavailable(
                    WorkspaceAuthorityOperation::AcquireWriterLock,
                    error,
                ));
            }
        }

        if let Err(error) = verify_locked_file(&self.control_directory, &lock_file) {
            let _ = lock_file.unlock();
            return Err(error);
        }
        if let Err(error) = self.verify_control_binding() {
            let _ = lock_file.unlock();
            return Err(error);
        }
        let identity_authority = match self.identity_registry.acquire(workspace_id) {
            Ok(authority) => authority,
            Err(error) => {
                let _ = lock_file.unlock();
                return Err(error);
            }
        };
        let diagnostic_published = publish_diagnostic(&self.control_directory, &diagnostic);
        Ok(Box::new(LocalWriterAuthority {
            root_directory: retained_root_directory,
            control_directory: retained_control_directory,
            lock_file,
            identity_authority: Some(identity_authority),
            diagnostic,
            diagnostic_published,
        }))
    }
}

#[derive(Debug)]
struct LocalWriterAuthority {
    root_directory: Dir,
    control_directory: Dir,
    lock_file: File,
    identity_authority: Option<IdentityWriterAuthority>,
    diagnostic: WriterDiagnostic,
    diagnostic_published: bool,
}

impl WorkspaceWriterAuthority for LocalWriterAuthority {
    fn diagnostic(&self) -> &WriterDiagnostic {
        &self.diagnostic
    }

    fn diagnostic_published(&self) -> bool {
        self.diagnostic_published
    }

    fn verify_authority(&self) -> Result<(), WorkspaceSessionError> {
        verify_control_binding(&self.root_directory, &self.control_directory)?;
        verify_locked_file(&self.control_directory, &self.lock_file)?;
        self.identity_authority
            .as_ref()
            .ok_or(WorkspaceSessionError::Closed)?
            .verify()
    }
}

impl Drop for LocalWriterAuthority {
    fn drop(&mut self) {
        drop(self.identity_authority.take());
        let _ = self.lock_file.unlock();
    }
}

fn inspect_control_directory(
    root_directory: &Dir,
) -> Result<cap_std::fs::Metadata, WorkspaceSessionError> {
    let metadata = match root_directory.symlink_metadata(CONTROL_DIRECTORY) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Err(unsafe_path(
                WorkspaceAuthorityPathIssue::ControlDirectoryMissing,
            ));
        }
        Err(error) => {
            return Err(authority_unavailable(
                WorkspaceAuthorityOperation::InspectControlDirectory,
                error,
            ));
        }
    };
    if metadata.file_type().is_symlink() {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::ControlDirectoryIsSymlink,
        ));
    }
    if !metadata.is_dir() {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::ControlDirectoryIsNotDirectory,
        ));
    }
    Ok(metadata)
}

fn verify_control_binding(
    root_directory: &Dir,
    control_directory: &Dir,
) -> Result<(), WorkspaceSessionError> {
    let current = inspect_control_directory(root_directory)?;
    let bound = control_directory.dir_metadata().map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::InspectControlDirectory, error)
    })?;
    if !same_bound_directory(&current, &bound) {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::ControlDirectoryWasReplaced,
        ));
    }
    Ok(())
}

fn open_control_directory(root_directory: &Dir) -> Result<Dir, WorkspaceSessionError> {
    inspect_control_directory(root_directory)?;
    root_directory
        .open_dir_nofollow(CONTROL_DIRECTORY)
        .map_err(|error| {
            authority_unavailable(WorkspaceAuthorityOperation::InspectControlDirectory, error)
        })
}

#[cfg(unix)]
fn same_bound_directory(current: &cap_std::fs::Metadata, bound: &cap_std::fs::Metadata) -> bool {
    use cap_std::fs::MetadataExt;
    current.dev() == bound.dev() && current.ino() == bound.ino()
}

#[cfg(windows)]
fn same_bound_directory(_current: &cap_std::fs::Metadata, _bound: &cap_std::fs::Metadata) -> bool {
    // cap-std retains Windows directories without FILE_SHARE_DELETE.
    true
}

#[cfg(not(any(unix, windows)))]
fn same_bound_directory(_current: &cap_std::fs::Metadata, _bound: &cap_std::fs::Metadata) -> bool {
    false
}

fn open_lock_file(control_directory: &Dir) -> Result<File, WorkspaceSessionError> {
    match control_directory.symlink_metadata(LOCK_FILE_NAME) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(unsafe_path(WorkspaceAuthorityPathIssue::LockFileIsSymlink));
        }
        Ok(metadata) if !metadata.is_file() => {
            return Err(unsafe_path(WorkspaceAuthorityPathIssue::LockPathIsNotFile));
        }
        Ok(_) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(authority_unavailable(
                WorkspaceAuthorityOperation::OpenLockFile,
                error,
            ));
        }
    }

    let mut options = private_open_options();
    options.read(true).write(true).create(true);
    #[cfg(windows)]
    {
        use cap_std::fs::OpenOptionsExt;
        // FILE_SHARE_READ | FILE_SHARE_WRITE; deliberately omit DELETE.
        options.share_mode(0x0000_0001 | 0x0000_0002);
    }
    let file = control_directory
        .open_with(LOCK_FILE_NAME, &options)
        .map_err(|error| authority_unavailable(WorkspaceAuthorityOperation::OpenLockFile, error))?;
    if !file
        .metadata()
        .map_err(|error| authority_unavailable(WorkspaceAuthorityOperation::OpenLockFile, error))?
        .is_file()
    {
        return Err(unsafe_path(WorkspaceAuthorityPathIssue::LockPathIsNotFile));
    }
    Ok(file.into_std())
}

fn verify_locked_file(control_directory: &Dir, file: &File) -> Result<(), WorkspaceSessionError> {
    let open_metadata = file
        .metadata()
        .map_err(|error| authority_unavailable(WorkspaceAuthorityOperation::OpenLockFile, error))?;
    let path_metadata = control_directory
        .symlink_metadata(LOCK_FILE_NAME)
        .map_err(|error| authority_unavailable(WorkspaceAuthorityOperation::OpenLockFile, error))?;
    if path_metadata.file_type().is_symlink() {
        return Err(unsafe_path(WorkspaceAuthorityPathIssue::LockFileIsSymlink));
    }
    if !open_metadata.is_file() || !path_metadata.is_file() {
        return Err(unsafe_path(WorkspaceAuthorityPathIssue::LockPathIsNotFile));
    }
    if !same_bound_file(&open_metadata, &path_metadata) {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::LockFileWasReplaced,
        ));
    }
    Ok(())
}

#[cfg(unix)]
fn same_bound_file(open: &std::fs::Metadata, current: &cap_std::fs::Metadata) -> bool {
    use cap_std::fs::MetadataExt as CapMetadataExt;
    use std::os::unix::fs::MetadataExt as StdMetadataExt;
    StdMetadataExt::dev(open) == CapMetadataExt::dev(current)
        && StdMetadataExt::ino(open) == CapMetadataExt::ino(current)
}

#[cfg(windows)]
fn same_bound_file(_open: &std::fs::Metadata, _current: &cap_std::fs::Metadata) -> bool {
    // open_lock_file omits FILE_SHARE_DELETE.
    true
}

#[cfg(not(any(unix, windows)))]
fn same_bound_file(_open: &std::fs::Metadata, _current: &cap_std::fs::Metadata) -> bool {
    false
}

fn private_open_options() -> OpenOptions {
    let mut options = OpenOptions::new();
    options.follow(FollowSymlinks::No);
    #[cfg(unix)]
    {
        use cap_std::fs::OpenOptionsExt;
        options.mode(PRIVATE_FILE_MODE);
    }
    #[cfg(not(unix))]
    let _ = PRIVATE_FILE_MODE;
    options
}

fn publish_diagnostic(control_directory: &Dir, diagnostic: &WriterDiagnostic) -> bool {
    match control_directory.symlink_metadata(DIAGNOSTIC_FILE_NAME) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => return false,
        Ok(_) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(_) => return false,
    }

    let temporary_name = format!(".workspace-writer.{}.json.tmp", diagnostic.session_id());
    let mut options = private_open_options();
    options.write(true).create_new(true);
    let Ok(mut file) = control_directory.open_with(&temporary_name, &options) else {
        return false;
    };
    let succeeded = serde_json::to_writer(&mut file, diagnostic).is_ok()
        && file.write_all(b"\n").is_ok()
        && file.sync_all().is_ok();
    drop(file);
    if !succeeded {
        let _ = control_directory.remove_file(&temporary_name);
        return false;
    }
    if !replace_diagnostic(control_directory, &temporary_name) {
        let _ = control_directory.remove_file(&temporary_name);
        return false;
    }
    true
}

#[cfg(not(windows))]
fn replace_diagnostic(control_directory: &Dir, temporary_name: &str) -> bool {
    control_directory
        .rename(temporary_name, control_directory, DIAGNOSTIC_FILE_NAME)
        .is_ok()
}

#[cfg(windows)]
fn replace_diagnostic(control_directory: &Dir, temporary_name: &str) -> bool {
    match control_directory.rename(temporary_name, control_directory, DIAGNOSTIC_FILE_NAME) {
        Ok(()) => return true,
        Err(error)
            if matches!(
                error.kind(),
                io::ErrorKind::AlreadyExists | io::ErrorKind::PermissionDenied
            ) => {}
        Err(_) => return false,
    }
    match control_directory.symlink_metadata(DIAGNOSTIC_FILE_NAME) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => return false,
        Ok(_) if control_directory.remove_file(DIAGNOSTIC_FILE_NAME).is_err() => return false,
        Ok(_) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(_) => return false,
    }
    control_directory
        .rename(temporary_name, control_directory, DIAGNOSTIC_FILE_NAME)
        .is_ok()
}

fn read_diagnostic(control_directory: &Dir) -> Option<WriterDiagnostic> {
    let mut options = private_open_options();
    options.read(true);
    let file = control_directory
        .open_with(DIAGNOSTIC_FILE_NAME, &options)
        .ok()?;
    let metadata = file.metadata().ok()?;
    if !metadata.is_file() || metadata.len() > DIAGNOSTIC_MAX_BYTES {
        return None;
    }
    let mut bytes = Vec::with_capacity(usize::try_from(metadata.len()).ok()?);
    file.take(DIAGNOSTIC_MAX_BYTES + 1)
        .read_to_end(&mut bytes)
        .ok()?;
    if u64::try_from(bytes.len()).ok()? > DIAGNOSTIC_MAX_BYTES {
        return None;
    }
    serde_json::from_slice(&bytes)
        .ok()
        .filter(WriterDiagnostic::is_current_format)
}

fn unsafe_path(issue: WorkspaceAuthorityPathIssue) -> WorkspaceSessionError {
    WorkspaceSessionError::UnsafeAuthorityPath { issue }
}

fn authority_unavailable(
    operation: WorkspaceAuthorityOperation,
    error: io::Error,
) -> WorkspaceSessionError {
    WorkspaceSessionError::AuthorityUnavailable {
        operation,
        failure: match error.kind() {
            io::ErrorKind::NotFound => WorkspaceAuthorityFailureKind::NotFound,
            io::ErrorKind::PermissionDenied => WorkspaceAuthorityFailureKind::PermissionDenied,
            io::ErrorKind::ReadOnlyFilesystem => WorkspaceAuthorityFailureKind::ReadOnlyFilesystem,
            io::ErrorKind::AlreadyExists
            | io::ErrorKind::WouldBlock
            | io::ErrorKind::ResourceBusy => WorkspaceAuthorityFailureKind::ResourceBusy,
            io::ErrorKind::OutOfMemory
            | io::ErrorKind::StorageFull
            | io::ErrorKind::QuotaExceeded => WorkspaceAuthorityFailureKind::ResourceExhausted,
            io::ErrorKind::Unsupported => WorkspaceAuthorityFailureKind::Unsupported,
            io::ErrorKind::InvalidData
            | io::ErrorKind::InvalidInput
            | io::ErrorKind::NotADirectory => WorkspaceAuthorityFailureKind::InvalidData,
            _ => WorkspaceAuthorityFailureKind::Unexpected,
        },
        source: Box::new(error),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DIAGNOSTIC_FILE_NAME, DIAGNOSTIC_MAX_BYTES, LOCK_FILE_NAME, LocalWorkspaceSessionAuthority,
        identity_registry,
    };
    use chrono::{DateTime, Utc};
    use liaison_shared_kernel::WorkspaceSessionId;
    use liaison_workspace::{
        WorkspaceAuthorityPathIssue, WorkspaceId, WorkspaceSessionError,
        WorkspaceWriterAuthorityPort, WriterDiagnostic,
    };
    use std::{
        error::Error,
        fs,
        io::{self, BufRead, BufReader, Read, Write},
        path::{Path, PathBuf},
        process::{Child, Command, ExitStatus, Stdio},
        sync::mpsc,
        thread,
        time::Duration,
    };
    use tempfile::{TempDir, tempdir};
    use uuid::Uuid;

    const CHILD_ROOT_ENV: &str = "LIAISON_TEST_CHILD_LOCK_ROOT";
    const CHILD_REGISTRY_ENV: &str = "LIAISON_TEST_CHILD_REGISTRY_ROOT";
    const CHILD_WORKSPACE_ID_ENV: &str = "LIAISON_TEST_CHILD_WORKSPACE_ID";
    const CHILD_PRODUCTION_ACTION_ENV: &str = "LIAISON_TEST_CHILD_PRODUCTION_ACTION";
    const CHILD_READY: &str = "LIAISON_CHILD_WRITER_LOCKED";
    const CHILD_PRODUCTION_ACQUIRED: &str = "LIAISON_CHILD_PRODUCTION_ACQUIRED";
    const CHILD_PRODUCTION_CONTENDED: &str = "LIAISON_CHILD_PRODUCTION_CONTENDED";

    fn workspace() -> Result<TempDir, io::Error> {
        let directory = tempdir()?;
        fs::create_dir(directory.path().join(".liaison"))?;
        Ok(directory)
    }

    fn diagnostic(seed: u128) -> WriterDiagnostic {
        WriterDiagnostic::new(
            WorkspaceSessionId::from_uuid(Uuid::from_u128(seed)),
            u32::try_from(seed).unwrap_or(u32::MAX),
            DateTime::<Utc>::UNIX_EPOCH,
        )
    }

    fn workspace_id(seed: u128) -> WorkspaceId {
        WorkspaceId::from_uuid(Uuid::from_u128(seed))
    }

    fn bind(
        root: &Path,
        registry_parent: &TempDir,
    ) -> Result<LocalWorkspaceSessionAuthority, WorkspaceSessionError> {
        LocalWorkspaceSessionAuthority::bind_with_registry(
            root,
            &registry_parent.path().join("writer-authority"),
        )
    }

    fn lock_path(root: &Path) -> PathBuf {
        root.join(".liaison").join(LOCK_FILE_NAME)
    }

    fn diagnostic_path(root: &Path) -> PathBuf {
        root.join(".liaison").join(DIAGNOSTIC_FILE_NAME)
    }

    fn identity_lock_path(registry_parent: &TempDir, identity: WorkspaceId) -> PathBuf {
        registry_parent
            .path()
            .join("writer-authority")
            .join(format!("workspace-{identity}.lock"))
    }

    #[test]
    fn independent_handles_exclude_until_authority_drop() -> Result<(), Box<dyn Error>> {
        let workspace = workspace()?;
        let registry = tempdir()?;
        let first_adapter = bind(workspace.path(), &registry)?;
        let second_adapter = bind(workspace.path(), &registry)?;
        let first_diagnostic = diagnostic(1);
        let first = first_adapter.acquire_writer(workspace_id(100), first_diagnostic.clone())?;

        assert!(matches!(
            second_adapter.acquire_writer(workspace_id(100), diagnostic(2)),
            Err(WorkspaceSessionError::WriterAlreadyActive {
                observed_diagnostic: Some(found)
            }) if found == first_diagnostic
        ));

        drop(first);
        let replacement = second_adapter.acquire_writer(workspace_id(100), diagnostic(3))?;
        assert_eq!(
            replacement.diagnostic().session_id(),
            diagnostic(3).session_id()
        );
        Ok(())
    }

    #[test]
    fn copied_paths_with_one_identity_share_one_writer_authority() -> Result<(), Box<dyn Error>> {
        let source = workspace()?;
        let copy = workspace()?;
        let registry = tempdir()?;
        let identity = workspace_id(101);
        let source_authority =
            bind(source.path(), &registry)?.acquire_writer(identity, diagnostic(4))?;

        assert!(matches!(
            bind(copy.path(), &registry)?.acquire_writer(identity, diagnostic(5)),
            Err(WorkspaceSessionError::IdentityWriterAlreadyActive)
        ));

        drop(source_authority);
        assert!(
            bind(copy.path(), &registry)?
                .acquire_writer(identity, diagnostic(6))
                .is_ok()
        );
        Ok(())
    }

    #[test]
    fn different_workspace_identities_can_write_at_the_same_time() -> Result<(), Box<dyn Error>> {
        let first = workspace()?;
        let second = workspace()?;
        let registry = tempdir()?;
        let first_authority =
            bind(first.path(), &registry)?.acquire_writer(workspace_id(102), diagnostic(7))?;
        let second_authority =
            bind(second.path(), &registry)?.acquire_writer(workspace_id(103), diagnostic(8))?;

        assert_ne!(
            first_authority.diagnostic().session_id(),
            second_authority.diagnostic().session_id()
        );
        Ok(())
    }

    #[test]
    fn stale_empty_identity_entry_does_not_grant_or_deny_authority() -> Result<(), Box<dyn Error>> {
        let workspace = workspace()?;
        let registry = tempdir()?;
        let identity = workspace_id(104);
        let adapter = bind(workspace.path(), &registry)?;
        fs::write(identity_lock_path(&registry, identity), b"")?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(
                identity_lock_path(&registry, identity),
                fs::Permissions::from_mode(0o600),
            )?;
        }

        assert!(adapter.acquire_writer(identity, diagnostic(9)).is_ok());
        Ok(())
    }

    #[test]
    fn hostile_registry_shapes_and_identity_entry_data_fail_closed() -> Result<(), Box<dyn Error>> {
        let workspace = workspace()?;
        let parent = tempdir()?;
        let relative = Path::new("relative-registry");
        assert!(matches!(
            LocalWorkspaceSessionAuthority::bind_with_registry(workspace.path(), relative),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::IdentityRegistryRootMustBeAbsolute
            })
        ));

        let ordinary_file = parent.path().join("ordinary-file");
        fs::write(&ordinary_file, b"preserve")?;
        assert!(matches!(
            LocalWorkspaceSessionAuthority::bind_with_registry(workspace.path(), &ordinary_file),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::IdentityRegistryRootIsNotDirectory
            })
        ));

        let registry = tempdir()?;
        let identity = workspace_id(105);
        let adapter = bind(workspace.path(), &registry)?;
        fs::write(identity_lock_path(&registry, identity), b"unexpected")?;
        assert!(matches!(
            adapter.acquire_writer(identity, diagnostic(10)),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::IdentityLockFileHasUnexpectedData
            })
        ));
        Ok(())
    }

    #[test]
    fn stale_malformed_and_oversized_diagnostics_never_grant_or_steal_authority()
    -> Result<(), Box<dyn Error>> {
        let workspace = workspace()?;
        let registry = tempdir()?;
        fs::write(
            diagnostic_path(workspace.path()),
            serde_json::to_vec(&diagnostic(10))?,
        )?;
        let adapter = bind(workspace.path(), &registry)?;
        let held = adapter.acquire_writer(workspace_id(110), diagnostic(11))?;
        fs::write(diagnostic_path(workspace.path()), b"not json")?;
        assert!(matches!(
            adapter.acquire_writer(workspace_id(110), diagnostic(12)),
            Err(WorkspaceSessionError::WriterAlreadyActive {
                observed_diagnostic: None
            })
        ));
        fs::write(
            diagnostic_path(workspace.path()),
            vec![b'x'; usize::try_from(DIAGNOSTIC_MAX_BYTES)? + 1],
        )?;
        assert!(matches!(
            adapter.acquire_writer(workspace_id(110), diagnostic(13)),
            Err(WorkspaceSessionError::WriterAlreadyActive {
                observed_diagnostic: None
            })
        ));
        drop(held);
        assert!(
            adapter
                .acquire_writer(workspace_id(110), diagnostic(14))
                .is_ok()
        );
        Ok(())
    }

    #[test]
    fn path_shape_failures_are_typed() -> Result<(), Box<dyn Error>> {
        assert!(matches!(
            LocalWorkspaceSessionAuthority::bind(Path::new("relative")),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::RootMustBeAbsolute
            })
        ));
        let ordinary_file_parent = tempdir()?;
        let ordinary_file = ordinary_file_parent.path().join("file");
        fs::write(&ordinary_file, b"file")?;
        assert!(matches!(
            LocalWorkspaceSessionAuthority::bind(&ordinary_file),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::RootIsNotDirectory
            })
        ));
        let no_control = tempdir()?;
        assert!(matches!(
            LocalWorkspaceSessionAuthority::bind(no_control.path()),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::ControlDirectoryMissing
            })
        ));
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn hostile_unix_identity_registry_state_fails_closed() -> Result<(), Box<dyn Error>> {
        use std::os::unix::{fs::PermissionsExt, fs::symlink};

        let workspace = workspace()?;
        let parent = tempdir()?;
        let outside = tempdir()?;
        let symlinked_registry = parent.path().join("symlinked-registry");
        symlink(outside.path(), &symlinked_registry)?;
        assert!(matches!(
            LocalWorkspaceSessionAuthority::bind_with_registry(
                workspace.path(),
                &symlinked_registry
            ),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::IdentityRegistryRootIsSymlink
            })
        ));

        let insecure_registry = parent.path().join("insecure-registry");
        fs::create_dir(&insecure_registry)?;
        fs::set_permissions(&insecure_registry, fs::Permissions::from_mode(0o755))?;
        assert!(matches!(
            LocalWorkspaceSessionAuthority::bind_with_registry(
                workspace.path(),
                &insecure_registry
            ),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::IdentityRegistryPermissionsUnsafe
            })
        ));

        let registry = tempdir()?;
        let identity = workspace_id(106);
        let adapter = bind(workspace.path(), &registry)?;
        let identity_lock = identity_lock_path(&registry, identity);
        let outside_lock = registry.path().join("outside-identity-lock");
        fs::write(&outside_lock, b"preserve")?;
        symlink(&outside_lock, &identity_lock)?;
        assert!(matches!(
            adapter.acquire_writer(identity, diagnostic(15)),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::IdentityLockFileIsSymlink
            })
        ));
        assert_eq!(fs::read(outside_lock)?, b"preserve");

        let permissions_identity = workspace_id(108);
        let permissions_lock = identity_lock_path(&registry, permissions_identity);
        fs::write(&permissions_lock, b"")?;
        fs::set_permissions(&permissions_lock, fs::Permissions::from_mode(0o644))?;
        assert!(matches!(
            adapter.acquire_writer(permissions_identity, diagnostic(17)),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::IdentityLockPermissionsUnsafe
            })
        ));

        let linked_identity = workspace_id(109);
        let linked_lock = identity_lock_path(&registry, linked_identity);
        fs::write(&linked_lock, b"")?;
        fs::set_permissions(&linked_lock, fs::Permissions::from_mode(0o600))?;
        fs::hard_link(&linked_lock, registry.path().join("linked-identity-lock"))?;
        assert!(matches!(
            adapter.acquire_writer(linked_identity, diagnostic(18)),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::IdentityLockLinkCountUnsafe
            })
        ));
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn live_authority_detects_identity_registry_replacement_before_more_work()
    -> Result<(), Box<dyn Error>> {
        use std::os::unix::fs::PermissionsExt;

        let workspace = workspace()?;
        let registry = tempdir()?;
        let identity = workspace_id(107);
        let authority =
            bind(workspace.path(), &registry)?.acquire_writer(identity, diagnostic(16))?;
        let live_registry = registry.path().join("writer-authority");
        fs::rename(
            &live_registry,
            registry.path().join("writer-authority-displaced"),
        )?;
        fs::create_dir(&live_registry)?;
        fs::set_permissions(&live_registry, fs::Permissions::from_mode(0o700))?;

        assert!(matches!(
            authority.verify_authority(),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::IdentityRegistryRootWasReplaced
            })
        ));
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn symlinks_and_replaced_control_directory_fail_closed() -> Result<(), Box<dyn Error>> {
        use std::os::unix::fs::symlink;

        let registry = tempdir()?;
        let symlinked_control = tempdir()?;
        let outside = tempdir()?;
        symlink(outside.path(), symlinked_control.path().join(".liaison"))?;
        assert!(matches!(
            LocalWorkspaceSessionAuthority::bind(symlinked_control.path()),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::ControlDirectoryIsSymlink
            })
        ));

        let lock_workspace = workspace()?;
        let outside_lock = lock_workspace.path().join("outside-lock");
        fs::write(&outside_lock, b"preserve")?;
        symlink(&outside_lock, lock_path(lock_workspace.path()))?;
        let adapter = bind(lock_workspace.path(), &registry)?;
        assert!(matches!(
            adapter.acquire_writer(workspace_id(120), diagnostic(20)),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::LockFileIsSymlink
            })
        ));
        assert_eq!(fs::read(&outside_lock)?, b"preserve");

        let replaced = workspace()?;
        let adapter = bind(replaced.path(), &registry)?;
        fs::rename(
            replaced.path().join(".liaison"),
            replaced.path().join(".liaison-old"),
        )?;
        fs::create_dir(replaced.path().join(".liaison"))?;
        assert!(matches!(
            adapter.acquire_writer(workspace_id(121), diagnostic(21)),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::ControlDirectoryWasReplaced
            })
        ));
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn live_authority_detects_external_lock_inode_replacement_before_more_work()
    -> Result<(), Box<dyn Error>> {
        let workspace = workspace()?;
        let registry = tempdir()?;
        let authority =
            bind(workspace.path(), &registry)?.acquire_writer(workspace_id(125), diagnostic(25))?;
        fs::remove_file(lock_path(workspace.path()))?;
        fs::write(lock_path(workspace.path()), b"replacement")?;

        assert!(matches!(
            authority.verify_authority(),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::LockFileWasReplaced
            })
        ));
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_diagnostic_is_not_followed() -> Result<(), Box<dyn Error>> {
        use std::os::unix::fs::symlink;

        let workspace = workspace()?;
        let registry = tempdir()?;
        let outside = workspace.path().join("outside-diagnostic");
        fs::write(&outside, b"preserve")?;
        symlink(&outside, diagnostic_path(workspace.path()))?;
        let authority =
            bind(workspace.path(), &registry)?.acquire_writer(workspace_id(122), diagnostic(22))?;
        assert!(!authority.diagnostic_published());
        assert_eq!(fs::read(outside)?, b"preserve");
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn symlink_alias_coordinates_with_the_canonical_workspace() -> Result<(), Box<dyn Error>> {
        use std::os::unix::fs::symlink;

        let workspace = workspace()?;
        let aliases = tempdir()?;
        let registry = tempdir()?;
        let alias = aliases.path().join("workspace-alias");
        symlink(workspace.path(), &alias)?;
        let held =
            bind(workspace.path(), &registry)?.acquire_writer(workspace_id(123), diagnostic(23))?;
        assert!(matches!(
            bind(&alias, &registry)?.acquire_writer(workspace_id(123), diagnostic(24)),
            Err(WorkspaceSessionError::WriterAlreadyActive { .. })
        ));
        drop(held);
        Ok(())
    }

    struct ChildGuard(Option<Child>);

    impl ChildGuard {
        fn finish(&mut self) -> Result<ExitStatus, io::Error> {
            let mut child = self
                .0
                .take()
                .ok_or_else(|| io::Error::other("child already reaped"))?;
            drop(child.stdin.take());
            child.wait()
        }

        fn terminate(&mut self) -> Result<ExitStatus, io::Error> {
            let mut child = self
                .0
                .take()
                .ok_or_else(|| io::Error::other("child already reaped"))?;
            child.kill()?;
            child.wait()
        }
    }

    impl Drop for ChildGuard {
        fn drop(&mut self) {
            if let Some(mut child) = self.0.take() {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }

    fn spawn_child_lock_holder(
        root: &Path,
        registry: &Path,
        identity: WorkspaceId,
    ) -> Result<ChildGuard, Box<dyn Error>> {
        let child = Command::new(std::env::current_exe()?)
            .args([
                "--exact",
                "tests::child_process_lock_holder",
                "--ignored",
                "--nocapture",
                "--test-threads=1",
            ])
            .env(CHILD_ROOT_ENV, root)
            .env(CHILD_REGISTRY_ENV, registry)
            .env(CHILD_WORKSPACE_ID_ENV, identity.to_string())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let mut child = ChildGuard(Some(child));
        let stdout = child
            .0
            .as_mut()
            .and_then(|child| child.stdout.take())
            .ok_or_else(|| io::Error::other("child stdout unavailable"))?;
        let (sender, receiver) = mpsc::sync_channel(1);
        let reader = thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line) {
                    Ok(0) | Err(_) => {
                        let _ = sender.send(false);
                        break;
                    }
                    Ok(_) if line.contains(CHILD_READY) => {
                        let _ = sender.send(true);
                        break;
                    }
                    Ok(_) => {}
                }
            }
        });
        if !receiver.recv_timeout(Duration::from_secs(10))? {
            return Err(io::Error::other("child did not acquire authority").into());
        }
        reader
            .join()
            .map_err(|_| io::Error::other("child reader panicked"))?;
        Ok(child)
    }

    #[cfg(any(target_os = "linux", target_os = "macos", windows))]
    fn apply_divergent_directory_environment(command: &mut Command, home: &Path, data_home: &Path) {
        command.env("HOME", home).env("XDG_DATA_HOME", data_home);
        #[cfg(windows)]
        command
            .env("USERPROFILE", home)
            .env("LOCALAPPDATA", data_home);
    }

    #[cfg(any(target_os = "linux", target_os = "macos", windows))]
    fn production_child_command(
        root: &Path,
        identity: WorkspaceId,
        action: &str,
        home: &Path,
        data_home: &Path,
    ) -> Result<Command, Box<dyn Error>> {
        let mut command = Command::new(std::env::current_exe()?);
        command
            .args([
                "--exact",
                "tests::production_child_lock_attempt",
                "--ignored",
                "--nocapture",
                "--test-threads=1",
            ])
            .env(CHILD_ROOT_ENV, root)
            .env(CHILD_WORKSPACE_ID_ENV, identity.to_string())
            .env(CHILD_PRODUCTION_ACTION_ENV, action);
        apply_divergent_directory_environment(&mut command, home, data_home);
        Ok(command)
    }

    #[cfg(any(target_os = "linux", target_os = "macos", windows))]
    fn spawn_production_child_lock_holder(
        root: &Path,
        identity: WorkspaceId,
        home: &Path,
        data_home: &Path,
    ) -> Result<ChildGuard, Box<dyn Error>> {
        let child = production_child_command(root, identity, "hold", home, data_home)?
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let mut child = ChildGuard(Some(child));
        let stdout = child
            .0
            .as_mut()
            .and_then(|child| child.stdout.take())
            .ok_or_else(|| io::Error::other("child stdout unavailable"))?;
        let (sender, receiver) = mpsc::sync_channel(1);
        let reader = thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line) {
                    Ok(0) | Err(_) => {
                        let _ = sender.send(false);
                        break;
                    }
                    Ok(_) if line.contains(CHILD_READY) => {
                        let _ = sender.send(true);
                    }
                    Ok(_) => {}
                }
            }
        });
        if !receiver.recv_timeout(Duration::from_secs(10))? {
            return Err(io::Error::other("production child did not acquire authority").into());
        }
        drop(reader);
        Ok(child)
    }

    #[cfg(any(target_os = "linux", target_os = "macos", windows))]
    fn run_production_child_attempt(
        root: &Path,
        identity: WorkspaceId,
        home: &Path,
        data_home: &Path,
    ) -> Result<std::process::Output, Box<dyn Error>> {
        Ok(production_child_command(root, identity, "attempt", home, data_home)?.output()?)
    }

    #[cfg(any(target_os = "linux", target_os = "macos", windows))]
    fn assert_child_marker(
        output: &std::process::Output,
        marker: &str,
    ) -> Result<(), Box<dyn Error>> {
        if output.status.success() && String::from_utf8_lossy(&output.stdout).contains(marker) {
            Ok(())
        } else {
            Err(io::Error::other(format!(
                "child did not report {marker}: status={:?}, stdout={}, stderr={}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ))
            .into())
        }
    }

    #[cfg(any(target_os = "linux", target_os = "macos", windows))]
    fn exercise_production_registry_environment_independence() -> Result<(), Box<dyn Error>> {
        let first_workspace = workspace()?;
        let copied_workspace = workspace()?;
        let first_home = tempdir()?;
        let first_data_home = tempdir()?;
        let second_home = tempdir()?;
        let second_data_home = tempdir()?;
        let identity = WorkspaceId::new();

        for (
            holder_root,
            holder_home,
            holder_data,
            contender_root,
            contender_home,
            contender_data,
        ) in [
            (
                first_workspace.path(),
                first_home.path(),
                first_data_home.path(),
                copied_workspace.path(),
                second_home.path(),
                second_data_home.path(),
            ),
            (
                copied_workspace.path(),
                second_home.path(),
                second_data_home.path(),
                first_workspace.path(),
                first_home.path(),
                first_data_home.path(),
            ),
        ] {
            let mut holder = spawn_production_child_lock_holder(
                holder_root,
                identity,
                holder_home,
                holder_data,
            )?;
            let contended = run_production_child_attempt(
                contender_root,
                identity,
                contender_home,
                contender_data,
            )?;
            assert_child_marker(&contended, CHILD_PRODUCTION_CONTENDED)?;

            assert!(holder.finish()?.success());
            let released = run_production_child_attempt(
                contender_root,
                identity,
                contender_home,
                contender_data,
            )?;
            assert_child_marker(&released, CHILD_PRODUCTION_ACQUIRED)?;
        }

        let registry_entry =
            identity_registry::default_registry_path()?.join(format!("workspace-{identity}.lock"));
        match fs::remove_file(registry_entry) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_production_registry_ignores_home_and_xdg_in_both_launch_orders()
    -> Result<(), Box<dyn Error>> {
        exercise_production_registry_environment_independence()
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn macos_production_registry_ignores_home_and_xdg_in_both_launch_orders()
    -> Result<(), Box<dyn Error>> {
        exercise_production_registry_environment_independence()
    }

    #[cfg(windows)]
    #[test]
    fn windows_production_registry_ignores_environment_overrides_in_both_launch_orders()
    -> Result<(), Box<dyn Error>> {
        exercise_production_registry_environment_independence()
    }

    #[test]
    fn forced_process_exit_releases_operating_system_authority() -> Result<(), Box<dyn Error>> {
        let workspace = workspace()?;
        let registry_parent = tempdir()?;
        let registry = registry_parent.path().join("writer-authority");
        let identity = workspace_id(130);
        let mut child = spawn_child_lock_holder(workspace.path(), &registry, identity)?;

        let adapter =
            LocalWorkspaceSessionAuthority::bind_with_registry(workspace.path(), &registry)?;
        assert!(matches!(
            adapter.acquire_writer(identity, diagnostic(31)),
            Err(WorkspaceSessionError::WriterAlreadyActive { .. })
        ));
        assert!(!child.terminate()?.success());
        assert!(adapter.acquire_writer(identity, diagnostic(32)).is_ok());
        Ok(())
    }

    #[test]
    fn forced_process_exit_releases_copied_workspace_identity_authority()
    -> Result<(), Box<dyn Error>> {
        let source = workspace()?;
        let copied = workspace()?;
        let registry_parent = tempdir()?;
        let registry = registry_parent.path().join("writer-authority");
        let identity = workspace_id(131);
        let mut child = spawn_child_lock_holder(source.path(), &registry, identity)?;
        let copied_adapter =
            LocalWorkspaceSessionAuthority::bind_with_registry(copied.path(), &registry)?;

        assert!(matches!(
            copied_adapter.acquire_writer(identity, diagnostic(33)),
            Err(WorkspaceSessionError::IdentityWriterAlreadyActive)
        ));
        assert!(!child.terminate()?.success());
        assert!(
            copied_adapter
                .acquire_writer(identity, diagnostic(34))
                .is_ok()
        );
        Ok(())
    }

    #[test]
    #[ignore = "spawned explicitly by the process-exit test"]
    fn child_process_lock_holder() -> Result<(), Box<dyn Error>> {
        let Some(root) = std::env::var_os(CHILD_ROOT_ENV) else {
            return Ok(());
        };
        let registry = std::env::var_os(CHILD_REGISTRY_ENV)
            .ok_or_else(|| io::Error::other("child registry root unavailable"))?;
        let identity = std::env::var(CHILD_WORKSPACE_ID_ENV)?.parse::<WorkspaceId>()?;
        let _authority = LocalWorkspaceSessionAuthority::bind_with_registry(
            Path::new(&root),
            Path::new(&registry),
        )?
        .acquire_writer(identity, diagnostic(30))?;
        println!("{CHILD_READY}");
        io::stdout().flush()?;
        let mut input = [0_u8; 1];
        let _ = io::stdin().read(&mut input)?;
        Ok(())
    }

    #[test]
    #[ignore = "spawned explicitly by the production-registry process tests"]
    fn production_child_lock_attempt() -> Result<(), Box<dyn Error>> {
        let Some(root) = std::env::var_os(CHILD_ROOT_ENV) else {
            return Ok(());
        };
        let identity = std::env::var(CHILD_WORKSPACE_ID_ENV)?.parse::<WorkspaceId>()?;
        let action = std::env::var(CHILD_PRODUCTION_ACTION_ENV)?;
        let adapter = LocalWorkspaceSessionAuthority::bind(Path::new(&root))?;
        match adapter.acquire_writer(identity, diagnostic(35)) {
            Ok(_authority) if action == "hold" => {
                println!("{CHILD_READY}");
                io::stdout().flush()?;
                let mut input = [0_u8; 1];
                let _ = io::stdin().read(&mut input)?;
            }
            Ok(_) if action == "attempt" => println!("{CHILD_PRODUCTION_ACQUIRED}"),
            Err(WorkspaceSessionError::IdentityWriterAlreadyActive) if action == "attempt" => {
                println!("{CHILD_PRODUCTION_CONTENDED}");
            }
            Ok(_) => return Err(io::Error::other("unknown production child action").into()),
            Err(error) => return Err(error.into()),
        }
        Ok(())
    }

    #[cfg(windows)]
    #[test]
    fn retained_windows_handles_prevent_delete_or_replacement() -> Result<(), Box<dyn Error>> {
        let workspace = workspace()?;
        let registry = tempdir()?;
        let adapter = bind(workspace.path(), &registry)?;
        let identity = workspace_id(140);
        let authority = adapter.acquire_writer(identity, diagnostic(40))?;
        let lock = lock_path(workspace.path());
        assert!(fs::rename(&lock, lock.with_extension("moved")).is_err());
        let control = workspace.path().join(".liaison");
        assert!(fs::rename(&control, workspace.path().join("moved")).is_err());
        let registry_root = registry.path().join("writer-authority");
        assert!(
            fs::rename(
                &registry_root,
                registry.path().join("writer-authority-moved")
            )
            .is_err()
        );
        let identity_lock = identity_lock_path(&registry, identity);
        assert!(fs::rename(&identity_lock, identity_lock.with_extension("moved")).is_err());
        drop(authority);
        Ok(())
    }
}
