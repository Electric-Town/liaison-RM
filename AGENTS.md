# Agent and contributor operating contract

This file is normative for human contributors and automated coding agents.

## Required read order

Before selecting or changing work, read:

1. `README.md` for the public product boundary;
2. `docs/PROJECT_CONTEXT.md` for the complete handoff;
3. `docs/STATUS.md` for current implementation and release gates;
4. `SPEC.md` for normative product requirements;
5. `AI_BUILD_INSTRUCTIONS.md` for execution order;
6. the owning bounded-context README and tests;
7. related decisions in `docs/decisions/`;
8. related knowledge articles in `docs/knowledge/`;
9. machine-readable requirements, UAT, feature gates, schemas, and implementation tasks;
10. `CHANGELOG.md` and the current pull-request stack.

Do not infer missing context from branch names or unfinished code. When authoritative sources conflict, stop and record the conflict in the pull request or propose a decision record.

## Product boundary

Liaison RM is a local-authoritative relationship memory and attention system. Canonical records are open files controlled by the user. No contribution may introduce:

- a mandatory hosted account or Electric Town backend;
- hidden telemetry, crash upload, remote logging, licence check, or update request;
- an undeclared network request;
- a provider-specific dependency in a domain crate;
- a second implementation of domain rules in UI, CLI, API, MCP, jobs, or plugins;
- an opaque database as the only source of truth;
- a relationship-strength, employee-value, trust, affection, productivity, or social-credit score inferred from activity volume.

## Core product distinctions

Keep these concepts separate:

- **relationship intent** is manually configured;
- **relationship evidence** consists of recorded or imported facts;
- **maintenance status** is an explainable calculation relative to the relationship's own policy;
- **profile readiness** is calculated for a stated purpose;
- **Review Priority** orders an attention queue and is not a measure of a person or relationship.

An empty field is not a negative fact. Field state must be explicit when meaning matters.

## Required working sequence

1. Identify the user problem and supplied or observed evidence.
2. Name the owning bounded context and its ubiquitous language.
3. Select one dependency-ready vertical slice.
4. Add failing domain, contract, recovery, accessibility, or integration tests.
5. Implement domain behaviour and application services before external surfaces.
6. Add or change ports owned by the consuming context.
7. Implement adapters without leaking provider or persistence vocabulary into the domain.
8. Expose the use case through CLI before adding equivalent desktop, API, MCP, or plugin behaviour where practical.
9. Update requirements, UAT, feature gates, schemas, knowledge, status, and changelog as applicable.
10. Run exact-head checks and report what did not run.
11. Complete the pull-request template with risk and rollback.

## Domain-driven design rules

- A bounded context owns its model, vocabulary, invariants, commands, queries, and events.
- Cross-context calls use explicit application interfaces, versioned events, read models, or anti-corruption layers.
- Domain crates must not import Tauri, React, HTTP clients, SQL clients, filesystem libraries, provider SDKs, secret stores, or OS APIs.
- Shared code enters the shared kernel only when its meaning is identical across contexts and the coupling is deliberate.
- Provider names such as Google Drive, S3, WebDAV, Gmail, or CardDAV do not appear in business entities unless provider identity is itself domain data.
- Persistence models, API DTOs, WIT records, form state, and provider payloads do not become domain entities by convenience.
- UI components do not calculate readiness, maintenance status, permissions, or Review Priority independently.
- A generic `metadata` map is not a substitute for an owned concept.

## Local-integrity rules

- Canonical Markdown, YAML, and JSONL formats are documented and versioned.
- Format structs remain separate from domain entities.
- Unknown fields and supported body sections survive round trips.
- Writes validate before replacement and use revision preconditions, content hashes, recovery journals, and interruption tests.
- SQLite, full-text indexes, caches, graph layouts, and thumbnails are rebuildable projections.
- Invalid records remain visible to validation and repair; they are not silently dropped.
- Remote connections are disabled until the user creates a purpose-bound grant.
- Secrets are references to an OS or approved secret store and never appear in canonical records, logs, fixtures, screenshots, exports, or PR descriptions.
- Airgap builds compile out network clients and listeners. A setting alone is insufficient.
- Destructive operations require preview, confirmation, audit evidence, and a recovery path where technically possible.
- Tests use synthetic people, organisations, domains, addresses, events, and access records.

