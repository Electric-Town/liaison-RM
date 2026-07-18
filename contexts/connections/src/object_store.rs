use sha2::{Digest, Sha256};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ObjectKey(String);

impl ObjectKey {
    pub fn parse(value: impl Into<String>) -> Result<Self, ObjectStoreError> {
        let value = value.into();
        if value.is_empty()
            || value.len() > 1_024
            || value.starts_with('/')
            || value.ends_with('/')
            || value.contains('\\')
            || value.chars().any(char::is_control)
        {
            return Err(ObjectStoreError::new(
                ObjectStoreErrorKind::InvalidKey,
                "object key is empty, absolute, too long, or contains unsafe characters",
            ));
        }
        if value
            .split('/')
            .any(|segment| segment.is_empty() || segment == "." || segment == "..")
        {
            return Err(ObjectStoreError::new(
                ObjectStoreErrorKind::InvalidKey,
                "object key contains an unsafe path segment",
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ObjectKey {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContentDigest(String);

impl ContentDigest {
    #[must_use]
    pub fn sha256(content: &[u8]) -> Self {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let digest = Sha256::digest(content);
        let mut value = String::with_capacity(64);
        for byte in digest {
            value.push(char::from(HEX[usize::from(byte >> 4)]));
            value.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
        Self(value)
    }

    pub fn parse(value: impl Into<String>) -> Result<Self, ObjectStoreError> {
        let value = value.into();
        if value.len() != 64
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(ObjectStoreError::new(
                ObjectStoreErrorKind::ChecksumMismatch,
                "SHA-256 digest must contain 64 lowercase hexadecimal characters",
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ContentDigest {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectMetadata {
    pub key: ObjectKey,
    pub size_bytes: u64,
    pub digest: ContentDigest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListPage {
    pub objects: Vec<ObjectMetadata>,
    pub next_cursor: Option<String>,
}

pub trait ObjectStore {
    fn put_immutable(
        &self,
        key: &ObjectKey,
        content: &[u8],
        expected_digest: &ContentDigest,
    ) -> Result<ObjectMetadata, ObjectStoreError>;

    fn get(&self, key: &ObjectKey) -> Result<Vec<u8>, ObjectStoreError>;

    fn head(&self, key: &ObjectKey) -> Result<ObjectMetadata, ObjectStoreError>;

    fn list(
        &self,
        prefix: &str,
        cursor: Option<&str>,
        limit: usize,
    ) -> Result<ListPage, ObjectStoreError>;

    fn delete_if_permitted(
        &self,
        key: &ObjectKey,
        expected_digest: &ContentDigest,
    ) -> Result<(), ObjectStoreError>;

    fn replace_manifest_if_revision(
        &self,
        key: &ObjectKey,
        expected_revision: Option<&ContentDigest>,
        content: &[u8],
        expected_digest: &ContentDigest,
    ) -> Result<ContentDigest, ObjectStoreError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObjectStoreErrorKind {
    AlreadyExists,
    NotFound,
    ChecksumMismatch,
    Conflict,
    InvalidKey,
    Io,
    Unsupported,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectStoreError {
    kind: ObjectStoreErrorKind,
    message: String,
}

impl ObjectStoreError {
    #[must_use]
    pub fn new(kind: ObjectStoreErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> ObjectStoreErrorKind {
        self.kind
    }
}

impl Display for ObjectStoreError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}: {}", self.kind, self.message)
    }
}

impl Error for ObjectStoreError {}

#[cfg(test)]
mod tests {
    use super::{ContentDigest, ObjectKey};

    #[test]
    fn rejects_path_traversal() {
        assert!(ObjectKey::parse("../secret").is_err());
        assert!(ObjectKey::parse("objects/../../secret").is_err());
        assert!(ObjectKey::parse("/absolute").is_err());
        assert!(ObjectKey::parse("safe/object").is_ok());
    }

    #[test]
    fn sha256_matches_known_vector() {
        assert_eq!(
            ContentDigest::sha256(b"abc").as_str(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
