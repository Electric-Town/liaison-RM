#![forbid(unsafe_code)]

use liaison_workspace::{
    BackupError, BackupFile, BackupManifest, BackupVerificationReport, RestoreReport,
    WorkspaceBackupStore, WorkspaceManifest,
};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

const MANIFEST_NAME: &str = "manifest.json";
const PAYLOAD_DIRECTORY: &str = "payload";
const RESTORE_MARKER: &str = ".liaison/restore-in-progress";
const EXCLUDED_PREFIXES: &[&str] = &[
    ".liaison/projections",
    ".liaison/locks",
    ".liaison/tmp",
    RESTORE_MARKER,
];

#[derive(Clone, Copy, Debug, Default)]
pub struct LocalWorkspaceBackup;

impl LocalWorkspaceBackup {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl WorkspaceBackupStore for LocalWorkspaceBackup {
    fn create_snapshot(
        &self,
        workspace: &Path,
        destination: &Path,
        workspace_manifest: &WorkspaceManifest,
    ) -> Result<BackupManifest, BackupError> {
        reject_symlink(workspace)?;
        let workspace_root = fs::canonicalize(workspace)
            .map_err(|error| storage("canonicalize source workspace", &error))?;
        if destination.exists() {
            return Err(BackupError::DestinationExists(
                destination.display().to_string(),
            ));
        }
        let destination = prepare_nonexistent_path(destination)?;
        if destination.starts_with(&workspace_root) {
            return Err(BackupError::DestinationInsideWorkspace(
                destination.display().to_string(),
            ));
        }

        let parent = destination.parent().ok_or_else(|| {
            BackupError::Storage("backup destination has no parent directory".to_owned())
        })?;
        let staging = StagingDirectory::new(parent, "backup")?;
        let payload_root = staging.path().join(PAYLOAD_DIRECTORY);
        fs::create_dir_all(&payload_root)
            .map_err(|error| storage("create backup payload directory", &error))?;

        let layout = collect_layout(&workspace_root, true)?;
        create_directories(&payload_root, &layout.directories)?;
        let mut files = Vec::new();
        for (relative, source) in layout.files {
            let target = payload_root.join(relative_path(&relative)?);
            let (size_bytes, sha256) = copy_and_digest(&source, &target)?;
            files.push(BackupFile::new(relative, size_bytes, sha256)?);
        }

        let manifest = BackupManifest::new(workspace_manifest, layout.directories, files)?;
        write_manifest(staging.path(), &manifest)?;
        staging.commit_to(&destination)?;
        Ok(manifest)
    }

    fn verify_snapshot(&self, snapshot: &Path) -> Result<BackupVerificationReport, BackupError> {
        reject_symlink(snapshot)?;
        let snapshot = fs::canonicalize(snapshot)
            .map_err(|error| storage("canonicalize backup snapshot", &error))?;
        let manifest = read_manifest(&snapshot)?;
        manifest.validate()?;

        let payload_root = snapshot.join(PAYLOAD_DIRECTORY);
        reject_symlink(&payload_root)?;
        if !payload_root.is_dir() {
            return Err(BackupError::PayloadMismatch(
                payload_root.display().to_string(),
            ));
        }

        let actual_layout = collect_layout(&payload_root, false)?;
        let actual_files = actual_layout
            .files
            .into_iter()
            .map(|(relative, _)| relative)
            .collect::<BTreeSet<_>>();
        let expected_files = manifest
            .files
            .iter()
            .map(|file| file.path.clone())
            .collect::<BTreeSet<_>>();
        compare_sets(&actual_files, &expected_files)?;

        let actual_directories = actual_layout.directories.into_iter().collect::<BTreeSet<_>>();
        let expected_directories = manifest
            .directories
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        compare_sets(&actual_directories, &expected_directories)?;

        let mut total_bytes = 0_u64;
        for file in &manifest.files {
            let path = payload_root.join(relative_path(&file.path)?);
            let (size_bytes, digest) = digest_file(&path)?;
            if size_bytes != file.size_bytes {
                return Err(BackupError::PayloadMismatch(file.path.clone()));
            }
            if digest != file.sha256 {
                return Err(BackupError::ChecksumMismatch {
                    path: file.path.clone(),
                    expected: file.sha256.clone(),
                    found: digest,
                });
            }
            total_bytes = total_bytes.checked_add(size_bytes).ok_or_else(|| {
                BackupError::Storage("backup byte count overflowed".to_owned())
            })?;
        }

        Ok(BackupVerificationReport {
            workspace_id: manifest.workspace_id,
            workspace_schema_version: manifest.workspace_schema_version,
            directories_checked: manifest.directories.len(),
            files_checked: manifest.files.len(),
            total_bytes,
        })
    }

