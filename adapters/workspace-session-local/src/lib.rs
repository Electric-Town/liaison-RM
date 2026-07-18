//! Local operating-system writer authority for Workspace Sessions.
//!
//! The live file lock is authority. The JSON sidecar is diagnostics only and
//! is ignored when deciding whether a writer may proceed.

#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use cap_fs_ext::{DirExt, FollowSymlinks, OpenOptionsFollowExt};
use cap_std::{
    ambient_authority,
    fs::{Dir, OpenOptions},
};
use liaison_workspace::{
    WorkspaceAuthorityFailureKind, WorkspaceAuthorityOperation, WorkspaceAuthorityPathIssue,
    WorkspaceSessionError, WorkspaceWriterAuthority, WorkspaceWriterAuthorityPort,
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
}

impl LocalWorkspaceSessionAuthority {
    #[cfg(any(unix, windows))]
    pub fn bind(root: &Path) -> Result<Self, WorkspaceSessionError> {
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
        Ok(Self {
            root_directory,
            control_directory,
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
        let diagnostic_published = publish_diagnostic(&self.control_directory, &diagnostic);
        Ok(Box::new(LocalWriterAuthority {
            root_directory: retained_root_directory,
            control_directory: retained_control_directory,
            lock_file,
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
        verify_locked_file(&self.control_directory, &self.lock_file)
    }
}

impl Drop for LocalWriterAuthority {
    fn drop(&mut self) {
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
    };
    use chrono::{DateTime, Utc};
    use liaison_shared_kernel::WorkspaceSessionId;
    use liaison_workspace::{
        WorkspaceAuthorityPathIssue, WorkspaceSessionError, WorkspaceWriterAuthorityPort,
        WriterDiagnostic,
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
    const CHILD_READY: &str = "LIAISON_CHILD_WRITER_LOCKED";

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

    fn lock_path(root: &Path) -> PathBuf {
        root.join(".liaison").join(LOCK_FILE_NAME)
    }

    fn diagnostic_path(root: &Path) -> PathBuf {
        root.join(".liaison").join(DIAGNOSTIC_FILE_NAME)
    }

    #[test]
    fn independent_handles_exclude_until_authority_drop() -> Result<(), Box<dyn Error>> {
        let workspace = workspace()?;
        let first_adapter = LocalWorkspaceSessionAuthority::bind(workspace.path())?;
        let second_adapter = LocalWorkspaceSessionAuthority::bind(workspace.path())?;
        let first_diagnostic = diagnostic(1);
        let first = first_adapter.acquire_writer(first_diagnostic.clone())?;

        assert!(matches!(
            second_adapter.acquire_writer(diagnostic(2)),
            Err(WorkspaceSessionError::WriterAlreadyActive {
                observed_diagnostic: Some(found)
            }) if found == first_diagnostic
        ));

        drop(first);
        let replacement = second_adapter.acquire_writer(diagnostic(3))?;
        assert_eq!(
            replacement.diagnostic().session_id(),
            diagnostic(3).session_id()
        );
        Ok(())
    }

    #[test]
    fn stale_malformed_and_oversized_diagnostics_never_grant_or_steal_authority()
    -> Result<(), Box<dyn Error>> {
        let workspace = workspace()?;
        fs::write(
            diagnostic_path(workspace.path()),
            serde_json::to_vec(&diagnostic(10))?,
        )?;
        let adapter = LocalWorkspaceSessionAuthority::bind(workspace.path())?;
        let held = adapter.acquire_writer(diagnostic(11))?;
        fs::write(diagnostic_path(workspace.path()), b"not json")?;
        assert!(matches!(
            adapter.acquire_writer(diagnostic(12)),
            Err(WorkspaceSessionError::WriterAlreadyActive {
                observed_diagnostic: None
            })
        ));
        fs::write(
            diagnostic_path(workspace.path()),
            vec![b'x'; usize::try_from(DIAGNOSTIC_MAX_BYTES)? + 1],
        )?;
        assert!(matches!(
            adapter.acquire_writer(diagnostic(13)),
            Err(WorkspaceSessionError::WriterAlreadyActive {
                observed_diagnostic: None
            })
        ));
        drop(held);
        assert!(adapter.acquire_writer(diagnostic(14)).is_ok());
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
    fn symlinks_and_replaced_control_directory_fail_closed() -> Result<(), Box<dyn Error>> {
        use std::os::unix::fs::symlink;

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
        let adapter = LocalWorkspaceSessionAuthority::bind(lock_workspace.path())?;
        assert!(matches!(
            adapter.acquire_writer(diagnostic(20)),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::LockFileIsSymlink
            })
        ));
        assert_eq!(fs::read(&outside_lock)?, b"preserve");

        let replaced = workspace()?;
        let adapter = LocalWorkspaceSessionAuthority::bind(replaced.path())?;
        fs::rename(
            replaced.path().join(".liaison"),
            replaced.path().join(".liaison-old"),
        )?;
        fs::create_dir(replaced.path().join(".liaison"))?;
        assert!(matches!(
            adapter.acquire_writer(diagnostic(21)),
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
        let authority = LocalWorkspaceSessionAuthority::bind(workspace.path())?
            .acquire_writer(diagnostic(25))?;
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
        let outside = workspace.path().join("outside-diagnostic");
        fs::write(&outside, b"preserve")?;
        symlink(&outside, diagnostic_path(workspace.path()))?;
        let authority = LocalWorkspaceSessionAuthority::bind(workspace.path())?
            .acquire_writer(diagnostic(22))?;
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
        let alias = aliases.path().join("workspace-alias");
        symlink(workspace.path(), &alias)?;
        let held = LocalWorkspaceSessionAuthority::bind(workspace.path())?
            .acquire_writer(diagnostic(23))?;
        assert!(matches!(
            LocalWorkspaceSessionAuthority::bind(&alias)?.acquire_writer(diagnostic(24)),
            Err(WorkspaceSessionError::WriterAlreadyActive { .. })
        ));
        drop(held);
        Ok(())
    }

    struct ChildGuard(Option<Child>);

    impl ChildGuard {
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

    #[test]
    fn forced_process_exit_releases_operating_system_authority() -> Result<(), Box<dyn Error>> {
        let workspace = workspace()?;
        let child = Command::new(std::env::current_exe()?)
            .args([
                "--exact",
                "tests::child_process_lock_holder",
                "--ignored",
                "--nocapture",
                "--test-threads=1",
            ])
            .env(CHILD_ROOT_ENV, workspace.path())
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
        assert!(receiver.recv_timeout(Duration::from_secs(10))?);
        reader
            .join()
            .map_err(|_| io::Error::other("child reader panicked"))?;

        let adapter = LocalWorkspaceSessionAuthority::bind(workspace.path())?;
        assert!(matches!(
            adapter.acquire_writer(diagnostic(31)),
            Err(WorkspaceSessionError::WriterAlreadyActive { .. })
        ));
        assert!(!child.terminate()?.success());
        assert!(adapter.acquire_writer(diagnostic(32)).is_ok());
        Ok(())
    }

    #[test]
    #[ignore = "spawned explicitly by the process-exit test"]
    fn child_process_lock_holder() -> Result<(), Box<dyn Error>> {
        let Some(root) = std::env::var_os(CHILD_ROOT_ENV) else {
            return Ok(());
        };
        let _authority = LocalWorkspaceSessionAuthority::bind(Path::new(&root))?
            .acquire_writer(diagnostic(30))?;
        println!("{CHILD_READY}");
        io::stdout().flush()?;
        let mut input = [0_u8; 1];
        let _ = io::stdin().read(&mut input)?;
        Ok(())
    }

    #[cfg(windows)]
    #[test]
    fn retained_windows_handles_prevent_delete_or_replacement() -> Result<(), Box<dyn Error>> {
        let workspace = workspace()?;
        let adapter = LocalWorkspaceSessionAuthority::bind(workspace.path())?;
        let authority = adapter.acquire_writer(diagnostic(40))?;
        let lock = lock_path(workspace.path());
        assert!(fs::rename(&lock, lock.with_extension("moved")).is_err());
        let control = workspace.path().join(".liaison");
        assert!(fs::rename(&control, workspace.path().join("moved")).is_err());
        drop(authority);
        Ok(())
    }
}
