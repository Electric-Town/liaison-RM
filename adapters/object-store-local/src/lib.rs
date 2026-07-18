#![forbid(unsafe_code)]

use liaison_connections::{
    ConfigurationField, ConfigurationValueType, ConformanceStatus, ContentDigest,
    ContractDescriptor, ListPage, ObjectKey, ObjectMetadata, ObjectStore, ObjectStoreError,
    ObjectStoreErrorKind, ProviderDescriptor, ProviderId, ProviderVersion, SafeMode,
};
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct LocalObjectStore {
    root: PathBuf,
}

impl LocalObjectStore {
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Builds the provider descriptor published by the local-folder package.
    ///
    /// # Errors
    ///
    /// Returns an error when a checked-in provider identifier, version,
    /// operation, safe mode, or configuration field violates the Connections
    /// domain contract.
    pub fn descriptor() -> Result<ProviderDescriptor, liaison_connections::ProviderDomainError> {
        ProviderDescriptor::new(
            ProviderId::parse("org.electric-town.local-folder")?,
            ProviderVersion::parse(env!("CARGO_PKG_VERSION"))?,
            "Local folder",
            vec![ContractDescriptor::new(
                "object-store",
                1,
                vec![
                    "put-immutable".to_owned(),
                    "get".to_owned(),
                    "head".to_owned(),
                    "list".to_owned(),
                    "delete-if-permitted".to_owned(),
                    "replace-manifest-if-revision".to_owned(),
                ],
                vec![SafeMode::Backup, SafeMode::SingleWriter],
                "Uses same-filesystem operations and a cooperative manifest lock. Multi-writer mode remains disabled.",
            )?],
            vec![ConfigurationField::new(
                "root_path",
                ConfigurationValueType::String,
                false,
                true,
                "User-selected directory for encrypted Liaison objects.",
            )?],
            vec![],
            ConformanceStatus::NotTested,
        )
    }

    fn objects_root(&self) -> PathBuf {
        self.root.join("objects")
    }

    fn manifests_root(&self) -> PathBuf {
        self.root.join("manifests")
    }

    fn locks_root(&self) -> PathBuf {
        self.root.join("locks")
    }

    fn object_path(&self, key: &ObjectKey) -> PathBuf {
        self.objects_root().join(key.as_str())
    }

    fn manifest_path(&self, key: &ObjectKey) -> PathBuf {
        self.manifests_root().join(key.as_str())
    }

    fn lock_path(&self, key: &ObjectKey) -> PathBuf {
        let mut path = self.locks_root().join(key.as_str());
        let extension = path
            .extension()
            .and_then(OsStr::to_str)
            .map_or_else(|| "lock".to_owned(), |value| format!("{value}.lock"));
        path.set_extension(extension);
        path
    }
}

impl ObjectStore for LocalObjectStore {
    fn put_immutable(
        &self,
        key: &ObjectKey,
        content: &[u8],
        expected_digest: &ContentDigest,
    ) -> Result<ObjectMetadata, ObjectStoreError> {
        verify_digest(content, expected_digest)?;
        let path = self.object_path(key);
        write_immutable(&path, content)?;
        Ok(ObjectMetadata {
            key: key.clone(),
            size_bytes: content.len() as u64,
            digest: expected_digest.clone(),
        })
    }

    fn get(&self, key: &ObjectKey) -> Result<Vec<u8>, ObjectStoreError> {
        read_bytes(&self.object_path(key))
    }

    fn head(&self, key: &ObjectKey) -> Result<ObjectMetadata, ObjectStoreError> {
        metadata_for_path(key.clone(), &self.object_path(key))
    }

