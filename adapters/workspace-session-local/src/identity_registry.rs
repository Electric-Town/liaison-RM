//! Per-user `WorkspaceId` writer exclusion.
//!
//! The registry is runtime coordination state, not canonical workspace data.
//! Its empty lock files contain no path, Person data, process identifier, or
//! authority metadata. Authority comes only from the live operating-system
//! file lock held by [`IdentityWriterAuthority`].

use cap_fs_ext::{DirExt, FollowSymlinks, OpenOptionsFollowExt};
use cap_std::{
    ambient_authority,
    fs::{Dir, DirBuilder, OpenOptions},
};
use liaison_workspace::{
    WorkspaceAuthorityFailureKind, WorkspaceAuthorityOperation, WorkspaceAuthorityPathIssue,
    WorkspaceId, WorkspaceSessionError,
};
use std::{
    ffi::OsStr,
    fs::{self, File, TryLockError},
    io,
    path::{Path, PathBuf},
};

const PRIVATE_DIRECTORY_MODE: u32 = 0o700;
const PRIVATE_FILE_MODE: u32 = 0o600;
const REGISTRY_DIRECTORY_NAME: &str = "io.github.electric-town.liaison-rm-writer-authority-v1";
#[cfg(windows)]
const WINDOWS_FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;

#[derive(Debug)]
pub(crate) struct IdentityRegistry {
    canonical_parent: PathBuf,
    parent_directory: Dir,
    directory: Dir,
    directory_name: PathBuf,
}

impl IdentityRegistry {
    pub(crate) fn bind(path: &Path) -> Result<Self, WorkspaceSessionError> {
        if !path.is_absolute() {
            return Err(unsafe_path(
                WorkspaceAuthorityPathIssue::IdentityRegistryRootMustBeAbsolute,
            ));
        }
        let parent = path.parent().ok_or_else(|| {
            unsafe_path(WorkspaceAuthorityPathIssue::IdentityRegistryRootMustBeAbsolute)
        })?;
        let directory_name = path.file_name().ok_or_else(|| {
            unsafe_path(WorkspaceAuthorityPathIssue::IdentityRegistryRootMustBeAbsolute)
        })?;
        let parent_metadata = fs::symlink_metadata(parent).map_err(|error| {
            authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
        })?;
        if ambient_file_type_is_link_or_reparse(&parent_metadata) || !parent_metadata.is_dir() {
            return Err(unsafe_path(
                WorkspaceAuthorityPathIssue::IdentityRegistryRootWasReplaced,
            ));
        }
        let canonical_parent = fs::canonicalize(parent).map_err(|error| {
            authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
        })?;
        let parent_directory = Dir::open_ambient_dir(&canonical_parent, ambient_authority())
            .map_err(|error| {
                authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
            })?;
        let registry_created = ensure_registry_directory(&parent_directory, directory_name)?;
        #[cfg(windows)]
        let created_security_handle = registry_created
            .then(|| open_and_harden_created_windows_registry(&parent_directory, directory_name))
            .transpose()?;
        #[cfg(not(windows))]
        let _ = registry_created;
        let directory = parent_directory
            .open_dir_nofollow(directory_name)
            .map_err(|error| {
                authority_unavailable(WorkspaceAuthorityOperation::InspectIdentityRegistry, error)
            })?;
        #[cfg(windows)]
        if let Some(created_security_handle) = created_security_handle {
            let created = cap_std::fs::File::from_std(created_security_handle)
                .metadata()
                .map_err(|error| {
                    authority_unavailable(
                        WorkspaceAuthorityOperation::InspectIdentityRegistry,
                        error,
                    )
                })?;
            let bound = directory.dir_metadata().map_err(|error| {
                authority_unavailable(WorkspaceAuthorityOperation::InspectIdentityRegistry, error)
            })?;
            if !same_bound_directory(&created, &bound) {
                return Err(unsafe_path(
                    WorkspaceAuthorityPathIssue::IdentityRegistryRootWasReplaced,
                ));
            }
            let mut entries = directory.entries().map_err(|error| {
                authority_unavailable(WorkspaceAuthorityOperation::InspectIdentityRegistry, error)
            })?;
            match entries.next() {
                None => {}
                Some(Ok(_)) => {
                    return Err(unsafe_path(
                        WorkspaceAuthorityPathIssue::IdentityRegistryPermissionsUnsafe,
                    ));
                }
                Some(Err(error)) => {
                    return Err(authority_unavailable(
                        WorkspaceAuthorityOperation::InspectIdentityRegistry,
                        error,
                    ));
                }
            }
        }
        let registry = Self {
            canonical_parent,
            parent_directory,
            directory,
            directory_name: PathBuf::from(directory_name),
        };
        registry.verify_binding()?;
        Ok(registry)
    }

    pub(crate) fn acquire(
        &self,
        workspace_id: WorkspaceId,
    ) -> Result<IdentityWriterAuthority, WorkspaceSessionError> {
        self.verify_binding()?;
        let file_name = identity_lock_name(workspace_id);
        let file = open_identity_lock_file(&self.directory, &file_name)?;
        match file.try_lock() {
            Ok(()) => {}
            Err(TryLockError::WouldBlock) => {
                return Err(WorkspaceSessionError::IdentityWriterAlreadyActive);
            }
            Err(TryLockError::Error(error)) => {
                return Err(authority_unavailable(
                    WorkspaceAuthorityOperation::AcquireIdentityLock,
                    error,
                ));
            }
        }
        if let Err(error) = verify_identity_lock_file(&self.directory, &file_name, &file) {
            let _ = file.unlock();
            return Err(error);
        }
        if let Err(error) = self.verify_binding() {
            let _ = file.unlock();
            return Err(error);
        }
        Ok(IdentityWriterAuthority {
            canonical_parent: self.canonical_parent.clone(),
            parent_directory: self.parent_directory.try_clone().map_err(|error| {
                authority_unavailable(WorkspaceAuthorityOperation::InspectIdentityRegistry, error)
            })?,
            directory: self.directory.try_clone().map_err(|error| {
                authority_unavailable(WorkspaceAuthorityOperation::InspectIdentityRegistry, error)
            })?,
            directory_name: self.directory_name.clone(),
            file_name,
            file,
        })
    }

    fn verify_binding(&self) -> Result<(), WorkspaceSessionError> {
        verify_registry_binding(
            &self.canonical_parent,
            &self.parent_directory,
            &self.directory_name,
            &self.directory,
        )
    }
}

