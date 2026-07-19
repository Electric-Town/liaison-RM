# Events bounded context

## Purpose

Events owns event identity, the attendee lifecycle, immutable cohort revisions, dietary-readiness derivation, and least-disclosure catering-brief content evidence. It does not own person profiles, dietary source facts, organisational structure, persistence formats, or delivery transports.

This crate is the domain core for the accepted decision [0010: Deliver event dietary readiness with structural least disclosure](../../docs/decisions/0010-deliver-event-dietary-readiness-with-structural-least-disclosure.md). It deliberately contains no application services or adapters: those wire in after the P03 recoverable-operation protocol stabilises.

## Language

- **Event** — a dated, named occasion with a status of planned or cancelled.
- **Cohort** — the attendee selection; finalization freezes it, and every later change is a superseding correction, walk-in, or identity resolution with history.
- **Attendee row** — one append-only membership row with identity, origin, participation, and status; removed and duplicate rows stay inspectable.
- **Active denominator** — the count of active, deduplicated rows every readiness total must reconcile against exactly.
- **Participation** — invited, confirmed, attended, declined, cancelled, no-show, or unknown (LRM-EV-002).
- **Dietary operational view** — the authorised per-person input supplied by People: separate availability, freshness, conflict, and disclosure facts plus the profile revision (LRM-EV-010).
- **Readiness policy** — an ordered, versioned, fail-closed decision table deriving exactly one outcome per attendee.
- **Outcome** — verified none, provided, pending, declined, unreachable, excluded from catering, conflicting, stale, or unknown; unresolved identities are counted separately (LRM-EV-003).
- **Operational instruction** — a bounded single-line catering instruction; there is no field capable of carrying a diagnosis or medical narrative (LRM-EV-007).
- **Brief** — immutable least-disclosure content for a named recipient and purpose with an expiry; names are structurally absent, and an event-local opaque token appears only under an explicitly named approved policy (LRM-EV-004, LRM-EV-013).

## Current invariants

- Unknown can never become verified none: policy construction rejects any rule deriving `VerifiedNone` (or `Provided`) from another availability fact, and an attendee without a supplied view fails closed to `Unknown`.
- Derivation reads source facts and never writes them back.
- Readiness entries and totals reconcile exactly to the active denominator or assessment fails.
- Selection closes at finalization; walk-ins open after it; every change appends a correction.
- One person cannot be an active attendee twice; resolving an identity onto an already-active person supersedes the row as a duplicate.
- Preview bytes and sealed brief bytes come from one deterministic rendering and match exactly; a later source-revision change marks a sealed brief stale without touching its bytes or checksum.
- A cancelled event refuses further cohort work, assessment, and briefs.

## Application services

None yet. `assess` and brief construction are pure domain operations; commands, queries, and persistence arrive with the composition-root and recoverable-operation work (P01–P03) and the P05 contract task.

## Outbound ports

None yet. Cohort storage (JSONL plus manifest per the accepted decision), brief evidence storage, and delivery are adapter concerns that will consume this domain model.

## Cross-context rules

- People owns dietary source facts; Events consumes them only as `DietaryOperationalView` values passed in by the caller and records the profile revisions it used.
- Events never mutates a person profile and stores no personal names.
- Follow-up work items (`EventReadinessFollowUp`, LRM-EV-009) and participation reporting (LRM-EV-006) are separate slices and are not implemented here.

## Data classification

Cohort membership, participation, readiness outcomes, and operational instructions are sensitive workplace data. Brief content is restricted to counts and grouped instructions; person identifiers appear only in internal evidence (`input_revisions`) and never in delivered bytes. Real workplace data remains blocked behind the pilot governance gate regardless of what this crate supports.

## Tests

`cargo test -p liaison-events` covers the fail-closed decision table exhaustively across every fact combination, the participation transition matrix, correction and duplicate reconciliation of the active denominator, missing-view fail-closed behaviour, preview/sealed byte equality, opaque-token gating, and stale marking without history rewrites.
