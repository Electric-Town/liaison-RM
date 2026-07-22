# P04–P11, B0, and A0 production-readiness audit

**Audit date:** 2026-07-22
**Disposition:** **HOLD — pre-alpha, not production-ready**
**Canonical execution boundary:** P03 current
**Primary merged-state anchor:** `49ee419e30f2d71524dd6fa15badf1ec4b8d0e27`
(`origin/main` when the audit began)
**B0 tag observation:** annotated `vB0` peels to
`d5af9b894bbbb7bfdfcb31ce86e5b7201720f57a`

## Purpose and authority

This audit reconciles the P04–P11, B0, and A0 claims against:

- `AGENTS.md`, `PROJECT_CONTEXT.md`, and
  `docs/product/working-state-delivery.md`;
- `SPEC.md` and `AI_BUILD_INSTRUCTIONS.md`;
- `spec/traceability-ownership.json` and generated
  `docs/product/traceability.md`;
- the merged source at the anchor commit;
- exact GitHub workflow state for that commit.

Machine-owned traceability is authoritative for current task status. Planning
archives, tags, prototypes, screenshots, paths to local artifacts, and unit
tests cannot independently change it.

## Executive result

B0 was not shipped, P04 was not complete, and A0 had not started. The merged
anchor combined useful domain preparation and design work with a production UI
that displayed fabricated people/event/readiness state and unavailable
capabilities. It also exposed a non-durable in-memory Event application path
that could turn a known Person into `VerifiedNone` without an authoritative
dietary fact.

The correct status is:

- P00, P01, and P02 complete;
- P03 current;
- P03D and P04–P11 blocked;
- B0 acceptance blocked;
- A0 blocked until B0 acceptance.

People cannot yet rely on this build for daily workplace or personal use
without error. The currently capability-backed boundary remains Workspace,
People, and read-only Health. Its presentation must nevertheless preserve the
accepted design direction and make later capabilities visibly incubating
without fabricating routes, state, or completion until their owning services
and evidence land.

## Evidence-integrity findings

### Unsupported completion record

Commit `d5af9b8` created a `vB0` archive and tag while canonical
traceability still marked later B0 work blocked. Subsequent planning files
claimed:

- all 156 requirements, 75 UAT cases, 48 gates, and 79 tasks were verified;
- P04 and an eight-screen product were complete;
- WCAG 2.2 AA compliance;
- a working installed application and DMG;
- no deferred work.

Those claims had no accepted exact-head/exact-artifact evidence. The
specification checker counts and validates contract records; it does not
execute every UAT case or prove every gate. Contrast ratios and labelled
controls do not constitute WCAG 2.2 or EN 301 549 conformance.

The affected planning and P04 evidence files now retain the statements only as
invalidated history.

### Exact main CI

For `49ee419e30f2d71524dd6fa15badf1ec4b8d0e27`:

| Workflow | Run | Result | Meaning |
|---|---|---|---|
| Repository policy | [29909982359](https://github.com/Electric-Town/liaison-RM/actions/runs/29909982359) | passed | Repository policy only |
| Rust workspace | [29909982384](https://github.com/Electric-Town/liaison-RM/actions/runs/29909982384) | failed | Formatting gate failed |
| Windows desktop alpha | [29909982317](https://github.com/Electric-Town/liaison-RM/actions/runs/29909982317) | failed | Formatting gate failed |

This audit did not run a local qualification suite and makes no local pass
claim for the in-progress remediation branch.

### Installed-artifact divergence found during re-audit

The first native re-audit exposed three different macOS bundles carrying the
same bundle identifier and version but different executable digests:

| Bundle | Executable SHA-256 | Observed surface |
|---|---|---|
| `/Applications/Liaison RM.app` | `cfe1ddb0149e497310aa4260e93eff0398430616357691c4d1efe61f6f481bb1` | Rich five-destination shell with the fabricated 214-person Directory, named sample records, Events, Airgap, and active-workspace claims |
| `/Applications/Liaison RM 2.app` | `6383a00cda96f31f3b0f24fc1507a992f344ae6b979d0c6400a883e8665d8295` | Hybrid shell with real Workspace controls but unsupported Today, Cohorts/Readiness, and Settings destinations |
| audit-branch local bundle | `bbb7c92c435a80754cb345433518ac58077b4f39793999e1fbd64fdfaf1ac0a7` | Capability-backed Workspace, People, and Health shell used for the initial native repair check |

Two copies were running concurrently under the same
`io.github.electric-town.liaison-rm` identifier. None of the installed copies
carried an inspectable commit-to-artifact receipt, so app name and version were
insufficient to establish which source or evidence applied. The workspace
owner explicitly observed that the narrow repair build was a major product
regression and that the richer installed surface was closer to the intended
end state.

That observation changes the remediation constraint without validating the
sample UI: reconciliation must retain and mature the richer information
architecture while replacing every fabricated value and unsupported enabled
action with actual Application capability data or a precise unavailable state.
It is not acceptable either to ship the sample state or to make promised work
disappear. Final installed qualification must leave one canonical app copy,
one exact executable digest, and one matching source/evidence receipt.

## Phase audit

| Work package | Canonical status | Implemented/preparatory value | Production gap |
|---|---|---|---|
| P04 typed desktop | blocked | Editorial Ledger design assets, semantic tokens, local fonts, and a native review shell exist | Required React/TypeScript/Vite adapter, stable generated DTO boundary, exact Workspace/People/Health parity, and rendered/native evidence are not accepted |
| P05 domain contracts | blocked | Useful People, Organisations, Events, and profile-domain types exist | No accepted dependency-complete P05 contract package; sensitive ownership must remain in P07/P08 |
| P05-OKF | blocked | ADR and schema direction exist | No accepted strict OKF People writer through the recoverable application port |
| P06 Directory | blocked | Some People reads and validation foundations exist | No accepted tolerant OKF Directory projection, quarantine, SQLite/FTS filtering, pagination, canonical revalidation, or 10,000/50,000 evidence |
| P06-REPAIR | blocked | P03 primitives can become a foundation | No guided preview, exact backup, failure-atomic repair, bounded receipt, or exact rollback |
| P07 security | blocked | Security contracts and decisions exist | No complete key lifecycle, authenticated sealed persistence, trusted-owner/device policy, purpose grants, or payload-minimal activity evidence |
| P08 recovery | blocked | Recovery/checkpoint language is specified | No proven encrypted recovery package or clean-install restore without prior Keychain state; UI controls at the anchor overstated availability |
| P09-OKF | blocked | Normalization contract exists | No previewable, exact-backup-first, journaled, idempotent, restart-recoverable, exactly reversible normalization |
| P09 onboarding | blocked | Basic canonical People operations exist | No complete streaming CSV preview/reconciliation for People, Organisations, Locations, Groups, and Memberships |
| P10 Events core | blocked | A valuable Events domain core models cohort and readiness concepts | Anchor application wiring was in memory, non-durable, and bypassed required persistence/security/recovery foundations |
| P11 B0 interface | blocked | Eight-screen visual concepts can be reused | The anchor surface used sample state and exposed Events/Readiness/Settings before their services existed; complete cohort-to-brief state machine absent |
| B0 acceptance | blocked | Earlier macOS review artifacts may be useful for setup learning | No exact-source universal Mac artifact passed scale, crash, repair, key, grant, leak, restore, accessibility, offline, readable-file, and egress evidence |
| A0 | blocked | Profiles, Review, and Customisation domain preparation is worth preserving | B0 prerequisite unmet; no installed personal-memory journey, source-complete profile, timeline, commitments, reason-only Review, or settings round trip |

## Critical source findings at the anchor

### Fabricated production UI state

`apps/desktop/ui/app.js` embedded named attendee records, dietary
instructions, verification dates, readiness outcomes, and a B0/Airgap product
status. `apps/desktop/ui/index.html` embedded an “All-hands lunch,” a
“Workplace Directory (214 people),” “Airgap ready,” “Local checkpoint intact,”
and encrypted-recovery controls. These were not loaded from authoritative
canonical records or capability state.

The same JavaScript returned a successful-looking
`{ contract_version: 1, value: null }` result when the Tauri bridge was
absent and supplied a fallback “ready” status. A missing native authority must
fail closed and visibly; it must never make a prototype look operational.

### Unsafe Event application seam

At the anchor, `liaison-application` stored Events in a process-local map.
Restarting the process lost them. While building an Event DTO, any Person found
in the People repository was assigned
`liaison_events::Availability::VerifiedNone`, `Fresh`, `Consistent`, and
`Allowed` without reading an authoritative dietary record or purpose grant.
The resolution command also ignored its requested action and discarded
finalisation/participation errors.

This violated the invariant that unknown is never verified none and could make
unsafe catering output appear ready. It also lacked P03 recoverable persistence,
P07 grants/sealing, P08 recovery, P09 Directory/onboarding, and P10 immutable
evidence.

The Events domain core itself remains valuable. The production correction is to
keep that capability incubating and structurally unavailable while maturing
its application, persistence, security, recovery, CLI, and installed UI
vertical slice—not to delete the domain work.

### Structural-boundary bypass

The prior shell checker rejected only a literal Events route. Equivalent route
or page names such as `readiness`, hard-coded event content, and native Event
commands could bypass the policy. A release boundary must inspect the whole
shipped surface and native command registration, not one label.

## UX and accessibility disposition

The design direction has strengths: locally bundled readable fonts, visible
focus styles, semantic labels, useful contrast calculations, a calm editorial
hierarchy, and explicit non-scoring language.

It is not yet production evidence because the audit found no accepted
exact-artifact coverage for:

- the complete keyboard journey and focus restoration after interruption;
- VoiceOver announcements and control semantics in the installed app;
- 400% zoom/reflow, narrow windows, long translated content, and pseudolocale;
- reduced motion and system setting persistence;
- loading, empty, partial, stale, conflict, denied-grant, failure, undo, and
  recovery states backed by real services;
- accessible alternatives to any future drag/spatial interaction;
- the full Nielsen, AskTog, Gestalt, ADHD/AuDHD, WCAG 2.2 AA, and applicable
  EN 301 549 evidence matrix required by `docs/standards/ux-review.md`.

A labelled 360px preview box and static HTML inspection cannot close these
items.

## Release and tag disposition

The `vB0` tag is an inaccurate historical marker, not a supported release.
This audit does not move or delete it. Changing a published tag is a destructive
release-history action requiring explicit maintainer authority and a separate
communication/compatibility decision.

Until the owning evidence changes, permitted claim language is:

- “pre-alpha local-authoritative review build”;
- “no connection configured in this review build”;
- “local checkpoint” only for an implemented deterministic local checkpoint;
- “encrypted recovery package” only after clean-install restore evidence;
- “accessibility target,” not conformance or certification.

## Remediation order

1. Keep production navigation and native commands inside the current
   Workspace/People/Health boundary; preserve later UI as labelled prototype
   source outside shipped assets.
2. Finish and accept P03, then close the P03D design observation/review gate.
3. Implement P04 as the typed desktop adapter with real bridge failure states
   and exact rendered/native evidence.
4. Mature P05/P05-OKF/P06/P06-REPAIR/P07/P08 in dependency order before
   exposing sensitive or recoverable workflows.
5. Complete P09-OKF/P09 and then P10 through CLI/application/persistence before
   P11 UI.
6. Run the full synthetic B0 cohort-to-brief journey, security/recovery matrix,
   scale budgets, accessibility matrix, and freshly installed universal-Mac
   qualification on one exact commit/artifact.
7. Start A0 only after B0 acceptance and keep the complete B0 regression matrix
   green.

No future capability or weak service should be deleted to make the status look
clean. Preserve it as domain or prototype input, then promote it only through
the owning dependency-complete vertical slice and exact evidence gate.