#[derive(Debug)]
pub(crate) struct IdentityWriterAuthority {
    canonical_parent: PathBuf,
    parent_directory: Dir,
    directory: Dir,
    directory_name: PathBuf,
    file_name: String,
    file: File,
}

impl IdentityWriterAuthority {
    pub(crate) fn verify(&self) -> Result<(), WorkspaceSessionError> {
        verify_registry_binding(
            &self.canonical_parent,
            &self.parent_directory,
            &self.directory_name,
            &self.directory,
        )?;
        verify_identity_lock_file(&self.directory, &self.file_name, &self.file)
    }
}

impl Drop for IdentityWriterAuthority {
    fn drop(&mut self) {
        let _ = self.file.unlock();
    }
}

pub(crate) fn default_registry_path() -> Result<PathBuf, WorkspaceSessionError> {
    let (base, account_home) = platform_registry_base()?;
    if !base.is_absolute()
        || account_home
            .as_ref()
            .is_some_and(|account_home| !account_home.is_absolute())
    {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityRegistryRootMustBeAbsolute,
        ));
    }
    prepare_default_data_root(&base, account_home.as_deref())?;
    let base = fs::canonicalize(base).map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
    })?;
    Ok(base.join(REGISTRY_DIRECTORY_NAME))
}

#[cfg(target_os = "linux")]
fn platform_registry_base() -> Result<(PathBuf, Option<PathBuf>), WorkspaceSessionError> {
    reject_flatpak_authority_namespace(Path::new("/.flatpak-info"))?;
    let account_home = unix_account_home()?;
    Ok((
        account_home.join(".local").join("share"),
        Some(account_home),
    ))
}

#[cfg(target_os = "macos")]
fn platform_registry_base() -> Result<(PathBuf, Option<PathBuf>), WorkspaceSessionError> {
    let account_home = unix_account_home()?;
    Ok((
        account_home.join("Library").join("Application Support"),
        Some(account_home),
    ))
}

#[cfg(all(unix, not(any(target_os = "linux", target_os = "macos"))))]
fn platform_registry_base() -> Result<(PathBuf, Option<PathBuf>), WorkspaceSessionError> {
    Err(authority_unavailable(
        WorkspaceAuthorityOperation::ResolveIdentityRegistry,
        io::Error::new(
            io::ErrorKind::Unsupported,
            "the canonical per-account writer-authority registry is implemented only for Linux and macOS Unix targets",
        ),
    ))
}

#[cfg(windows)]
fn platform_registry_base() -> Result<(PathBuf, Option<PathBuf>), WorkspaceSessionError> {
    windows_registry_base(Some(windows_current_account_local_app_data()?))
}

#[cfg(windows)]
fn windows_current_account_local_app_data() -> Result<PathBuf, WorkspaceSessionError> {
    use winsafe::{self as w, co};

    // Passing the current process token makes the account explicit. A null
    // token lets the shell resolve the current user through process state,
    // which can be redirected by inherited USERPROFILE values and fork the
    // cross-process writer-authority registry. Microsoft requires QUERY and
    // IMPERSONATE access for a non-null token passed to SHGetKnownFolderPath.
    let token = w::HPROCESS::GetCurrentProcess()
        .OpenProcessToken(co::TOKEN::QUERY | co::TOKEN::IMPERSONATE)
        .map_err(|error| {
            authority_unavailable(
                WorkspaceAuthorityOperation::ResolveIdentityRegistry,
                windows_system_error(error),
            )
        })?;
    let value = w::SHGetKnownFolderPath(
        &co::KNOWNFOLDERID::LocalAppData,
        co::KF::DEFAULT,
        Some(&token),
    )
    .map_err(|error| {
        authority_unavailable(
            WorkspaceAuthorityOperation::ResolveIdentityRegistry,
            windows_hresult_error(error),
        )
    })?;
    let path = PathBuf::from(value);
    if path.as_os_str().is_empty() || !path.is_absolute() {
        return Err(authority_unavailable(
            WorkspaceAuthorityOperation::ResolveIdentityRegistry,
            io::Error::new(
                io::ErrorKind::InvalidData,
                "the Windows LocalAppData Known Folder returned an invalid directory",
            ),
        ));
    }
    Ok(path)
}

#[cfg(windows)]
fn windows_system_error(error: winsafe::co::ERROR) -> io::Error {
    classified_windows_error(error.raw(), error)
}

#[cfg(windows)]
fn windows_hresult_error(error: winsafe::co::HRESULT) -> io::Error {
    const HRESULT_FACILITY_MASK: u32 = 0xffff_0000;
    const HRESULT_FROM_WIN32_PREFIX: u32 = 0x8007_0000;

    let raw = error.raw();
    if raw & HRESULT_FACILITY_MASK == HRESULT_FROM_WIN32_PREFIX {
        return classified_windows_error(raw & 0x0000_ffff, error);
    }
    io::Error::other(error)
}

#[cfg(windows)]
fn classified_windows_error<E>(code: u32, error: E) -> io::Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    use winsafe::co::ERROR;

    let kind = if code == ERROR::FILE_NOT_FOUND.raw()
        || code == ERROR::PATH_NOT_FOUND.raw()
        || code == ERROR::PROFILE_NOT_FOUND.raw()
    {
        Some(io::ErrorKind::NotFound)
    } else if code == ERROR::ACCESS_DENIED.raw() {
        Some(io::ErrorKind::PermissionDenied)
    } else {
        None
    };
    if let Some(kind) = kind {
        return io::Error::new(kind, error);
    }
    match i32::try_from(code) {
        Ok(code) => io::Error::from_raw_os_error(code),
        Err(_) => io::Error::other(error),
    }
}

#[cfg(windows)]
fn windows_registry_base(
    local_app_data: Option<PathBuf>,
) -> Result<(PathBuf, Option<PathBuf>), WorkspaceSessionError> {
    let base = local_app_data.ok_or_else(|| {
        authority_unavailable(
            WorkspaceAuthorityOperation::ResolveIdentityRegistry,
            io::Error::new(
                io::ErrorKind::NotFound,
                "Windows did not provide the current account's LocalAppData Known Folder",
            ),
        )
    })?;
    Ok((base, None))
}

