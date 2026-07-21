//! Capability-bound recoverable operation storage for canonical workspace files.

use cap_fs_ext::{DirExt, FollowSymlinks, OpenOptionsFollowExt};
use cap_std::fs::{Dir, OpenOptions};
use liaison_shared_kernel::{OperationId, WorkspaceId};
use liaison_workspace::{
    CanonicalDigest, CanonicalOperationManifest, CanonicalOperationTarget, CanonicalPath,
    CanonicalPrecondition, CanonicalWrite, FaultPoint, OperationContext, OperationPhase,
    OperationReceipt, OperationRecoveryReport, RecoverableOperationError,
    RecoverableOperationErrorKind,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

const OPERATIONS_DIRECTORY: &str = ".liaison/operations";
const MANIFEST_FILE: &str = "manifest.yaml";
const COMMIT_FILE: &str = "COMMIT";
const COMPLETE_FILE: &str = "COMPLETE";
const STAGED_DIRECTORY: &str = "staged";
const PUBLISHED_DIRECTORY: &str = "published";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct CompleteRecord {
    operation_id: OperationId,
    completed_at: chrono::DateTime<chrono::Utc>,
    published_targets: u32,
}

pub(crate) fn execute(
    root: &Dir,
    workspace_id: WorkspaceId,
    context: OperationContext,
    writes: Vec<CanonicalWrite>,
) -> Result<OperationReceipt, RecoverableOperationError> {
    execute_with_fault(root, workspace_id, context, writes, FaultPoint::None)
}

// The staged commit runs as one sequential procedure — stage, flush, record
// the durable commit decision, publish, finish — that is clearer read top to
// bottom than split across helpers whose only caller is this function.
#[allow(clippy::too_many_lines)]
pub(crate) fn execute_with_fault(
    root: &Dir,
    workspace_id: WorkspaceId,
    context: OperationContext,
    writes: Vec<CanonicalWrite>,
    fault: FaultPoint,
) -> Result<OperationReceipt, RecoverableOperationError> {
    if writes.is_empty() {
        return Err(operation_error(
            RecoverableOperationErrorKind::Contract,
            "operation must contain at least one canonical write",
        )
        .with_operation(context.operation_id));
    }

    let operations = operations_directory(root)?;
    let operation_name = context.operation_id.to_string();
    create_directory(&operations, &operation_name).map_err(|error| {
        map_io("create operation directory", error).with_operation(context.operation_id)
    })?;
    let operation = operations
        .open_dir_nofollow(&operation_name)
        .map_err(|error| {
            map_io("open operation directory", error).with_operation(context.operation_id)
        })?;
    create_directory(&operation, STAGED_DIRECTORY).map_err(|error| {
        map_io("create staged directory", error).with_operation(context.operation_id)
    })?;
    create_directory(&operation, PUBLISHED_DIRECTORY).map_err(|error| {
        map_io("create progress directory", error).with_operation(context.operation_id)
    })?;
    let staged = operation
        .open_dir_nofollow(STAGED_DIRECTORY)
        .map_err(|error| {
            map_io("open staged directory", error).with_operation(context.operation_id)
        })?;

    let mut prepared = Vec::with_capacity(writes.len());
    for write in writes {
        let digest = digest_bytes(&write.content)?;
        let size_bytes = u64::try_from(write.content.len()).map_err(|_| {
            operation_error(
                RecoverableOperationErrorKind::Contract,
                "canonical write is too large",
            )
            .with_operation(context.operation_id)
            .with_path(write.path.clone())
        })?;
        prepared.push((write, digest, size_bytes));
    }
    prepared.sort_by(|left, right| left.0.path.cmp(&right.0.path));
    if prepared
        .windows(2)
        .any(|window| window[0].0.path == window[1].0.path)
    {
        return Err(operation_error(
            RecoverableOperationErrorKind::Contract,
            "operation contains a duplicate canonical target",
        )
        .with_operation(context.operation_id));
    }

    let targets = prepared
        .iter()
        .enumerate()
        .map(|(index, (write, digest, size_bytes))| {
            Ok(CanonicalOperationTarget {
                ordinal: u32::try_from(index).map_err(|_| {
                    operation_error(
                        RecoverableOperationErrorKind::Contract,
                        "operation contains too many targets",
                    )
                    .with_operation(context.operation_id)
                })?,
                path: write.path.clone(),
                content_digest: digest.clone(),
                size_bytes: *size_bytes,
                precondition: write.precondition.clone(),
            })
        })
        .collect::<Result<Vec<_>, RecoverableOperationError>>()?;
    let manifest = CanonicalOperationManifest::new(
        context.operation_id,
        workspace_id,
        context.started_at,
        targets,
    )
    .map_err(|error| {
        operation_error(RecoverableOperationErrorKind::Contract, error.to_string())
            .with_operation(context.operation_id)
    })?;

    for target in &manifest.targets {
        let write = prepared
            .iter()
            .find(|(write, _, _)| write.path == target.path)
            .ok_or_else(|| {
                operation_error(
                    RecoverableOperationErrorKind::Contract,
                    "operation target lost its staged content",
                )
                .with_operation(context.operation_id)
                .with_path(target.path.clone())
            })?;
        write_new_file(&staged, &staged_name(target.ordinal), &write.0.content).map_err(
            |error| {
                map_io("write staged target", error)
                    .with_operation(context.operation_id)
                    .with_path(target.path.clone())
            },
        )?;
    }
    sync_directory(&staged).map_err(|error| {
        map_io("flush staged directory", error).with_operation(context.operation_id)
    })?;

    let manifest_bytes = serde_yaml::to_string(&manifest)
        .map_err(|error| {
            operation_error(RecoverableOperationErrorKind::Storage, error.to_string())
                .with_operation(context.operation_id)
        })?
        .into_bytes();
    write_new_file(&operation, MANIFEST_FILE, &manifest_bytes).map_err(|error| {
        map_io("write operation manifest", error).with_operation(context.operation_id)
    })?;
    sync_directory(&operation).map_err(|error| {
        map_io("flush operation manifest", error).with_operation(context.operation_id)
    })?;

    if fault == FaultPoint::AfterStaging {
        return Err(injected(context.operation_id, "after staging"));
    }

    verify_all_preconditions(root, &manifest)?;
    write_new_file(&operation, COMMIT_FILE, b"COMMIT\n").map_err(|error| {
        map_io("write durable commit decision", error).with_operation(context.operation_id)
    })?;
    sync_directory(&operation).map_err(|error| {
        map_io("flush durable commit decision", error).with_operation(context.operation_id)
    })?;

    if fault == FaultPoint::AfterCommitDecision {
        return Err(injected(context.operation_id, "after commit decision"));
    }

    publish_manifest_targets(root, &operation, &manifest, fault)?;
    finish_operation(root, &operation, &manifest, context.started_at)
}

pub(crate) fn recover(
    root: &Dir,
    workspace_id: WorkspaceId,
) -> Result<OperationRecoveryReport, RecoverableOperationError> {
    let operations = operations_directory(root)?;
    let mut names = Vec::new();
    for entry in operations
        .entries()
        .map_err(|error| map_io("list operation directory", error))?
    {
        let entry = entry.map_err(|error| map_io("read operation entry", error))?;
        let metadata = entry
            .metadata()
            .map_err(|error| map_io("inspect operation entry", error))?;
        if metadata.is_dir() {
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    names.sort();

    let mut report = OperationRecoveryReport::default();
    for name in names {
        let operation = operations
            .open_dir_nofollow(&name)
            .map_err(|error| map_io("open pending operation", error))?;
        if !file_exists(&operation, MANIFEST_FILE)? {
            if file_exists(&operation, COMMIT_FILE)? {
                return Err(operation_error(
                    RecoverableOperationErrorKind::RecoveryConflict,
                    "committed operation is missing its manifest",
                ));
            }
            operations
                .remove_dir_all(&name)
                .map_err(|error| map_io("discard incomplete uncommitted operation", error))?;
            sync_directory(&operations)
                .map_err(|error| map_io("flush discarded incomplete operation", error))?;
            report.discarded_before_commit = report.discarded_before_commit.saturating_add(1);
            continue;
        }
        let manifest = read_manifest(&operation)?;
        if manifest.workspace_id != workspace_id {
            return Err(operation_error(
                RecoverableOperationErrorKind::RecoveryConflict,
                "operation belongs to a different workspace identity",
            )
            .with_operation(manifest.operation_id));
        }
        if file_exists(&operation, COMPLETE_FILE)? {
            report.already_complete = report.already_complete.saturating_add(1);
            cleanup_staged(&operation)?;
            continue;
        }
        if !file_exists(&operation, COMMIT_FILE)? {
            operations.remove_dir_all(&name).map_err(|error| {
                map_io("discard uncommitted operation", error).with_operation(manifest.operation_id)
            })?;
            sync_directory(&operations).map_err(|error| {
                map_io("flush discarded operation", error).with_operation(manifest.operation_id)
            })?;
            report.discarded_before_commit = report.discarded_before_commit.saturating_add(1);
            continue;
        }
        publish_manifest_targets(root, &operation, &manifest, FaultPoint::None)?;
        let _receipt = finish_operation(root, &operation, &manifest, manifest.started_at)?;
        report.rolled_forward = report.rolled_forward.saturating_add(1);
    }
    Ok(report)
}

fn publish_manifest_targets(
    root: &Dir,
    operation: &Dir,
    manifest: &CanonicalOperationManifest,
    fault: FaultPoint,
) -> Result<(), RecoverableOperationError> {
    let staged = operation
        .open_dir_nofollow(STAGED_DIRECTORY)
        .map_err(|error| {
            map_io("open staged directory", error).with_operation(manifest.operation_id)
        })?;
    let published = operation
        .open_dir_nofollow(PUBLISHED_DIRECTORY)
        .map_err(|error| {
            map_io("open progress directory", error).with_operation(manifest.operation_id)
        })?;

    let mut published_count = 0_u32;
    for target in &manifest.targets {
        let progress = progress_name(target.ordinal);
        if file_exists(&published, &progress)? {
            verify_target_digest(root, target, &target.content_digest)?;
            published_count = published_count.saturating_add(1);
            continue;
        }

        match target_digest(root, &target.path)? {
            Some(current) if current == target.content_digest => {
                write_new_file(&published, &progress, b"published\n").map_err(|error| {
                    map_io("record idempotent publication", error)
                        .with_operation(manifest.operation_id)
                        .with_path(target.path.clone())
                })?;
                sync_directory(&published).map_err(|error| {
                    map_io("flush idempotent publication", error)
                        .with_operation(manifest.operation_id)
                })?;
                published_count = published_count.saturating_add(1);
                continue;
            }
            _ => {}
        }

        verify_precondition(root, target)?;
        let bytes =
            read_file(&staged, Path::new(&staged_name(target.ordinal))).map_err(|error| {
                error
                    .with_operation(manifest.operation_id)
                    .with_path(target.path.clone())
            })?;
        if digest_bytes(&bytes)? != target.content_digest {
            return Err(operation_error(
                RecoverableOperationErrorKind::RecoveryConflict,
                "staged target digest does not match the operation manifest",
            )
            .with_operation(manifest.operation_id)
            .with_path(target.path.clone()));
        }
        publish_target(root, manifest.operation_id, target, &bytes)?;
        write_new_file(&published, &progress, b"published\n").map_err(|error| {
            map_io("record publication progress", error)
                .with_operation(manifest.operation_id)
                .with_path(target.path.clone())
        })?;
        sync_directory(&published).map_err(|error| {
            map_io("flush publication progress", error).with_operation(manifest.operation_id)
        })?;
        published_count = published_count.saturating_add(1);

        if fault == FaultPoint::AfterPublishedTargets(published_count) {
            return Err(injected(manifest.operation_id, "after publishing a target"));
        }
    }

    if fault == FaultPoint::BeforeComplete {
        return Err(injected(manifest.operation_id, "before completion"));
    }
    Ok(())
}

fn finish_operation(
    root: &Dir,
    operation: &Dir,
    manifest: &CanonicalOperationManifest,
    completed_at: chrono::DateTime<chrono::Utc>,
) -> Result<OperationReceipt, RecoverableOperationError> {
    if !file_exists(operation, COMPLETE_FILE)? {
        let record = CompleteRecord {
            operation_id: manifest.operation_id,
            completed_at,
            published_targets: u32::try_from(manifest.targets.len()).unwrap_or(u32::MAX),
        };
        let bytes = serde_yaml::to_string(&record)
            .map_err(|error| {
                operation_error(RecoverableOperationErrorKind::Storage, error.to_string())
                    .with_operation(manifest.operation_id)
            })?
            .into_bytes();
        write_new_file(operation, COMPLETE_FILE, &bytes).map_err(|error| {
            map_io("write operation completion", error).with_operation(manifest.operation_id)
        })?;
        sync_directory(operation).map_err(|error| {
            map_io("flush operation completion", error).with_operation(manifest.operation_id)
        })?;
    }
    mark_projection_stale(root, manifest.operation_id)?;
    cleanup_staged(operation)?;
    Ok(OperationReceipt {
        operation_id: manifest.operation_id,
        workspace_id: manifest.workspace_id,
        completed_at,
        published_targets: u32::try_from(manifest.targets.len()).unwrap_or(u32::MAX),
        phase: OperationPhase::Complete,
    })
}

fn cleanup_staged(operation: &Dir) -> Result<(), RecoverableOperationError> {
    match operation.remove_dir_all(STAGED_DIRECTORY) {
        Ok(()) => sync_directory(operation).map_err(|error| map_io("flush staged cleanup", error)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(map_io("remove staged content", error)),
    }
}

fn verify_all_preconditions(
    root: &Dir,
    manifest: &CanonicalOperationManifest,
) -> Result<(), RecoverableOperationError> {
    for target in &manifest.targets {
        verify_precondition(root, target)?;
    }
    Ok(())
}

fn verify_precondition(
    root: &Dir,
    target: &CanonicalOperationTarget,
) -> Result<(), RecoverableOperationError> {
    let current = target_digest(root, &target.path)?;
    match (&target.precondition, current) {
        (CanonicalPrecondition::Absent, None) => Ok(()),
        (CanonicalPrecondition::Absent, Some(_)) => Err(operation_error(
            RecoverableOperationErrorKind::Precondition,
            "canonical target was created after the operation began",
        )
        .with_path(target.path.clone())),
        (CanonicalPrecondition::ExactDigest { digest, .. }, Some(current))
            if digest == &current =>
        {
            Ok(())
        }
        (CanonicalPrecondition::ExactDigest { .. }, None) => Err(operation_error(
            RecoverableOperationErrorKind::Precondition,
            "canonical target disappeared after the operation began",
        )
        .with_path(target.path.clone())),
        (CanonicalPrecondition::ExactDigest { .. }, Some(_)) => Err(operation_error(
            RecoverableOperationErrorKind::Precondition,
            "canonical target changed after the operation began",
        )
        .with_path(target.path.clone())),
    }
}

fn verify_target_digest(
    root: &Dir,
    target: &CanonicalOperationTarget,
    expected: &CanonicalDigest,
) -> Result<(), RecoverableOperationError> {
    match target_digest(root, &target.path)? {
        Some(current) if &current == expected => Ok(()),
        _ => Err(operation_error(
            RecoverableOperationErrorKind::RecoveryConflict,
            "published target no longer matches the committed digest",
        )
        .with_path(target.path.clone())),
    }
}

fn publish_target(
    root: &Dir,
    operation_id: OperationId,
    target: &CanonicalOperationTarget,
    bytes: &[u8],
) -> Result<(), RecoverableOperationError> {
    let (parent_path, file_name) = split_target(&target.path)?;
    let parent = root.open_dir_nofollow(&parent_path).map_err(|error| {
        map_io("open canonical target directory", error)
            .with_operation(operation_id)
            .with_path(target.path.clone())
    })?;
    let temporary = format!(".liaison-op-{operation_id}-{}", target.ordinal);
    write_new_file(&parent, &temporary, bytes).map_err(|error| {
        map_io("write canonical publication candidate", error)
            .with_operation(operation_id)
            .with_path(target.path.clone())
    })?;

    let publish_result = match &target.precondition {
        CanonicalPrecondition::Absent => parent.hard_link(&temporary, &parent, &file_name),
        CanonicalPrecondition::ExactDigest { .. } => {
            verify_precondition(root, target)?;
            parent.rename(&temporary, &parent, &file_name)
        }
    };
    if let Err(error) = publish_result {
        let _ = parent.remove_file(&temporary);
        return Err(map_io("publish canonical target", error)
            .with_operation(operation_id)
            .with_path(target.path.clone()));
    }
    let _ = parent.remove_file(&temporary);
    sync_directory(&parent).map_err(|error| {
        map_io("flush canonical target directory", error)
            .with_operation(operation_id)
            .with_path(target.path.clone())
    })?;
    verify_target_digest(root, target, &target.content_digest)
}

fn mark_projection_stale(
    root: &Dir,
    operation_id: OperationId,
) -> Result<(), RecoverableOperationError> {
    let projections = root
        .open_dir_nofollow(".liaison/projections")
        .map_err(|error| map_io("open projection directory", error).with_operation(operation_id))?;
    // A pre-existing regular marker already proves every projection is stale.
    // Retaining it avoids a platform-dependent replacement and any interval in
    // which stale evidence disappears between committed operations.
    if file_exists(&projections, "stale")? {
        return Ok(());
    }
    let bytes = format!("operation_id: {operation_id}\n").into_bytes();
    let temporary = format!(".stale-{operation_id}.tmp");
    write_new_file(&projections, &temporary, &bytes).map_err(|error| {
        map_io("write projection stale marker", error).with_operation(operation_id)
    })?;
    match projections.rename(&temporary, &projections, "stale") {
        Ok(()) => {}
        Err(error) => {
            let _ = projections.remove_file(&temporary);
            return Err(
                map_io("publish projection stale marker", error).with_operation(operation_id)
            );
        }
    }
    sync_directory(&projections).map_err(|error| {
        map_io("flush projection stale marker", error).with_operation(operation_id)
    })?;
    Ok(())
}

fn target_digest(
    root: &Dir,
    path: &CanonicalPath,
) -> Result<Option<CanonicalDigest>, RecoverableOperationError> {
    let path = Path::new(path.as_str());
    match root.symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
            Err(operation_error(
                RecoverableOperationErrorKind::RecoveryConflict,
                "canonical target is not a regular file",
            )
            .with_path(
                CanonicalPath::parse(path.to_string_lossy().into_owned()).map_err(|error| {
                    operation_error(RecoverableOperationErrorKind::Contract, error.to_string())
                })?,
            ))
        }
        Ok(_) => {
            let bytes = read_file(root, path)?;
            Ok(Some(digest_bytes(&bytes)?))
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => {
            Err(map_io("inspect canonical target", error).with_path(path_to_canonical(path)?))
        }
    }
}

fn split_target(path: &CanonicalPath) -> Result<(PathBuf, String), RecoverableOperationError> {
    let path = Path::new(path.as_str());
    let parent = path.parent().ok_or_else(|| {
        operation_error(
            RecoverableOperationErrorKind::Contract,
            "canonical target has no parent directory",
        )
    })?;
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            operation_error(
                RecoverableOperationErrorKind::Contract,
                "canonical target has no portable file name",
            )
        })?
        .to_owned();
    Ok((parent.to_path_buf(), file_name))
}

fn path_to_canonical(path: &Path) -> Result<CanonicalPath, RecoverableOperationError> {
    CanonicalPath::parse(path.to_string_lossy().replace('\\', "/")).map_err(|error| {
        operation_error(RecoverableOperationErrorKind::Contract, error.to_string())
    })
}

fn read_manifest(operation: &Dir) -> Result<CanonicalOperationManifest, RecoverableOperationError> {
    let bytes = read_file(operation, Path::new(MANIFEST_FILE))?;
    let manifest: CanonicalOperationManifest = serde_yaml::from_slice(&bytes).map_err(|error| {
        operation_error(
            RecoverableOperationErrorKind::RecoveryConflict,
            error.to_string(),
        )
    })?;
    manifest.validate().map_err(|error| {
        operation_error(
            RecoverableOperationErrorKind::RecoveryConflict,
            error.to_string(),
        )
        .with_operation(manifest.operation_id)
    })?;
    Ok(manifest)
}

fn operations_directory(root: &Dir) -> Result<Dir, RecoverableOperationError> {
    root.open_dir_nofollow(OPERATIONS_DIRECTORY)
        .map_err(|error| map_io("open operations directory", error))
}

fn create_directory(directory: &Dir, name: &str) -> io::Result<()> {
    directory.create_dir(name)
}

fn write_new_file(directory: &Dir, name: &str, bytes: &[u8]) -> io::Result<()> {
    let mut options = OpenOptions::new();
    options
        .write(true)
        .create_new(true)
        .follow(FollowSymlinks::No);
    let mut file = directory.open_with(name, &options)?;
    file.write_all(bytes)?;
    file.sync_all()
}

fn read_file(directory: &Dir, path: &Path) -> Result<Vec<u8>, RecoverableOperationError> {
    let metadata = directory
        .symlink_metadata(path)
        .map_err(|error| map_io("inspect operation file", error))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(operation_error(
            RecoverableOperationErrorKind::RecoveryConflict,
            "operation file is not a regular file",
        ));
    }
    let mut file = directory
        .open(path)
        .map_err(|error| map_io("open operation file", error))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|error| map_io("read operation file", error))?;
    Ok(bytes)
}