    fn list(
        &self,
        prefix: &str,
        cursor: Option<&str>,
        limit: usize,
    ) -> Result<ListPage, ObjectStoreError> {
        if limit == 0 || limit > 10_000 {
            return Err(ObjectStoreError::new(
                ObjectStoreErrorKind::InvalidKey,
                "list limit must be between 1 and 10000",
            ));
        }
        if prefix.contains('\\')
            || prefix.starts_with('/')
            || prefix
                .split('/')
                .any(|segment| segment == "." || segment == "..")
        {
            return Err(ObjectStoreError::new(
                ObjectStoreErrorKind::InvalidKey,
                "list prefix contains an unsafe path segment",
            ));
        }

        let root = self.objects_root();
        if !root.exists() {
            return Ok(ListPage {
                objects: vec![],
                next_cursor: None,
            });
        }

        let mut keys = Vec::new();
        collect_keys(&root, &root, &mut keys)?;
        keys.sort();
        let start = cursor
            .and_then(|value| keys.iter().position(|key| key.as_str() > value))
            .unwrap_or_else(|| if cursor.is_some() { keys.len() } else { 0 });

        let filtered = keys
            .into_iter()
            .skip(start)
            .filter(|key| key.as_str().starts_with(prefix))
            .collect::<Vec<_>>();

        let has_more = filtered.len() > limit;
        let selected = filtered.into_iter().take(limit).collect::<Vec<_>>();
        let next_cursor = has_more
            .then(|| selected.last().map(|key| key.as_str().to_owned()))
            .flatten();
        let objects = selected
            .into_iter()
            .map(|key| metadata_for_path(key.clone(), &self.object_path(&key)))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ListPage {
            objects,
            next_cursor,
        })
    }

    fn delete_if_permitted(
        &self,
        key: &ObjectKey,
        expected_digest: &ContentDigest,
    ) -> Result<(), ObjectStoreError> {
        let path = self.object_path(key);
        reject_symlink_if_present(&path)?;
        let current_digest = ContentDigest::sha256(&read_bytes(&path)?);
        if &current_digest != expected_digest {
            return Err(ObjectStoreError::new(
                ObjectStoreErrorKind::Conflict,
                format!(
                    "object digest changed; expected {expected_digest}, found {current_digest}"
                ),
            ));
        }
        fs::remove_file(&path).map_err(|error| map_io("delete object", &error))
    }

    fn replace_manifest_if_revision(
        &self,
        key: &ObjectKey,
        expected_revision: Option<&ContentDigest>,
        content: &[u8],
        expected_digest: &ContentDigest,
    ) -> Result<ContentDigest, ObjectStoreError> {
        verify_digest(content, expected_digest)?;
        let path = self.manifest_path(key);
        let lock_path = self.lock_path(key);
        reject_symlink_if_present(&path)?;
        reject_symlink_if_present(&lock_path)?;
        recover_manifest(&path)?;

        let _lock = ManifestLock::acquire(&lock_path)?;
        let current = if path.exists() {
            Some(ContentDigest::sha256(&read_bytes(&path)?))
        } else {
            None
        };

        match (expected_revision, current.as_ref()) {
            (None, None) => {}
            (Some(expected), Some(actual)) if expected == actual => {}
            (None, Some(_)) => {
                return Err(ObjectStoreError::new(
                    ObjectStoreErrorKind::Conflict,
                    "manifest already exists",
                ));
            }
            (Some(_), None) => {
                return Err(ObjectStoreError::new(
                    ObjectStoreErrorKind::Conflict,
                    "manifest does not exist",
                ));
            }
            (Some(_), Some(_)) => {
                return Err(ObjectStoreError::new(
                    ObjectStoreErrorKind::Conflict,
                    "manifest revision does not match",
                ));
            }
        }

        replace_file_with_recovery(&path, content)?;
        Ok(expected_digest.clone())
    }
}

fn verify_digest(content: &[u8], expected_digest: &ContentDigest) -> Result<(), ObjectStoreError> {
    let actual = ContentDigest::sha256(content);
    if &actual == expected_digest {
        Ok(())
    } else {
        Err(ObjectStoreError::new(
            ObjectStoreErrorKind::ChecksumMismatch,
            format!("expected digest {expected_digest}; calculated {actual}"),
        ))
    }
}

