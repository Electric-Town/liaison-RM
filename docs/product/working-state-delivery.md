# Working-state delivery contract

Last reconciled: 2026-07-19

This document tells a contributor what the next product is and prevents roadmap breadth from being mistaken for current implementation. Normative domain and safety rules remain in `SPEC.md`, accepted decision records, schemas, and tests.

## Verified implementation boundary

At the reconciliation point, `main` contains:

- Workspace and People domain foundations;
- readable Workspace YAML and Person Markdown storage;
- workspace create/open/validate and Person create/list in the CLI;
- a native Tauri alpha exposing the same narrow workflow;
- profile/readiness and reason-only Review domain foundations that are not yet persisted or surfaced as a complete product;
- provider-neutral contracts and a local-folder reference adapter with limited claims;
- macOS and Windows packaging workflows.

The installed macOS review application is version `0.1.0-alpha.1` and is an ad-hoc-signed universal internal alpha. It is not Developer ID signed, notarised, or a supported public release.

The application does not yet provide a complete event workflow, encrypted recovery, a Workspace Session, recoverable multi-target writes, pinned OKF People authoring or normalization, Directory projection, supported Airgap proof, or Personal Memory Alpha. Documentation and prototypes describing those features are plans until exact-head implementation evidence exists.

The approved Claudia/PingCRM/OKF strategy overlay is integrated into the machine contracts at SHA-256 `795a6e6751cd29a995478e254323f491e68a53ef7c35fa729d8627b87cd37089`. It adopts specific person-record outcomes and rejects blanket parity or provider-count claims.

## Accepted order

Work-package scores and the rules for dependency and safety overrides are in [B0 and A0 RICE prioritization](rice-prioritization.md). The score makes assumptions reviewable; it does not permit A0 to bypass B0 or visible features to bypass integrity, privacy, accessibility, or recovery gates.

1. Reconcile contracts, decisions, versions, commands, evidence, and stale branch claims.
2. Add one application composition root and Workspace Session.
3. Route every canonical mutation through recoverable multi-target operations.
4. After P03, run design consultation to create canonical `DESIGN.md`, then plan design review to approve an amended P04 direction. G0 does not create or pre-empt this artifact.
5. Replace the disposable vanilla shell with a typed React/Tauri adapter and an evidence-tested semantic design system.
6. After P04, add the pinned OKF People schema and strict writer in P05-OKF, tolerant Directory reading in P06, and required legacy-People normalization in P09-OKF; keep sensitive contracts exclusively in P07/P08 and event/dietary contracts in the G3 P05/P10 path.
7. Implement Directory security/local policy, checkpoint and encrypted recovery, onboarding import, Events core, and B0 interface.
8. Qualify B0 in the compiled installed Mac application.
9. Implement and independently qualify A0.

## B0 Workplace Review Alpha

B0 is complete only when an installed universal Mac build can:

- create or open a workplace workspace;
- import a deterministic synthetic directory with People, Organisations, Groups, Locations, and Memberships;
- build and finalize an immutable event cohort;
- place every attendee in exactly one explicit dietary outcome;
- prevent unknown from becoming verified none;
- show the exact least-disclosure payload before generation and delivery;
- commit a sealed immutable internal brief and verified CSV/HTML delivery evidence;
- survive interruption, relaunch, committed-operation recovery, external edits, stale inputs, and denied grants;
- create and restore an encrypted recovery package on a clean installation without the original Keychain entry;
- pass keyboard, VoiceOver, contrast, zoom/reflow, reduced-motion, narrow-window, offline, readable-file, and egress evidence.
- write every non-reserved People record through the pinned OKF v0.1 Draft envelope while Liaison domain extensions remain authoritative and sealed facts never enter plaintext;
- tolerate unknown OKF keys/types, body sections, links, curated indexes, and malformed siblings, and complete the required OKF People normalization with preview, exact backup, journaled recovery, idempotent rerun, and exact rollback.