#[cfg(unix)]
fn unix_account_home() -> Result<PathBuf, WorkspaceSessionError> {
    use uzers::os::unix::UserExt;

    let effective_uid = rustix::process::geteuid().as_raw();
    let user = uzers::get_user_by_uid(effective_uid).ok_or_else(|| {
        authority_unavailable(
            WorkspaceAuthorityOperation::ResolveIdentityRegistry,
            io::Error::new(
                io::ErrorKind::NotFound,
                "the operating-system account database has no entry for the effective user",
            ),
        )
    })?;
    let account_home = user.home_dir().to_path_buf();
    if account_home.as_os_str().is_empty() || !account_home.is_absolute() {
        return Err(authority_unavailable(
            WorkspaceAuthorityOperation::ResolveIdentityRegistry,
            io::Error::new(
                io::ErrorKind::InvalidData,
                "the operating-system account database returned an invalid home directory",
            ),
        ));
    }
    Ok(account_home)
}

#[cfg(target_os = "linux")]
fn reject_flatpak_authority_namespace(marker: &Path) -> Result<(), WorkspaceSessionError> {
    match fs::symlink_metadata(marker) {
        Ok(_) => Err(authority_unavailable(
            WorkspaceAuthorityOperation::ResolveIdentityRegistry,
            io::Error::new(
                io::ErrorKind::Unsupported,
                "Flatpak writer authority requires a host-shared broker or authority namespace",
            ),
        )),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(authority_unavailable(
            WorkspaceAuthorityOperation::ResolveIdentityRegistry,
            error,
        )),
    }
}

fn prepare_default_data_root(
    base: &Path,
    account_home: Option<&Path>,
) -> Result<(), WorkspaceSessionError> {
    #[cfg(windows)]
    {
        // LocalAppData is operating-system Known Folder traversal
        // infrastructure, not a Liaison-owned security boundary, and may be
        // owned by SYSTEM or Administrators on an elevated account. Liaison
        // never creates a missing Known Folder; it verifies the bound
        // directory and applies a private ACL only to its own registry below.
        let _ = account_home;
        verify_existing_default_data_root(base)
    }
    #[cfg(not(windows))]
    {
        let account_home = account_home.ok_or_else(|| {
            authority_unavailable(
                WorkspaceAuthorityOperation::ResolveIdentityRegistry,
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "the Unix identity-registry locator omitted the account home",
                ),
            )
        })?;
        ensure_data_root_from_home(base, account_home)
    }
}

#[cfg(any(not(windows), test))]
fn ensure_data_root_from_home(base: &Path, home: &Path) -> Result<(), WorkspaceSessionError> {
    let Ok(relative) = base.strip_prefix(home) else {
        return verify_existing_default_data_root(base);
    };
    let canonical_home = fs::canonicalize(home).map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
    })?;
    let mut directory =
        Dir::open_ambient_dir(&canonical_home, ambient_authority()).map_err(|error| {
            authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
        })?;
    verify_owned_data_directory(&directory)?;

    for component in relative.components() {
        let std::path::Component::Normal(name) = component else {
            return Err(unsafe_path(
                WorkspaceAuthorityPathIssue::IdentityRegistryRootMustBeAbsolute,
            ));
        };
        match directory.symlink_metadata(name) {
            Ok(_) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                let builder = private_directory_builder();
                match directory.create_dir_with(name, &builder) {
                    Ok(()) => {}
                    Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                    Err(error) => {
                        return Err(authority_unavailable(
                            WorkspaceAuthorityOperation::ResolveIdentityRegistry,
                            error,
                        ));
                    }
                }
            }
            Err(error) => {
                return Err(authority_unavailable(
                    WorkspaceAuthorityOperation::ResolveIdentityRegistry,
                    error,
                ));
            }
        }
        let current = directory.symlink_metadata(name).map_err(|error| {
            authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
        })?;
        if metadata_is_link_or_reparse(&current) || !current.is_dir() {
            return Err(unsafe_path(
                WorkspaceAuthorityPathIssue::IdentityRegistryRootWasReplaced,
            ));
        }
        let next = directory.open_dir_nofollow(name).map_err(|error| {
            authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
        })?;
        let bound = next.dir_metadata().map_err(|error| {
            authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
        })?;
        if !same_bound_directory(&current, &bound) {
            return Err(unsafe_path(
                WorkspaceAuthorityPathIssue::IdentityRegistryRootWasReplaced,
            ));
        }
        verify_owned_data_directory(&next)?;
        directory = next;
    }
    Ok(())
}

fn verify_existing_default_data_root(base: &Path) -> Result<(), WorkspaceSessionError> {
    let metadata = fs::symlink_metadata(base).map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
    })?;
    if ambient_file_type_is_link_or_reparse(&metadata) || !metadata.is_dir() {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityRegistryRootWasReplaced,
        ));
    }
    let directory = Dir::open_ambient_dir(base, ambient_authority()).map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
    })?;
    let rebound = Dir::open_ambient_dir(base, ambient_authority()).map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
    })?;
    let bound = directory.dir_metadata().map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
    })?;
    let current = rebound.dir_metadata().map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
    })?;
    if !same_bound_directory(&current, &bound) {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityRegistryRootWasReplaced,
        ));
    }
    verify_owned_data_directory(&directory)
}

fn verify_owned_data_directory(directory: &Dir) -> Result<(), WorkspaceSessionError> {
    let metadata = directory.dir_metadata().map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
    })?;
    if metadata_is_link_or_reparse(&metadata) || !metadata.is_dir() {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityRegistryRootWasReplaced,
        ));
    }
    #[cfg(unix)]
    {
        use cap_std::fs::MetadataExt;
        if metadata.uid() != rustix::process::geteuid().as_raw() {
            return Err(unsafe_path(
                WorkspaceAuthorityPathIssue::IdentityRegistryOwnerMismatch,
            ));
        }
    }
    Ok(())
}

fn ensure_registry_directory(parent: &Dir, name: &OsStr) -> Result<bool, WorkspaceSessionError> {
    let created = match parent.symlink_metadata(name) {
        Ok(_) => false,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            let builder = private_directory_builder();
            match parent.create_dir_with(name, &builder) {
                Ok(()) => true,
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => false,
                Err(error) => {
                    return Err(authority_unavailable(
                        WorkspaceAuthorityOperation::InspectIdentityRegistry,
                        error,
                    ));
                }
            }
        }
        Err(error) => {
            return Err(authority_unavailable(
                WorkspaceAuthorityOperation::InspectIdentityRegistry,
                error,
            ));
        }
    };
    let metadata = parent.symlink_metadata(name).map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::InspectIdentityRegistry, error)
    })?;
    verify_registry_metadata(&metadata)?;
    Ok(created)
}

