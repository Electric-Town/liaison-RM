# Provider-neutral connections

## Problem

Liaison RM must support Google Drive, WebDAV, S3-compatible storage, AWS S3, MinIO, Google Cloud Storage, Azure Blob Storage, contact and calendar servers, email providers, local folders, removable media, and future services without allowing any vendor to shape the domain model.

A connection is therefore modelled as a configured implementation of one or more versioned capability contracts. Product workflows request capabilities; adapters translate those requests into provider-specific APIs.

## Core model

### Provider descriptor

A signed or locally trusted descriptor declares:

- stable provider ID and version;
- implementation package and licence;
- capability contracts and supported versions;
- operations;
- supported release profiles;
- configuration schema;
- secret references;
- network destinations and redirect rules;
- consistency, ordering, size, rate, and pagination limits;
- safe modes: backup, restore, single-writer publication, multi-writer transport, contact sync, calendar import, or other defined modes;
- conformance evidence and date;
- migrations and compatibility range.

A descriptor is a claim. The conformance suite and provider evidence determine whether the claim is accepted.

### Connection instance

A connection binds a provider descriptor to user-supplied configuration and secret references. It has lifecycle states:

```text
draft -> configured -> tested -> active -> suspended -> revoked
                                \-> error
```

Creating or testing a connection does not grant data access.

### Grant

A grant records:

```yaml
purpose: encrypted workspace backup
provider_id: s3-compatible
connection_id: 018f...
endpoint: https://objects.example.test
operations: [put_immutable, get, head, list, replace_manifest]
data_classes: [encrypted_backup]
record_scope: workspace:018f...
schedule: "0 2 * * *"
retention: P90D
expires_at: 2027-07-17T00:00:00Z
approved_by: member:018f...
revocable: true
```

Grant evaluation occurs in the application layer before an adapter receives data or an operation. Jobs record the grant revision used. Revocation prevents new operations and initiates provider-specific cleanup only when the user separately authorises deletion.

### Job

A job has a stable idempotency key, input manifest, status, attempt history, progress, result, affected objects, retry decision, grant revision, and local evidence path. It distinguishes `partial` from `success` and never converts an ambiguous provider response into an unconditional success.

## Capability contracts

### `object-store@1`

Required semantics:

- `put_immutable(key, bytes, expected_hash)`
- `get(key)`
- `head(key)`
- `list(prefix, cursor)`
- `delete_if_permitted(key, precondition)`
- `replace_manifest_if_revision(key, expected_revision, bytes)`

Keys are normalised relative identifiers; providers reject traversal, absolute paths, empty segments, and unsafe encoding. Immutable publication succeeds only when absent or byte-identical. Manifest replacement uses the strongest available compare-and-set mechanism and declares any weaker fallback.

### `backup@1`

- create encrypted snapshot locally;
- publish immutable chunks and manifest;
- verify remote object existence and hashes;
- list snapshots;
- apply retention with preview;
- download to an isolated restore area;
- verify manifest, signature, hash, schema, and completeness;
- restore through the workspace recovery service.

The provider never receives plaintext solely because it implements backup.

### `contacts@1`

- discover address books;
- import and export vCard;
- selected-view sync;
- stable UID and ETag/precondition support;
- unknown vCard property preservation;
- conflict reporting;
- field and group mapping.

Provider contacts are address-book representations. They are translated to People and Organisations commands rather than used as domain entities.

### `calendar@1`

- discover calendars;
- bounded date-range import;
- recurring event identity;
- source UID, recurrence ID, ETag, and content-hash preservation;
- attendee mapping;
- incremental refresh;
- deletion/tombstone policy.

### `email-metadata@1`

- bounded query by account, folder/label, participant, and time;
- header and thread metadata retrieval;
- incremental cursor;
- message and thread identity;
- source links when permitted;
- optional body access as a separately granted capability.

The default contract does not retrieve message bodies or attachments.

### `webhook@1`

- signed delivery;
- destination allow-list;
- event and field filtering;
- idempotency key;
- replay window;
- bounded retry and dead-letter evidence;
- test delivery with synthetic data.

## Adapter package shape

```text
providers/<provider>/
├── descriptor.json
├── README.md
├── schemas/
├── migrations/
├── src/
│   ├── config.rs
│   ├── client.rs
│   ├── anticorruption.rs
│   └── lib.rs
├── tests/
└── evidence/
```

The provider SDK supplies contract types, test fixtures, mock clocks, deterministic network fakes, redaction helpers, and conformance runners. It does not supply domain aggregates.

## Secrets

Descriptors name secret slots but never values. A connection stores opaque secret references. The provider host resolves a reference only for the approved operation and zeroises transient material where supported.

OAuth providers must declare:

- authorisation and token endpoints;
- redirect mechanism;
- scopes and why each is needed;
- refresh behaviour;
- token storage reference;
- revocation endpoint;
- account identity check;
- data destinations beyond the primary API.

A provider cannot add scopes during refresh without a new grant review.

## Network controls

Connected-local builds route provider traffic through an egress controller that checks:

- active connection and grant;
- declared destination and redirect target;
- operation and data class;
- schedule and expiry;
- build profile;
- rate and size limits;
- TLS policy;
- audit correlation ID.

Airgap builds do not compile the egress controller’s network implementation or provider clients.

## Backup, publication, and synchronisation labels

Provider UI labels reflect demonstrated semantics:

- **Backup** — publish and verify encrypted recovery snapshots.
- **Single-writer publication** — one authorised writer replaces manifests; other devices read.
- **Transport** — exchange immutable operations; Liaison performs reconciliation locally.
- **Multi-writer sync** — reserved for a contract and evidence covering concurrent writers, ordering, conflicts, tombstones, and recovery.

Generic WebDAV and object storage are not described as multi-writer sync merely because they support PUT and GET.

## Conformance evidence

Each accepted provider publishes:

- contract version;
- provider and SDK versions;
- test environment;
- passed, failed, skipped, and unsupported cases;
- consistency and ordering observations;
- maximum tested object/list/import sizes;
- retry and idempotency evidence;
- credential revocation result;
- redaction check;
- Airgap exclusion check;
- known limitations and safe modes.

Evidence expires when the provider API, dependency, contract, or security model changes materially.

## Adding a provider

1. Select an existing contract; propose a contract separately if none fits.
2. Create descriptor and configuration schema.
3. Implement provider DTOs and anti-corruption translation.
4. Use secret references and declared destinations.
5. Pass the conformance suite using synthetic accounts or local emulators.
6. Document safe modes and limitations.
7. Add knowledge for setup, test, revocation, backup/restore, and recovery.
8. Add provider-specific threat analysis.
9. Keep the pull request separate from unrelated domain changes.
