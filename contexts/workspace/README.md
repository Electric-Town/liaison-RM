# Workspace bounded context

## Purpose

Workspace owns the identity and lifecycle of a Liaison workspace. It defines the manifest invariants, supported schema version, build-profile declaration, enabled workspace profile, validation report, and ports required to initialise and inspect storage.

## Language

- **Workspace** — the local-authoritative collection of canonical records and supporting control data.
- **Manifest** — the readable declaration of workspace identity, schema, profile, locale, and modules.
- **Validation finding** — a non-destructive description of a defect, severity, path, and recovery action.
- **Build profile** — evidence that the last writer was Airgap or Connected-local; it does not change binary capability.

## Invariants

- Workspace ID is stable.
- Name and locale are non-empty.
- Format and schema version are explicit.
- Initialisation refuses an existing manifest.
- Validation does not silently delete or rewrite invalid records.
- Projections remain disposable.

## Application services

- `InitialiseWorkspace`
- `ValidateWorkspace`

Later releases add open, lock, migrate, rebuild, repair, backup, and restore services.

## Outbound ports

`WorkspaceStore` is owned by this context. Storage adapters implement it. The current R1 draft accepts a workspace path at the application boundary; a later workspace-session service will bind an opened path to a stable workspace handle before concurrent desktop and local-service work begins.

## Upstream and downstream

Every bounded context depends on an opened workspace session for identity and policy. Workspace does not import their domain models.

## Data classification

The manifest is normally `shared` or `private`. It contains no credentials or private keys.
