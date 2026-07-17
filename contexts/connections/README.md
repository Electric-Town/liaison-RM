# Connections bounded context

## Purpose

Connections owns provider-neutral capability contracts, provider descriptors, configured connections, purpose-bound grants, jobs, conformance claims, and revocation. It prevents Google Drive, WebDAV, S3, CardDAV, calendars, email, webhooks, and future services from defining business-domain models.

## Language

- **Provider descriptor** — a versioned declaration of contracts, operations, destinations, secret slots, consistency, limits, safe modes, and evidence.
- **Connection** — a configured instance of a provider descriptor.
- **Grant** — explicit authority binding purpose, provider, endpoint, operations, data classes, record scope, schedule, retention, expiry, and approver.
- **Job** — one idempotent or explicitly qualified execution under a specific grant revision.
- **Capability contract** — reusable semantics such as `object-store@1` or `backup@1`.
- **Safe mode** — a behaviour demonstrated by conformance evidence, such as backup or single-writer publication.

## Invariants

- A descriptor must name at least one versioned contract.
- Duplicate contract claims are invalid.
- Creating a connection grants no access.
- A grant requires purpose, operations, data classes, and record scope.
- Expired or revoked grants permit no new operation.
- Provider capability labels cannot exceed current conformance evidence.
- Object keys reject traversal, absolute paths, platform drive prefixes, backslashes, empty segments, and unsafe separators.
- Immutable publication never overwrites different content.
- Manifest replacement requires a revision precondition.

## Ports

- `ProviderRegistry`
- `ObjectStore`

Provider and storage adapters implement these ports. A later `EgressController` application port evaluates destinations and grants before a network adapter receives an operation.

## Context relationships

Business contexts request capabilities through application workflows. They do not import provider descriptors or SDK DTOs. Sharing uses object-store transports for encrypted operations but owns reconciliation and conflict semantics. Workspace owns backup content and restore activation; Connections owns provider publication.

## Data classification

Descriptors are public or shared configuration. Connection configuration is private. Grants can be restricted because they describe disclosure scope. Credentials and private keys are `secret` and appear only as opaque references in later connection records.