    fn stage_restore(&self, snapshot: &Path, target: &Path) -> Result<RestoreReport, BackupError> {
        let verified = self.verify_snapshot(snapshot)?;
        let snapshot_root = fs::canonicalize(snapshot)
            .map_err(|error| storage("canonicalize backup snapshot", &error))?;
        if target.exists() {
            return Err(BackupError::RestoreTargetExists(
                target.display().to_string(),
            ));
        }
        let target = prepare_nonexistent_path(target)?;
        if target.starts_with(&snapshot_root) {
            return Err(BackupError::RestoreTargetInsideSnapshot(
                target.display().to_string(),
            ));
        }

        let manifest = read_manifest(&snapshot_root)?;
        let parent = target.parent().ok_or_else(|| {
            BackupError::Storage("restore target has no parent directory".to_owned())
        })?;
        let staging = StagingDirectory::new(parent, "restore")?;
        create_directories(staging.path(), &manifest.directories)?;
        let marker = staging.path().join(relative_path(RESTORE_MARKER)?);
        write_new(&marker, b"restore validation pending\n")?;

        let payload_root = snapshot_root.join(PAYLOAD_DIRECTORY);
        let mut total_bytes = 0_u64;
        for file in &manifest.files {
            let source = payload_root.join(relative_path(&file.path)?);
            let destination = staging.path().join(relative_path(&file.path)?);
            let (size_bytes, digest) = copy_and_digest(&source, &destination)?;
            if size_bytes != file.size_bytes || digest != file.sha256 {
                return Err(BackupError::ChecksumMismatch {
                    path: file.path.clone(),
                    expected: file.sha256.clone(),
                    found: digest,
                });
            }
            total_bytes = total_bytes.checked_add(size_bytes).ok_or_else(|| {
                BackupError::Storage("restore byte count overflowed".to_owned())
            })?;
        }

        staging.commit_to(&target)?;
        Ok(RestoreReport {
            workspace_id: verified.workspace_id,
            workspace_schema_version: verified.workspace_schema_version,
            target: target.display().to_string(),
            directories_restored: verified.directories_checked,
            files_restored: verified.files_checked,
            total_bytes,
        })
    }

    fn finalize_restore(&self, target: &Path) -> Result<(), BackupError> {
        let marker = target.join(relative_path(RESTORE_MARKER)?);
        reject_symlink(&marker)?;
        if !marker.is_file() {
            return Err(BackupError::Storage(format!(
                "restore marker is missing: {}",
                marker.display()
            )));
        }
        fs::remove_file(marker).map_err(|error| storage("remove restore marker", &error))
    }

    fn discard_restore(&self, target: &Path) -> Result<(), BackupError> {
        let marker = target.join(relative_path(RESTORE_MARKER)?);
        reject_symlink(&marker)?;
        if !marker.is_file() {
            return Err(BackupError::Storage(format!(
                "refusing to remove unmarked restore target: {}",
                target.display()
            )));
        }
        fs::remove_dir_all(target).map_err(|error| storage("discard staged restore", &error))
    }
}

#[derive(Debug)]
struct WorkspaceLayout {
    directories: Vec<String>,
    files: Vec<(String, PathBuf)>,
}

fn prepare_nonexistent_path(path: &Path) -> Result<PathBuf, BackupError> {
    let file_name = path.file_name().ok_or_else(|| {
        BackupError::Storage("destination must name a directory below a parent".to_owned())
    })?;
    if file_name == OsStr::new(".") || file_name == OsStr::new("..") {
        return Err(BackupError::UnsafePath(path.display().to_string()));
    }
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).map_err(|error| storage("create destination parent", &error))?;
    let parent = fs::canonicalize(parent)
        .map_err(|error| storage("canonicalize destination parent", &error))?;
    Ok(parent.join(file_name))
}

