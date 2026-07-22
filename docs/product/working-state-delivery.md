# Working-state delivery contract

Last reconciled: 2026-07-22

This document tells a contributor what the next product is and prevents roadmap breadth from being mistaken for current implementation. Normative domain and safety rules remain in `SPEC.md`, accepted decision records, schemas, and tests.

## Verified implementation boundary

The current merged source foundation contains accepted P00–P02 plus the unaccepted P03 candidate:

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
- candidate recoverable multi-target canonical operations with final mutation preconditions, durable COMMIT/progress/COMPLETE evidence, and roll-forward recovery;
- profile/readiness and reason-only Review domain foundations that are not yet persisted or surfaced as a complete product;
- provider-neutral contracts and a local-folder reference adapter with limited claims;
- macOS and Windows packaging workflows.

The installed macOS review application is version `0.1.0-alpha.1` and contains
a universal executable with a linker/ad-hoc code directory, but strict bundle
verification reports that the application bundle is not signed. It is not
Developer ID signed, notarised, or a supported public release.

The merged P02 foundation provides a write-authoritative Workspace Session with
workspace-local and per-user `WorkspaceId` operating-system exclusion,
retained capability roots, quiescence, and lock-free read-only Health. Current
cooperating ordinary unconfined Liaison processes on one OS account and machine
reject copied workspaces with the same identity even when `HOME`/XDG values
differ. Flatpak, macOS App Sandbox, and Windows AppContainer host/GUI pairings
remain explicitly unsupported until a shared authority broker/namespace exists
and are not treated as valid fallback registries. `T-B0-P02` is complete with accepted exact-head platform evidence.

The application still does not provide a complete event workflow, encrypted recovery, pinned OKF People authoring or normalization, Directory projection, supported Airgap proof, or Personal Memory Alpha. Recoverable multi-target writes and final mutation preconditions are implemented candidate source, not accepted P03 capability, until hardening, technical qualification, qualified-code/merge-result/attestation identities, and the exact executable artifact receipt complete. Recovery, key, and projection capabilities remain explicitly unavailable until their owning phases land. Exact-head P02 Linux, macOS, and Windows evidence is accepted, but installed-artifact requalification remains pending. Documentation and prototypes describing later features are plans until exact-head implementation evidence exists.

The accepted navigation contract requires Events to remain structurally absent
until `T-B0-P11` delivers the complete compiled workflow. The reviewed
pre-reconciliation main at 2026-07-22 violates that contract: `49ee419` exposes an Events-labelled destination through
the `readiness` route alias, and the current static-shell policy check does not
reject that alias. A route placeholder, page-local attendee list, or inert filter
is not partial Event implementation. The current source and checker are defective;
the dependency sequence, contracts, and evidence required to enable the
destination are recorded in
[KCS-0014](../knowledge/KCS-0014-when-may-the-events-destination-be-enabled.md).

The approved Claudia/PingCRM/OKF strategy overlay is integrated into the machine contracts at SHA-256 `795a6e6751cd29a995478e254323f491e68a53ef7c35fa729d8627b87cd37089`. It adopts specific person-record outcomes and rejects blanket parity or provider-count claims. G0, P00, P01, and P02 are complete; G1 is current and P03 is the active package.

PR #65 established execution baseline `3499a6e9278fc72d2498a9978df59f30d03722e6`. All seven ordinary-push workflows for that merge result succeeded in runs `29899084738`, `29899084740`, `29899084741`, `29899084751`, `29899084753`, `29899084769`, and `29899084789`, including Windows. Separately dispatched notarized-bundle run `29899498005` failed for missing Apple credentials and is not ordinary-push CI or release evidence. Neither fact technically accepts P03. The maintainer selected D1-B: after technical P03 qualification and attestation, `T-B0-P03-OBS` (D9) consumes that exact identity tuple, uses synthetic or redacted workplace scenarios, and ends with a distinct Continue, Change, or Stop decision. P03 technical acceptance makes OBS current but leaves P03D blocked; every recorded outcome completes OBS. Only Continue makes P03D eligible; Change advances exactly one recorded replacement task and Stop records a structured preservation/support disposition while P03D and P04 remain blocked.

