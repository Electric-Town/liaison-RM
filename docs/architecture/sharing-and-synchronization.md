# Sharing and synchronisation

## Principle

Sharing is exchange of authorised domain operations, not concurrent editing of a plaintext vault through an arbitrary remote folder. A readable local Markdown view remains canonical for each workspace member, while remote transports carry encrypted immutable objects and signed manifests.

## Why shared plaintext WebDAV is insufficient

A general shared directory cannot reliably provide Liaison RM with:

- record-level authorisation;
- field-level private overlays;
- atomic multi-file domain changes;
- durable device acknowledgement;
- predictable conflict handling across clients;
- tombstone and retention semantics;
- safe credential and key rotation;
- least-disclosure exports;
- recovery from partial upload or provider conflict copies.

WebDAV remains useful as an object transport after Liaison provides those semantics.

## Identities and keys

A workspace has a stable ID and a workspace signing policy. Each member and device has:

- stable identifier;
- signing public key;
- encryption public key;
- role and scoped grants;
- registration and revocation operations;
- last acknowledged operation per stream.

Private keys remain local or in platform-backed secret storage. Recovery packages are explicit, encrypted, and testable; the project does not silently escrow keys with Electric Town.

## Operation envelope

```json
{
  "format": "liaison-operation",
  "version": 1,
  "operation_id": "018f...",
  "workspace_id": "018f...",
  "stream": "people",
  "aggregate_id": "018f...",
  "base_revision": 7,
  "result_revision": 8,
  "command": "people.update-dietary-coverage",
  "actor_id": "member:018f...",
  "device_id": "device:018f...",
  "occurred_at": "2026-07-17T12:00:00Z",
  "classification": "restricted",
  "ciphertext": "...",
  "cipher": "xchacha20poly1305",
  "key_envelopes": [],
  "previous_operation_hash": "sha256:...",
  "operation_hash": "sha256:...",
  "signature": "..."
}
```

The outer envelope contains only routing and verification fields permitted by workspace policy. Domain payload is encrypted to authorised recipients.

## Materialisation

A receiving device:

1. downloads immutable operations and manifest revisions;
2. verifies provider object hash;
3. verifies workspace, actor, device, signature, chain, and schema;
4. evaluates local authorisation and revocation state;
5. decrypts the payload when it has a valid key envelope;
6. validates the command through the owning application service;
7. detects revision conflict or missing dependency;
8. applies the operation to the local canonical workspace;
9. rebuilds affected projections;
10. publishes an acknowledgement or conflict operation.

Transport cannot apply raw patches to Markdown files.

## Conflict policy

Automatic reconciliation is allowed only for contract-defined independent changes, such as distinct profile fields with compatible base provenance or append-only interaction entries. Conflicts that affect the same invariant produce an inspectable conflict record.

A conflict view shows:

- record and context;
- base revision;
- local and incoming operations;
- field-level differences;
- provenance and actor;
- classification and disclosure scope;
- available merge actions;
- effect on dependent summaries, events, and exports;
- recovery and undo options.

No conflict is resolved by silently choosing the latest wall-clock timestamp.

## Roles and scope

Initial roles are owner, workspace administrator, relationship editor, executive assistant, receptionist, event manager, dietary coordinator, facilities importer, read-only member, and guest contributor.

Policies can constrain:

- bounded context;
- organisation, location, group, event, or saved cohort;
- record type and identifier;
- field and classification;
- read, propose, write, export, disclose, delete, or administer operation;
- purpose and expiry;
- provider and destination.

Role names provide defaults. Effective authority is derived from scoped grants.

## Private overlays

An overlay is a separate aggregate linked to a shared record. It has an independent authorised member set and encryption key. Shared summaries, search, AI, exports, and provider jobs exclude overlay content unless explicitly granted.

Examples:

- an EA’s private preparation note associated with a shared contact;
- a family member’s private gift idea;
- a manager’s personal relationship note that must not enter workplace operations;
- a medical-detail note separated from a catering instruction.

Overlay existence may itself be sensitive and should not be exposed to unauthorised members.

## Self-service requests

A `.liaison-request` is a signed package containing requester, requested fields or claims, purpose, disclosure policy, expiry, response key, and return method. A recipient can open it in Liaison RM or a small offline responder, choose what to provide, and return a signed encrypted `.liaison-response`.

The recipient is shown:

- who is asking;
- why;
- exact requested information;
- optional versus required fields;
- expiry and retention statement;
- who will receive the response;
- how to decline or provide less detail.

The importer preserves source, consent, purpose, and verification date. A response never silently overwrites a newer local value.

## Liaison Cards

A card contains selected signed claims and update policy. Cards can be transported by file, QR payload for small cards, local exchange, WebDAV, or object provider. A card can publish an updated claim but cannot remotely erase historical evidence from another workspace; revocation affects trust and future refresh.

## Transports

### Local folder or removable media

Encrypted operation packs and manifests are exported to a chosen directory or removable device. Airgap builds support this transport. Import is previewed and does not auto-run executable content.

### WebDAV

Objects use deterministic keys and immutable writes. Manifests use ETag preconditions where available. The adapter records server behaviour and refuses stronger sync labels than its conformance evidence supports.

### S3-compatible object stores

Immutable objects use content hashes or operation IDs. Manifest updates use conditional requests and versioning when available. Bucket lifecycle policy is surfaced because it can delete recovery data independently of Liaison retention settings.

### Provider drive APIs

Drive-style providers store opaque encrypted objects and manifests in an application folder selected or created through a grant. Provider-native conflict copies are detected and imported as conflicts rather than accepted as latest.

## Deletion and revocation

Deletion semantics are explicit:

- archive locally;
- delete canonical local record;
- issue shared tombstone;
- delete provider object if grant permits;
- expire a retention class;
- revoke member/device keys;
- rotate keys for future operations;
- cryptographically revoke access to future updates.

The UI never promises that a previously authorised recipient has erased an exported plaintext copy. It records what Liaison controlled and what remains outside its control.

## Recovery

The workspace supports:

- operation-chain validation;
- missing-object detection;
- provider manifest rollback when version history exists;
- restore into an isolated workspace;
- member/device re-registration;
- key recovery from explicit recovery packages;
- projection rebuild;
- conflict replay;
- transport replacement without changing domain identity.

Recovery procedures are exercised and recorded as release evidence before a sharing feature gate opens.