fn collect_layout(root: &Path, exclude_transient: bool) -> Result<WorkspaceLayout, BackupError> {
    let mut directories = Vec::new();
    let mut files = Vec::new();
    let walker = WalkDir::new(root).follow_links(false).into_iter();

    if exclude_transient {
        for entry in walker.filter_entry(|entry| !is_excluded(root, entry.path())) {
            collect_entry(root, entry, &mut directories, &mut files)?;
        }
    } else {
        for entry in walker {
            collect_entry(root, entry, &mut directories, &mut files)?;
        }
    }

    directories.sort();
    files.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(WorkspaceLayout { directories, files })
}

fn collect_entry(
    root: &Path,
    entry: Result<walkdir::DirEntry, walkdir::Error>,
    directories: &mut Vec<String>,
    files: &mut Vec<(String, PathBuf)>,
) -> Result<(), BackupError> {
    let entry = entry.map_err(|error| BackupError::Storage(format!("walk workspace: {error}")))?;
    if entry.file_type().is_symlink() {
        return Err(BackupError::SymbolicLink(
            entry.path().display().to_string(),
        ));
    }
    if entry.path() == root {
        return Ok(());
    }
    if entry.file_type().is_dir() {
        directories.push(portable_relative(root, entry.path())?);
    } else if entry.file_type().is_file() {
        files.push((portable_relative(root, entry.path())?, entry.into_path()));
    }
    Ok(())
}

fn create_directories(root: &Path, directories: &[String]) -> Result<(), BackupError> {
    for directory in directories {
        let path = root.join(relative_path(directory)?);
        fs::create_dir_all(&path)
            .map_err(|error| storage("create snapshot directory", &error))?;
    }
    Ok(())
}

fn compare_sets(actual: &BTreeSet<String>, expected: &BTreeSet<String>) -> Result<(), BackupError> {
    if actual == expected {
        return Ok(());
    }
    let path = actual
        .symmetric_difference(expected)
        .next()
        .cloned()
        .unwrap_or_else(|| PAYLOAD_DIRECTORY.to_owned());
    Err(BackupError::PayloadMismatch(path))
}

fn is_excluded(root: &Path, path: &Path) -> bool {
    let Ok(relative) = path.strip_prefix(root) else {
        return false;
    };
    let portable = relative.to_string_lossy().replace('\\', "/");
    EXCLUDED_PREFIXES
        .iter()
        .any(|prefix| portable == *prefix || portable.starts_with(&format!("{prefix}/")))
}

fn portable_relative(root: &Path, path: &Path) -> Result<String, BackupError> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| BackupError::UnsafePath(path.display().to_string()))?;
    let mut parts = Vec::new();
    for component in relative.components() {
        match component {
            Component::Normal(value) => {
                let value = value
                    .to_str()
                    .ok_or_else(|| BackupError::UnsafePath(relative.display().to_string()))?;
                parts.push(value);
            }
            Component::CurDir => {}
            _ => return Err(BackupError::UnsafePath(relative.display().to_string())),
        }
    }
    let portable = parts.join("/");
    BackupFile::new(&portable, 0, "0".repeat(64))?;
    Ok(portable)
}

fn relative_path(value: &str) -> Result<PathBuf, BackupError> {
    BackupFile::new(value, 0, "0".repeat(64))?;
    Ok(value.split('/').collect())
}

fn write_manifest(root: &Path, manifest: &BackupManifest) -> Result<(), BackupError> {
    let content = serde_json::to_vec_pretty(manifest)
        .map_err(|error| BackupError::Storage(format!("serialize backup manifest: {error}")))?;
    write_new(&root.join(MANIFEST_NAME), &content)
}

fn read_manifest(root: &Path) -> Result<BackupManifest, BackupError> {
    let path = root.join(MANIFEST_NAME);
    reject_symlink(&path)?;
    if !path.is_file() {
        return Err(BackupError::ManifestMissing(path.display().to_string()));
    }
    let content = fs::read(&path).map_err(|error| storage("read backup manifest", &error))?;
    serde_json::from_slice(&content)
        .map_err(|error| BackupError::Storage(format!("parse backup manifest: {error}")))
}

fn write_new(path: &Path, content: &[u8]) -> Result<(), BackupError> {
    let parent = path
        .parent()
        .ok_or_else(|| BackupError::Storage(format!("path has no parent: {}", path.display())))?;
    fs::create_dir_all(parent).map_err(|error| storage("create parent directory", &error))?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| storage("create file", &error))?;
    file.write_all(content)
        .map_err(|error| storage("write file", &error))?;
    file.sync_all()
        .map_err(|error| storage("sync file", &error))
}