fn file_exists(directory: &Dir, path: &str) -> Result<bool, RecoverableOperationError> {
    match directory.symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
            Err(operation_error(
                RecoverableOperationErrorKind::RecoveryConflict,
                "operation marker is not a regular file",
            ))
        }
        Ok(_) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(map_io("inspect operation marker", error)),
    }
}

fn sync_directory(directory: &Dir) -> io::Result<()> {
    let file = directory.open(".")?;
    file.sync_all()
}

pub(crate) fn digest_bytes(bytes: &[u8]) -> Result<CanonicalDigest, RecoverableOperationError> {
    let digest = Sha256::digest(bytes);
    CanonicalDigest::parse(format!("{digest:x}")).map_err(|error| {
        operation_error(RecoverableOperationErrorKind::Contract, error.to_string())
    })
}

fn staged_name(ordinal: u32) -> String {
    format!("{ordinal:08}.bin")
}

fn progress_name(ordinal: u32) -> String {
    format!("{ordinal:08}.published")
}

// Takes the io::Error by value so callers can pass it straight out of a
// `map_err` closure; it is only formatted, never retained.
#[allow(clippy::needless_pass_by_value)]
fn map_io(action: &str, error: io::Error) -> RecoverableOperationError {
    operation_error(
        RecoverableOperationErrorKind::Storage,
        format!("{action}: {error}"),
    )
}

