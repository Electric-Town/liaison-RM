# Agent and contributor operating contract

This file is normative for human contributors and automated coding agents.

## Mandatory first reads

Before selecting or implementing work, read:

1. `PROJECT_CONTEXT.md` — complete product, architecture, status, and handoff context.
2. `docs/product/working-state-delivery.md` — accepted B0-before-A0 delivery and claim boundary.
3. `spec/traceability-ownership.json` and generated `docs/product/traceability.md` — exact requirement, UAT, gate, task, milestone, status, and evidence ownership.
4. `SPEC.md` — product and build contract.
5. `AI_BUILD_INSTRUCTIONS.md` — executable order of work.
6. The owning bounded-context README and tests.
7. Relevant decisions, knowledge articles, requirements, UAT cases, feature gates, implementation tasks, and open pull requests.

Confirm the repository, branch, base, exact head, and current CI state. Do not assume an open pull request is part of `main`, a prototype is production code, or a passing unit test proves a platform or release claim.

## Product boundary

Liaison RM is a local-authoritative relationship memory and attention system. Canonical records are open files controlled by the user. No contribution may introduce a mandatory hosted account, hidden telemetry, remote licence check, undeclared network request, or provider-specific dependency into the domain model.

The product is not a sales pipeline and does not measure human worth, closeness, trust, affection, employee performance, productivity, attendance compliance, or risk from communication or access volume.

## Current delivery boundary

The first independently reviewable product is **B0 Workplace Review Alpha**. B0 assumes one trusted local workspace owner and covers local Directory onboarding, event cohort finalisation, exact dietary-readiness reconciliation, purpose-bound least disclosure, immutable brief evidence, verified delivery, recovery, and installed-Mac evidence. It does not allocate, rank, or score relationships. **A0 Personal Memory Alpha starts only after B0 is accepted.** Personal interactions, commitments, reason-only Review, user-organised profile tabs, mobile products, provider transports, AI/MCP, Meitheal integration, and broad integrations must not become hidden B0 prerequisites.

Accepted architecture is not implementation evidence. In particular, ADR 0004 defines separate Airgap and Connected-local artifacts, but the current review application is not an Airgap artifact until dependency, packaging, and socket-denial gates pass. Use the claim language in `docs/product/working-state-delivery.md`.

## Truth and conflict handling

Use this authority order:

1. released compatibility and canonical-format contracts;
2. accepted decisions;
3. bounded-context invariants and tests;
4. security, privacy, and local-integrity invariants;
5. machine-readable requirements, UAT, gates, and task dependencies;
6. product specifications;
7. knowledge articles;
8. prototypes, screenshots, issues, and discussion.

When sources conflict, stop and create a focused clarification or decision. Do not silently invent a resolution.

## Required working sequence

1. Search existing code, documents, decisions, knowledge, requirements, UAT, gates, tasks, issues, and pull requests.
2. State the user problem and observed or supplied evidence. Do not substitute a feature idea for a problem statement.
3. Identify the owning bounded context and its ubiquitous language.
4. Select one dependency-complete vertical slice.
5. Add or update tests before claiming completion.
6. Keep domain rules in domain code, orchestration in application services, and external mechanisms in adapters.
7. Expose stable use cases through the CLI before or with desktop/API/MCP surfaces where applicable.
8. Update or cite the relevant KCS-informed knowledge article.
9. Update `CHANGELOG.md` for user-visible, operator-visible, or contributor-visible behaviour.
10. Complete the pull-request template with risks, rollback, accessibility evidence, and exact validation results.

The accepted working sequence is P00 through P03, `T-B0-P03-OBS`, the design-consultation and plan-design-review gate, P04 through P11, B0 qualification, then A0. Under D1-B, the observation uses the exact qualified and attested P03 artifact with synthetic or redacted workplace scenarios and records a distinct Continue, Change, or Stop decision. Only Continue permits P03D; Change or Stop keeps P03D and P04 blocked. G0 records that future design gate but does not create `DESIGN.md`. Do not revive the older personal-desktop-before-event ordering from R1/R2/R3 roadmap prose or a stale pull request.

## Domain-driven design rules

- A bounded context owns its model and vocabulary.
- Cross-context calls use explicit application interfaces, events, or anti-corruption layers.
- Domain crates must not import Tauri, React, HTTP clients, SQL clients, filesystem libraries, or provider SDKs.
- Shared code is admitted to the shared kernel only when its meaning is identical across contexts and the coupling is deliberate.
- Provider names such as Google Drive, S3, WebDAV, Gmail, or CardDAV must not appear in business-domain entities unless provider identity is itself domain data.
- Persistence models and transport DTOs do not become domain entities by convenience.
- UI, CLI, API, MCP, importers, providers, and plugins do not duplicate business rules.
- A database schema is not the context map.
- A generic `metadata` map is not a substitute for an owned concept.

## Relationship-domain guardrails

Keep these separate:

- relationship intent;
- relationship evidence;
- maintenance status;
- purpose-specific profile readiness.

`Review Priority` may order a review queue. It must not be called relationship strength or presented as an objective quality score. Reason-only review is the default. Hard states such as archived, do-not-contact, ended, paused, snoozed, or excluded override any score.

Workplace workflows must structurally omit relationship allocation, relationship-value ranking, and relationship-strength scoring from domain types, projections, filters, exports, automation, and UI. Hiding those controls is insufficient.

