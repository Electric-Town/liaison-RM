# Current project status

Last reviewed: 2026-07-18  
Release status: pre-release  
Public production release: blocked

This file is the short operational source for implementation and release state. Update it when a pull request changes a material capability, test boundary, or release gate.

## Review stack

The repository currently uses stacked pull requests. Review and merge in dependency order, retargeting each descendant to `main` after its parent lands and rerunning exact-head checks.

| Order | Pull request | Scope | Current review state |
|---:|---|---|---|
| 1 | #2 | governance, KCS-informed workflow, DDD, UX and content standards | open draft |
| 2 | #3 | product specification, architecture, requirements, UAT and interaction prototype | open draft |
| 3 | #4 | Rust Workspace/People core, Markdown adapter and CLI | open draft; cross-platform Rust checks have passed on its reviewed head |
| 4 | #7 | provider-neutral Connections contract and local provider | open and ready for review; exact-head provider and Rust matrices passed |
| 5 | #8 | native Tauri desktop alpha and macOS review bundles | open draft; native source is present and platform evidence must be verified on the latest head |

Do not merge a descendant before its parent merely to place files on the default branch.

## Implemented and tested foundations

- repository policy and PR evidence checks;
- DDD, KCS-informed knowledge, UX, accessibility, security, and content standards;
- machine-readable requirements, UAT, feature gates, personas, releases, schemas, and implementation plan;
- product specification, context map, ubiquitous language, threat model, open-workspace, provider, and sharing architecture;
- static interaction prototype with desktop/mobile review screens and Chromium tests;
- Rust shared IDs and revisions;
- Workspace manifest, lifecycle services, and validation report;
- People profile, typed email/phone, partial dates, archive/restore, and repository port;
- Markdown/YAML adapter with initial unknown-field/body preservation and revision preconditions;
- CLI workspace initialise/inspect/validate and person create/list workflows;
- provider descriptors, registry, `object-store@1`, WIT contract, local-folder adapter, and conformance suite;
- native Tauri desktop alpha for local workspace and basic People workflows;
- Linux, macOS, and Windows Rust matrices;
- Apple Silicon and Intel macOS review-bundle workflow definitions.

## Capabilities under review, not release claims

- the desktop application can be built as review artifacts;
- the local-folder provider passed its bounded object-store conformance suite;
- initial Markdown records can be created and listed;
- macOS app/DMG generation and ad-hoc signature checks are part of pull-request evidence.

These statements do not mean the product is suitable as the sole copy of important data.

## R1 integrity gates still open

- journalled file replacement and interrupted-write recovery;
- stale lock handling;
- complete unknown-field and unknown-body-section compatibility coverage;
- projection delete/rebuild lifecycle;
- content-addressed attachment lifecycle;
- migration dry-run and rollback framework;
- vCard and CSV reconciliation foundations;
- backup snapshot completeness;
- checksum and manifest verification;
- isolated restore and activation;
- Airgap dependency and socket-denial evidence;
- CLI dry-run and complete stable error/output contract.

## R2 desktop and platform gates still open

- complete person edit, archive, restore, profile tabs, layouts and field ordering;
- interruption-safe form drafts;
- search and configurable dashboard framework;
- relationship list, graph and equivalent table/tree;
- localization runtime and expansion evidence;
- screen-reader, keyboard, zoom, reflow, contrast, target-size and reduced-motion evidence;
- Flatpak packaging and sandbox review;
- Windows installer and clean-machine smoke test;
- Developer ID signing, Apple notarization, stapling and Gatekeeper verification;
- clean Apple Silicon and Intel Mac UAT;
- uninstall/reinstall data-preservation evidence.

## Product contexts not yet implemented as production runtime

- Topic Packs and field-state registry;
- Organisations, Groups, Households, Locations, and Memberships;
- Relationships and relationship intent;
- Interactions and Commitments;
- Events, attendance, cohorts, and dietary readiness;
- Review and Attention;
- Knowledge and Resources;
- Reminders;
- Facilities access import;
- Sharing, roles, encrypted operations and private overlays;
- connection instances, grants, secret references and schedules;
- WebDAV, S3-compatible, Google Drive, CardDAV, CalDAV, email, and migration adapters;
- local OpenAPI, MCP, webhooks, Ollama, and WASI plugin host.

## Publication blockers

A public release remains blocked until the release's declared feature set has:

1. exact-head tests on every supported platform;
2. backup creation, verification, isolated restore, and recovery evidence;
3. schema and migration compatibility evidence;
4. platform installation and clean-machine UAT;
5. accessibility evidence for declared workflows;
6. privacy, security, threat-model, and dependency review;
7. signed release manifests and reproducible checksums;
8. macOS signing/notarization for direct distribution or an approved alternative channel;
9. no unsupported claim that planned providers, AI, sharing, or compliance features already exist;
10. changelog, knowledge, rollback, and release evidence complete.

## Immediate dependency order

1. Review and merge the existing stack in order.
2. Close R1 integrity and recovery gates before broad feature expansion.
3. Complete R2 desktop foundations and platform packaging.
4. Establish Topic Pack field-state and reason-only Review and Attention contracts.
5. Implement Organisations and Groups.
6. Implement Interactions, Commitments, Events, Resources, and Reminders as separate vertical slices.
7. Implement the workplace dietary-readiness workflow.
8. Add sharing and remote providers only after grants, backup, recovery, and audit exist.
9. Add API, MCP, AI, and plugins only after application authority is enforceable.

## Claim discipline

Use these terms precisely:

- **specified**: documented but not necessarily implemented;
- **implemented**: code exists and has local tests;
- **validated**: exact submitted commit passed named checks;
- **review artifact**: build intended for review, not public release;
- **release candidate**: complete declared gates, pending final approval;
- **published release**: signed and distributed with release evidence.

When uncertain, use the weaker claim and link the evidence.