The reviewed pre-reconciliation main at 2026-07-22 was `49ee419e30f2d71524dd6fa15badf1ec4b8d0e27`, reached through an unreviewed mixed sequence of P03 hardening, design, Events, Profiles, Customisation, and static desktop commits. That source is preserved for selective salvage, but it does not advance this delivery contract: exact-head Rust and Windows checks fail formatting; no accepted qualified-code, merge-result, attestation, qualification-receipt, executable-artifact, observation, or Continue identity exists. Machine authority therefore keeps P03 `current`, OBS blocked, P03D, P04, P05-P11, and B0 blocked, and PILOT deferred. The C3 commit and any merge result are distinct identities requiring their own exact-head receipt; absent that receipt they do not advance acceptance.

The independently reviewed authority package is preserved at local commit `86775d4b2f8a9edd62e48530a414d7bc1dc8e22c`, with 32-path content identity `90f9584ccb323dbca7894cc597acf23db80e972ad5ed7160b99fa22c12d03d78`. It is reconciliation input, not a current-main acceptance or a wholesale cherry-pick. Separate branch commit `c2f852cef7239cd2d952d3f2a8078b67dbb9f7bb` and its observation archive are premature: they use the competing `T-B0-P03O`/ADR-0014 identity and claim P03 complete at `3499a6e`. Preserve that branch, but do not run or accept its sessions. The only canonical observation chain is ADR 0015, `LRM-PK-010`, `T-B0-P03-OBS` (D9), and `FG-B0-DESIGN-001` after a distinct accepted P03 identity tuple.

The unsigned annotated `vB0` tag targets `d5af9b894bbbb7bfdfcb31ce86e5b7201720f57a`, three commits before `49ee419`, and has no GitHub Release or source-bound installed-artifact evidence. It is preserved as an unsupported historical claim, not a release. The active `.planning` vB0 archive and P04 static review/UAT/verification claims are superseded by this document and machine traceability. The runnable `49ee419` shell also exposes an Events-labelled destination through the `readiness` route alias while P11 is blocked; that is a current source/checker defect, not a permitted preview or accepted capability.

Current `DESIGN.md`, `design/semantic-tokens.v1.json`, the P03D consultation/plan-review records, the amended P04 plan, and the approved atlas remain non-accepted candidate material. They preserve useful visual and planning evidence, but P03D is machine-blocked and no accepted P03/D1-B Continue chain exists. This C3 authority correction deliberately does not rewrite those files or infer design acceptance from their presence on `main`.

## Accepted order

Work-package scores and the rules for dependency and safety overrides are in [B0 and A0 RICE prioritization](rice-prioritization.md). The score makes assumptions reviewable; it does not permit A0 to bypass B0 or visible features to bypass integrity, privacy, accessibility, or recovery gates.

1. Reconcile contracts, decisions, versions, commands, evidence, and stale branch claims.
2. Add one application composition root, then establish write-authoritative Workspace Session ownership.
3. Qualify and technically accept the candidate recoverable multi-target operations with qualified-code SHA, merge-result SHA, attestation SHA, and exact executable artifact receipt. This does not close the broader `FG-B0-001`.
4. Under D1-B, observe that same exact attested executable with synthetic or redacted workplace scenarios and record Continue, Change, or Stop. Do not use real workplace personal data; technical acceptance is a prerequisite, not the design decision.
5. Only after Continue, run design consultation to create canonical `DESIGN.md`, then plan design review to approve an amended P04 direction. G0 does not create or pre-empt this artifact.
6. Replace the disposable vanilla shell with a typed React/Tauri adapter, semantic token/component foundation, and installed Workspace/People/Health built-in-theme/recovery matrix. P04 does not own Events or the full B0 journey.
7. Establish the P05 Directory/Event/dietary domain contracts in G1 before P06/P07; after P04, add the pinned OKF People schema and strict writer in P05-OKF, tolerant Directory reading in P06, guided backup-first repair in P06-REPAIR, and required legacy-People normalization in P09-OKF. Keep sensitive contracts exclusively in P07/P08, close `FG-B0-001` only at P09-OKF, and let G3 consume the P05 contracts in P09/P10.
8. Implement Directory security/local policy, checkpoint and encrypted recovery, onboarding import, Events core, and the B0 interface with built-in theme choice and persistence only.
9. Qualify B0 in the compiled installed Mac application.
10. Implement and independently qualify A0.

