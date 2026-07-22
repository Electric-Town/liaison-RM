---
id: KCS-0014
title: When may the Events destination be enabled?
state: Draft
owner: events
created: 2026-07-21
reviewed: 2026-07-21
applies_to:
  - liaison-desktop
  - liaison-application
  - events
search_terms:
  - Events destination
  - stale attendees
  - cross-workspace event state
  - inert event controls
  - dietary readiness
  - T-B0-P11
related_requirements:
  - LRM-EV-001
  - LRM-EV-002
  - LRM-EV-003
  - LRM-EV-004
  - LRM-EV-005
  - LRM-EV-007
  - LRM-EV-008
  - LRM-EV-009
  - LRM-EV-010
  - LRM-EV-011
  - LRM-EV-012
  - LRM-EV-013
related_uat:
  - UAT-009
  - UAT-010
  - UAT-011
  - UAT-012
  - UAT-041
  - UAT-062
related_adrs:
  - ADR-0010
---

# When may the Events destination be enabled?

## Problem

An unmerged desktop prototype exposed an Events destination before the Event bounded context, application use cases, canonical adapter, and delivery evidence existed. Browser reproduction on 2026-07-21 found three user-visible failures:

- opening Events without a workspace raised an undefined-list error while static totals and a synthetic attendee remained visible;
- after a user created People in one workspace and opened a different empty workspace, Events retained the first workspace's attendee rows and count;
- action-needed and location filters and the history control accepted clicks but produced no state change, command, or recovery message.

The attendee rows were page-local JavaScript state seeded from the People screen. They were not an Event cohort, did not belong to the active `WorkspaceSession`, and had no Event application command or persisted canonical source. The controls were presentational markup without implemented operations. Correcting individual listeners would leave that false architecture in place.

## Current resolution

Keep Events absent from review-build navigation until `T-B0-P11` is complete. Its identifier and order may be reserved in the typed route contract, but no Events control is rendered in the DOM, including a hidden, disabled, placeholder, or “coming soon” control.

`scripts/check_desktop_shell.py` reads the machine-owned `T-B0-P11` status from `spec/traceability-ownership.json` and rejects any static-shell `data-route="events"` control while that task is not complete. The checker contains a negative self-test for the premature-navigation shape. When P04 replaces the static shell, its route tests must preserve the same status-bound rule before this static check is retired.

Do not repair the prototype by adding more page-local attendee arrays, hard-coded totals, fake success messages, browser storage, or direct filesystem access. Event work begins in its owning domain and reaches the interface through the shared application boundary.

## Dependency-complete delivery order

The machine-readable task graph remains authoritative. The shortest accepted route to a working Events destination is:

1. Complete `T-B0-P03`, including exact-head evidence for recoverable multi-target operations and final mutation preconditions.
2. Run `T-B0-P03D`; create canonical `DESIGN.md`, complete plan design review, and amend P04 before desktop implementation.
3. Complete `T-B0-P04` for the typed React/Tauri client, generated DTOs, semantic components, localisation, and accessibility ratchet.
4. Complete `T-B0-P05` for revisioned Directory, Event, and dietary operational-view contracts. Complete `T-B0-P05-OKF`, `T-B0-P06`, `T-B0-P06-REPAIR`, and `T-B0-P09-OKF` so Event selection reads a tolerant, repaired, normalised Directory rather than copying the People screen.
5. Complete `T-B0-P07` and `T-B0-P08` for purpose grants, sealed values, audit, checkpoints, encrypted recovery packages, and clean-install restore.
6. Complete `T-B0-P09` for typed organisations, locations, memberships, visible cohort predicates, staged import, and duplicate reconciliation.
7. Complete `T-B0-P10` for the Event domain, application contract, canonical persistence, immutable cohorts, attendee lifecycle, exact readiness, immutable briefs, and verified delivery.
8. Complete `T-B0-P11` by wiring the five-stage workflow into the compiled desktop, including interruption and recovery states. Only this task enables the Events destination.
9. Keep the product labelled a review alpha until `T-B0-ACCEPT` passes against the installed universal Mac artifact.

`T-B0-P05` can start after P03 while P03D/P04 proceeds. Later steps may proceed only when their declared dependencies are complete. A draft pull request may show partial work, but it must list the gates that remain closed and must not expose Events as working product behaviour.

## Required implementation contracts

### Domain and cross-context boundaries

- The Events context owns Event Details, cohort finalisation, attendee rows, lifecycle transitions, corrections, readiness derivation, briefs, and delivery evidence.
- Directory supplies revisioned cohort candidates and source predicates through a typed port. Events records the selection method, source, predicates, and Person revision snapshot; it does not mirror the People page's current array.
- Profiles supplies a purpose-authorised dietary operational view. Diagnosis, treatment, medical history, private notes, and unrestricted detailed values are not representable in the Event view.
- Workspace Security owns the purpose grant, recipient, expiry, key access, and audit contract. An Event or interface component cannot invent permission from a role label.
- Workplace types structurally omit relationship strength, value, allocation, cadence, attention weight, and review ranking.

### Event state and persistence