## Provider, sharing, AI, and plugin rules

- Provider registration grants no authority.
- A provider implements a versioned capability contract and publishes consistency limits and conformance evidence.
- Backup, single-writer publication, and multi-writer synchronization are separate claims.
- Sharing owns signed operations, ordering, acknowledgement, reconciliation, conflict handling, member/device identity, and key envelopes.
- A shared plaintext directory is not a safe multi-writer database.
- AI reads identify their source records and grant.
- AI writes are staged proposals by default and pass ordinary validation.
- Plugins and MCP clients receive no ambient network, filesystem, database, secret, or cross-workspace access.
- Removing a connector does not silently delete imported local records; retention and deletion policy decides.

## User experience review

Every user-facing change uses `docs/standards/ux-review.md`. At minimum, review:

- interruption recovery and cognitive load for ADHD and AuDHD users;
- stable navigation, visible focus, keyboard and screen-reader operation;
- WCAG 2.2 Level AA and applicable EN 301 549 evidence;
- 200% zoom, reflow, text expansion, target size, contrast, and reduced motion;
- all ten Nielsen heuristics;
- AskTog interaction principles;
- Gestalt grouping and information hierarchy;
- relevant IxDF research topics;
- empty, loading, partial, stale, conflict, permission, success, undo, and recovery states;
- semantic alternatives to graph-only, colour-only, hover-only, and drag-only interaction;
- non-shaming language and capacity-bounded review experiences.

Do not claim formal accessibility or legal compliance without the corresponding evidence.

## Content quality

Automated text is not rejected because of its origin. It is rejected when it is vague, repetitive, falsely confident, unaccountable, or disconnected from evidence. Follow `docs/standards/content-quality.md`.

Do not:

- insert deliberate mistakes to appear human;
- use an AI detector or stylometric guess as an authorship gate;
- claim research, testing, compliance, security, privacy, packaging, or platform support that did not occur;
- leave generic promotional prose where a technical decision, failure condition, source, or measurable result is required;
- generate comments that merely restate code;
- describe planned functionality in the present tense;
- hide a failed check behind a broad “tests passed” statement.

## Pull-request and branch rules

- Keep one coherent context or vertical slice per pull request.
- Use a draft while dependencies, checks, evidence, or migrations are incomplete.
- Stacked pull requests name their base and merge order explicitly.
- Do not merge around the stack merely to make files appear on `main`.
- Generated files must be reproducible and included only when the review or release process needs them.
- Temporary payload, diagnostic, repair, or self-modifying workflows do not belong in a final diff.
- Update `docs/STATUS.md` when a change materially alters implemented capability or a release gate.

## Required checks

Run the checks relevant to the submitted diff. The baseline is:

```bash
python scripts/check_repository.py
python scripts/check_spec.py
python scripts/check_architecture.py
python scripts/check_providers.py
python scripts/check_wit_contract.py

cargo fmt --all --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

Add context, schema, provider, migration, backup/restore, accessibility, packaging, and platform checks when the change affects them.

## Completion gate

A pull request is not ready for review until:

- the problem, persona, scope, owner context, and ubiquitous-language change are explicit;
- tests and repository checks pass on the submitted commit or every failure is disclosed;
- requirements, UAT, feature gates, status, knowledge, and changelog decisions are recorded;
- migration, compatibility, privacy, security, accessibility, localization, and rollback effects are addressed;
- generated files are reproducible;
- no unrelated change is included;
- the implementation does not claim a release gate that its evidence has not satisfied.

Draft pull requests may be incomplete, but their description must state exactly what remains incomplete.
