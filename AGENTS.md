# Agent and contributor operating contract

This file is normative for human contributors and automated coding agents.

## Product boundary

Liaison RM is a local-authoritative relationship manager. Canonical records are open files controlled by the user. No contribution may introduce a mandatory hosted account, hidden telemetry, remote licence check, undeclared network request, or provider-specific dependency into the domain model.

## Required working sequence

1. Read the relevant bounded-context README, architecture decision records, knowledge articles, requirements, and feature gates.
2. State the user problem and the observed or supplied evidence. Do not substitute a feature idea for a problem statement.
3. Identify the owning bounded context and its ubiquitous language.
4. Add or update tests before claiming completion.
5. Keep domain rules in domain code, orchestration in application services, and external mechanisms in adapters.
6. Update or cite the relevant KCS-informed knowledge article.
7. Update `CHANGELOG.md` for user-visible, operator-visible, or contributor-visible behaviour.
8. Complete the pull-request template with risks, rollback, accessibility evidence, and validation results.

## Domain-driven design rules

- A bounded context owns its model and vocabulary.
- Cross-context calls use explicit application interfaces, events, or anti-corruption layers.
- Domain crates must not import Tauri, React, HTTP clients, SQL clients, filesystem libraries, or provider SDKs.
- Shared code is admitted to the shared kernel only when its meaning is identical across contexts and the coupling is deliberate.
- Provider names such as Google Drive, S3, WebDAV, Gmail, or CardDAV must not appear in domain entities unless the provider identity is itself domain data.
- Persistence models and transport DTOs do not become domain entities by convenience.

## Local-integrity rules

- Canonical Markdown, YAML, and JSONL formats must be documented and versioned.
- SQLite, full-text indexes, caches, and graph projections must be rebuildable.
- Remote connections are disabled until the user creates a scoped grant.
- Secrets are referenced through a secret store and are never written into canonical records, logs, fixtures, or screenshots.
- Airgap builds compile out network clients and listeners.
- Destructive operations require preview, confirmation, audit evidence, and a recovery path where technically possible.

## User experience review

Every user-facing change must use `docs/standards/ux-review.md`. At minimum, review:

- interruption recovery and cognitive load for ADHD and AuDHD users;
- keyboard and screen-reader operation;
- WCAG 2.2 Level AA and applicable EN 301 549 evidence;
- all ten Nielsen heuristics;
- AskTog interaction principles;
- Gestalt grouping and information hierarchy;
- relevant IxDF research topics;
- empty, loading, partial, stale, conflict, permission, success, undo, and recovery states;
- a semantic alternative to graph-only or drag-only interaction.

## Content quality

Automated text is not rejected because of its origin. It is rejected when it is vague, repetitive, falsely confident, unaccountable, or disconnected from evidence. Follow `docs/standards/content-quality.md`.

Do not:

- insert deliberate mistakes to appear human;
- use an AI detector as an authorship gate;
- claim research, testing, compliance, or validation that did not occur;
- leave generic promotional prose where a technical decision or measurable result is required;
- use generated comments that merely restate code.

## Pull-request completion gate

A pull request is not ready for review until:

- the scope is coherent and its context owner is named;
- tests and repository checks pass or failures are disclosed;
- the knowledge and changelog decision is recorded;
- migration, compatibility, privacy, security, accessibility, and rollback effects are addressed;
- generated files are reproducible;
- no unrelated changes are included.

Draft pull requests may be incomplete, but their body must state what remains incomplete.
