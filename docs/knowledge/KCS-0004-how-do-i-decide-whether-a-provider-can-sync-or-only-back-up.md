---
id: KCS-0004
title: How do I decide whether a provider can share, sync, or only back up?
state: Draft
owner: connections
created: 2026-07-17
reviewed: 2026-07-17
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
  - REQ-161
  - REQ-162
  - REQ-163
related_uat:
  - UAT-089
  - UAT-090
  - UAT-092
related_adrs:
  - ADR-0003
---

# How do I decide whether a provider can share, sync, or only back up?

## Context

A provider can store and retrieve files, but a contributor needs to know whether Liaison RM may use it for concurrent workspace exchange.

## Answer

Run the provider conformance suite against the configured service.

Use the result:

| Proven capability | Allowed label |
|---|---|
| immutable put, get, head, list | backup |
| backup plus one controlled writer | single-writer |
| single-writer plus safe conditional manifest replacement under concurrent tests | multi-writer |

Do not infer multi-writer safety from a provider name, an S3-compatible claim, versioning, or a successful upload.

For the local reference adapter:

```bash
liaison provider conformance \
  --provider org.electric-town.local-folder \
  --path ./provider-test
```

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

The current local adapter remains backup or single-writer even when its basic suite passes. Its cooperative lock does not protect against arbitrary non-cooperating writers or all network filesystems.
