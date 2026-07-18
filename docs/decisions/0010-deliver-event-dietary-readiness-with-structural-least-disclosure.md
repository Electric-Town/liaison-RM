# 0010: Deliver event dietary readiness with structural least disclosure

- Status: accepted
- Date: 2026-07-18
- Deciders: Electric Town maintainer
- Contexts: events, profiles, directory, workspace security
- Requirements: LRM-EV-001, LRM-EV-002, LRM-EV-003, LRM-EV-004, LRM-EV-005
- Feature gates: FG-R3-001, FG-R3-002, FG-R3-003, FG-R3-004, FG-R3-005

## Context and problem

An event operator must reconcile every attendee without turning missing information into “no restriction” or exposing medical detail to a caterer. Large cohorts and historical briefs also need immutable, inspectable evidence rather than a mutable readiness percentage.

## Decision

The Events context owns Event state, immutable cohort revisions, event-local resolution, readiness derivation, brief evidence, and delivery evidence. Cohorts use JSONL plus a manifest containing the normalized predicate, source revisions, stable identity set, schema, and content hash.

Every attendee resolves to exactly one explicit outcome. Unknown, pending, declined, unreachable, excluded, conflicting, provided, and verified none remain distinct. Staleness is derived from verified time, purpose policy, and current source revision; it does not rewrite the profile.

B0 stores a constrained dietary category and operational catering instruction. It does not collect diagnosis, medical history, treatment detail, or free-form diagnostic narrative. The `DietaryOperationalView` accepted by brief generation structurally excludes restricted detail.

Internal brief generation commits sealed immutable evidence in the workspace. External CSV or print-safe HTML delivery is a separate verified operation that never overwrites an earlier file. Failed delivery leaves the internal brief valid and retryable.

## Consequences

- Readiness totals must reconcile exactly to cohort size.
- CSV delivery prevents spreadsheet formula execution.
- A changed source creates a new brief revision and marks older evidence stale; history is not rewritten.
- The graph is not required for the B0 workflow.

## Migration, rollback, or reversal conditions

No event dietary format becomes canonical before schemas, round-trip fixtures, key/grant enforcement, and clean recovery pass. A richer health-data product would require a separate decision and regulatory review.
