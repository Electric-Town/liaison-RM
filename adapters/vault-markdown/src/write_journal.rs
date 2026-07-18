//! Durable, workspace-scoped write journal for canonical vault records.
//!
//! The journal is an infrastructure mechanism. Domain contexts see repository
//! errors and never depend on filesystem transaction details.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Component, Path, PathBuf};
use uuid::Uuid;

const JOURNAL_DIRECTORY: &str = ".liaison/journal";
const JOURNAL_FORMAT: &str = "liaison-write-journal";
const JOURNAL_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum WriteMode {
    Create,
    Replace,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct JournalRecord {
    format: String,
    schema_version: u32,
    operation_id: String,
    mode: WriteMode,
    target: String,
    stage: String,
    backup: Option<String>,
    new_sha256: String,
    previous_sha256: Option<String>,
}

#[derive(Clone, Debug)]
struct JournalPaths {
    journal: PathBuf,
    stage: PathBuf,
    backup: PathBuf,
}

pub(crate) fn transactional_create(
    workspace: &Path,
    target: &Path,
    bytes: &[u8],
) -> io::Result<()> {
    validate_target(workspace, target)?;
    if target.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "target already exists",
        ));
    }

    let operation_id = Uuid::now_v7().to_string();
    let paths = journal_paths(workspace, &operation_id)?;
    let target_relative = portable_relative(workspace, target)?;
    let record = JournalRecord {
        format: JOURNAL_FORMAT.to_owned(),
        schema_version: JOURNAL_SCHEMA_VERSION,
        operation_id,
        mode: WriteMode::Create,
        target: target_relative,
        stage: portable_relative(workspace, &paths.stage)?,
        backup: None,
        new_sha256: sha256(bytes),
        previous_sha256: None,
    };

    write_stage(&paths.stage, bytes)?;
    write_journal(&paths.journal, &record)?;

    let result = publish_create(target, &paths.stage, &record.new_sha256);
    if result.is_ok() {
        remove_file_if_present(&paths.journal)?;
        sync_directory(paths.journal.parent())?;
    }
    result
}

pub(crate) fn transactional_replace(
    workspace: &Path,
    target: &Path,
    bytes: &[u8],
) -> io::Result<()> {
    validate_target(workspace, target)?;
    if !target.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "replacement target does not exist",
        ));
    }

    let previous_sha256 = sha256_file(target)?;
    let operation_id = Uuid::now_v7().to_string();
    let paths = journal_paths(workspace, &operation_id)?;
    let record = JournalRecord {
        format: JOURNAL_FORMAT.to_owned(),
        schema_version: JOURNAL_SCHEMA_VERSION,
        operation_id,
        mode: WriteMode::Replace,
        target: portable_relative(workspace, target)?,
        stage: portable_relative(workspace, &paths.stage)?,
        backup: Some(portable_relative(workspace, &paths.backup)?),
        new_sha256: sha256(bytes),
        previous_sha256: Some(previous_sha256),
    };

    write_stage(&paths.stage, bytes)?;
    write_journal(&paths.journal, &record)?;

    let result = publish_replace(target, &paths, &record);
    if result.is_ok() {
        remove_file_if_present(&paths.journal)?;
        sync_directory(paths.journal.parent())?;
    }
    result
}

pub(crate) fn recover_pending_writes(workspace: &Path) -> io::Result<usize> {
    let journal_directory = workspace.join(JOURNAL_DIRECTORY);
    if !journal_directory.exists() {
        return Ok(0);
    }
    reject_symlink(&journal_directory)?;

    let mut journals = fs::read_dir(&journal_directory)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension() == Some(OsStr::new("yaml")))
        .collect::<Vec<_>>();
    journals.sort();

    let mut recovered = 0;
    for journal in journals {
        let record = read_journal(&journal)?;
        validate_record(&record)?;
        let target = resolve_record_path(workspace, &record.target)?;
        let stage = resolve_record_path(workspace, &record.stage)?;
        let backup = record
            .backup
            .as_deref()
            .map(|value| resolve_record_path(workspace, value))
            .transpose()?;

        validate_target(workspace, &target)?;
        reject_symlink_if_present(&stage)?;
        if let Some(backup) = &backup {
            reject_symlink_if_present(backup)?;
        }

        match record.mode {
            WriteMode::Create => recover_create(&target, &stage, &record)?,
            WriteMode::Replace => {
                let backup = backup.ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "replace journal is missing backup path",
                    )
                })?;
                recover_replace(&target, &stage, &backup, &record)?;
            }
        }
        remove_file_if_present(&journal)?;
        sync_directory(journal.parent())?;
        recovered += 1;
    }
    Ok(recovered)
}

