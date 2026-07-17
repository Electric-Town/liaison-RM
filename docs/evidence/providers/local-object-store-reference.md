# Local object-store reference evidence

- Provider: `local-folder`
- Provider version: `0.1.0`
- Contract: `object-store@1`
- Evidence status: pending CI
- Safe modes under review: backup, restore, single-writer publication
- Explicitly unsupported: multi-writer synchronisation

## Implementation under review

The reference adapter implements:

- safe `ObjectKey` parsing;
- caller-supplied SHA-256 verification;
- immutable publication and byte-identical idempotency;
- verified retrieval and metadata;
- prefix listing and pagination cursor;
- deletion guarded by current digest;
- manifest creation and replacement guarded by revision;
- object and page limits;
- encountered symbolic-link rejection;
- process-local serialisation of manifest replacement.

## Required automated evidence

`liaison-provider-sdk::run_object_store_conformance` tests:

1. immutable creation;
2. identical idempotent publication;
3. expected-digest mismatch rejection;
4. immutable content-conflict rejection;
5. byte retrieval;
6. metadata correctness;
7. prefix listing;
8. manifest creation;
9. stale manifest revision rejection;
10. current manifest revision replacement;
11. delete-precondition rejection;
12. matching-digest deletion;
13. post-deletion absence.

The provider descriptor remains without an accepted conformance reference until the exact branch commit passes this suite on the supported Rust CI matrix. This document must then be updated with:

- source commit;
- Rust and SDK versions;
- operating systems and filesystems tested;
- passed, failed, skipped, and unsupported cases;
- maximum tested object and listing sizes;
- interruption and filesystem fault results;
- symbolic-link and path-race results;
- isolated backup/restore evidence;
- known limitations and evidence expiry condition.

## Current limitations

- The manifest lock coordinates clones within one process, not independent processes or devices.
- Cross-platform atomic replacement and directory durability still require fault-injection evidence.
- The adapter checks encountered symbolic links but does not yet use a capability-based directory handle that removes every time-of-check/time-of-use race.
- This adapter stores provider objects; it is not the readable canonical workspace.
- Backup and restore application services are outside this PR.

## Release decision

Do not register this provider as release-ready while `conformance` is `null` in its descriptor. Passing object-store tests will support the object contract. Backup/restore labels additionally require the Workspace backup service and isolated restore gate. Multi-writer synchronisation remains prohibited.
