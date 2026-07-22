# 0016: Scope general migration safety to R5

- Status: accepted
- Date: 2026-07-22
- Deciders: Electric Town maintainer
- Acceptance provenance: maintainer-authorised full implementation mandate on 2026-07-22, constrained by the accepted B0-before-A0 boundary and zero-orphan review.
- Contexts: connections, workspace, migration, people
- Requirements: LRM-WS-007, LRM-WS-017
- UAT: UAT-066
- Tasks: T-R5-005, T-B0-P09-OKF
- Feature gates: FG-R5-005, FG-B0-001

## Context and problem

At exact base `49ee419e30f2d71524dd6fa15badf1ec4b8d0e27`, `LRM-WS-007` was still a release-R1 requirement owned by `T-B0-P03` and `FG-B0-001`, even though its universal “Migrations shall” wording described general migration safety. The existing general-migration task `T-R5-005` and gate `FG-R5-005` were already scoped to R5. That drift could silently make generic dry-run and backup/restore acceptance a hidden B0 prerequisite or let the narrow B0 OKF People normalization claim evidence for a broader future capability.

B0 has one deliberate format exception: Liaison's own legacy People files must be normalized to the pinned OKF v0.1 Draft authoring profile. ADR 0013 already gives that operation a narrower and more specific failure-atomic contract through `LRM-WS-017` and `UAT-066`. General and third-party imports or migrations remain post-A0 work.

Reassigning `LRM-WS-007` to the existing `T-R5-005`/`FG-R5-005` pair is the least-invented correction: it aligns the generic requirement with the already-declared future migration owner, creates no new B0 capability or task, and removes the accidental P03/B0 ownership edge.

## Decision

`LRM-WS-007` applies only to general and third-party post-A0 migrations owned by `T-R5-005` and accepted by `FG-R5-005`. Its statement and acceptance explicitly name the scope.

The required B0 OKF People normalization is governed exclusively by `LRM-WS-017` and `UAT-066` under `T-B0-P09-OKF` and `FG-B0-001`. It does not inherit `LRM-WS-007`, and B0 tasks and gates must not claim that generic requirement as owned or supporting evidence.

The B0 operation still requires exact preview, exact backup, journaled failure-atomic commit, final preconditions, restart recovery, idempotent rerun, curated-content preservation, and exact rollback. This decision narrows ownership; it does not weaken the OKF normalization safety contract.

## Rationale

- One requirement now describes one release boundary and has one accountable task and gate.
- The narrow B0 exception remains more precise than the generic future migration contract.
- General migration capability cannot enter B0 by wording inference, and future R5 work cannot cite B0 normalization as proof of arbitrary-format migration safety.
- Generated traceability and repository checks can reject either boundary drifting back into the other.

## Alternatives considered

### Keep the universal wording and make B0 satisfy both requirements

Rejected. It creates cross-release ownership, broadens B0 without a user-outcome dependency, and makes a People-format exception appear to prove general migration capability.

### Delete the generic migration requirement

Rejected. Later first- and third-party migration work still needs dry-run, backup, deterministic execution, validation, and rollback or explicit irreversibility.

### Make `LRM-WS-017` inherit `LRM-WS-007`

Rejected. Inheritance would obscure the exact B0 acceptance owner and make later edits to a generic contract capable of silently changing the accepted OKF normalization boundary.

## Compatibility and migration impact

This decision changes planning and claim scope only. It changes no canonical format, runtime behavior, or released compatibility promise. Existing or future B0 OKF normalization implementations continue to meet their specific safety contract; no data migration is caused by this ADR.

## Consequences

- `T-R5-005` and `FG-R5-005` exclusively own `LRM-WS-007`.
- `T-B0-P09-OKF` and `FG-B0-001` exclusively own `LRM-WS-017` and `UAT-066`.
- P03, P04, and the B0 acceptance path cannot cite generic migration dry-run or backup evidence as their own deliverable.
- The specification checker locks both the positive ownership edges and the negative cross-release boundary.

## Rollback or reversal conditions

Moving a general or third-party migration into B0 requires a separate accepted product and architecture decision, its own requirement, UAT, task, gate, privacy and compatibility analysis, and a zero-orphan traceability update. Broadening the wording of `LRM-WS-007` alone cannot reverse this decision.