fn publish_create(target: &Path, stage: &Path, expected_digest: &str) -> io::Result<()> {
    verify_file_digest(stage, expected_digest)?;
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
        reject_symlink(parent)?;
    }
    fs::hard_link(stage, target).map_err(|error| {
        if error.kind() == io::ErrorKind::AlreadyExists {
            io::Error::new(io::ErrorKind::AlreadyExists, "target already exists")
        } else {
            error
        }
    })?;
    sync_file(target)?;
    remove_file_if_present(stage)?;
    sync_directory(target.parent())
}

fn publish_replace(
    target: &Path,
    paths: &JournalPaths,
    record: &JournalRecord,
) -> io::Result<()> {
    verify_file_digest(&paths.stage, &record.new_sha256)?;
    let previous = record.previous_sha256.as_deref().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "replace journal is missing previous digest",
        )
    })?;
    verify_file_digest(target, previous)?;

    fs::rename(target, &paths.backup)?;
    sync_directory(target.parent())?;
    if let Err(error) = fs::rename(&paths.stage, target) {
        let _ = fs::rename(&paths.backup, target);
        return Err(error);
    }
    sync_file(target)?;
    sync_directory(target.parent())?;
    remove_file_if_present(&paths.backup)?;
    sync_directory(target.parent())
}

fn recover_create(target: &Path, stage: &Path, record: &JournalRecord) -> io::Result<()> {
    if target.exists() {
        verify_file_digest(target, &record.new_sha256)?;
        remove_file_if_present(stage)?;
        return Ok(());
    }
    if !stage.exists() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "create journal has neither committed target nor staged content",
        ));
    }
    publish_create(target, stage, &record.new_sha256)
}

fn recover_replace(
    target: &Path,
    stage: &Path,
    backup: &Path,
    record: &JournalRecord,
) -> io::Result<()> {
    let previous = record.previous_sha256.as_deref().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "replace journal is missing previous digest",
        )
    })?;

    if target.exists() {
        let target_digest = sha256_file(target)?;
        if target_digest == record.new_sha256 {
            remove_file_if_present(stage)?;
            if backup.exists() {
                verify_file_digest(backup, previous)?;
                remove_file_if_present(backup)?;
            }
            sync_directory(target.parent())?;
            return Ok(());
        }
        if target_digest != previous {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "target digest matches neither journal revision",
            ));
        }
    }

    if stage.exists() {
        verify_file_digest(stage, &record.new_sha256)?;
        if target.exists() && !backup.exists() {
            fs::rename(target, backup)?;
            sync_directory(target.parent())?;
        }
        if !target.exists() {
            fs::rename(stage, target)?;
            sync_file(target)?;
            sync_directory(target.parent())?;
        }
        if backup.exists() {
            verify_file_digest(backup, previous)?;
            remove_file_if_present(backup)?;
            sync_directory(target.parent())?;
        }
        return Ok(());
    }

    if !target.exists() && backup.exists() {
        verify_file_digest(backup, previous)?;
        fs::rename(backup, target)?;
        sync_file(target)?;
        sync_directory(target.parent())?;
        return Ok(());
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "replace journal cannot be completed or rolled back safely",
    ))
}

fn journal_paths(workspace: &Path, operation_id: &str) -> io::Result<JournalPaths> {
    let directory = workspace.join(JOURNAL_DIRECTORY);
    fs::create_dir_all(&directory)?;
    reject_symlink(&directory)?;
    Ok(JournalPaths {
        journal: directory.join(format!("{operation_id}.yaml")),
        stage: directory.join(format!("{operation_id}.stage")),
        backup: directory.join(format!("{operation_id}.backup")),
    })
}

fn write_stage(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    sync_directory(path.parent())
}

fn write_journal(path: &Path, record: &JournalRecord) -> io::Result<()> {
    let temporary = path.with_extension("yaml.new");
    let bytes = serde_yaml::to_string(record)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)?;
    file.write_all(bytes.as_bytes())?;
    file.sync_all()?;
    fs::rename(&temporary, path)?;
    sync_directory(path.parent())
}

