# 0011: Separate local checkpoints from encrypted recovery packages

- Status: accepted
- Date: 2026-07-18
- Deciders: Electric Town maintainer
- Contexts: workspace, workspace security, connections
- Requirements: LRM-CO-008, LRM-CO-009
- Feature gates: FG-R1-005, FG-R4-003

## Context and problem

A deterministic copied directory can prove accidental corruption and support diagnostics, but it exposes canonical data and cannot restore encrypted values on a clean machine by itself. Calling that copy a secure backup overstates its guarantee.

## Decision

Liaison provides two differently named products:

1. A **local checkpoint** is a quiescent, deterministic manifest of canonical files and directories with sizes and hashes. It excludes locks, pending operations, and projections after session coordination. A diagnostic checkpoint may preserve an invalid workspace and is labelled non-activatable; an activatable checkpoint passes workspace validation.
2. An **encrypted recovery package** is a versioned, user-portable encrypted archive containing canonical data, integrity manifests, minimal audit, and the passphrase-wrapped workspace recovery envelope. It restores on a clean Mac with no prior Keychain state.

Provider upload occurs only after local encrypted-package validation. Upload completion is not recovery success; an isolated restore exercise is the proof.

## Consequences

- PR #25 may donate manifest, path, tamper, and isolated-restore mechanics only after Workspace Session and recoverable operations land.
- Its initial terminology changes from backup to checkpoint.
- Real dietary data is not considered recoverable until the encrypted package passes clean-install restore.

## Migration, rollback, or reversal conditions

Checkpoint and recovery formats have independent identifiers and versions. Encryption cannot be added invisibly to the checkpoint format, and a provider object transport cannot redefine either format.