fn verify_registry_binding(
    canonical_parent: &Path,
    parent: &Dir,
    name: &Path,
    directory: &Dir,
) -> Result<(), WorkspaceSessionError> {
    verify_parent_binding(canonical_parent, parent)?;
    let current = parent.symlink_metadata(name).map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::InspectIdentityRegistry, error)
    })?;
    verify_registry_metadata(&current)?;
    let bound = directory.dir_metadata().map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::InspectIdentityRegistry, error)
    })?;
    verify_registry_metadata(&bound)?;
    if !same_bound_directory(&current, &bound) {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityRegistryRootWasReplaced,
        ));
    }
    #[cfg(windows)]
    verify_windows_security(
        directory,
        true,
        WorkspaceAuthorityOperation::InspectIdentityRegistry,
        WorkspaceAuthorityPathIssue::IdentityRegistryPermissionsUnsafe,
        WorkspaceAuthorityPathIssue::IdentityRegistryOwnerMismatch,
    )?;
    Ok(())
}

fn verify_registry_metadata(metadata: &cap_std::fs::Metadata) -> Result<(), WorkspaceSessionError> {
    if metadata_is_link_or_reparse(metadata) {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityRegistryRootIsSymlink,
        ));
    }
    if !metadata.is_dir() {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityRegistryRootIsNotDirectory,
        ));
    }
    #[cfg(unix)]
    {
        use cap_std::fs::MetadataExt;
        if metadata.uid() != rustix::process::geteuid().as_raw() {
            return Err(unsafe_path(
                WorkspaceAuthorityPathIssue::IdentityRegistryOwnerMismatch,
            ));
        }
        if metadata.mode() & 0o777 != PRIVATE_DIRECTORY_MODE {
            return Err(unsafe_path(
                WorkspaceAuthorityPathIssue::IdentityRegistryPermissionsUnsafe,
            ));
        }
    }
    Ok(())
}

fn verify_parent_binding(
    canonical_parent: &Path,
    parent: &Dir,
) -> Result<(), WorkspaceSessionError> {
    let ambient = fs::symlink_metadata(canonical_parent).map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
    })?;
    if !ambient.is_dir() || ambient_file_type_is_link_or_reparse(&ambient) {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityRegistryRootWasReplaced,
        ));
    }
    let ambient_directory =
        Dir::open_ambient_dir(canonical_parent, ambient_authority()).map_err(|error| {
            authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
        })?;
    let current = ambient_directory.dir_metadata().map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
    })?;
    let bound = parent.dir_metadata().map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
    })?;
    if !same_bound_directory(&current, &bound) {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityRegistryRootWasReplaced,
        ));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if ambient.uid() != rustix::process::geteuid().as_raw() {
            return Err(unsafe_path(
                WorkspaceAuthorityPathIssue::IdentityRegistryOwnerMismatch,
            ));
        }
    }
    Ok(())
}

fn identity_lock_name(workspace_id: WorkspaceId) -> String {
    format!("workspace-{workspace_id}.lock")
}

fn open_identity_lock_file(
    directory: &Dir,
    file_name: &str,
) -> Result<File, WorkspaceSessionError> {
    let missing = match directory.symlink_metadata(file_name) {
        Ok(metadata) if metadata_is_link_or_reparse(&metadata) => {
            return Err(unsafe_path(
                WorkspaceAuthorityPathIssue::IdentityLockFileIsSymlink,
            ));
        }
        Ok(metadata) if !metadata.is_file() => {
            return Err(unsafe_path(
                WorkspaceAuthorityPathIssue::IdentityLockPathIsNotFile,
            ));
        }
        Ok(metadata) if metadata.len() != 0 => {
            return Err(unsafe_path(
                WorkspaceAuthorityPathIssue::IdentityLockFileHasUnexpectedData,
            ));
        }
        Ok(_) => false,
        Err(error) if error.kind() == io::ErrorKind::NotFound => true,
        Err(error) => {
            return Err(authority_unavailable(
                WorkspaceAuthorityOperation::OpenIdentityLockFile,
                error,
            ));
        }
    };

    let (file, created) = if missing {
        match open_identity_lock_handle(directory, file_name, true) {
            Ok(file) => (file, true),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => (
                open_identity_lock_handle(directory, file_name, false).map_err(|error| {
                    authority_unavailable(WorkspaceAuthorityOperation::OpenIdentityLockFile, error)
                })?,
                false,
            ),
            Err(error) => {
                return Err(authority_unavailable(
                    WorkspaceAuthorityOperation::OpenIdentityLockFile,
                    error,
                ));
            }
        }
    } else {
        (
            open_identity_lock_handle(directory, file_name, false).map_err(|error| {
                authority_unavailable(WorkspaceAuthorityOperation::OpenIdentityLockFile, error)
            })?,
            false,
        )
    };
    #[cfg(windows)]
    let file = if created {
        let mut file = file;
        apply_private_windows_security(
            &mut file,
            false,
            WorkspaceAuthorityOperation::OpenIdentityLockFile,
        )?;
        file
    } else {
        file
    };
    #[cfg(not(windows))]
    let _ = created;
    verify_identity_open_metadata(&file)?;
    verify_identity_lock_file(directory, file_name, &file)?;
    Ok(file)
}

fn open_identity_lock_handle(
    directory: &Dir,
    file_name: &str,
    create_new: bool,
) -> io::Result<File> {
    let mut options = private_open_options();
    options.read(true).write(true).create_new(create_new);
    #[cfg(windows)]
    {
        use cap_std::fs::OpenOptionsExt;
        use windows_permissions::constants::AccessRights;

        let mut access =
            AccessRights::GenericRead | AccessRights::GenericWrite | AccessRights::ReadControl;
        if create_new {
            access |= AccessRights::WriteDac | AccessRights::WriteOwner;
        }
        options.access_mode(access.bits());
        options.share_mode(0x0000_0001 | 0x0000_0002);
    }
    directory
        .open_with(file_name, &options)
        .map(cap_std::fs::File::into_std)
}

