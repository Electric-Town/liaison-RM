//! Provider development and conformance helpers.
//!
//! The SDK exposes contract tests and evidence types. It deliberately does not
//! expose business-domain aggregates or a raw workspace/database handle.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate
)]

use chrono::{DateTime, Utc};
use liaison_connections::{
    ListPage, ObjectKey, ObjectMetadata, ObjectStore, ObjectStoreError, PutOutcome, Sha256Digest,
    OBJECT_STORE_CONTRACT_V1,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CaseStatus {
    Pass,
    Fail,
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConformanceCase {
    pub id: String,
    pub status: CaseStatus,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectStoreConformanceReport {
    pub contract: String,
    pub tested_at: DateTime<Utc>,
    pub namespace: String,
    pub cases: Vec<ConformanceCase>,
}

impl ObjectStoreConformanceReport {
    pub fn passed(&self) -> bool {
        self.cases
            .iter()
            .all(|case| case.status != CaseStatus::Fail)
    }

    pub fn failures(&self) -> impl Iterator<Item = &ConformanceCase> {
        self.cases
            .iter()
            .filter(|case| case.status == CaseStatus::Fail)
    }
}

pub fn run_object_store_conformance<S>(store: &S) -> ObjectStoreConformanceReport
where
    S: ObjectStore,
{
    let namespace = format!("conformance/{}", Uuid::now_v7());
    let mut report = ObjectStoreConformanceReport {
        contract: OBJECT_STORE_CONTRACT_V1.to_owned(),
        tested_at: Utc::now(),
        namespace: namespace.clone(),
        cases: Vec::new(),
    };

    let object_key = match ObjectKey::parse(format!("{namespace}/objects/sample.bin")) {
        Ok(key) => key,
        Err(error) => {
            report.cases.push(failed("setup.object-key", error));
            return report;
        }
    };
    let second_key = match ObjectKey::parse(format!("{namespace}/objects/second.bin")) {
        Ok(key) => key,
        Err(error) => {
            report.cases.push(failed("setup.second-key", error));
            return report;
        }
    };
    let manifest_key = match ObjectKey::parse(format!("{namespace}/manifests/latest.json")) {
        Ok(key) => key,
        Err(error) => {
            report.cases.push(failed("setup.manifest-key", error));
            return report;
        }
    };

    let bytes = b"liaison-object-store-conformance-v1";
    let changed = b"liaison-object-store-conformance-v1-changed";
    let digest = Sha256Digest::from_bytes(bytes);
    let changed_digest = Sha256Digest::from_bytes(changed);

    report.cases.push(result_case(
        "object.put-immutable",
        match store.put_immutable(&object_key, bytes, &digest) {
            Ok(PutOutcome::Created) => Ok("immutable object created".to_owned()),
            Ok(outcome) => Err(format!("expected Created, received {outcome:?}")),
            Err(error) => Err(error.to_string()),
        },
    ));

    report.cases.push(result_case(
        "object.put-identical-idempotent",
        match store.put_immutable(&object_key, bytes, &digest) {
            Ok(PutOutcome::AlreadyPresent) => Ok("identical object accepted without rewrite".to_owned()),
            Ok(outcome) => Err(format!("expected AlreadyPresent, received {outcome:?}")),
            Err(error) => Err(error.to_string()),
        },
    ));

    report.cases.push(result_case(
        "object.reject-digest-mismatch",
        match store.put_immutable(&second_key, bytes, &changed_digest) {
            Err(ObjectStoreError::DigestMismatch { .. }) => {
                Ok("mismatched expected digest rejected".to_owned())
            }
            Err(error) => Err(format!("unexpected error: {error}")),
            Ok(outcome) => Err(format!("mismatched digest was accepted: {outcome:?}")),
        },
    ));

    report.cases.push(result_case(
        "object.reject-immutable-conflict",
        match store.put_immutable(&object_key, changed, &changed_digest) {
            Err(ObjectStoreError::ImmutableConflict(_)) => {
                Ok("different content did not replace immutable object".to_owned())
            }
            Err(error) => Err(format!("unexpected error: {error}")),
            Ok(outcome) => Err(format!("conflicting content was accepted: {outcome:?}")),
        },
    ));

    report.cases.push(result_case(
        "object.get-verifies-bytes",
        match store.get(&object_key) {
            Ok(returned) if returned == bytes => Ok("retrieved bytes match source".to_owned()),
            Ok(_) => Err("retrieved bytes differ from source".to_owned()),
            Err(error) => Err(error.to_string()),
        },
    ));

    let metadata = store.head(&object_key);
    report.cases.push(result_case(
        "object.head-metadata",
        match metadata.as_ref() {
            Ok(value)
                if value.key == object_key
                    && value.digest == digest
                    && value.size == u64::try_from(bytes.len()).unwrap_or(u64::MAX) =>
            {
                Ok("metadata contains key, size, digest, and revision".to_owned())
            }
            Ok(value) => Err(format!("metadata did not match source: {value:?}")),
            Err(error) => Err(error.to_string()),
        },
    ));

    let list_prefix = format!("{namespace}/objects");
    report.cases.push(result_case(
        "object.list-prefix",
        list_contains(store.list(Some(&list_prefix), None, 100), &object_key),
    ));

    let manifest_v1 = br#"{"revision":1}"#;
    let manifest_v2 = br#"{"revision":2}"#;
    let first_manifest = store.replace_manifest_if_revision(&manifest_key, None, manifest_v1);
    report.cases.push(result_case(
        "manifest.create-with-none",
        metadata_result(&first_manifest, "manifest created"),
    ));

    let first_revision = first_manifest.as_ref().ok().map(|value| value.revision.clone());
    report.cases.push(result_case(
        "manifest.reject-stale-revision",
        match store.replace_manifest_if_revision(&manifest_key, Some("stale-revision"), manifest_v2) {
            Err(ObjectStoreError::RevisionConflict { .. }) => {
                Ok("stale manifest revision rejected".to_owned())
            }
            Err(error) => Err(format!("unexpected error: {error}")),
            Ok(value) => Err(format!("stale revision replaced manifest: {value:?}")),
        },
    ));

    let second_manifest = first_revision.as_deref().map_or_else(
        || Err(ObjectStoreError::Storage("first manifest revision unavailable".to_owned())),
        |revision| store.replace_manifest_if_revision(&manifest_key, Some(revision), manifest_v2),
    );
    report.cases.push(result_case(
        "manifest.replace-current-revision",
        metadata_result(&second_manifest, "current revision replaced manifest"),
    ));

    report.cases.push(result_case(
        "object.reject-delete-precondition",
        match store.delete_if_digest(&object_key, &changed_digest) {
            Err(ObjectStoreError::DigestMismatch { .. }) => {
                Ok("wrong delete digest rejected".to_owned())
            }
            Err(error) => Err(format!("unexpected error: {error}")),
            Ok(()) => Err("object was deleted with the wrong digest".to_owned()),
        },
    ));

    report.cases.push(result_case(
        "object.delete-current-digest",
        match store.delete_if_digest(&object_key, &digest) {
            Ok(()) => Ok("object deleted with matching digest".to_owned()),
            Err(error) => Err(error.to_string()),
        },
    ));

    report.cases.push(result_case(
        "object.deleted-is-not-found",
        match store.head(&object_key) {
            Err(ObjectStoreError::NotFound(_)) => Ok("deleted object is absent".to_owned()),
            Err(error) => Err(format!("unexpected error: {error}")),
            Ok(value) => Err(format!("deleted object remains visible: {value:?}")),
        },
    ));

    report
}

fn list_contains(
    page: Result<ListPage, ObjectStoreError>,
    expected: &ObjectKey,
) -> Result<String, String> {
    match page {
        Ok(page) if page.objects.iter().any(|object| &object.key == expected) => {
            Ok("prefix listing contains the object".to_owned())
        }
        Ok(page) => Err(format!("prefix listing omitted object: {page:?}")),
        Err(error) => Err(error.to_string()),
    }
}

fn metadata_result(
    result: &Result<ObjectMetadata, ObjectStoreError>,
    success: &str,
) -> Result<String, String> {
    match result {
        Ok(_) => Ok(success.to_owned()),
        Err(error) => Err(error.to_string()),
    }
}

fn result_case(id: &str, result: Result<String, String>) -> ConformanceCase {
    match result {
        Ok(detail) => ConformanceCase {
            id: id.to_owned(),
            status: CaseStatus::Pass,
            detail,
        },
        Err(detail) => ConformanceCase {
            id: id.to_owned(),
            status: CaseStatus::Fail,
            detail,
        },
    }
}

fn failed(id: &str, error: impl std::fmt::Display) -> ConformanceCase {
    ConformanceCase {
        id: id.to_owned(),
        status: CaseStatus::Fail,
        detail: error.to_string(),
    }
}
