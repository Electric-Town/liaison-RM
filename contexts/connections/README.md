# Connections bounded context

## Purpose

Own configured external capabilities without letting provider vocabulary enter relationship, event, sharing, or AI policy.

## Language

- provider
- connection
- capability contract
- descriptor
- safe mode
- configuration field
- secret reference
- grant
- conformance
- health

## Aggregates and policies

The initial `ProviderDescriptor` protects:

- stable reverse-domain provider ID;
- semantic provider version;
- unique contract versions;
- explicit operations and safe modes;
- secret-reference field typing;
- non-empty consistency statement;
- declared network destinations;
- conformance status.

Connection and grant aggregates arrive in later slices.

In deferred R5 migration work, Connections owns the anti-corruption boundary for
Meerkat, Monica, CRM-in-Markdown, and other general and third-party source
adapters. `T-R5-005` maps provider-shaped input into staged Liaison proposals and
reconciliation evidence; provider vocabulary never becomes a Person,
Relationship, Interaction, Event, or Reminder domain model by convenience.

That R5 task exclusively owns generic migration safety `LRM-WS-007`: dry-run,
pre-migration backup, deterministic execution, validation, and rollback or
explicit irreversibility. It does not move general migration into B0 and does
not own, satisfy, or broaden the narrow B0 OKF People normalization governed by
`LRM-WS-017` and `UAT-066`.

Google Drive remains a separate R5 adapter outcome. `T-R4-008` establishes the
local-folder, WebDAV, and S3-compatible transport foundation without claiming a
Google Drive product integration. After the R4 provider-neutral backup service
and transports exist, `T-R5-010` owns `LRM-CO-012` and `UAT-074` under
`FG-R5-007`: the Google Drive adapter must reuse `object-store@1` and the shared
encrypted-backup/isolated-restore workflow, keep provider SDK types outside
product and domain code, enforce grants and egress, and avoid a multi-writer
synchronisation claim without separate evidence.

## Application use cases

- register provider;
- list providers;
- later: create connection, grant purpose, test health, export settings, import settings, pause, and revoke.

## Ports

- `ProviderRegistry`
- `ObjectStore`

Provider adapters implement ports. Other contexts consume the ports through application services and do not import provider SDKs.

## Requirements and UAT

- `LRM-CO-001` through `LRM-CO-009`
- `LRM-CO-012`
- `LRM-CO-013`
- deferred ownership of `LRM-WS-007` through `T-R5-005`
- `UAT-026` through `UAT-028`
- `UAT-034`
- `UAT-038`
- `UAT-074`
