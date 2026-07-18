# Current project status

Last reviewed: 2026-07-18  
Release status: pre-release  
Public production release: blocked

This file is the short operational source for implementation and release state. Update it when a pull request changes a material capability, test boundary, or release gate.

## Merged foundation

The following review layers are merged into `main`:

| Pull request | Scope |
|---:|---|
| #2 | governance, KCS-informed workflow, DDD, UX and content standards |
| #3 | product specification, architecture, requirements, UAT and interaction prototype |
| #4 | Rust Workspace/People core, Markdown adapter and CLI |
| #7 | provider-neutral Connections contract and local provider |
| #9 | relationship-memory, Topic Pack, field-state and review-policy specification |
| #10 | Identity and Profiles readiness runtime and reason-only Review and Attention runtime |

## Active review work

| Pull request | Scope | State and action |
|---:|---|---|
| #8 | native Tauri desktop alpha and macOS review bundles | open draft based on `main`; verify latest exact-head Mac artifacts before ready |
| #13 | localization architecture and human-review gates | open draft; rebase or retarget after resolving its stale relationship-contract base |
| #17 | repository README, project context and agent handoff | open draft stacked on #8 |

## Overlapping drafts requiring reconciliation

The following open drafts overlap contracts already merged through #9 and #10 or each other:

- #12: Topic Pack and reason-based relationship-review contract on an older desktop base;
- #15: relationship intent and Topic Pack documentation on a stale relationship base;
- #16: Topic Pack/Review and Attention contract stacked on the current desktop branch.

Do not merge these as cumulative work. Compare their unique content against `main`, preserve only non-duplicate validated improvements, then retarget or close them with an explanatory comment.

## Implemented and tested foundations

- repository policy and PR evidence checks;
- DDD, KCS-informed knowledge, UX, accessibility, security, and content standards;
- machine-readable requirements, UAT, feature gates, personas, releases, schemas, and implementation plan;
- product specification, context map, decisions, threat model, open-workspace, provider, and sharing architecture;
- static interaction prototype and relationship-review screens with browser checks;
- Rust shared IDs and revisions;
- Workspace manifest, lifecycle services, and validation report;
- People profile, typed email/phone, partial dates, archive/restore, and repository port;
- Markdown/YAML adapter with initial unknown-field/body preservation and revision preconditions;
- CLI workspace initialise/inspect/validate and person create/list workflows;
- provider descriptors, registry, `object-store@1`, WIT contract, local-folder adapter, and conformance suite;
- Topic Pack stable IDs, field definitions, classification, explicit information states and sealed-value invariants;
- versioned Purpose Definitions and purpose-specific readiness gaps;
- reason-only review policies, factual reasons, hard suppressions, deterministic ordering and capacity bounds;
- native Tauri desktop alpha for local workspace and basic People workflows on the active branch;
- Linux, macOS, and Windows Rust matrices;
- Apple Silicon and Intel macOS review-bundle workflow definitions.

## Capabilities under review, not release claims

- the desktop application can be built as review artifacts;
- the local-folder provider passed its bounded object-store conformance suite;
- initial Markdown records can be created and listed;
- profile readiness and reason-only queues execute as domain logic but do not yet persist profile configuration or review sessions;
- macOS app/DMG generation and ad-hoc signature checks are review evidence only.

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

## Profile and review gates still open

- Markdown persistence for Field Definitions, Topic Packs, profile values and Purpose Definitions;
- Topic Pack activation inheritance across workspace, template, organisation, group, person and plugin;
- actual encryption/sealing adapter and key lifecycle;
- profile layouts and editing;
- cadence, interactions, commitments, dates and event adapters as factual review inputs;
- maintenance-status calculation;
- review-session persistence and interruption recovery;
- monthly Markdown review output;
- weighted policy simulation and UAT;
- shared-workspace privacy enforcement for assessments and sealed fields.

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

- Organisations, Groups, Households, Locations, and Memberships;
- Relationships and relationship-intent persistence;
- Interactions and Commitments;
- Events, attendance, cohorts, and dietary readiness;
- Knowledge and Resources;
- Reminders;
- Facilities access import;
- Sharing, roles, encrypted operations and private overlays;
- connection instances, grants, secret references and schedules;
- WebDAV, S3-compatible, Google Drive, CardDAV, CalDAV, email, and migration adapters;
- local OpenAPI, MCP, webhooks, Ollama, and WASI plugin host.

## Publication blockers

A public release remains blocked until the declared feature set has:

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

1. Reconcile or close overlapping drafts #12, #15, and #16.
2. Complete and review #8 against the latest `main`.
3. Land #17 after #8 or retarget it to `main` if the desktop work lands first.
4. Close R1 integrity and recovery gates before broad feature expansion.
5. Persist profile/readiness data and connect reason-only review to factual sources.
6. Complete localization architecture before proliferating UI strings.
7. Implement Organisations and Groups before event cohorts.
8. Implement Interactions, Commitments, Events, Resources, and Reminders as separate vertical slices.
9. Implement the workplace dietary-readiness workflow.
10. Add sharing and remote providers only after grants, backup, recovery, and audit exist.
11. Add API, MCP, AI, and plugins only after application authority is enforceable.

## Claim discipline

Use these terms precisely:

- **specified**: documented but not necessarily implemented;
- **implemented**: code exists and has local tests;
- **validated**: exact submitted commit passed named checks;
- **review artifact**: build intended for review, not public release;
- **release candidate**: complete declared gates, pending final approval;
- **published release**: signed and distributed with release evidence.

When uncertain, use the weaker claim and link the evidence.
