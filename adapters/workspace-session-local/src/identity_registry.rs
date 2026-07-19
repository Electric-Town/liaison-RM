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
        ensure_registry_directory(&parent_directory, directory_name)?;
        let directory = parent_directory
            .open_dir_nofollow(directory_name)
            .map_err(|error| {
                authority_unavailable(WorkspaceAuthorityOperation::InspectIdentityRegistry, error)
            })?;
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
    if !base.is_absolute() || !account_home.is_absolute() {
        return Err(unsafe_path(
            WorkspaceAuthorityPathIssue::IdentityRegistryRootMustBeAbsolute,
        ));
    }
    prepare_default_data_root(&base, &account_home)?;
    let base = fs::canonicalize(base).map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::ResolveIdentityRegistry, error)
    })?;
    Ok(base.join(REGISTRY_DIRECTORY_NAME))
}

#[cfg(target_os = "linux")]
fn platform_registry_base() -> Result<(PathBuf, PathBuf), WorkspaceSessionError> {
    reject_flatpak_authority_namespace(Path::new("/.flatpak-info"))?;
    let account_home = unix_account_home()?;
    Ok((account_home.join(".local").join("share"), account_home))
}

#[cfg(target_os = "macos")]
fn platform_registry_base() -> Result<(PathBuf, PathBuf), WorkspaceSessionError> {
    let account_home = unix_account_home()?;
    Ok((
        account_home.join("Library").join("Application Support"),
        account_home,
    ))
}

#[cfg(all(unix, not(any(target_os = "linux", target_os = "macos"))))]
fn platform_registry_base() -> Result<(PathBuf, PathBuf), WorkspaceSessionError> {
    Err(authority_unavailable(
        WorkspaceAuthorityOperation::ResolveIdentityRegistry,
        io::Error::new(
            io::ErrorKind::Unsupported,
            "the canonical per-account writer-authority registry is implemented only for Linux and macOS Unix targets",
        ),
    ))
}

#[cfg(windows)]
fn platform_registry_base() -> Result<(PathBuf, PathBuf), WorkspaceSessionError> {
    let base = dirs::data_local_dir().ok_or_else(|| {
        authority_unavailable(
            WorkspaceAuthorityOperation::ResolveIdentityRegistry,
            io::Error::new(
                io::ErrorKind::NotFound,
                "Windows did not provide the current account's LocalAppData known folder",
            ),
        )
    })?;
    let account_home = dirs::home_dir().ok_or_else(|| {
        authority_unavailable(
            WorkspaceAuthorityOperation::ResolveIdentityRegistry,
            io::Error::new(
                io::ErrorKind::NotFound,
                "Windows did not provide the current account's Profile known folder",
            ),
        )
    })?;
    Ok((base, account_home))
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
    account_home: &Path,
) -> Result<(), WorkspaceSessionError> {
    ensure_data_root_from_home(base, account_home)
}

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
    #[cfg(windows)]
    verify_windows_security(
        directory,
        WorkspaceAuthorityOperation::ResolveIdentityRegistry,
        WorkspaceAuthorityPathIssue::IdentityRegistryPermissionsUnsafe,
        WorkspaceAuthorityPathIssue::IdentityRegistryOwnerMismatch,
    )?;
    Ok(())
}

fn ensure_registry_directory(parent: &Dir, name: &OsStr) -> Result<(), WorkspaceSessionError> {
    match parent.symlink_metadata(name) {
        Ok(_) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            let builder = private_directory_builder();
            match parent.create_dir_with(name, &builder) {
                Ok(()) => {}
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
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
    }
    let metadata = parent.symlink_metadata(name).map_err(|error| {
        authority_unavailable(WorkspaceAuthorityOperation::InspectIdentityRegistry, error)
    })?;
    verify_registry_metadata(&metadata)
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
    #[cfg(windows)]
    verify_windows_security(
        parent,
        WorkspaceAuthorityOperation::InspectIdentityRegistry,
        WorkspaceAuthorityPathIssue::IdentityRegistryPermissionsUnsafe,
        WorkspaceAuthorityPathIssue::IdentityRegistryOwnerMismatch,
    )?;
    Ok(())
}

fn identity_lock_name(workspace_id: WorkspaceId) -> String {
    format!("workspace-{workspace_id}.lock")
}

fn open_identity_lock_file(
    directory: &Dir,
    file_name: &str,
) -> Result<File, WorkspaceSessionError> {
    match directory.symlink_metadata(file_name) {
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
        Ok(_) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(authority_unavailable(
                WorkspaceAuthorityOperation::OpenIdentityLockFile,
                error,
            ));
        }
    }

    let mut options = private_open_options();
    options.read(true).write(true).create(true);
    #[cfg(windows)]
    {
        use cap_std::fs::OpenOptionsExt;
        options.share_mode(0x0000_0001 | 0x0000_0002);
    }
    let file = directory
        .open_with(file_name, &options)
        .map_err(|error| {
            authority_unavailable(WorkspaceAuthorityOperation::OpenIdentityLockFile, error)
        })?
        .into_std();
    verify_identity_open_metadata(&file)?;
    verify_identity_lock_file(directory, file_name, &file)?;
    Ok(file)
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
fn verify_windows_security<T>(
    handle: &T,
    operation: WorkspaceAuthorityOperation,
    permissions_issue: WorkspaceAuthorityPathIssue,
    owner_issue: WorkspaceAuthorityPathIssue,
) -> Result<(), WorkspaceSessionError>
where
    T: std::os::windows::io::AsRawHandle,
{
    use windows_permissions::{
        Sid, Trustee,
        constants::{AccessRights, SeObjectType, SecurityInformation},
        utilities::current_process_sid,
        wrappers::GetSecurityInfo,
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
    let broad_sids = ["S-1-1-0", "S-1-5-11", "S-1-5-32-545"];
    let unsafe_rights = AccessRights::GenericWrite
        | AccessRights::GenericAll
        | AccessRights::WriteDac
        | AccessRights::WriteOwner
        | AccessRights::Delete
        | AccessRights::Bit1
        | AccessRights::Bit2
        | AccessRights::Bit4
        | AccessRights::Bit6
        | AccessRights::Bit8;
    for broad_sid in broad_sids {
        let sid: windows_permissions::LocalBox<Sid> = broad_sid
            .parse()
            .map_err(|error| authority_unavailable(operation, error))?;
        let trustee = Trustee::from(&*sid);
        let rights = dacl
            .effective_rights(&trustee)
            .map_err(|error| authority_unavailable(operation, error))?;
        if rights.intersects(unsafe_rights) {
            return Err(unsafe_path(permissions_issue));
        }
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
    use super::{ensure_data_root_from_home, prepare_default_data_root};
    use liaison_workspace::{
        WorkspaceAuthorityFailureKind, WorkspaceAuthorityOperation, WorkspaceSessionError,
    };
    use std::error::Error;
    #[cfg(unix)]
    use std::fs;
    use tempfile::tempdir;

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
            prepare_default_data_root(&canonical_data_root, &missing_home),
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
            prepare_default_data_root(&missing_data_root, account_home.path()),
            Err(WorkspaceSessionError::AuthorityUnavailable {
                operation: WorkspaceAuthorityOperation::ResolveIdentityRegistry,
                failure: WorkspaceAuthorityFailureKind::NotFound,
                ..
            })
        ));
        assert!(!missing_data_root.exists());
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
