# Working-state delivery contract

Last reconciled: 2026-07-19

This document tells a contributor what the next product is and prevents roadmap breadth from being mistaken for current implementation. Normative domain and safety rules remain in `SPEC.md`, accepted decision records, schemas, and tests.

## Verified implementation boundary

The current P02 source state, built on the merged P00 and P01 foundation, contains:

- Workspace and People domain foundations;
- readable Workspace YAML and Person Markdown storage;
- one `liaison-application` composition root used by the CLI and native Tauri alpha;
- write-authoritative Workspace Sessions that own identity/schema, one retained
  capability root, path-free repositories, environment-independent composite
  path/identity operating-system writer authority for ordinary unconfined
  processes, quiescence, and explicit unavailable recovery/key/projection states;
- lock-free one-shot Health for contended, malformed, and newer-schema
  workspaces, plus shared structured result/error envelopes;
- workspace create/open/validate and Person create/list through both inbound adapters;
- profile/readiness and reason-only Review domain foundations that are not yet persisted or surfaced as a complete product;
- provider-neutral contracts and a local-folder reference adapter with limited claims;
- macOS and Windows packaging workflows.

The installed macOS review application is version `0.1.0-alpha.1` and contains
a universal executable with a linker/ad-hoc code directory, but strict bundle
verification reports that the application bundle is not signed. It is not
Developer ID signed, notarised, or a supported public release.

The P02 source state provides a write-authoritative Workspace Session with
workspace-local and per-user `WorkspaceId` operating-system exclusion,
retained capability roots, quiescence, and lock-free read-only Health. Current
cooperating ordinary unconfined Liaison processes on one OS account and machine
reject copied workspaces with the same identity even when `HOME`/XDG values
differ. Flatpak, macOS App Sandbox, and Windows AppContainer host/GUI pairings
remain explicitly unsupported until a shared authority broker/namespace exists
and are not treated as valid fallback registries. `T-B0-P02` is complete with accepted exact-head platform evidence.
pending exact-head native Linux, macOS, and Windows evidence.

The application also does not yet provide a complete event workflow, encrypted recovery, recoverable multi-target writes, final mutation preconditions, pinned OKF People authoring or normalization, Directory projection, supported Airgap proof, or Personal Memory Alpha. Recovery, key, and projection capabilities remain explicitly unavailable until their owning phases land. Local P02 source-worktree evidence is recorded, but exact-head remote Linux, macOS, and Windows matrices and installed-artifact requalification remain pending. Documentation and prototypes describing later features are plans until exact-head implementation evidence exists.

The current review-build navigation therefore keeps Events absent until
`T-B0-P11` delivers the complete compiled workflow. A route placeholder,
page-local attendee list, or inert filter is not partial Event implementation.
The static-shell policy check enforces that boundary; the dependency sequence,
contracts, and evidence required to enable the destination are recorded in
[KCS-0014](../knowledge/KCS-0014-when-may-the-events-destination-be-enabled.md).

The approved Claudia/PingCRM/OKF strategy overlay is integrated into the machine contracts at SHA-256 `795a6e6751cd29a995478e254323f491e68a53ef7c35fa729d8627b87cd37089`. It adopts specific person-record outcomes and rejects blanket parity or provider-count claims. G0, P00, P01, and P02 are complete; G1 is current and P03 is the active package.

## Accepted order

Work-package scores and the rules for dependency and safety overrides are in [B0 and A0 RICE prioritization](rice-prioritization.md). The score makes assumptions reviewable; it does not permit A0 to bypass B0 or visible features to bypass integrity, privacy, accessibility, or recovery gates.