fn read_journal(path: &Path) -> io::Result<JournalRecord> {
    reject_symlink(path)?;
    let content = fs::read_to_string(path)?;
    serde_yaml::from_str(&content)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

fn validate_record(record: &JournalRecord) -> io::Result<()> {
    if record.format != JOURNAL_FORMAT || record.schema_version != JOURNAL_SCHEMA_VERSION {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "unsupported write journal format or schema",
        ));
    }
    if Uuid::parse_str(&record.operation_id).is_err() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "write journal operation ID is invalid",
        ));
    }
    Ok(())
}

fn validate_target(workspace: &Path, target: &Path) -> io::Result<()> {
    if !target.is_absolute() && workspace.is_absolute() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "target path must use the workspace path form",
        ));
    }
    let relative = target.strip_prefix(workspace).map_err(|_| {
        io::Error::new(
            io::ErrorKind::PermissionDenied,
            "target is outside the workspace",
        )
    })?;
    validate_relative(relative)?;
    reject_symlink_if_present(workspace)?;

    let mut current = workspace.to_path_buf();
    for component in relative.components() {
        current.push(component.as_os_str());
        reject_symlink_if_present(&current)?;
    }
    Ok(())
}

fn portable_relative(workspace: &Path, path: &Path) -> io::Result<String> {
    let relative = path.strip_prefix(workspace).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidInput, "path is outside workspace")
    })?;
    validate_relative(relative)?;
    Ok(relative.to_string_lossy().replace('\\', "/"))
}

fn resolve_record_path(workspace: &Path, value: &str) -> io::Result<PathBuf> {
    if value.contains('\\') || value.starts_with('/') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "journal path is not a portable relative path",
        ));
    }
    let relative = Path::new(value);
    validate_relative(relative)?;
    Ok(workspace.join(relative))
}

fn validate_relative(path: &Path) -> io::Result<()> {
    if path.as_os_str().is_empty()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "path contains an unsafe component",
        ));
    }
    Ok(())
}

fn reject_symlink(path: &Path) -> io::Result<()> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!("symbolic links are not permitted: {}", path.display()),
        ))
    } else {
        Ok(())
    }
}

fn reject_symlink_if_present(path: &Path) -> io::Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!("symbolic links are not permitted: {}", path.display()),
        )),
        Ok(_) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn sha256_file(path: &Path) -> io::Result<String> {
    reject_symlink(path)?;
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 8192];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{hasher:x}"))
}

fn verify_file_digest(path: &Path, expected: &str) -> io::Result<()> {
    let actual = sha256_file(path)?;
    if actual == expected {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "write journal checksum mismatch for {}: expected {expected}, found {actual}",
                path.display()
            ),
        ))
    }
}

fn sync_file(path: &Path) -> io::Result<()> {
    File::open(path)?.sync_all()
}

#[cfg(unix)]
fn sync_directory(directory: Option<&Path>) -> io::Result<()> {
    if let Some(directory) = directory {
        File::open(directory)?.sync_all()?;
    }
    Ok(())
}

#[cfg(not(unix))]
fn sync_directory(_directory: Option<&Path>) -> io::Result<()> {
    // Windows does not provide a portable stable-Rust directory fsync API.
    // File contents are flushed and recovery remains journal-driven.
    Ok(())
}