fn operation_error(
    kind: RecoverableOperationErrorKind,
    message: impl Into<String>,
) -> RecoverableOperationError {
    RecoverableOperationError::new(kind, message)
}

fn injected(operation_id: OperationId, phase: &str) -> RecoverableOperationError {
    operation_error(
        RecoverableOperationErrorKind::FaultInjected,
        format!("fault injected {phase}"),
    )
    .with_operation(operation_id)
}

#[cfg(test)]
mod tests {
    use super::{execute_with_fault, recover, target_digest};
    use cap_std::{ambient_authority, fs::Dir};
    use chrono::{DateTime, Utc};
    use liaison_shared_kernel::{OperationId, WorkspaceId};
    use liaison_workspace::{
        CanonicalPath, CanonicalPrecondition, CanonicalWrite, FaultPoint, OperationContext,
        RecoverableOperationErrorKind,
    };
    use std::fs;
    use tempfile::tempdir;
    use uuid::Uuid;

    fn setup() -> Result<(tempfile::TempDir, Dir), Box<dyn std::error::Error>> {
        let temporary = tempdir()?;
        fs::create_dir_all(temporary.path().join(".liaison/operations"))?;
        fs::create_dir_all(temporary.path().join(".liaison/projections"))?;
        fs::create_dir_all(temporary.path().join("people"))?;
        let directory = Dir::open_ambient_dir(temporary.path(), ambient_authority())?;
        Ok((temporary, directory))
    }

