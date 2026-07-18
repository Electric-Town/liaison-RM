# Working-state delivery contract

Last reconciled: 2026-07-18

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

The application does not yet provide a complete event workflow, encrypted recovery, a Workspace Session, recoverable multi-target writes, Directory projection, supported Airgap proof, or Personal Memory Alpha. Documentation and prototypes describing those features are plans until exact-head implementation evidence exists.

## Accepted order

Work-package scores and the rules for dependency and safety overrides are in [B0 and A0 RICE prioritization](rice-prioritization.md). The score makes assumptions reviewable; it does not permit A0 to bypass B0 or visible features to bypass integrity, privacy, accessibility, or recovery gates.

1. Reconcile contracts, decisions, versions, commands, evidence, and stale branch claims.
2. Add one application composition root and Workspace Session.
3. Route every canonical mutation through recoverable multi-target operations.
4. Replace the disposable vanilla shell with a typed React/Tauri adapter and an evidence-tested semantic design system.
5. Implement Directory, security/local policy, checkpoint and encrypted recovery, import, Events core, and B0 interface.
6. Qualify B0 in the compiled installed Mac application.
7. Implement and independently qualify A0.

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

B0 assumes one trusted local workspace owner. It supports event-bounded preparation and gap resolution, but it has no generic task engine and cannot allocate, score, rank, weight, or assign relationship attention in a workplace workspace. Mobile clients, provider transports, AI, MCP, and Meitheal integration are later independent outcomes, not hidden B0 dependencies.

Missing Developer ID signing or notarisation keeps the artifact labelled an internal review alpha.

## A0 Personal Memory Alpha

A0 starts only after B0 acceptance and adds Person/profile editing, user-organised profile tabs with stable layout identifiers and settings round trips, meaningful interactions, commitments, reason-only Review, last-interaction context, open loops, and interruption-safe personal workflows. It reuses the B0 authority, storage, security, recovery, and UI foundations and must keep the B0 matrix green.

## Active-branch disposition

| PR | Disposition | Reason |
|---|---|---|
| #21 profile persistence | Rebuild after the operation/security contracts | Its plaintext-plus-marker model and self-writing workflow are not acceptable persistence authority. |
| #22 Organisations/Groups | Selectively transplant domain concepts later | It is not a complete revisioned repository/application/UI slice. |
| #25 backup/restore | Transplant after Workspace Session as local checkpoint mechanics | It is unencrypted, lacks writer coordination, and cannot be called recoverable backup. |
| #27 personal CRM baseline | Closed and superseded | It contains truncated hidden payloads rather than reviewable product source and starts A before B. |
| #28 write journal | Transplant algorithms into the multi-target operation engine | It is single-target and unwired, with no explicit durable commit decision. |
| #29 parity/WCAG audit | Closed as historical intake | Its inventory/checklist is useful, but its personal-first plan contradicts the accepted order. |

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