fn verify_identity_lock_file(
    directory: &Dir,
    file_name: &str,
    file: &File,
) -> Result<(), WorkspaceSessionError> {
    let open_metadata = cap_std::fs::File::from_std(file.try_clone().map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::OpenIdentityLockFile, error)
    })?)
    .metadata()
    .map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::OpenIdentityLockFile, error)
    })?;
    let path_metadata = directory.symlink_metadata(file_name).map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::OpenIdentityLockFile, error)
    })?;
    if metadata_is_link_or_reparse(&path_metadata) {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityLockFileIsSymlink,
        ));
    }
    if !open_metadata.is_file() || !path_metadata.is_file() {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityLockPathIsNotFile,
        ));
    }
    if open_metadata.len() != 0 || path_metadata.len() != 0 {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityLockFileHasUnexpectedData,
        ));
    }
    verify_identity_open_metadata(file)?;
    verify_identity_path_metadata(&path_metadata)?;
    if !same_bound_file(&open_metadata, &path_metadata) {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityLockFileWasReplaced,
        ));
    }
    Ok(())
}

fn verify_identity_path_metadata(
    metadata: &cap_std::fs::Metadata,
) -> Result<(), WorkspaceSessionError> {
    use cap_fs_ext::MetadataExt;

    if metadata.nlink() != 1 {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityLockLinkCountUnsafe,
        ));
    }
    #[cfg(unix)]
    {
        use cap_std::fs::MetadataExt;
        if metadata.uid() != rustix::process::geteuid().as_raw() {
            return Err(unsafe_path(
                WorkspaceAuthorityPathIssue::IdentityLockOwnerMismatch,
            ));
        }
        if metadata.mode() & 0o777 != PRIVATE_FILE_MODE {
            return Err(unsafe_path(
                WorkspaceAuthorityPathIssue::IdentityLockPermissionsUnsafe,
            ));
        }
    }
    Ok(())
}

fn verify_identity_open_metadata(file: &File) -> Result<(), WorkspaceSessionError> {
    let metadata = file.metadata().map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::OpenIdentityLockFile, error)
    })?;
    if !metadata.is_file() {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityLockPathIsNotFile,
        ));
    }
    if metadata.len() != 0 {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityLockFileHasUnexpectedData,
        ));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if metadata.uid() != rustix::process::geteuid().as_raw() {
            return Err(unsafe_path(
                WorkspaceAuthorityPathIssue::IdentityLockOwnerMismatch,
            ));
        }
        if metadata.mode() & 0o777 != PRIVATE_FILE_MODE {
            return Err(unsafe_path(
                WorkspaceAuthorityPathIssue::IdentityLockPermissionsUnsafe,
            ));
        }
        if metadata.nlink() != 1 {
            return Err(unsafe_path(
                WorkspaceAuthorityPathIssue::IdentityLockLinkCountUnsafe,
            ));
        }
    }
    #[cfg(windows)]
    {
        verify_windows_security(
            file,
            false,
            WorkspaceAuthorityOperation::OpenIdentityLockFile,
            WorkspaceAuthorityPathIssue::IdentityLockPermissionsUnsafe,
            WorkspaceAuthorityPathIssue::IdentityLockOwnerMismatch,
        )?;
    }
    Ok(())
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

fn private_directory_builder() -> DirBuilder {
    #[cfg(unix)]
    {
        use cap_std::fs::DirBuilderExt;
        let mut builder = DirBuilder::new();
        builder.mode(PRIVATE_DIRECTORY_MODE);
        builder
    }
    #[cfg(not(unix))]
    {
        let _ = PRIVATE_DIRECTORY_MODE;
        DirBuilder::new()
    }
}

#[cfg(unix)]
fn ambient_file_type_is_link_or_reparse(metadata: &fs::Metadata) -> bool {
    metadata.file_type().is_symlink()
}

#[cfg(windows)]
fn ambient_file_type_is_link_or_reparse(metadata: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    metadata.file_type().is_symlink()
        || metadata.file_attributes() & WINDOWS_FILE_ATTRIBUTE_REPARSE_POINT != 0
}

#[cfg(not(any(unix, windows)))]
fn ambient_file_type_is_link_or_reparse(metadata: &fs::Metadata) -> bool {
    metadata.file_type().is_symlink()
}

#[cfg(unix)]
fn metadata_is_link_or_reparse(metadata: &cap_std::fs::Metadata) -> bool {
    metadata.file_type().is_symlink()
}

#[cfg(windows)]
fn metadata_is_link_or_reparse(metadata: &cap_std::fs::Metadata) -> bool {
    use cap_std::fs::MetadataExt;
    metadata.file_type().is_symlink()
        || metadata.file_attributes() & WINDOWS_FILE_ATTRIBUTE_REPARSE_POINT != 0
}

#[cfg(not(any(unix, windows)))]
fn metadata_is_link_or_reparse(metadata: &cap_std::fs::Metadata) -> bool {
    metadata.file_type().is_symlink()
}

fn same_bound_directory(current: &cap_std::fs::Metadata, bound: &cap_std::fs::Metadata) -> bool {
    use cap_fs_ext::MetadataExt;
    current.dev() == bound.dev() && current.ino() == bound.ino()
}

#[cfg(windows)]
fn open_and_harden_created_windows_registry(
    parent: &Dir,
    name: &OsStr,
) -> Result<File, WorkspaceSessionError> {
    let mut handle = open_windows_directory_security_handle(parent, name)?;
    verify_windows_creation_owner(
        &handle,
        WorkspaceAuthorityOperation::InspectIdentityRegistry,
    )?;
    apply_private_windows_security(
        &mut handle,
        true,
        WorkspaceAuthorityOperation::InspectIdentityRegistry,
    )?;
    verify_windows_security(
        &handle,
        true,
        WorkspaceAuthorityOperation::InspectIdentityRegistry,
        WorkspaceAuthorityPathIssue::IdentityRegistryPermissionsUnsafe,
        WorkspaceAuthorityPathIssue::IdentityRegistryOwnerMismatch,
    )?;
    Ok(handle)
}

