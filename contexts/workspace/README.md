# Workspace bounded context

## Purpose

Workspace owns the identity and lifecycle of a Liaison workspace. It defines manifest invariants, the supported schema version, build-profile declaration, validation, local backup eligibility, isolated restore activation, and the ports required to initialise, inspect, back up, verify, and restore storage.

## Language

- **Workspace** — the local-authoritative collection of canonical records and supporting control data.
- **Manifest** — the readable declaration of workspace identity, schema, profile, locale, and modules.
- **Validation finding** — a non-destructive description of a defect, severity, path, and recovery action.
- **Build profile** — evidence that the last writer was Airgap or Connected-local; it does not change binary capability.
- **Backup snapshot** — an immutable manifest and payload set whose files, sizes, and SHA-256 digests are explicit.
- **Isolated restore** — a restore into a new target that is validated before the restore marker is removed.

## Invariants

- Workspace ID is stable.
- Name and locale are non-empty.
- Format and schema version are explicit.
- Initialisation refuses an existing manifest.
- Validation does not silently delete or rewrite invalid records.
- Projections remain disposable and are excluded from canonical backup snapshots.
- A backup is not successful until every declared payload verifies.
- A restore never overwrites an existing directory.
- Restore activation requires matching workspace identity and schema plus no error-level layout finding.
- Cleanup can remove only a restore target that retains the context-owned in-progress marker.

## Application services

- `InitialiseWorkspace`
- `ValidateWorkspace`
- `CreateWorkspaceBackup`
- `VerifyWorkspaceBackup`
- `RestoreWorkspaceBackup`

Later releases add open-session binding, locking, migration, rebuild, repair, encrypted backup packaging, retention, and provider publication.

## Outbound ports

- `WorkspaceStore`
- `WorkspaceBackupStore`

Storage adapters implement these ports. The current R1 draft accepts paths at the application boundary. A later workspace-session service will bind an opened path to a stable workspace handle before concurrent desktop and local-service work begins.

## Upstream and downstream

Every bounded context depends on an opened workspace session for identity and policy. Workspace does not import their domain models. Connections may publish an already-created encrypted backup through a provider, but provider upload does not define backup completeness or restore success.

## Data classification

The workspace manifest is normally `shared` or `private`. It contains no credentials or private keys. A local backup currently preserves the source classification and is not encrypted by this slice; users must not place it on an untrusted provider or medium.