fn copy_and_digest(source: &Path, destination: &Path) -> Result<(u64, String), BackupError> {
    reject_symlink(source)?;
    let parent = destination.parent().ok_or_else(|| {
        BackupError::Storage(format!("path has no parent: {}", destination.display()))
    })?;
    fs::create_dir_all(parent).map_err(|error| storage("create copy directory", &error))?;

    let mut input = File::open(source).map_err(|error| storage("open source file", &error))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(destination)
        .map_err(|error| storage("create copied file", &error))?;
    let mut hasher = Sha256::new();
    let mut total = 0_u64;
    let mut buffer = vec![0_u8; 64 * 1024];
    loop {
        let count = input
            .read(&mut buffer)
            .map_err(|error| storage("read source file", &error))?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
        output
            .write_all(&buffer[..count])
            .map_err(|error| storage("write copied file", &error))?;
        total = total
            .checked_add(u64::try_from(count).map_err(|error| {
                BackupError::Storage(format!("convert copy byte count: {error}"))
            })?)
            .ok_or_else(|| BackupError::Storage("copy byte count overflowed".to_owned()))?;
    }
    output
        .sync_all()
        .map_err(|error| storage("sync copied file", &error))?;
    Ok((total, hexadecimal(&hasher.finalize())))
}

fn digest_file(path: &Path) -> Result<(u64, String), BackupError> {
    reject_symlink(path)?;
    let mut input = File::open(path).map_err(|error| storage("open payload file", &error))?;
    let mut hasher = Sha256::new();
    let mut total = 0_u64;
    let mut buffer = vec![0_u8; 64 * 1024];
    loop {
        let count = input
            .read(&mut buffer)
            .map_err(|error| storage("read payload file", &error))?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
        total = total
            .checked_add(u64::try_from(count).map_err(|error| {
                BackupError::Storage(format!("convert digest byte count: {error}"))
            })?)
            .ok_or_else(|| BackupError::Storage("digest byte count overflowed".to_owned()))?;
    }
    Ok((total, hexadecimal(&hasher.finalize())))
}

fn hexadecimal(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut value = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        value.push(char::from(HEX[usize::from(byte >> 4)]));
        value.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    value
}

fn reject_symlink(path: &Path) -> Result<(), BackupError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(BackupError::SymbolicLink(
            path.display().to_string(),
        )),
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(storage("inspect path", &error)),
    }
}

fn storage(action: &str, error: &std::io::Error) -> BackupError {
    BackupError::Storage(format!("{action}: {error}"))
}

#[derive(Debug)]
struct StagingDirectory {
    path: PathBuf,
    committed: bool,
}

impl StagingDirectory {
    fn new(parent: &Path, label: &str) -> Result<Self, BackupError> {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| duration.as_nanos());
        for attempt in 0_u8..100 {
            let path = parent.join(format!(
                ".liaison-{label}-{}-{nonce:x}-{attempt}",
                std::process::id()
            ));
            match fs::create_dir(&path) {
                Ok(()) => {
                    return Ok(Self {
                        path,
                        committed: false,
                    });
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => return Err(storage("create staging directory", &error)),
            }
        }
        Err(BackupError::Storage(
            "could not allocate a unique staging directory".to_owned(),
        ))
    }

    fn path(&self) -> &Path {
        &self.path
    }

    fn commit_to(mut self, destination: &Path) -> Result<(), BackupError> {
        fs::rename(&self.path, destination)
            .map_err(|error| storage("publish staged directory", &error))?;
        self.committed = true;
        Ok(())
    }
}