#[cfg(windows)]
fn open_windows_directory_security_handle(
    parent: &Dir,
    name: &OsStr,
) -> Result<File, WorkspaceSessionError> {
    use cap_std::fs::OpenOptionsExt;
    use windows_permissions::constants::AccessRights;

    const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
    const FILE_SHARE_READ: u32 = 0x0000_0001;
    const FILE_SHARE_WRITE: u32 = 0x0000_0002;

    let mut options = private_open_options();
    options.read(true);
    options.access_mode(
        (AccessRights::ReadControl
            | AccessRights::WriteDac
            | AccessRights::WriteOwner
            | AccessRights::Bit7)
            .bits(),
    );
    options.share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE);
    options.custom_flags(FILE_FLAG_BACKUP_SEMANTICS);
    let handle = parent
        .open_with(name, &options)
        .map(cap_std::fs::File::into_std)
        .map_err(|error| {
            authority_unavailable(WorkspaceAuthorityOperation::InspectIdentityRegistry, error)
        })?;
    let metadata = cap_std::fs::File::from_std(handle.try_clone().map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::InspectIdentityRegistry, error)
    })?)
    .metadata()
    .map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::InspectIdentityRegistry, error)
    })?;
    if metadata_is_link_or_reparse(&metadata) || !metadata.is_dir() {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityRegistryRootWasReplaced,
        ));
    }
    Ok(handle)
}

#[cfg(windows)]
fn verify_windows_creation_owner<T>(
    handle: &T,
    operation: WorkspaceAuthorityOperation,
) -> Result<(), WorkspaceSessionError>
where
    T: std::os::windows::io::AsRawHandle,
{
    use windows_permissions::{
        LocalBox, Sid,
        constants::{SeObjectType, SecurityInformation},
        utilities::current_process_sid,
        wrappers::GetSecurityInfo,
    };

    let descriptor = GetSecurityInfo(
        handle,
        SeObjectType::SE_FILE_OBJECT,
        SecurityInformation::Owner,
    )
    .map_err(|error| authority_unavailable(operation, error))?;
    let process_sid =
        current_process_sid().map_err(|error| authority_unavailable(operation, error))?;
    let system_sid: LocalBox<Sid> = "S-1-5-18"
        .parse()
        .map_err(|error| authority_unavailable(operation, error))?;
    let administrators_sid: LocalBox<Sid> = "S-1-5-32-544"
        .parse()
        .map_err(|error| authority_unavailable(operation, error))?;
    let owner = descriptor
        .owner()
        .ok_or_else(|| unsafe_path(WorkspaceAuthorityPathIssue::IdentityRegistryOwnerMismatch))?;
    if owner != &*process_sid && owner != &*system_sid && owner != &*administrators_sid {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityRegistryOwnerMismatch,
        ));
    }
    Ok(())
}

#[cfg(windows)]
fn apply_private_windows_security<T>(
    handle: &mut T,
    container: bool,
    operation: WorkspaceAuthorityOperation,
) -> Result<(), WorkspaceSessionError>
where
    T: std::os::windows::io::AsRawHandle,
{
    use windows_permissions::{
        LocalBox, SecurityDescriptor,
        constants::{SeObjectType, SecurityInformation},
        utilities::current_process_sid,
        wrappers::SetSecurityInfo,
    };

    let process_sid =
        current_process_sid().map_err(|error| authority_unavailable(operation, error))?;
    let inheritance = if container { "OICI" } else { "" };
    let descriptor: LocalBox<SecurityDescriptor> = format!(
        "O:{process_sid}D:P(A;{inheritance};FA;;;{process_sid})(A;{inheritance};FA;;;SY)(A;{inheritance};FA;;;BA)"
    )
    .parse()
    .map_err(|error| authority_unavailable(operation, error))?;
    let dacl = descriptor.dacl().ok_or_else(|| {
        authority_unavailable(
            operation,
            io::Error::new(
                io::ErrorKind::InvalidData,
                "the private Windows authority descriptor has no DACL",
            ),
        )
    })?;
    SetSecurityInfo(
        handle,
        SeObjectType::SE_FILE_OBJECT,
        SecurityInformation::Owner | SecurityInformation::Dacl | SecurityInformation::ProtectedDacl,
        Some(&process_sid),
        None,
        Some(dacl),
        None,
    )
    .map_err(|error| authority_unavailable(operation, error))
}

#[cfg(windows)]
fn verify_windows_security<T>(
    handle: &T,
    container: bool,
    operation: WorkspaceAuthorityOperation,
    permissions_issue: WorkspaceAuthorityPathIssue,
    owner_issue: WorkspaceAuthorityPathIssue,
) -> Result<(), WorkspaceSessionError>
where
    T: std::os::windows::io::AsRawHandle,
{
    use windows_permissions::{
        LocalBox, Sid,
        constants::{AccessRights, AceFlags, AceType, SeObjectType, SecurityInformation},
        utilities::current_process_sid,
        wrappers::{ConvertSecurityDescriptorToStringSecurityDescriptor, GetSecurityInfo},
    };

    // `windows-permissions` 0.2.4's blanket `WindowsSecure` implementation
    // queries handles as `SE_UNKNOWN_OBJECT_TYPE`. These handles always name
    // files or directories, so Win32 requires `SE_FILE_OBJECT` explicitly.
    let descriptor = GetSecurityInfo(
        handle,
        SeObjectType::SE_FILE_OBJECT,
        SecurityInformation::Owner | SecurityInformation::Dacl,
    )
    .map_err(|error| authority_unavailable(operation, error))?;
    let process_sid =
        current_process_sid().map_err(|error| authority_unavailable(operation, error))?;
    if descriptor.owner() != Some(&*process_sid) {
        return Err(unsafe_path(owner_issue));
    }
    let dacl = descriptor
        .dacl()
        .ok_or_else(|| unsafe_path(permissions_issue))?;
    let dacl_sddl =
        ConvertSecurityDescriptorToStringSecurityDescriptor(&descriptor, SecurityInformation::Dacl)
            .map_err(|error| authority_unavailable(operation, error))?;
    if !dacl_sddl.to_string_lossy().starts_with("D:P") || dacl.len() != 3 {
        return Err(unsafe_path(permissions_issue));
    }

    let system_sid: LocalBox<Sid> = "S-1-5-18"
        .parse()
        .map_err(|error| authority_unavailable(operation, error))?;
    let administrators_sid: LocalBox<Sid> = "S-1-5-32-544"
        .parse()
        .map_err(|error| authority_unavailable(operation, error))?;
    let expected_flags = if container {
        AceFlags::ObjectInherit | AceFlags::ContainerInherit
    } else {
        AceFlags::empty()
    };
    let mut process_seen = false;
    let mut system_seen = false;
    let mut administrators_seen = false;
    for index in 0..dacl.len() {
        let ace = dacl
            .get_ace(index)
            .ok_or_else(|| unsafe_path(permissions_issue))?;
        if ace.ace_type() != AceType::ACCESS_ALLOWED_ACE_TYPE
            || ace.flags() != expected_flags
            || ace.mask() != AccessRights::FileAllAccess
        {
            return Err(unsafe_path(permissions_issue));
        }
        let sid = ace.sid().ok_or_else(|| unsafe_path(permissions_issue))?;
        if sid == &*process_sid && !process_seen {
            process_seen = true;
        } else if sid == &*system_sid && !system_seen {
            system_seen = true;
        } else if sid == &*administrators_sid && !administrators_seen {
            administrators_seen = true;
        } else {
            return Err(unsafe_path(permissions_issue));
        }
    }
    if !process_seen || !system_seen || !administrators_seen {
        return Err(unsafe_path(permissions_issue));
    }
    Ok(())
}