fn reject_symlink_if_present(path: &Path) -> Result<(), ObjectStoreError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(ObjectStoreError::new(
            ObjectStoreErrorKind::Io,
            format!("symbolic link is not allowed: {}", path.display()),
        )),
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(map_io("inspect path", &error)),
    }
}

fn read_bytes(path: &Path) -> Result<Vec<u8>, ObjectStoreError> {
    reject_symlink_if_present(path)?;
    let mut file = File::open(path).map_err(|error| map_io("open object", &error))?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)
        .map_err(|error| map_io("read object", &error))?;
    Ok(content)
}

fn metadata_for_path(key: ObjectKey, path: &Path) -> Result<ObjectMetadata, ObjectStoreError> {
    let content = read_bytes(path)?;
    Ok(ObjectMetadata {
        key,
        size_bytes: content.len() as u64,
        digest: ContentDigest::sha256(&content),
    })
}

fn collect_keys(
    root: &Path,
    directory: &Path,
    keys: &mut Vec<ObjectKey>,
) -> Result<(), ObjectStoreError> {
    for entry in fs::read_dir(directory).map_err(|error| map_io("list objects", &error))? {
        let entry = entry.map_err(|error| map_io("read object entry", &error))?;
        let file_type = entry
            .file_type()
            .map_err(|error| map_io("inspect object entry", &error))?;
        let path = entry.path();
        if file_type.is_symlink() {
            return Err(ObjectStoreError::new(
                ObjectStoreErrorKind::Io,
                format!(
                    "symbolic link is not allowed in object store: {}",
                    path.display()
                ),
            ));
        }
        if file_type.is_dir() {
            collect_keys(root, &path, keys)?;
        } else if file_type.is_file() {
            let relative = path
                .strip_prefix(root)
                .map_err(|_| {
                    ObjectStoreError::new(
                        ObjectStoreErrorKind::Io,
                        "object path escaped store root",
                    )
                })?
                .to_string_lossy()
                .replace('\\', "/");
            keys.push(ObjectKey::parse(relative)?);
        }
    }
    Ok(())
}

fn write_immutable(path: &Path, content: &[u8]) -> Result<(), ObjectStoreError> {
    let parent = path.parent().ok_or_else(|| {
        ObjectStoreError::new(ObjectStoreErrorKind::Io, "object path has no parent")
    })?;
    fs::create_dir_all(parent).map_err(|error| map_io("create object directory", &error))?;
    let temporary = temporary_path(path, "new");

    let result = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|error| map_io("create temporary object", &error))?;
        file.write_all(content)
            .map_err(|error| map_io("write temporary object", &error))?;
        file.sync_all()
            .map_err(|error| map_io("sync temporary object", &error))?;
        fs::hard_link(&temporary, path).map_err(|error| {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                ObjectStoreError::new(ObjectStoreErrorKind::AlreadyExists, "object already exists")
            } else {
                map_io("publish immutable object", &error)
            }
        })
    })();

    let _ = fs::remove_file(&temporary);
    result
}

fn replace_file_with_recovery(path: &Path, content: &[u8]) -> Result<(), ObjectStoreError> {
    let parent = path.parent().ok_or_else(|| {
        ObjectStoreError::new(ObjectStoreErrorKind::Io, "manifest path has no parent")
    })?;
    fs::create_dir_all(parent).map_err(|error| map_io("create manifest directory", &error))?;

    let temporary = temporary_path(path, "next");
    let backup = backup_path(path);
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|error| map_io("create temporary manifest", &error))?;
    file.write_all(content)
        .map_err(|error| map_io("write temporary manifest", &error))?;
    file.sync_all()
        .map_err(|error| map_io("sync temporary manifest", &error))?;
    drop(file);

    if path.exists() {
        let _ = fs::remove_file(&backup);
        fs::rename(path, &backup).map_err(|error| map_io("stage current manifest", &error))?;
    }

    if let Err(error) = fs::rename(&temporary, path) {
        if backup.exists() && !path.exists() {
            let _ = fs::rename(&backup, path);
        }
        let _ = fs::remove_file(&temporary);
        return Err(map_io("publish manifest", &error));
    }

    let _ = fs::remove_file(&backup);
    Ok(())
}