Empty personal, dietary, accessibility, or workplace fields are ambiguous. Use explicit information states such as verified none, unknown, declined, stale, conflicting, or needs clarification.

## Local-integrity rules

- Canonical Markdown, YAML, and JSONL formats must be documented and versioned.
- SQLite, full-text indexes, caches, thumbnails, and graph projections must be rebuildable.
- Unknown fields and user-authored Markdown sections survive supported round trips.
- One `WorkspaceSession` owns writer authority, recovery state, key state, repositories, and projection status after the session migration. Interfaces must not recreate repositories from raw paths per command.
- Canonical mutations use the accepted recoverable multi-target operation protocol; a single-file rename is not a cross-record commit.
- Remote connections are disabled until the user creates a scoped, purpose-bound grant.
- Secrets are referenced through a secret store and are never written into canonical records, logs, fixtures, screenshots, or exported settings.
- Airgap builds compile out network clients and listeners; a runtime toggle is insufficient evidence.
- A local checkpoint and an encrypted recovery package are distinct capabilities. Only the encrypted package may be represented as user-portable recovery after a clean-install restore without prior Keychain state.
- Destructive operations require preview, confirmation, audit evidence, and a recovery path where technically possible.
- General and third-party post-A0 migrations follow `LRM-WS-007`: dry-run, pre-migration backup, deterministic execution, validation, and rollback or explicit irreversibility. The narrow B0 OKF People normalization does not inherit that generic requirement; it follows its stricter, separately owned `LRM-WS-017` and `UAT-066` exact-preview, exact-backup, failure-atomic recovery, idempotent rerun, and exact-rollback contract.
- Use synthetic fixtures only.

## Provider, sharing, and plugin rules

- A provider implements a versioned capability contract or proposes a new contract in a separate architecture change.
- Provider registration grants no data access.
- Backup, single-writer publication, and multi-writer synchronisation are distinct claims with distinct evidence.
- A successful upload/download test does not prove synchronisation.
- Shared plaintext folders are not treated as safe multi-writer databases.
- Sharing transports authorised encrypted operations and materialises local readable views; it does not redefine domain invariants.
- Plugins receive no ambient filesystem, database, network, or private-data authority.
- WASI/WIT capabilities must be explicit, reviewable, limited, and revocable.

## AI, MCP, and automation rules

- AI output is untrusted input and passes ordinary validation.
- Read tools return sources and the grant used.
- Write tools stage proposals by default.
- Proposal review identifies affected records, fields, old and new values, provenance, and consequences.
- No personal data is sent to a model without an explicit provider, purpose, scope, and expiry grant.
- Local Ollama-compatible workflows must not require a remote account.
- API, MCP, webhooks, n8n, and plugins call normal application services and cannot bypass grants or domain rules.
- Liaison does not grow a generic task-management engine. Personal commitments and event workflow states remain owned concepts with bounded transitions.

## User experience review

Every user-facing change must use `docs/standards/ux-review.md`. At minimum, review:

- interruption recovery and cognitive load for ADHD and AuDHD users;
- keyboard and screen-reader operation;
- WCAG 2.2 Level AA and applicable EN 301 549 evidence;
- all ten Nielsen heuristics;
- AskTog interaction principles;
- Gestalt grouping and information hierarchy;
- relevant IxDF research topics;
- 400% zoom, reflow, reduced motion, long content, and localisation;
- empty, loading, partial, stale, conflict, permission, success, undo, and recovery states;
- a semantic alternative to graph-only or drag-only interaction.

Do not hide important state in colour, an icon, hover, animation, spatial position, or an unexplained score.

## Content quality

Automated text is not rejected because of its origin. It is rejected when it is vague, repetitive, falsely confident, unaccountable, or disconnected from evidence. Follow `docs/standards/content-quality.md`.

Do not:

- insert deliberate mistakes to appear human;
- use an AI detector as an authorship gate;
- claim research, testing, compliance, packaging, platform support, or validation that did not occur;
- leave promotional prose where a decision, limitation, owner, or measurable result is required;
- use generated comments that merely restate code;
- describe planned work as implemented.

## Dependency and licence review

Before adding a dependency, record:

- licence and compatibility with AGPL-3.0;
- maintenance and release history;
- transitive surface and supply-chain risk;
- platform and toolchain support;
- Airgap and binary-size effect;
- why existing code or a smaller dependency is insufficient.

Code copied or adapted from another project requires provenance, licence review, and attribution. Product inspiration is not permission to reuse code.

## Pull-request completion gate

A pull request is not ready for review until:

- the scope is coherent and its context owner is named;
- implemented and planned behaviour are distinguished;
- tests and repository checks pass or failures are disclosed;
- exact commit and workflow evidence is recorded for completion claims;
- the knowledge and changelog decision is recorded;
- migration, compatibility, privacy, security, accessibility, and rollback effects are addressed;
- generated files are reproducible;
- no unrelated changes, staging payloads, diagnostics, or one-shot repair workflows remain;
- the PR body states every gate that remains blocked, deferred, current, or otherwise not complete.

Draft pull requests may be incomplete, but their body must identify the missing work and must not be represented as release-ready.

## Handoff expectation

Leave enough committed context for another contributor or agent to continue without private prompt history. Update `PROJECT_CONTEXT.md` when a durable product boundary, implementation order, status assumption, or handoff rule materially changes. Do not place secrets, personal data, or private conversation history in the repository.