P02 owns the readable manifest and write-authoritative session boundary, including `enabled_modules`; it does not own installed-artifact/no-egress, projection-rebuild, or full workspace round-trip acceptance. P01 contributes its completed application contract to `FG-B0-001`; it does not prematurely close the broader R1 CLI gate. B0 acceptance owns local workspace creation and `UAT-001`, which prove an approved installed local-authoritative review artifact with network denied and no account, and is the first task able to close `FG-R1-004` with the complete CLI/UAT evidence. Compiled-out Airgap proof remains exclusively `UAT-024` under `FG-R2-005`. The complete `FG-R1-001` round trip remains blocked until final A0 acceptance can combine B0-owned `UAT-001` with A0-owned `UAT-002`.

The real workplace-data pilot is a separate deferred `PILOT` milestone after B0, not a predecessor of synthetic B0, A0, or provider delivery. Before B0 acceptance, `T-B0-PILOT`, `LRM-EV-012`, and `FG-B0-PILOT-001` remain deferred and the `real-workplace-data` capability remains denied. Once B0 is accepted and pilot work begins, the task and requirement become current, the gate becomes blocked, and the capability remains denied. Only an affirmative independent review with every condition resolved, followed by completion of both the task and gate, permits real employee data for the reviewed scope. A later material change returns the task and requirement to current, the gate to blocked, and the capability to denied until the changed scope is affirmatively reviewed and requalified. Synthetic B0 qualification remains available throughout without real workplace data.

P03 owns `LRM-WS-004`, `LRM-WS-005`, `LRM-WS-010`, and `UAT-042`, and must output the exact qualification identity tuple before OBS. ADR 0016 scopes general and third-party post-A0 migration safety under `LRM-WS-007` to R5 `T-R5-005`/`FG-R5-005`; the narrow B0 OKF normalization is governed exclusively by `LRM-WS-017` and `UAT-066`.

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
| #31 desktop visual direction | Closed; branch preserved as deferred design input | It may be reviewed only after accepted P03 technical qualification, exact-artifact OBS, and Continue make P03D eligible; design consultation, canonical `DESIGN.md`, and plan design review must still complete. It is not current P04 authority. |
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

## Machine traceability authority anchors

These identifier-exact anchors bind the machine ownership exceptions that this
normative delivery contract authorises. They are evidence anchors, not new
requirements or implementation claims.

- `cross-release::LRM-PK-007`
- `forward-evidence::T-B0-P01::UAT-042`
- `forward-evidence::T-B0-P02::UAT-042`
- `forward-evidence::T-B0-P03D::LRM-UX-009`
- `forward-evidence::T-B0-P03D::LRM-UX-012`
- `forward-evidence::T-B0-P03D::LRM-UX-016`
- `forward-evidence::T-B0-P03D::UAT-062`
- `forward-evidence::T-B0-P04::UAT-041`
- `forward-evidence::T-B0-P06::UAT-041`
- `forward-evidence::T-B0-P07::UAT-043`
- `forward-evidence::T-B0-P09::UAT-041`
- `forward-evidence::T-B0-P10::UAT-041`
- `forward-evidence::T-B0-P11::LRM-PK-006`
- `forward-evidence::T-B0-P11::UAT-041`
- `forward-evidence::T-R4-001::UAT-026`
- `forward-evidence::T-R4-001::UAT-034`
- `forward-evidence::T-R5-002::LRM-IN-002`