fn remove_file_if_present(path: &Path) -> io::Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        journal_paths, portable_relative, recover_pending_writes, sha256, write_journal,
        write_stage, JournalRecord, WriteMode, JOURNAL_FORMAT, JOURNAL_SCHEMA_VERSION,
    };
    use std::fs;
    use tempfile::tempdir;
    use uuid::Uuid;

    #[test]
    fn recovers_staged_create_after_interruption() {
        let directory = tempdir().unwrap_or_else(|error| panic!("tempdir failed: {error}"));
        let root = directory.path();
        fs::create_dir_all(root.join(".liaison/journal"))
            .unwrap_or_else(|error| panic!("journal directory failed: {error}"));
        fs::create_dir_all(root.join("people"))
            .unwrap_or_else(|error| panic!("people directory failed: {error}"));

        let operation_id = Uuid::now_v7().to_string();
        let paths = journal_paths(root, &operation_id)
            .unwrap_or_else(|error| panic!("journal paths failed: {error}"));
        let target = root.join("people/alex.md");
        let bytes = b"new profile";
        write_stage(&paths.stage, bytes)
            .unwrap_or_else(|error| panic!("stage write failed: {error}"));
        let record = JournalRecord {
            format: JOURNAL_FORMAT.to_owned(),
            schema_version: JOURNAL_SCHEMA_VERSION,
            operation_id,
            mode: WriteMode::Create,
            target: portable_relative(root, &target)
                .unwrap_or_else(|error| panic!("target path failed: {error}")),
            stage: portable_relative(root, &paths.stage)
                .unwrap_or_else(|error| panic!("stage path failed: {error}")),
            backup: None,
            new_sha256: sha256(bytes),
            previous_sha256: None,
        };
        write_journal(&paths.journal, &record)
            .unwrap_or_else(|error| panic!("journal write failed: {error}"));

        assert_eq!(recover_pending_writes(root), Ok(1));
        assert_eq!(fs::read(&target), Ok(bytes.to_vec()));
        assert!(!paths.stage.exists());
        assert!(!paths.journal.exists());
    }

    #[test]
    fn rolls_back_replace_when_only_backup_remains() {
        let directory = tempdir().unwrap_or_else(|error| panic!("tempdir failed: {error}"));
        let root = directory.path();
        fs::create_dir_all(root.join(".liaison/journal"))
            .unwrap_or_else(|error| panic!("journal directory failed: {error}"));
        fs::create_dir_all(root.join("people"))
            .unwrap_or_else(|error| panic!("people directory failed: {error}"));

        let operation_id = Uuid::now_v7().to_string();
        let paths = journal_paths(root, &operation_id)
            .unwrap_or_else(|error| panic!("journal paths failed: {error}"));
        let target = root.join("people/alex.md");
        let previous = b"previous profile";
        fs::write(&paths.backup, previous)
            .unwrap_or_else(|error| panic!("backup write failed: {error}"));
        let record = JournalRecord {
            format: JOURNAL_FORMAT.to_owned(),
            schema_version: JOURNAL_SCHEMA_VERSION,
            operation_id,
            mode: WriteMode::Replace,
            target: portable_relative(root, &target)
                .unwrap_or_else(|error| panic!("target path failed: {error}")),
            stage: portable_relative(root, &paths.stage)
                .unwrap_or_else(|error| panic!("stage path failed: {error}")),
            backup: Some(
                portable_relative(root, &paths.backup)
                    .unwrap_or_else(|error| panic!("backup path failed: {error}")),
            ),
            new_sha256: sha256(b"new profile"),
            previous_sha256: Some(sha256(previous)),
        };
        write_journal(&paths.journal, &record)
            .unwrap_or_else(|error| panic!("journal write failed: {error}"));

        assert_eq!(recover_pending_writes(root), Ok(1));
        assert_eq!(fs::read(&target), Ok(previous.to_vec()));
        assert!(!paths.backup.exists());
        assert!(!paths.journal.exists());
    }

    #[test]
    fn finalises_committed_replace_and_removes_backup() {
        let directory = tempdir().unwrap_or_else(|error| panic!("tempdir failed: {error}"));
        let root = directory.path();
        fs::create_dir_all(root.join(".liaison/journal"))
            .unwrap_or_else(|error| panic!("journal directory failed: {error}"));
        fs::create_dir_all(root.join("people"))
            .unwrap_or_else(|error| panic!("people directory failed: {error}"));

        let operation_id = Uuid::now_v7().to_string();
        let paths = journal_paths(root, &operation_id)
            .unwrap_or_else(|error| panic!("journal paths failed: {error}"));
        let target = root.join("people/alex.md");
        let previous = b"previous profile";
        let next = b"new profile";
        fs::write(&target, next)
            .unwrap_or_else(|error| panic!("target write failed: {error}"));
        fs::write(&paths.backup, previous)
            .unwrap_or_else(|error| panic!("backup write failed: {error}"));
        let record = JournalRecord {
            format: JOURNAL_FORMAT.to_owned(),
            schema_version: JOURNAL_SCHEMA_VERSION,
            operation_id,
            mode: WriteMode::Replace,
            target: portable_relative(root, &target)
                .unwrap_or_else(|error| panic!("target path failed: {error}")),
            stage: portable_relative(root, &paths.stage)
                .unwrap_or_else(|error| panic!("stage path failed: {error}")),
            backup: Some(
                portable_relative(root, &paths.backup)
                    .unwrap_or_else(|error| panic!("backup path failed: {error}")),
            ),
            new_sha256: sha256(next),
            previous_sha256: Some(sha256(previous)),
        };
        write_journal(&paths.journal, &record)
            .unwrap_or_else(|error| panic!("journal write failed: {error}"));

        assert_eq!(recover_pending_writes(root), Ok(1));
        assert_eq!(fs::read(&target), Ok(next.to_vec()));
        assert!(!paths.backup.exists());
        assert!(!paths.journal.exists());
    }
}
