---
id: KCS-0004
title: How do I decide whether a provider can share, sync, or only back up?
state: Validated
owner: connections
created: 2026-07-17
reviewed: 2026-07-18
applies_to:
  - object-store contract 1
search_terms:
  - provider conformance
  - S3 compatible
  - WebDAV
  - Google Drive
  - backup only
  - multi writer
related_requirements:
  - LRM-CO-001
  - LRM-CO-004
  - LRM-CO-005
related_uat:
  - UAT-027
  - UAT-034
related_adrs:
  - ADR-0003
---

# How do I decide whether a provider can share, sync, or only back up?

## Context

A provider can store and retrieve files, but a contributor needs to know whether Liaison RM may use it for concurrent workspace exchange.

## Answer

Run the provider conformance suite against the configured adapter and service. A descriptor's safe-mode label is bounded by the exact tested behavior and its evidence record; it does not make an incomplete Workspace recovery workflow complete.

Use the result:

| Proven provider behavior | Maximum provider-transport label |
|---|---|
| immutable put, get, head, list, and guarded deletion; used only with Liaison's separately verified encrypted recovery package | backup transport |
| backup transport plus one controlled publisher and proven manifest replacement | single-writer publication |
| single-writer publication plus concurrent operation/reconciliation and conditional-manifest tests | multi-writer synchronization |

These labels describe the provider transport. They do not open Liaison's encrypted-recovery or sharing release gates by themselves.

Do not infer multi-writer safety from a provider name, an S3-compatible claim, versioning, or a successful upload.

The current pre-alpha CLI does not implement `liaison provider conformance`. For the local reference adapter, run the checked-in source and behaviour checks:

```bash
python3 scripts/check_providers.py
python3 scripts/check_wit_contract.py
cargo test \
  -p liaison-connections \
  -p liaison-provider-registry-memory \
  -p liaison-object-store-local \
  -p liaison-provider-sdk \
  --all-features --locked
```

`check_wit_contract.py` verifies the committed contract shape. It is not a WIT parser or a component-host execution test.

## Why this works

Encrypted operation exchange needs immutable objects and a manifest update that fails when another writer changed the expected revision. Last-writer-wins replacement can lose operations without a visible conflict.

## Verify

The conformance report includes:

- immutable overwrite rejection;
- checksum mismatch rejection;
- read and head agreement;
- prefix listing;
- guarded deletion with the expected digest;
- absent-manifest creation;
- stale-revision rejection;
- current-revision replacement.

## Limits and recovery

A conformance result applies to the tested adapter, service configuration, endpoint, and time. Provider upgrades or server policy changes require a rerun.

The current local adapter is `passed-with-limits` for `object-store@1` and remains limited to backup transport or controlled single-writer use. Its cooperative lock does not protect against arbitrary non-cooperating writers or all network filesystems. The result does not prove a complete encrypted recovery package, isolated restore, or multi-writer synchronization.