    fn context(value: u128) -> OperationContext {
        OperationContext::new(
            OperationId::from_uuid(Uuid::from_u128(value)),
            DateTime::<Utc>::UNIX_EPOCH,
        )
    }

    fn workspace_id() -> WorkspaceId {
        WorkspaceId::from_uuid(Uuid::from_u128(9))
    }

    fn write(path: &str, content: &[u8]) -> CanonicalWrite {
        CanonicalWrite::new(
            CanonicalPath::parse(path)
                .unwrap_or_else(|error| unreachable!("test path must be valid: {error}")),
            content.to_vec(),
            CanonicalPrecondition::Absent,
        )
        .unwrap_or_else(|error| unreachable!("test write must be valid: {error}"))
    }

    #[test]
    fn uncommitted_staging_is_discarded() -> Result<(), Box<dyn std::error::Error>> {
        let (temporary, root) = setup()?;
        let result = execute_with_fault(
            &root,
            workspace_id(),
            context(1),
            vec![write("people/a.md", b"alpha")],
            FaultPoint::AfterStaging,
        );
        assert!(matches!(
            result,
            Err(error) if error.kind == RecoverableOperationErrorKind::FaultInjected
        ));
        let report = recover(&root, workspace_id())?;
        assert_eq!(report.discarded_before_commit, 1);
        assert!(!temporary.path().join("people/a.md").exists());
        Ok(())
    }

