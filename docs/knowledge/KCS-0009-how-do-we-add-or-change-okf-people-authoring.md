# KCS-0009: How do we add or change OKF People authoring?

- Status: verified
- Audience: workspace, People, Directory, import, and release contributors
- Last reviewed: 2026-07-19
- Applies to: OKF People profile, Markdown writer/reader, Directory projection, normalization, indexes, and interoperability claims

## Problem

A contributor needs to change a People Markdown field, add a reader or writer, normalize a legacy workspace, or claim compatibility with a newer OKF version. A locally valid YAML edit can still disclose sealed data, discard unknown Markdown, block healthy People, overwrite a curated index, or create a format claim that Liaison cannot reverse.

## Current contract

ADR 0013 pins Liaison People authoring to OKF v0.1 Draft at source commit `ee67a5ca27044ebe7c38385f5b6cffc2305a9c1a` and raw specification SHA-256 `b9655e607346dbbdc6de21190e9a953313eda6a7eba68d4d272a65975940ad6e`. OKF is the portable envelope; Liaison's domain schema owns identity, purpose, revision, provenance, information state, sensitivity, disclosure, and operational meaning.

The B0 implementation seams are deliberately separate:

- `T-B0-P05-OKF`: strict writer, schema/port, field mapping, reserved paths, Liaison extension, and plaintext-sealed-data denial;
- `T-B0-P06`: tolerant reader, bounded findings, domain-validity quarantine, healthy Directory projection;
- `T-B0-P09-OKF`: preview, exact backup, journaled failure-atomic normalization, restart recovery, idempotent rerun, curated-index preservation, and exact rollback.

P01 and P02 do not receive OKF work. P03 must establish the recoverable operation contract first.

## Resolution

Before changing the profile:

1. Identify whether the change belongs to strict writing, tolerant reading, or normalization. Do not make one task own acceptance in two gates.
2. Keep a non-empty `type: person` and the pinned supported OKF mappings. Put Liaison meaning in the versioned domain extension.
3. Treat OKF validation and Liaison domain validation as separate results. A domain-invalid fact remains inert or quarantined and cannot affect readiness.
4. Preserve unknown safe keys and sections, stable IDs, links, original body bytes outside controlled regions, malformed siblings, reserved names, and curated indexes.
5. Keep sealed facts out of plaintext frontmatter, body, indexes, projections, errors, logs, fixtures, screenshots, and reports.
6. For normalization, provide exact preview and backup, then stage and commit through `WorkspaceSession` with final preconditions, restart recovery, idempotent rerun, and exact rollback.
7. Reuse `UAT-065` for writer/interoperability/leak evidence and `UAT-066` for normalization/fault/recovery evidence. A0 consumes `UAT-065` only as B0 regression evidence.
8. Update ADR 0013, requirements, UAT, the owning task/gate/evidence record, schema examples, compatibility matrix, changelog, and release claims together.

## B0 migration boundary

The required OKF People normalization is part of B0 because the first released People writer must have one canonical portable envelope. General and third-party migrations remain outside B0. Do not use this exception to pull Meerkat, Monica, CRM-in-Markdown, broad CSV/vCard conversion, provider sync, or arbitrary import work into P01, P02, or the B0 cut line.

## Verification

- Every B0 UI and CLI writer emits the same pinned profile through one typed application port.
- Strict-write fixtures validate required OKF fields and the Liaison extension.
- Tolerant-read fixtures preserve unknown types, keys, extensions, body bytes, links, curated indexes, and malformed siblings while healthy People remain available.
- Domain-invalid or unsealed facts never enter event readiness.
- Leak scans find no sealed sensitive plaintext.
- Fault injection at every normalization boundary leaves no partial profile/index state.
- Restart recovery, idempotent rerun, and exact rollback pass.
- `python3 scripts/check_spec.py` and generated traceability checks pass.

## Common mistakes

- treating OKF as the Liaison person schema;
- adding a second writer in UI, CLI, import, or plugin code;
- copying a sealed value into plaintext for interoperability;
- rejecting an entire Directory because one sibling or optional OKF field is malformed;
- rewriting the whole Markdown body and losing unknown content;
- overwriting a curated `index.md` with a generated list;
- claiming a newer OKF draft without a version adapter, fixtures, migration, downgrade decision, and rollback;
- calling the required OKF normalizer permission for general or third-party B0 migrations.
