//! Local filesystem reference implementation of `object-store@1`.
//!
//! The adapter demonstrates immutable object publication, digest verification,
//! prefix listing, guarded deletion, and process-local serialisation of manifest
//! compare-and-set operations. It advertises backup and single-writer use, not
//! general multi-writer synchronisation.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use liaison_connections::{
    ListPage, ObjectKey, ObjectMetadata, ObjectStore, ObjectStoreError, PutOutcome, Sha256Digest,
};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex, MutexGuard},
};
use tempfile::NamedTempFile;
use walkdir::WalkDir;

const DEFAULT_MAXIMUM_OBJECT_BYTES: u64 = 512 * 1024 * 1024;
const MAXIMUM_LIST_LIMIT: u32 = 10_000;

#[derive(Debug, Clone)]
pub struct LocalObjectStore {
    root: PathBuf,
    maximum_object_bytes: u64,
    manifest_lock: Arc<Mutex<()>>,
}

impl LocalObjectStore {
    pub fn new(root: impl AsRef<Path>) -> Result<Self, ObjectStoreError> {
        let root = root.as_ref();
        if root.exists() {
            reject_symlink(root)?;
        } else {
            fs::create_dir_all(root).map_err(storage)?;
        }
        let root = root.canonicalize().map_err(storage)?;
        reject_symlink(&root)?;
        Ok(Self {
            root,
            maximum_object_bytes: DEFAULT_MAXIMUM_OBJECT_BYTES,
            manifest_lock: Arc::new(Mutex::new(())),
        })
    }

    pub fn with_maximum_object_bytes(mut self, maximum_object_bytes: u64) -> Self {
        self.maximum_object_bytes = maximum_object_bytes;
        self
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    fn lock_manifests(&self) -> Result<MutexGuard<'_, ()>, ObjectStoreError> {
        self.manifest_lock
            .lock()
            .map_err(|_| ObjectStoreError::Storage("manifest lock was poisoned".to_owned()))
    }

    fn resolve(&self, key: &ObjectKey) -> Result<PathBuf, ObjectStoreError> {
        let mut path = self.root.clone();
        for segment in key.as_str().split('/') {
            path.push(segment);
            if path.exists() {
                reject_symlink(&path)?;
            }
        }
        if !path.starts_with(&self.root) {
            return Err(ObjectStoreError::InvalidKey(key.to_string()));
        }
        Ok(path)
    }

    fn ensure_parent(&self, key: &ObjectKey) -> Result<PathBuf, ObjectStoreError> {
        let path = self.resolve(key)?;
        let parent = path
            .parent()
            .ok_or_else(|| ObjectStoreError::InvalidKey(key.to_string()))?;
        let relative = parent
            .strip_prefix(&self.root)
            .map_err(|error| ObjectStoreError::Storage(error.to_string()))?;
        let mut current = self.root.clone();
        for segment in relative.components() {
            current.push(segment.as_os_str());
            if current.exists() {
                reject_symlink(&current)?;
            } else {
                fs::create_dir(&current).map_err(storage)?;
            }
        }
        Ok(path)
    }

    fn read_verified(&self, key: &ObjectKey) -> Result<(Vec<u8>, ObjectMetadata), ObjectStoreError> {
        let path = self.resolve(key)?;
        if !path.is_file() {
            return Err(ObjectStoreError::NotFound(key.clone()));
        }
        reject_symlink(&path)?;
        let bytes = fs::read(&path).map_err(storage)?;
        let digest = Sha256Digest::from_bytes(&bytes);
        let size = u64::try_from(bytes.len()).map_err(|error| ObjectStoreError::Storage(error.to_string()))?;
        let metadata = ObjectMetadata {
            key: key.clone(),
            size,
            revision: digest.to_string(),
            digest,
        };
        Ok((bytes, metadata))
    }