    #[test]
    fn committed_operation_rolls_forward_after_partial_publication()
    -> Result<(), Box<dyn std::error::Error>> {
        let (temporary, root) = setup()?;
        let result = execute_with_fault(
            &root,
            workspace_id(),
            context(2),
            vec![
                write("people/a.md", b"alpha"),
                write("people/b.md", b"bravo"),
            ],
            FaultPoint::AfterPublishedTargets(1),
        );
        assert!(result.is_err());
        assert!(temporary.path().join("people/a.md").exists());
        assert!(!temporary.path().join("people/b.md").exists());
        let report = recover(&root, workspace_id())?;
        assert_eq!(report.rolled_forward, 1);
        assert_eq!(fs::read(temporary.path().join("people/a.md"))?, b"alpha");
        assert_eq!(fs::read(temporary.path().join("people/b.md"))?, b"bravo");
        Ok(())
    }

    #[test]
    fn recovery_refuses_an_external_edit() -> Result<(), Box<dyn std::error::Error>> {
        let (temporary, root) = setup()?;
        let result = execute_with_fault(
            &root,
            workspace_id(),
            context(3),
            vec![write("people/a.md", b"committed")],
            FaultPoint::AfterCommitDecision,
        );
        assert!(result.is_err());
        fs::write(temporary.path().join("people/a.md"), b"external")?;
        let recovery = recover(&root, workspace_id());
        assert!(matches!(
            recovery,
            Err(error) if error.kind == RecoverableOperationErrorKind::Precondition
        ));
        assert_eq!(fs::read(temporary.path().join("people/a.md"))?, b"external");
        Ok(())
    }

    #[test]
    fn target_digest_reports_published_bytes() -> Result<(), Box<dyn std::error::Error>> {
        let (_temporary, root) = setup()?;
        let receipt = execute_with_fault(
            &root,
            workspace_id(),
            context(4),
            vec![write("people/a.md", b"alpha")],
            FaultPoint::None,
        )?;
        assert_eq!(receipt.published_targets, 1);
        let path = CanonicalPath::parse("people/a.md")?;
        assert!(target_digest(&root, &path)?.is_some());
        Ok(())
    }
}