B0 assumes one trusted local workspace owner. It supports event-bounded preparation and gap resolution, but it has no generic task engine and cannot allocate, score, rank, weight, or assign relationship attention in a workplace workspace. Mobile clients, provider transports, AI, MCP, and Meitheal integration are later independent outcomes, not hidden B0 dependencies.

The required OKF People normalization is the only format-migration exception in B0. General and third-party migrations remain excluded, including Meerkat, Monica, CRM-in-Markdown, broad vCard conversion, provider sync, and arbitrary format adapters.

Missing Developer ID signing or notarisation keeps the artifact labelled an internal review alpha.

## A0 Personal Memory Alpha

A0 starts only after B0 acceptance and adds quick and full capture, a source-complete purpose-scoped profile, user-organised profile tabs with stable layout identifiers and settings round trips, explicit fact states, reversible identity review, a source- and range-labelled unified timeline, meaningful interactions, commitments, reason-only Review, distinct last-note and last-interaction values, open loops, and interruption-safe personal workflows. It reuses the B0 authority, storage, security, recovery, OKF, and UI foundations and must keep the B0 matrix green. A0 has no global person score, automatic exact or fuzzy merge, or generic task engine.

Later provider operations require visible grants, source ranges, receipts, retries, revocation, retention, and history; hidden sync, hidden refresh, and unreported egress are prohibited. AI, MCP, plugin, import, and provider enrichment can produce source-backed staged proposals only and cannot write confirmed facts directly.

## Active-branch disposition

| PR | Disposition | Reason |
|---|---|---|
| #21 profile persistence | Rebuild after the operation/security contracts | Its plaintext-plus-marker model and self-writing workflow are not acceptable persistence authority. |
| #22 Organisations/Groups | Selectively transplant domain concepts later | It is not a complete revisioned repository/application/UI slice. |
| #25 backup/restore | Transplant after Workspace Session as local checkpoint mechanics | It is unencrypted, lacks writer coordination, and cannot be called recoverable backup. |
| #27 personal CRM baseline | Closed and superseded | It contains truncated hidden payloads rather than reviewable product source and starts A before B. |
| #28 write journal | Transplant algorithms into the multi-target operation engine | It is single-target and unwired, with no explicit durable commit decision. |
| #29 parity/WCAG audit | Closed as historical intake | Its inventory/checklist is useful, but its personal-first plan contradicts the accepted order. |
| #31 desktop visual direction | Closed; branch preserved as deferred design input | It may be reviewed only after P03, design consultation, canonical `DESIGN.md`, and plan design review. It is not current P04 authority. |
| #32 relationship model | Closed; branch preserved for A0 | Relationship intent and cadence are G2B work blocked until B0 acceptance. |
| #33 relationship YAML/CLI | Closed; branch preserved for A0 | Its G2B implementation cannot execute or merge before B0 acceptance. |

Branches are preserved until useful material has been transplanted or deliberately rejected. None is merged wholesale.

The separate `claude/pr1-handdrawn-shell` worktree is preserved as recoverable design source material only. Its staged deletion, untracked React tree and generated directories, incomplete Vite/Tauri wiring, stale tests and documentation, and native-payload mismatch make it unsuitable for wholesale staging or merge.

## Claim language

Until evidence changes, user-facing surfaces say:

- “local-authoritative review build” rather than “Airgap build”;
- “no connection configured in this review build” rather than “network clients compiled out”;
- “local checkpoint” for an unencrypted deterministic snapshot;
- “encrypted recovery package” only after clean-install restore passes;
- “accessibility target” rather than conformance or certification.

## Evidence required before a status change

Every implementation PR updates the changelog, applicable knowledge, decision/requirement/UAT/gate/task traceability, and exact-head checks. A screenshot is design evidence, not persistence, privacy, accessibility, platform, or release evidence. The installed `.app` is the authority for final B0/A0 design review and native QA.
