#![forbid(unsafe_code)]

use liaison_connections::{
    ContentDigest, ObjectKey, ObjectStore, ObjectStoreError, ObjectStoreErrorKind,
};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConformanceCheck {
    pub name: String,
    pub passed: bool,
    pub detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConformanceReport {
    pub contract: String,
    pub suite_version: u32,
    pub checks: Vec<ConformanceCheck>,
}

impl ConformanceReport {
    #[must_use]
    pub fn all_passed(&self) -> bool {
        self.checks.iter().all(|check| check.passed)
    }
}

/// Executes the reusable `object-store@1` behavioural conformance suite.
///
/// The suite writes only synthetic content under a unique `conformance/`
/// namespace. A passing report verifies the checked contract operations. It
/// does not award a backup or synchronization mode to the provider.
///
/// # Errors
///
/// Returns an error when the suite cannot construct its safe synthetic object
/// keys. Provider-operation failures are retained as failed checks in the
/// returned report so callers receive the complete evidence set.
pub fn run_object_store_conformance<Store>(
    store: &Store,
) -> Result<ConformanceReport, ObjectStoreError>
where
    Store: ObjectStore,
{
    let keys = SuiteKeys::new()?;
    let mut checks = Vec::new();
    check_immutable_object(store, &keys, &mut checks);
    check_listing_and_deletion(store, &keys, &mut checks);
    check_manifest_revisions(store, &keys, &mut checks);

    Ok(ConformanceReport {
        contract: "object-store@1".to_owned(),
        suite_version: 1,
        checks,
    })
}

#[derive(Debug)]
struct SuiteKeys {
    prefix: String,
    object: ObjectKey,
    checksum_mismatch: ObjectKey,
    delete: ObjectKey,
    manifest: ObjectKey,
}

impl SuiteKeys {
    fn new() -> Result<Self, ObjectStoreError> {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| duration.as_nanos());
        let prefix = format!("conformance/{nonce:x}");
        Ok(Self {
            object: ObjectKey::parse(format!("{prefix}/object-a"))?,
            checksum_mismatch: ObjectKey::parse(format!("{prefix}/checksum-mismatch"))?,
            delete: ObjectKey::parse(format!("{prefix}/delete-me"))?,
            manifest: ObjectKey::parse(format!("{prefix}/manifest"))?,
            prefix,
        })
    }
}

fn check_immutable_object<Store>(
    store: &Store,
    keys: &SuiteKeys,
    checks: &mut Vec<ConformanceCheck>,
) where
    Store: ObjectStore,
{
    let content = b"liaison-provider-conformance";
    let digest = ContentDigest::sha256(content);

    checks.push(match store.put_immutable(&keys.object, content, &digest) {
        Ok(metadata) => ConformanceCheck {
            name: "put immutable object".to_owned(),
            passed: metadata.digest == digest && metadata.size_bytes == content.len() as u64,
            detail: format!(
                "stored {} bytes with digest {}",
                metadata.size_bytes, metadata.digest
            ),
        },
        Err(error) => failed("put immutable object", error.to_string()),
    });

    checks.push(match store.get(&keys.object) {
        Ok(received) => ConformanceCheck {
            name: "get object".to_owned(),
            passed: received == content,
            detail: format!("received {} bytes", received.len()),
        },
        Err(error) => failed("get object", error.to_string()),
    });

    checks.push(match store.head(&keys.object) {
        Ok(metadata) => ConformanceCheck {
            name: "head object".to_owned(),
            passed: metadata.digest == digest && metadata.size_bytes == content.len() as u64,
            detail: format!(
                "reported {} bytes with digest {}",
                metadata.size_bytes, metadata.digest
            ),
        },
        Err(error) => failed("head object", error.to_string()),
    });

    checks.push(expected_error(
        "reject immutable overwrite",
        store.put_immutable(&keys.object, content, &digest),
        ObjectStoreErrorKind::AlreadyExists,
    ));

    let wrong_digest = ContentDigest::sha256(b"different-content");
    checks.push(expected_error(
        "reject checksum mismatch",
        store.put_immutable(&keys.checksum_mismatch, content, &wrong_digest),
        ObjectStoreErrorKind::ChecksumMismatch,
    ));
}