fn recover_manifest(path: &Path) -> Result<(), ObjectStoreError> {
    let backup = backup_path(path);
    if !path.exists() && backup.exists() {
        fs::rename(&backup, path).map_err(|error| map_io("recover manifest", &error))?;
    } else if path.exists() && backup.exists() {
        fs::remove_file(&backup).map_err(|error| map_io("remove stale manifest backup", &error))?;
    }
    Ok(())
}

fn temporary_path(path: &Path, label: &str) -> PathBuf {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let name = path.file_name().and_then(OsStr::to_str).unwrap_or("object");
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    parent.join(format!(".{name}.{label}-{}-{nonce:x}", std::process::id()))
}

fn backup_path(path: &Path) -> PathBuf {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let name = path
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or("manifest");
    parent.join(format!(".{name}.previous"))
}

struct ManifestLock {
    path: PathBuf,
}

impl ManifestLock {
    fn acquire(path: &Path) -> Result<Self, ObjectStoreError> {
        let parent = path.parent().ok_or_else(|| {
            ObjectStoreError::new(ObjectStoreErrorKind::Io, "lock path has no parent")
        })?;
        fs::create_dir_all(parent).map_err(|error| map_io("create lock directory", &error))?;
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(|error| {
                if error.kind() == std::io::ErrorKind::AlreadyExists {
                    ObjectStoreError::new(
                        ObjectStoreErrorKind::Conflict,
                        "manifest is locked by another local writer",
                    )
                } else {
                    map_io("create manifest lock", &error)
                }
            })?;
        Ok(Self {
            path: path.to_owned(),
        })
    }
}

impl Drop for ManifestLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn map_io(action: &str, error: &std::io::Error) -> ObjectStoreError {
    let kind = match error.kind() {
        std::io::ErrorKind::NotFound => ObjectStoreErrorKind::NotFound,
        std::io::ErrorKind::AlreadyExists => ObjectStoreErrorKind::AlreadyExists,
        _ => ObjectStoreErrorKind::Io,
    };
    ObjectStoreError::new(kind, format!("{action}: {error}"))
}

#[cfg(test)]
mod tests {
    use super::LocalObjectStore;
    use liaison_connections::{ContentDigest, ObjectKey, ObjectStore, ObjectStoreErrorKind};
    use liaison_provider_sdk::run_object_store_conformance;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new() -> Result<Self, std::io::Error> {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(0, |duration| duration.as_nanos());
            let path = std::env::temp_dir().join(format!(
                "liaison-object-store-test-{}-{nonce:x}",
                std::process::id()
            ));
            fs::create_dir_all(&path)?;
            Ok(Self(path))
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn passes_reference_conformance() -> Result<(), Box<dyn std::error::Error>> {
        let directory = TestDirectory::new()?;
        let store = LocalObjectStore::new(&directory.0);
        let report = run_object_store_conformance(&store)?;
        assert!(report.all_passed(), "{report:?}");
        Ok(())
    }

    #[test]
    fn rejects_wrong_checksum_before_writing() -> Result<(), Box<dyn std::error::Error>> {
        let directory = TestDirectory::new()?;
        let store = LocalObjectStore::new(&directory.0);
        let key = ObjectKey::parse("test/object")?;
        let digest = ContentDigest::sha256(b"different");
        let result = store.put_immutable(&key, b"content", &digest);
        assert!(result.is_err());
        let Err(error) = result else {
            return Ok(());
        };
        assert_eq!(error.kind(), ObjectStoreErrorKind::ChecksumMismatch);
        assert!(store.get(&key).is_err());
        Ok(())
    }
}