1. Reconcile contracts, decisions, versions, commands, evidence, and stale branch claims.
2. Add one application composition root, then establish write-authoritative Workspace Session ownership.
3. Route every canonical mutation through recoverable multi-target operations.
4. After P03, run design consultation to create canonical `DESIGN.md`, then plan design review to approve an amended P04 direction. G0 does not create or pre-empt this artifact.
5. Replace the disposable vanilla shell with a typed React/Tauri adapter and an evidence-tested semantic design system.
6. Establish the P05 Directory/Event/dietary domain contracts in G1 before P06/P07; after P04, add the pinned OKF People schema and strict writer in P05-OKF, tolerant Directory reading in P06, guided backup-first repair in P06-REPAIR, and required legacy-People normalization in P09-OKF. Keep sensitive contracts exclusively in P07/P08, close `FG-B0-001` only at P09-OKF, and let G3 consume the P05 contracts in P09/P10.
7. Implement Directory security/local policy, checkpoint and encrypted recovery, onboarding import, Events core, and the B0 interface with built-in theme choice and persistence only.
8. Qualify B0 in the compiled installed Mac application.
9. Implement and independently qualify A0.

P02 owns the readable manifest and write-authoritative session boundary, including `enabled_modules`; it does not own installed-artifact/no-egress, projection-rebuild, or full workspace round-trip acceptance. P01 contributes its completed application contract to `FG-B0-001`; it does not prematurely close the broader R1 CLI gate. B0 acceptance owns local workspace creation and `UAT-001`, which prove an approved installed local-authoritative review artifact with network denied and no account, and is the first task able to close `FG-R1-004` with the complete CLI/UAT evidence. Compiled-out Airgap proof remains exclusively `UAT-024` under `FG-R2-005`. The complete `FG-R1-001` round trip remains blocked until final A0 acceptance can combine B0-owned `UAT-001` with A0-owned `UAT-002`.

## B0 Workplace Review Alpha

B0 is complete only when an installed universal Mac build can:

- create or open a workplace workspace;
- close and reopen it, delete and rebuild its projection, and retain both canonical People while network access remains denied and no account is used;
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
- identify a duplicate Person ID and invalid birthday through tolerant Directory validation, then complete guided preview, exact backup, failure-atomic repair, bounded receipt, and exact rollback without silently deleting either source.

B0 assumes one trusted local workspace owner. It supports event-bounded preparation and gap resolution, but it has no generic task engine and cannot allocate, score, rank, weight, or assign relationship attention in a workplace workspace. Its appearance scope is built-in theme choice and persistence; versioned settings export/import and clean-device settings transfer begin in A0. Mobile clients, provider transports, AI, MCP, and Meitheal integration are later independent outcomes, not hidden B0 dependencies.

The required OKF People normalization is the only format-migration exception in B0. General and third-party migrations remain excluded, including Meerkat, Monica, CRM-in-Markdown, broad vCard conversion, provider sync, and arbitrary format adapters.

Missing Developer ID signing or notarisation keeps the artifact labelled an internal review alpha.

## A0 Personal Memory Alpha

A0 starts only after B0 acceptance and adds quick and full capture, a source-complete purpose-scoped profile, user-organised profile tabs with stable layout identifiers and settings round trips, explicit fact states, reversible identity review, a source- and range-labelled unified timeline, meaningful interactions, commitments, reason-only Review, distinct last-note and last-interaction values, open loops, and interruption-safe personal workflows. It reuses the B0 authority, storage, security, recovery, OKF, and UI foundations and must keep the B0 matrix green. A0 has no global person score, automatic exact or fuzzy merge, or generic task engine.

Later provider operations require visible grants, source ranges, receipts, retries, revocation, retention, and history; hidden sync, hidden refresh, and unreported egress are prohibited. AI, MCP, plugin, import, and provider enrichment can produce source-backed staged proposals only and cannot write confirmed facts directly.

A0 also adds previewable density, text-scale, reduced-motion, and validated-palette controls with settings export/import. Declarative third-party theme packages remain post-A0 work and must be schema-validated, checksummed, licensed, conformance-tested, and rollback-safe; they receive no arbitrary CSS, code, remote asset, filesystem, or network authority.

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