fn same_bound_file(open: &cap_std::fs::Metadata, current: &cap_std::fs::Metadata) -> bool {
    use cap_fs_ext::MetadataExt;
    open.dev() == current.dev() && open.ino() == current.ino()
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
    #[cfg(target_os = "linux")]
    use super::reject_flatpak_authority_namespace;
    #[cfg(windows)]
    use super::{
        IdentityRegistry, identity_lock_name, open_identity_lock_handle,
        open_windows_directory_security_handle, verify_windows_security, windows_hresult_error,
        windows_registry_base, windows_system_error,
    };
    use super::{ensure_data_root_from_home, prepare_default_data_root};
    use liaison_workspace::{
        WorkspaceAuthorityFailureKind, WorkspaceAuthorityOperation, WorkspaceSessionError,
    };
    #[cfg(windows)]
    use liaison_workspace::{WorkspaceAuthorityPathIssue, WorkspaceId};
    use std::error::Error;
    #[cfg(unix)]
    use std::fs;
    use tempfile::tempdir;

    #[cfg(windows)]
    fn set_windows_test_security<T>(
        handle: &mut T,
        inheritance: &str,
        extra_ace: &str,
    ) -> Result<(), Box<dyn Error>>
    where
        T: std::os::windows::io::AsRawHandle,
    {
        use windows_permissions::{
            LocalBox, SecurityDescriptor,
            constants::{SeObjectType, SecurityInformation},
            utilities::current_process_sid,
            wrappers::SetSecurityInfo,
        };

        let process_sid = current_process_sid()?;
        let descriptor: LocalBox<SecurityDescriptor> = format!(
            "O:{process_sid}D:P(A;{inheritance};FA;;;{process_sid})(A;{inheritance};FA;;;SY)(A;{inheritance};FA;;;BA){extra_ace}"
        )
        .parse()?;
        SetSecurityInfo(
            handle,
            SeObjectType::SE_FILE_OBJECT,
            SecurityInformation::Owner
                | SecurityInformation::Dacl
                | SecurityInformation::ProtectedDacl,
            Some(&process_sid),
            None,
            descriptor.dacl(),
            None,
        )?;
        Ok(())
    }

    #[cfg(windows)]
    fn windows_test_dacl<T>(handle: &T) -> Result<std::ffi::OsString, Box<dyn Error>>
    where
        T: std::os::windows::io::AsRawHandle,
    {
        use windows_permissions::{
            constants::{SeObjectType, SecurityInformation},
            wrappers::{ConvertSecurityDescriptorToStringSecurityDescriptor, GetSecurityInfo},
        };

        let descriptor = GetSecurityInfo(
            handle,
            SeObjectType::SE_FILE_OBJECT,
            SecurityInformation::Dacl,
        )?;
        Ok(ConvertSecurityDescriptorToStringSecurityDescriptor(
            &descriptor,
            SecurityInformation::Dacl,
        )?)
    }

    #[cfg(windows)]
    fn open_windows_test_file_security(
        directory: &cap_std::fs::Dir,
        name: &str,
    ) -> Result<std::fs::File, Box<dyn Error>> {
        use cap_fs_ext::{FollowSymlinks, OpenOptionsFollowExt};
        use cap_std::fs::{OpenOptions, OpenOptionsExt};
        use windows_permissions::constants::AccessRights;

        let mut options = OpenOptions::new();
        options.read(true).write(true).follow(FollowSymlinks::No);
        options.access_mode(
            (AccessRights::GenericRead
                | AccessRights::GenericWrite
                | AccessRights::ReadControl
                | AccessRights::WriteDac
                | AccessRights::WriteOwner)
                .bits(),
        );
        options.share_mode(0x0000_0001 | 0x0000_0002);
        Ok(directory.open_with(name, &options)?.into_std())
    }

    #[test]
    fn first_use_creates_only_missing_owned_data_root_components() -> Result<(), Box<dyn Error>> {
        let home = tempdir()?;
        let data_root = home.path().join("missing").join("local-data");

        ensure_data_root_from_home(&data_root, home.path())?;

        assert!(data_root.is_dir());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                fs::metadata(home.path().join("missing"))?
                    .permissions()
                    .mode()
                    & 0o777,
                0o700
            );
            assert_eq!(
                fs::metadata(&data_root)?.permissions().mode() & 0o777,
                0o700
            );
        }
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn first_use_rejects_a_symlinked_data_root_component() -> Result<(), Box<dyn Error>> {
        use std::os::unix::fs::symlink;

        let home = tempdir()?;
        let outside = tempdir()?;
        symlink(outside.path(), home.path().join("linked"))?;

        assert!(
            ensure_data_root_from_home(&home.path().join("linked").join("data"), home.path())
                .is_err()
        );
        assert!(!outside.path().join("data").exists());
        Ok(())
    }

    #[test]
    fn missing_canonical_account_home_fails_closed_with_a_typed_error() -> Result<(), Box<dyn Error>>
    {
        let parent = tempdir()?;
        let missing_home = parent.path().join("missing-account-home");
        let canonical_data_root = missing_home.join(".local").join("share");

        assert!(matches!(
            prepare_default_data_root(&canonical_data_root, Some(&missing_home)),
            Err(WorkspaceSessionError::AuthorityUnavailable {
                operation: WorkspaceAuthorityOperation::ResolveIdentityRegistry,
                failure: WorkspaceAuthorityFailureKind::NotFound,
                ..
            })
        ));
        assert!(!missing_home.exists());
        Ok(())
    }

    #[test]
    fn missing_canonical_data_root_outside_account_home_is_never_replaced_by_a_fallback()
    -> Result<(), Box<dyn Error>> {
        let account_home = tempdir()?;
        let outside = tempdir()?;
        let missing_data_root = outside.path().join("missing-canonical-data-root");

        assert!(matches!(
            prepare_default_data_root(&missing_data_root, Some(account_home.path())),
            Err(WorkspaceSessionError::AuthorityUnavailable {
                operation: WorkspaceAuthorityOperation::ResolveIdentityRegistry,
                failure: WorkspaceAuthorityFailureKind::NotFound,
                ..
            })
        ));
        assert!(!missing_data_root.exists());
        Ok(())
    }

    #[cfg(windows)]
    #[test]
    fn broad_windows_ancestor_does_not_weaken_created_authority_objects()
    -> Result<(), Box<dyn Error>> {
        use cap_std::{ambient_authority, fs::Dir};
        use std::{ffi::OsStr, fs};

        let root = tempdir()?;
        fs::create_dir(root.path().join("broad-parent"))?;
        let root_handle = Dir::open_ambient_dir(root.path(), ambient_authority())?;
        let mut broad_parent =
            open_windows_directory_security_handle(&root_handle, OsStr::new("broad-parent"))?;
        set_windows_test_security(&mut broad_parent, "OICI", "(A;OICI;FA;;;BU)")?;
        assert!(matches!(
            verify_windows_security(
                &broad_parent,
                true,
                WorkspaceAuthorityOperation::InspectIdentityRegistry,
                WorkspaceAuthorityPathIssue::IdentityRegistryPermissionsUnsafe,
                WorkspaceAuthorityPathIssue::IdentityRegistryOwnerMismatch,
            ),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::IdentityRegistryPermissionsUnsafe
            })
        ));

        let registry_path = root.path().join("broad-parent").join("registry");
        let registry = IdentityRegistry::bind(&registry_path)?;
        let workspace_id = WorkspaceId::new();
        let authority = registry.acquire(workspace_id)?;
        authority.verify()?;
        drop(authority);

        assert!(
            verify_windows_security(
                &registry.directory,
                true,
                WorkspaceAuthorityOperation::InspectIdentityRegistry,
                WorkspaceAuthorityPathIssue::IdentityRegistryPermissionsUnsafe,
                WorkspaceAuthorityPathIssue::IdentityRegistryOwnerMismatch,
            )
            .is_ok()
        );
        let lock = open_identity_lock_handle(
            &registry.directory,
            &identity_lock_name(workspace_id),
            false,
        )?;
        assert!(
            verify_windows_security(
                &lock,
                false,
                WorkspaceAuthorityOperation::OpenIdentityLockFile,
                WorkspaceAuthorityPathIssue::IdentityLockPermissionsUnsafe,
                WorkspaceAuthorityPathIssue::IdentityLockOwnerMismatch,
            )
            .is_ok()
        );
        Ok(())
    }

    #[cfg(windows)]
    #[test]
    fn windows_registry_locator_requires_only_local_app_data() -> Result<(), Box<dyn Error>> {
        let local_app_data = tempdir()?;
        let (base, account_home) =
            windows_registry_base(Some(local_app_data.path().to_path_buf()))?;

        assert_eq!(base, local_app_data.path());
        assert!(account_home.is_none());
        Ok(())
    }

    #[cfg(windows)]
    #[test]
    fn windows_locator_errors_preserve_recovery_categories() {
        use winsafe::co;

        assert_eq!(
            windows_system_error(co::ERROR::ACCESS_DENIED).kind(),
            std::io::ErrorKind::PermissionDenied
        );
        assert_eq!(
            windows_hresult_error(co::ERROR::PATH_NOT_FOUND.to_hresult()).kind(),
            std::io::ErrorKind::NotFound
        );
        assert_eq!(
            windows_hresult_error(co::ERROR::PROFILE_NOT_FOUND.to_hresult()).kind(),
            std::io::ErrorKind::NotFound
        );
    }

    #[cfg(windows)]
    #[test]
    fn existing_noncanonical_windows_authority_objects_fail_closed() -> Result<(), Box<dyn Error>> {
        use cap_std::{ambient_authority, fs::Dir};
        use std::{ffi::OsStr, fs};

        let root = tempdir()?;
        fs::create_dir(root.path().join("unsafe-registry"))?;
        let root_handle = Dir::open_ambient_dir(root.path(), ambient_authority())?;
        let mut unsafe_registry =
            open_windows_directory_security_handle(&root_handle, OsStr::new("unsafe-registry"))?;
        set_windows_test_security(
            &mut unsafe_registry,
            "OICI",
            "(A;OICIIO;FW;;;S-1-5-21-1-2-3-1001)",
        )?;
        let before = windows_test_dacl(&unsafe_registry)?;
        drop(unsafe_registry);
        assert!(matches!(
            IdentityRegistry::bind(&root.path().join("unsafe-registry")),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::IdentityRegistryPermissionsUnsafe
            })
        ));
        let after_handle =
            open_windows_directory_security_handle(&root_handle, OsStr::new("unsafe-registry"))?;
        assert_eq!(windows_test_dacl(&after_handle)?, before);

        let safe_registry = IdentityRegistry::bind(&root.path().join("safe-registry"))?;
        let workspace_id = WorkspaceId::new();
        drop(safe_registry.acquire(workspace_id)?);
        let lock_name = identity_lock_name(workspace_id);
        let mut lock = open_windows_test_file_security(&safe_registry.directory, &lock_name)?;
        set_windows_test_security(&mut lock, "", "(A;;FW;;;S-1-5-21-1-2-3-1001)")?;
        drop(lock);
        assert!(matches!(
            safe_registry.acquire(workspace_id),
            Err(WorkspaceSessionError::UnsafeAuthorityPath {
                issue: WorkspaceAuthorityPathIssue::IdentityLockPermissionsUnsafe
            })
        ));
        assert_eq!(
            fs::metadata(root.path().join("safe-registry").join(lock_name))?.len(),
            0
        );
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn flatpak_namespace_is_explicitly_unsupported_until_authority_is_shared()
    -> Result<(), Box<dyn Error>> {
        let marker_parent = tempdir()?;
        let marker = marker_parent.path().join(".flatpak-info");
        fs::write(&marker, b"[Application]\nname=synthetic.test\n")?;

        assert!(matches!(
            reject_flatpak_authority_namespace(&marker),
            Err(WorkspaceSessionError::AuthorityUnavailable {
                operation: WorkspaceAuthorityOperation::ResolveIdentityRegistry,
                failure: WorkspaceAuthorityFailureKind::Unsupported,
                ..
            })
        ));
        Ok(())
    }
}