fn check_listing_and_deletion<Store>(
    store: &Store,
    keys: &SuiteKeys,
    checks: &mut Vec<ConformanceCheck>,
) where
    Store: ObjectStore,
{
    checks.push(match store.list(&keys.prefix, None, 100) {
        Ok(page) => ConformanceCheck {
            name: "list by prefix".to_owned(),
            passed: page
                .objects
                .iter()
                .any(|metadata| metadata.key == keys.object),
            detail: format!(
                "listed {} object(s); next cursor present: {}",
                page.objects.len(),
                page.next_cursor.is_some()
            ),
        },
        Err(error) => failed("list by prefix", error.to_string()),
    });

    let content = b"delete-after-check";
    let digest = ContentDigest::sha256(content);
    let result = store
        .put_immutable(&keys.delete, content, &digest)
        .and_then(|_| store.delete_if_permitted(&keys.delete, &digest))
        .and_then(|()| match store.get(&keys.delete) {
            Err(error) if error.kind() == ObjectStoreErrorKind::NotFound => Ok(()),
            Err(error) => Err(error),
            Ok(_) => Err(ObjectStoreError::new(
                ObjectStoreErrorKind::Conflict,
                "deleted object remained readable",
            )),
        });
    checks.push(match result {
        Ok(()) => ConformanceCheck {
            name: "delete with expected digest".to_owned(),
            passed: true,
            detail: "object became unreadable after guarded deletion".to_owned(),
        },
        Err(error) => failed("delete with expected digest", error.to_string()),
    });
}

fn check_manifest_revisions<Store>(
    store: &Store,
    keys: &SuiteKeys,
    checks: &mut Vec<ConformanceCheck>,
) where
    Store: ObjectStore,
{
    let first_content = b"{\"revision\":1}";
    let first_digest = ContentDigest::sha256(first_content);
    let first_revision = store.replace_manifest_if_revision(
        &keys.manifest,
        None,
        first_content,
        &first_digest,
    );
    checks.push(match &first_revision {
        Ok(revision) => ConformanceCheck {
            name: "create manifest with absent precondition".to_owned(),
            passed: revision == &first_digest,
            detail: format!("manifest revision {revision}"),
        },
        Err(error) => failed(
            "create manifest with absent precondition",
            error.to_string(),
        ),
    });

    let stale_revision = ContentDigest::sha256(b"stale-revision");
    let second_content = b"{\"revision\":2}";
    let second_digest = ContentDigest::sha256(second_content);
    checks.push(expected_error(
        "reject stale manifest revision",
        store.replace_manifest_if_revision(
            &keys.manifest,
            Some(&stale_revision),
            second_content,
            &second_digest,
        ),
        ObjectStoreErrorKind::Conflict,
    ));

    checks.push(match first_revision {
        Ok(revision) => match store.replace_manifest_if_revision(
            &keys.manifest,
            Some(&revision),
            second_content,
            &second_digest,
        ) {
            Ok(new_revision) => ConformanceCheck {
                name: "replace manifest with current revision".to_owned(),
                passed: new_revision == second_digest,
                detail: format!("new manifest revision {new_revision}"),
            },
            Err(error) => failed("replace manifest with current revision", error.to_string()),
        },
        Err(error) => failed(
            "replace manifest with current revision",
            format!("initial manifest creation failed: {error}"),
        ),
    });
}

fn expected_error<T>(
    name: &str,
    result: Result<T, ObjectStoreError>,
    expected: ObjectStoreErrorKind,
) -> ConformanceCheck {
    match result {
        Ok(_) => ConformanceCheck {
            name: name.to_owned(),
            passed: false,
            detail: format!("operation succeeded; expected {expected:?}"),
        },
        Err(error) => ConformanceCheck {
            name: name.to_owned(),
            passed: error.kind() == expected,
            detail: format!("received {:?}", error.kind()),
        },
    }
}

fn failed(name: &str, detail: String) -> ConformanceCheck {
    ConformanceCheck {
        name: name.to_owned(),
        passed: false,
        detail,
    }
}