    fn validate_size(&self, bytes: &[u8]) -> Result<(), ObjectStoreError> {
        let size = u64::try_from(bytes.len()).map_err(|error| ObjectStoreError::Storage(error.to_string()))?;
        if size > self.maximum_object_bytes {
            Err(ObjectStoreError::LimitExceeded)
        } else {
            Ok(())
        }
    }

    fn temporary(&self, target: &Path) -> Result<NamedTempFile, ObjectStoreError> {
        let parent = target
            .parent()
            .ok_or_else(|| ObjectStoreError::Storage("target has no parent".to_owned()))?;
        NamedTempFile::new_in(parent).map_err(storage)
    }

    fn write_temp(
        &self,
        target: &Path,
        bytes: &[u8],
    ) -> Result<NamedTempFile, ObjectStoreError> {
        let mut temporary = self.temporary(target)?;
        temporary.write_all(bytes).map_err(storage)?;
        temporary.as_file().sync_all().map_err(storage)?;
        Ok(temporary)
    }
}

impl ObjectStore for LocalObjectStore {
    fn put_immutable(
        &self,
        key: &ObjectKey,
        bytes: &[u8],
        expected_digest: &Sha256Digest,
    ) -> Result<PutOutcome, ObjectStoreError> {
        self.validate_size(bytes)?;
        let actual_digest = Sha256Digest::from_bytes(bytes);
        if &actual_digest != expected_digest {
            return Err(ObjectStoreError::DigestMismatch {
                key: key.clone(),
                expected: expected_digest.clone(),
                found: actual_digest,
            });
        }

        let path = self.ensure_parent(key)?;
        if path.exists() {
            let (_, metadata) = self.read_verified(key)?;
            return if &metadata.digest == expected_digest {
                Ok(PutOutcome::AlreadyPresent)
            } else {
                Err(ObjectStoreError::ImmutableConflict(key.clone()))
            };
        }

        let temporary = self.write_temp(&path, bytes)?;
        match temporary.persist_noclobber(&path) {
            Ok(_) => Ok(PutOutcome::Created),
            Err(error) if error.error.kind() == io::ErrorKind::AlreadyExists => {
                let (_, metadata) = self.read_verified(key)?;
                if &metadata.digest == expected_digest {
                    Ok(PutOutcome::AlreadyPresent)
                } else {
                    Err(ObjectStoreError::ImmutableConflict(key.clone()))
                }
            }
            Err(error) => Err(storage(error.error)),
        }
    }

    fn get(&self, key: &ObjectKey) -> Result<Vec<u8>, ObjectStoreError> {
        self.read_verified(key).map(|(bytes, _)| bytes)
    }

    fn head(&self, key: &ObjectKey) -> Result<ObjectMetadata, ObjectStoreError> {
        self.read_verified(key).map(|(_, metadata)| metadata)
    }

