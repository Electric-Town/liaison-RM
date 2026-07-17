---
id: KCS-0004
title: How do I decide whether a provider can synchronise or only back up?
status: draft
audience: contributor
contexts: [connections, sharing, workspace]
symptoms:
  - A provider can upload and download files, but its safe product label is unclear.
  - An integration is described as sync without tested conflict or recovery semantics.
search_terms:
  - provider sync
  - backup versus sync
  - WebDAV conflicts
  - S3 multi writer
  - object store conformance
last_validated: 2026-07-17
validated_against:
  - object-store@1 draft contract
related_requirements:
  - LRM-CO-005
  - LRM-CO-009
  - LRM-SH-001
related_gates:
  - FG-R4-001
  - FG-R4-003
  - FG-R4-007
---

# How do I decide whether a provider can synchronise or only back up?

## Problem

A remote service may support PUT, GET, listing, and deletion while still lacking the semantics Liaison RM needs for safe concurrent writers. Calling every two-way transfer “sync” hides the risk of lost updates, rollback, conflict copies, partial publication, and unrecoverable backups.

## Decision procedure

Classify the provider by the strongest behaviour demonstrated by current conformance evidence.

### Backup

Use **backup** when Liaison can:

1. encrypt and validate a snapshot locally;
2. publish immutable objects and a manifest;
3. verify remote existence, size, and content hashes;
4. download into an isolated restore directory;
5. validate and complete a restore without replacing the current workspace prematurely.

Upload success alone is not backup success.

### Single-writer publication

Use **single-writer publication** when one authorised writer can replace a manifest using a tested revision precondition and readers can retrieve the resulting immutable objects. The provider or operating procedure must prevent unsupported simultaneous writers.

A local folder with a process-local lock can fit this mode for one Liaison process. It does not prove coordination across processes, devices, network mounts, or external editors.

### Immutable transport

Use **transport** when the provider can carry signed immutable Liaison operations and manifests. Sharing still verifies, orders, authorises, reconciles, and materialises operations locally. The provider is not the conflict-resolution authority.

### Multi-writer synchronisation

Use **multi-writer synchronisation** only when evidence covers:

- simultaneous writers;
- conditional manifest or operation publication;
- stable ordering or explicit causal dependencies;
- duplicate delivery and idempotency;
- tombstones and deletion;
- rollback and stale listing;
- partial publication;
- conflict detection and recovery;
- revoked writers and replay;
- provider versioning and lifecycle policy;
- restore after corruption or omission.

A generic WebDAV folder, mounted drive, object bucket, or provider-native conflict-copy feature does not satisfy these requirements by itself.

## Verification

Inspect:

- provider descriptor contracts and safe modes;
- conformance report version and date;
- consistency and limit statement;
- failed, skipped, and unsupported cases;
- backup restore evidence;
- two-writer and rollback fixtures when sync is claimed;
- the UI and CLI label exposed to users.

Run:

```bash
python scripts/check_providers.py
cargo test -p liaison-object-store-local
```

A descriptor without accepted conformance evidence remains draft and must not be presented as release-ready.

## Recovery

When evidence no longer supports the advertised mode:

1. suspend new jobs requiring that mode;
2. preserve existing local canonical data;
3. mark the provider evidence stale;
4. downgrade the visible safe mode;
5. verify the latest recoverable backup independently;
6. publish a knowledge and changelog update;
7. require a new grant if later evidence restores broader capability.

## Known limitations

The initial `object-store@1` contract covers object transport and manifest revision semantics. It does not itself define shared-domain reconciliation. Sharing owns operation authentication, ordering, conflicts, and materialisation.

## Related material

- `docs/architecture/provider-connections.md`
- `docs/architecture/sharing-and-synchronization.md`
- `providers/local-folder/README.md`
- `docs/evidence/providers/local-object-store-reference.md`