impl Drop for StagingDirectory {
    fn drop(&mut self) {
        if !self.committed {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LocalWorkspaceBackup, PAYLOAD_DIRECTORY, RESTORE_MARKER};
    use liaison_workspace::{
        BackupError, BuildProfile, WorkspaceBackupStore, WorkspaceManifest, WorkspaceProfile,
    };
    use std::fs;

    #[test]
    fn creates_verifies_and_restores_an_isolated_snapshot() {
        let root = tempfile::tempdir()
            .unwrap_or_else(|error| unreachable!("could not create test directory: {error}"));
        let workspace = root.path().join("workspace");
        fs::create_dir_all(workspace.join(".liaison/projections"))
            .unwrap_or_else(|error| unreachable!("could not create workspace directories: {error}"));
        fs::create_dir_all(workspace.join("people"))
            .unwrap_or_else(|error| unreachable!("could not create people directory: {error}"));
        fs::create_dir_all(workspace.join("organisations"))
            .unwrap_or_else(|error| unreachable!("could not create empty directory fixture: {error}"));
        fs::write(workspace.join(".liaison/workspace.yaml"), b"workspace: fixture\n")
            .unwrap_or_else(|error| unreachable!("could not write manifest fixture: {error}"));
        fs::write(workspace.join("people/alex.md"), b"# Alex\n")
            .unwrap_or_else(|error| unreachable!("could not write person fixture: {error}"));
        fs::write(
            workspace.join(".liaison/projections/cache"),
            b"disposable\n",
        )
        .unwrap_or_else(|error| unreachable!("could not write projection fixture: {error}"));

        let workspace_manifest = WorkspaceManifest::new(
            "Fixture",
            WorkspaceProfile::Personal,
            BuildProfile::Airgap,
            "en-IE",
        )
        .unwrap_or_else(|error| unreachable!("could not create workspace manifest: {error}"));
        let adapter = LocalWorkspaceBackup::new();
        let backup = root.path().join("backup");
        let manifest = adapter
            .create_snapshot(&workspace, &backup, &workspace_manifest)
            .unwrap_or_else(|error| unreachable!("could not create backup: {error}"));
        assert_eq!(manifest.files.len(), 2);
        assert!(manifest.directories.contains(&"organisations".to_owned()));
        assert!(!backup
            .join(PAYLOAD_DIRECTORY)
            .join(".liaison/projections/cache")
            .exists());

        let verified = adapter
            .verify_snapshot(&backup)
            .unwrap_or_else(|error| unreachable!("could not verify backup: {error}"));
        assert_eq!(verified.files_checked, 2);

        let restored = root.path().join("restored");
        let report = adapter
            .stage_restore(&backup, &restored)
            .unwrap_or_else(|error| unreachable!("could not stage restore: {error}"));
        assert_eq!(report.files_restored, 2);
        assert!(restored.join("organisations").is_dir());
        assert!(restored.join(RESTORE_MARKER).is_file());
        adapter
            .finalize_restore(&restored)
            .unwrap_or_else(|error| unreachable!("could not finalize restore: {error}"));
        assert!(!restored.join(RESTORE_MARKER).exists());
        assert_eq!(
            fs::read(restored.join("people/alex.md"))
                .unwrap_or_else(|error| unreachable!("could not read restored person: {error}")),
            b"# Alex\n"
        );
    }

    #[test]
    fn rejects_tampered_payload_and_nested_destination() {
        let root = tempfile::tempdir()
            .unwrap_or_else(|error| unreachable!("could not create test directory: {error}"));
        let workspace = root.path().join("workspace");
        fs::create_dir_all(workspace.join(".liaison"))
            .unwrap_or_else(|error| unreachable!("could not create workspace directory: {error}"));
        fs::write(workspace.join(".liaison/workspace.yaml"), b"fixture\n")
            .unwrap_or_else(|error| unreachable!("could not write workspace fixture: {error}"));
        let workspace_manifest = WorkspaceManifest::new(
            "Fixture",
            WorkspaceProfile::Personal,
            BuildProfile::Airgap,
            "en-IE",
        )
        .unwrap_or_else(|error| unreachable!("could not create workspace manifest: {error}"));
        let adapter = LocalWorkspaceBackup::new();

        let nested = workspace.join("backup");
        assert!(matches!(
            adapter.create_snapshot(&workspace, &nested, &workspace_manifest),
            Err(BackupError::DestinationInsideWorkspace(_))
        ));

        let backup = root.path().join("backup");
        adapter
            .create_snapshot(&workspace, &backup, &workspace_manifest)
            .unwrap_or_else(|error| unreachable!("could not create backup: {error}"));
        fs::write(
            backup
                .join(PAYLOAD_DIRECTORY)
                .join(".liaison/workspace.yaml"),
            b"changed\n",
        )
        .unwrap_or_else(|error| unreachable!("could not tamper with backup: {error}"));
        assert!(matches!(
            adapter.verify_snapshot(&backup),
            Err(BackupError::ChecksumMismatch { .. })
        ));
    }
}
