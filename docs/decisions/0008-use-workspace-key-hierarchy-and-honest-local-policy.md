# 0008: Use a workspace key hierarchy and honest local authorization

- Status: accepted
- Date: 2026-07-18
- Deciders: Electric Town maintainer
- Contexts: workspace security, profiles, events, sharing
- Requirements: LRM-PE-008, LRM-PE-009, LRM-PE-010, LRM-CO-006
- Feature gates: FG-R3-001, FG-R3-004

## Context and problem

Dietary requirements and their association with a named attendee are sensitive. A plaintext value carrying `sealed: true` is not encryption. A macOS Keychain entry alone is not a recovery strategy, and local role labels do not create isolation from the person who controls the unlocked operating-system account and workspace files.

## Decision

Workspace Security owns a random workspace data-encryption key, authenticated sealed envelopes, key rotation, a passphrase-wrapped recovery envelope, and optional macOS Keychain caching. Authenticated context binds the workspace, aggregate, field, schema, and revision. Sensitive persistable types contain ciphertext envelopes or absence; they cannot represent sensitive plaintext plus a marker.

The initial policy boundary is one trusted local workspace owner and stable device. Purpose grants constrain capability, scope, purpose, expiry, retention, and approver. Role names are convenience presets that materialize grant bundles. Application services evaluate grants before decrypting or projecting sensitive data.

Liaison does not claim multi-user confidentiality, non-repudiation, or protection from a person controlling the unlocked account. Missing, expired, or revoked authority fails closed without placing secret values in errors, logs, browser state, projections, or audit.

## Consequences

- There is no plaintext fallback when a key or grant is unavailable.
- Clean-install recovery must succeed without the original Keychain entry.
- Cryptographic algorithms and envelope versions require registries, fixed vectors, tamper tests, nonce tests, and migration rules.
- Real sensitive records cannot enter B0 fixtures until these gates pass.

## Migration, rollback, or reversal conditions

The current profile `payload` plus `sealed` shape is pre-release and must be replaced before persistence. A cryptographic change requires versioned migration and recovery evidence; it cannot reinterpret existing envelopes in place.
