# 0009: Use a disposable Directory projection

- Status: accepted
- Date: 2026-07-18
- Deciders: Electric Town maintainer
- Contexts: people, organisations, groups, locations, memberships, workspace
- Requirements: LRM-WS-003, LRM-OR-003
- Feature gates: FG-R1-003, FG-R3-002

## Context and problem

Parsing every Person file for lookup, filtering, and cohort selection does not meet the 10,000-person and 50,000-membership working-state target. It also allows one malformed file to prevent access to every healthy record.

## Decision

Canonical scanning is tolerant: invalid records remain untouched, produce bounded Health findings, and do not remove healthy records from Directory reads. A rebuildable SQLite/FTS adapter stores stable identifiers, source paths, hashes, revisions, non-sensitive public fields, membership filter keys, projection schema, and parse findings.

The projection is never authority. Cohort finalization revalidates canonical revisions and digests. Projection failure marks Directory stale and can be repaired by deletion and deterministic rebuild. Sensitive dietary payloads and person-to-outcome associations are not stored in plaintext SQLite.

## Consequences

- Directory supports stable pagination and filters without loading the full workspace.
- Health remains available when normal editing is degraded.
- Deterministic 10,000/50,000 synthetic fixtures and query budgets are release evidence.
- Every projection schema can be discarded without semantic loss.

## Migration, rollback, or reversal conditions

The first implementation may scan canonically while the projection is unavailable, but must preserve tolerant findings and bounded results. Any replacement index must remain disposable and revalidate canonical sources before mutations or cohort finalization.