    fn list(
        &self,
        prefix: Option<&str>,
        cursor: Option<&str>,
        limit: u32,
    ) -> Result<ListPage, ObjectStoreError> {
        if limit == 0 || limit > MAXIMUM_LIST_LIMIT {
            return Err(ObjectStoreError::LimitExceeded);
        }
        let mut keys = Vec::new();
        for entry in WalkDir::new(&self.root).follow_links(false) {
            let entry = entry.map_err(storage)?;
            if entry.file_type().is_symlink() {
                return Err(ObjectStoreError::SymbolicLink(
                    entry.path().display().to_string(),
                ));
            }
            if !entry.file_type().is_file() {
                continue;
            }
            let relative = entry
                .path()
                .strip_prefix(&self.root)
                .map_err(|error| ObjectStoreError::Storage(error.to_string()))?;
            let key_text = relative
                .components()
                .map(|component| component.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/");
            if key_text.starts_with('.') {
                continue;
            }
            if prefix.is_some_and(|value| !key_text.starts_with(value)) {
                continue;
            }
            if cursor.is_some_and(|value| key_text.as_str() <= value) {
                continue;
            }
            keys.push(ObjectKey::parse(key_text)?);
        }
        keys.sort();

        let requested = usize::try_from(limit).map_err(|error| ObjectStoreError::Storage(error.to_string()))?;
        let has_more = keys.len() > requested;
        keys.truncate(requested);
        let mut objects = Vec::with_capacity(keys.len());
        for key in keys {
            objects.push(self.head(&key)?);
        }
        let next_cursor = if has_more {
            objects.last().map(|metadata| metadata.key.to_string())
        } else {
            None
        };
        Ok(ListPage {
            objects,
            next_cursor,
        })
    }

    fn delete_if_digest(
        &self,
        key: &ObjectKey,
        expected_digest: &Sha256Digest,
    ) -> Result<(), ObjectStoreError> {
        let metadata = self.head(key)?;
        if &metadata.digest != expected_digest {
            return Err(ObjectStoreError::DigestMismatch {
                key: key.clone(),
                expected: expected_digest.clone(),
                found: metadata.digest,
            });
        }
        let path = self.resolve(key)?;
        fs::remove_file(path).map_err(storage)
    }

    fn replace_manifest_if_revision(
        &self,
        key: &ObjectKey,
        expected_revision: Option<&str>,
        bytes: &[u8],
    ) -> Result<ObjectMetadata, ObjectStoreError> {
        self.validate_size(bytes)?;
        let _guard = self.lock_manifests()?;
        let path = self.ensure_parent(key)?;
        let current = if path.exists() {
            Some(self.head(key)?)
        } else {
            None
        };
        let current_revision = current.as_ref().map(|metadata| metadata.revision.as_str());
        if current_revision != expected_revision {
            return Err(ObjectStoreError::RevisionConflict {
                key: key.clone(),
                expected: expected_revision.map(str::to_owned),
                found: current_revision.map(str::to_owned),
            });
        }

        let temporary = self.write_temp(&path, bytes)?;
        let result = if path.exists() {
            temporary.persist(&path)
        } else {
            temporary.persist_noclobber(&path)
        };
        result.map_err(|error| storage(error.error))?;
        self.head(key)
    }
}

fn reject_symlink(path: &Path) -> Result<(), ObjectStoreError> {
    let metadata = fs::symlink_metadata(path).map_err(storage)?;
    if metadata.file_type().is_symlink() {
        Err(ObjectStoreError::SymbolicLink(path.display().to_string()))
    } else {
        Ok(())
    }
}

fn storage(error: impl std::fmt::Display) -> ObjectStoreError {
    ObjectStoreError::Storage(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::LocalObjectStore;
    use liaison_provider_sdk::run_object_store_conformance;
    use std::error::Error;
    use tempfile::tempdir;

    #[test]
    fn passes_object_store_conformance() -> Result<(), Box<dyn Error>> {
        let directory = tempdir()?;
        let store = LocalObjectStore::new(directory.path())?;
        let report = run_object_store_conformance(&store);
        if !report.passed() {
            let details = report
                .failures()
                .map(|case| format!("{}: {}", case.id, case.detail))
                .collect::<Vec<_>>()
                .join("; ");
            return Err(format!("object-store conformance failed: {details}").into());
        }
        Ok(())
    }

    #[test]
    fn enforces_configured_size_limit() -> Result<(), Box<dyn Error>> {
        let directory = tempdir()?;
        let store = LocalObjectStore::new(directory.path())?.with_maximum_object_bytes(3);
        let key = liaison_connections::ObjectKey::parse("objects/sample")?;
        let bytes = b"four";
        let digest = liaison_connections::Sha256Digest::from_bytes(bytes);
        let result = liaison_connections::ObjectStore::put_immutable(&store, &key, bytes, &digest);
        assert!(matches!(result, Err(liaison_connections::ObjectStoreError::LimitExceeded)));
        Ok(())
    }
}
