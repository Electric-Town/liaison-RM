//! Provider contract conformance helpers.

#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

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

    #[must_use]
    pub fn passed_count(&self) -> usize {
        self.checks.iter().filter(|check| check.passed).count()
    }
}

pub fn run_object_store_conformance<Store>(
    store: &Store,
) -> Result<ConformanceReport, ObjectStoreError>
where
    Store: ObjectStore + ?Sized,
{
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    let prefix = format!("conformance/{nonce:x}");
    let object_key = ObjectKey::parse(format!("{prefix}/object-a"))?;
    let second_key = ObjectKey::parse(format!("{prefix}/object-b"))?;
    let mismatch_key = ObjectKey::parse(format!("{prefix}/checksum-mismatch"))?;
    let delete_key = ObjectKey::parse(format!("{prefix}/delete-me"))?;
    let manifest_key = ObjectKey::parse(format!("{prefix}/manifest"))?;

    let content = b"liaison-provider-conformance";
    let digest = ContentDigest::sha256(content);
    let expected_size = u64::try_from(content.len()).map_err(|_| {
        ObjectStoreError::new(
            ObjectStoreErrorKind::Unsupported,
            "platform object size does not fit the contract's u64 size",
        )
    })?;
    let mut checks = Vec::new();

    checks.push(match store.put_immutable(&object_key, content, &digest) {
        Ok(metadata) => ConformanceCheck {
            name: "put immutable object".to_owned(),
            passed: metadata.digest == digest && metadata.size_bytes == expected_size,
            detail: format!(
                "stored {} bytes with digest {}",
                metadata.size_bytes, metadata.digest
            ),
        },
        Err(error) => failed("put immutable object", error.to_string()),
    });

    checks.push(match store.get(&object_key) {
        Ok(received) => ConformanceCheck {
            name: "get object".to_owned(),
            passed: received == content,
            detail: format!("received {} bytes", received.len()),
        },
        Err(error) => failed("get object", error.to_string()),
    });

    checks.push(match store.head(&object_key) {
        Ok(metadata) => ConformanceCheck {
            name: "head object".to_owned(),
            passed: metadata.digest == digest && metadata.size_bytes == expected_size,
            detail: format!(
                "reported {} bytes with digest {}",
                metadata.size_bytes, metadata.digest
            ),
        },
        Err(error) => failed("head object", error.to_string()),
    });

    checks.push(expected_error(
        "reject immutable overwrite",
        store.put_immutable(&object_key, content, &digest),
        ObjectStoreErrorKind::AlreadyExists,
    ));

    let wrong_digest = ContentDigest::sha256(b"different-content");
    checks.push(expected_error(
        "reject checksum mismatch",
        store.put_immutable(&mismatch_key, content, &wrong_digest),
        ObjectStoreErrorKind::ChecksumMismatch,
    ));

    let second_content = b"second-object";
    let second_digest = ContentDigest::sha256(second_content);
    let second_put = store.put_immutable(&second_key, second_content, &second_digest);
    checks.push(match second_put {
        Ok(_) => match store.list(&prefix, None, 1) {
            Ok(first_page) => {
                let cursor = first_page.next_cursor.clone();
                match cursor.as_deref() {
                    Some(cursor) => match store.list(&prefix, Some(cursor), 1) {
                        Ok(second_page) => ConformanceCheck {
                            name: "list by prefix with cursor".to_owned(),
                            passed: first_page.objects.len() == 1
                                && second_page.objects.len() == 1
                                && first_page.objects[0].key != second_page.objects[0].key,
                            detail: format!(
                                "first page {}, second page {}, continuation {}",
                                first_page.objects.len(),
                                second_page.objects.len(),
                                cursor
                            ),
                        },
                        Err(error) => failed("list by prefix with cursor", error.to_string()),
                    },
                    None => failed(
                        "list by prefix with cursor",
                        "first page omitted a continuation cursor".to_owned(),
                    ),
                }
            }
            Err(error) => failed("list by prefix with cursor", error.to_string()),
        },
        Err(error) => failed("list by prefix with cursor", error.to_string()),
    });

    let delete_content = b"delete-after-check";
    let delete_digest = ContentDigest::sha256(delete_content);
    let wrong_delete_digest = ContentDigest::sha256(b"wrong-delete-digest");
    let delete_setup = store.put_immutable(&delete_key, delete_content, &delete_digest);
    checks.push(match delete_setup {
        Ok(_) => expected_error(
            "reject guarded delete with wrong digest",
            store.delete_if_permitted(&delete_key, &wrong_delete_digest),
            ObjectStoreErrorKind::Conflict,
        ),
        Err(error) => failed(
            "reject guarded delete with wrong digest",
            format!("test object setup failed: {error}"),
        ),
    });

    let delete_result = store
        .delete_if_permitted(&delete_key, &delete_digest)
        .and_then(|()| match store.get(&delete_key) {
            Err(error) if error.kind() == ObjectStoreErrorKind::NotFound => Ok(()),
            Err(error) => Err(error),
            Ok(_) => Err(ObjectStoreError::new(
                ObjectStoreErrorKind::Conflict,
                "deleted object remained readable",
            )),
        });
    checks.push(match delete_result {
        Ok(()) => ConformanceCheck {
            name: "delete with expected digest".to_owned(),
            passed: true,
            detail: "object became unreadable after guarded deletion".to_owned(),
        },
        Err(error) => failed("delete with expected digest", error.to_string()),
    });

    let first_manifest = b"{\"revision\":1}";
    let first_digest = ContentDigest::sha256(first_manifest);
    let first_revision =
        store.replace_manifest_if_revision(&manifest_key, None, first_manifest, &first_digest);
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
    let second_manifest = b"{\"revision\":2}";
    let second_manifest_digest = ContentDigest::sha256(second_manifest);
    checks.push(expected_error(
        "reject stale manifest revision",
        store.replace_manifest_if_revision(
            &manifest_key,
            Some(&stale_revision),
            second_manifest,
            &second_manifest_digest,
        ),
        ObjectStoreErrorKind::Conflict,
    ));

    checks.push(match first_revision {
        Ok(revision) => match store.replace_manifest_if_revision(
            &manifest_key,
            Some(&revision),
            second_manifest,
            &second_manifest_digest,
        ) {
            Ok(new_revision) => ConformanceCheck {
                name: "replace manifest with current revision".to_owned(),
                passed: new_revision == second_manifest_digest,
                detail: format!("new manifest revision {new_revision}"),
            },
            Err(error) => failed(
                "replace manifest with current revision",
                error.to_string(),
            ),
        },
        Err(error) => failed(
            "replace manifest with current revision",
            format!("initial manifest creation failed: {error}"),
        ),
    });

    Ok(ConformanceReport {
        contract: "object-store@1".to_owned(),
        suite_version: 1,
        checks,
    })
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