- A valid Details draft survives navigation, process interruption, and relaunch.
- Finalising a cohort creates an immutable revision. Later duplicate resolution, identity resolution, removal, reinstatement, cancellation, walk-in, attendance, no-show, and event cancellation are superseding corrections with history, not destructive edits.
- Every active deduplicated attendee contributes to exactly one ordered, versioned, fail-closed readiness outcome. Unknown never means verified none.
- Availability, freshness, conflict, and disclosure remain separate source facts. The decision-table version and source revisions remain inspectable.
- Event canonical mutations use the recoverable multi-target operation protocol. Adapters preserve unknown fields and user-authored Markdown sections on supported round trips.
- Internal brief generation and external CSV/HTML delivery are separate operations. Retry cannot overwrite an existing brief or export.
- A brief is immutable, purpose-bound, recipient-bound, expiring, checksummed evidence. Preview and emitted bytes are identical. Names are absent by default; an opaque attendee identifier appears only when the approved policy requires it.

### Application and inbound adapters

- Stable commands and queries are exposed by `liaison-application` and exercised through the CLI before or with the desktop adapter.
- Every Event command after open accepts the opaque `WorkspaceSessionId`; it does not accept a raw workspace path or reconstruct repositories.
- Command results use the shared versioned envelope and typed `ApplicationError`. Display copy contains the safe message and recovery action, never private diagnostic detail.
- Tauri and CLI parity fixtures cover the same command identifiers, DTO fields, state revisions, results, and error codes.
- Opening or creating a replacement workspace closes the prior session and reloads the selected Event from the replacement session. No attendee, count, filter, drawer, or pending result may survive from the prior workspace.
- One native operation owns the interface at a time. Generation and starting-session checks reject stale async results and clean up superseded sessions.

### Desktop behaviour

The compiled interface implements Details, Cohort, Attendees, Readiness, and Brief as one resumable state machine. The candidate presentation contract in `docs/evidence/ux/b0-events-design-contract-candidate.md` is an input to P03D; it is not implementation authority by itself.

Before P11 can complete:

- every visible button, filter, row action, dialog action, step transition, and history control has a real application operation or an explicit deterministic presentation effect;
- empty, loading, partial, stale, conflict, permission, invalid, busy, success, interruption, undo-or-correction, and recovery states have visible text and a safe next action;
- counts derive from the persisted Event revision and reconcile with the semantic table after every transition;
- filters expose removable predicates and keyboard operation, and do not silently mutate the finalized cohort;
- focus returns to the initiating control after a dialog, invalid submission focuses the first error, and status changes use bounded live-region announcements;
- 400 percent zoom, narrow-window reflow, VoiceOver, keyboard-only operation, reduced motion, long content, and the supported locale fixtures retain meaning and action;
- colour, icon, position, animation, hover, graph, and drag are never the only carriers of state or action;
- synthetic fixtures replace static fake product data, and the no-workspace state contains no attendee names or totals.

## Verification matrix

| Layer | Evidence required before P11 completion |
|---|---|
| Domain | Property tests for every readiness combination and invalid combination; the full attendee transition matrix; denominator reconciliation after every allowed correction |
| Application | Session-scoped command/query tests, optimistic revision conflicts, stale-result rejection, typed errors, interruption, idempotent retry, and cross-workspace isolation |
| Canonical adapter | Versioned round trips, unknown-field preservation, external-edit refusal, pre/post-commit crash injection, roll-forward recovery, stale projection marking, and no direct path-based bypass |
| Directory and Profiles | Reproducible cohort predicates, duplicate and unresolved identity cases, operational-view least disclosure, purpose denial, stale source revisions, and 10,000-person synthetic scale fixtures |
| CLI and Tauri | Shared parity fixtures for request and result DTOs, command identifiers, revisions, safe errors, and readable canonical effects |
| Brief and delivery | Byte-identical preview/output, negative-disclosure fixtures, immutable history, checksum and receipt verification, failed delivery retry, and no-overwrite proof |
| Browser interaction | Every control changes the documented state or announces a typed failure; workspace switching clears Event presentation state; keyboard, focus, reflow, localisation, and reduced-motion cases pass |
| Installed application | Exact-head compiled WebKit QA, VoiceOver, offline and socket-denial evidence, interruption/relaunch recovery, clean-install recovery-package restore, and universal Mac artifact provenance |

Use synthetic People, organisations, dietary facts, recipients, and files in every test and screenshot. A Chromium fake bridge proves browser interaction only. It does not prove native WebKit, canonical persistence, platform packaging, disclosure, or recovery.

## Pull-request and rollback rule

If P10 or P11 spans more than one pull request, each pull request names its owning task, exact dependency state, implemented vertical slice, tests, and closed and open gates. Partial work stays draft or remains unreachable. Do not mark `T-B0-P10` or `T-B0-P11` complete for scaffolding, route declarations, screenshots, or a happy-path browser fixture.

If a later Events interface regresses before B0 acceptance, remove or hide the Events destination in an ordinary rollback pull request while preserving valid canonical Event records and recovery evidence. Do not delete user records to make the interface appear consistent. Re-enable the destination only after the failing invariant passes again on the submitted exact head.
