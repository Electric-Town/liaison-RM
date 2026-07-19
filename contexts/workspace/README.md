# Workspace bounded context

## Purpose

Workspace owns the identity, authority, and lifecycle of a Liaison workspace. It defines the manifest invariants, supported schema version, build-profile declaration, enabled workspace profile, validation report, writer-authority contract, quiescence barrier, and ports required to initialise or open storage.

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
- An open write session owns one composite operating-system authority: the
  workspace-local lock and the per-user lock keyed by stable Workspace ID.
- For current cooperating, ordinary unconfined Liaison processes on one OS
  account and machine, copying or renaming a workspace does not transfer or
  duplicate a live writer lease; only explicit close or process-handle cleanup
  releases that lease. The canonical identity-authority namespace is
  independent of process `HOME`/XDG overrides and never falls back when it is
  inaccessible.
- Cross-container coordination is unsupported until a reviewed shared broker
  or authority namespace exists. Flatpak is denied by the local adapter;
  macOS App Sandbox and Windows AppContainer GUI/host-CLI pairings are not
  current authority claims. Older builds, another account or machine, and
  hostile direct writes also remain outside this coordination boundary.
- Diagnostic metadata cannot grant, steal, or release writer authority.
- Authority and repositories derive from the same retained root capability.
- New work is rejected once quiescence starts; issued work drains before the
  authority handle is released.
- Projections remain disposable.

## Application services

- `InitialiseWorkspace`
- `OpenWorkspaceSession`
- `ValidateWorkspace`

`WorkspaceSession` is an `Arc`-owned, non-`Clone` capability aggregate. Its
work guard is the only public route to session-bound repositories. Recovery,
key, and projection state are explicit unavailable variants until their owning
phases deliver real capabilities. Later releases add recoverable operations,
migration, rebuild, repair, checkpoint, and encrypted recovery services.

## Outbound ports

`WorkspaceStore` is the bootstrap and one-shot inspection port.
`WorkspaceWriterAuthorityPort`, `BoundWorkspaceStore`, and
`BoundWorkspaceSessionPort` compose one authority-bearing binding without
passing a raw path into later operations. Storage adapters implement these
ports and expose path-free repositories only through the session work guard.

## Upstream and downstream

Every bounded context depends on an opened workspace session for identity and policy. Workspace does not import their domain models.

## Data classification

The manifest is normally `shared` or `private`. It contains no credentials or private keys.
