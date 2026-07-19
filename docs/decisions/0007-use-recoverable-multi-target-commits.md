# 0007: Use recoverable multi-target commits for canonical writes

- Status: accepted
- Date: 2026-07-18
- Deciders: Electric Town maintainer
- Contexts: workspace, all canonical record owners
- Requirements: LRM-WS-004, LRM-WS-005, LRM-WS-006
- Feature gates: FG-R1-002

## Context and problem

One product command can update several open files and an audit envelope. Filesystems do not provide portable atomic visibility across those targets. A temporary-file rename protects one file but cannot decide or recover the whole command.

## Decision

Canonical mutations use a workspace-owned recoverable operation:

1. Render and stage every target with expected existence, content digest, and revision.
2. Flush staged bytes and the operation manifest.
3. Recheck all preconditions immediately before commitment.
4. Persist and flush an explicit `COMMIT` decision.
5. Publish targets one at a time, recording durable progress and directory durability.
6. Persist `COMPLETE`, then update a disposable projection or mark it stale.

Recovery discards an operation that never reached `COMMIT`. After `COMMIT`, recovery rolls forward. Liaison does not promise rollback after the durable decision. If a canonical target differs from both the expected and committed digest, recovery stops with a conflict and never overwrites the external edit.

Minimal audit evidence that belongs to a sensitive mutation is a target in the same operation. Completed evidence is bounded and contains identifiers, hashes, phases, and results rather than sensitive payloads.

## Consequences

- Current workspace and Person writes must migrate to the operation engine.
- The engine is multi-target even when an early caller has one target.
- Fault injection covers every phase and publish boundary.
- A non-cooperating editor may briefly observe partial post-crash publication; recovery, not impossible cross-file atomicity, is the documented guarantee.

## Migration, rollback, or reversal conditions

The unwired single-target journal in PR #28 is source material only. Its digest and durability helpers may be transplanted, but its format is not canonical. A replacement must retain explicit commit decisions, final preconditions, roll-forward recovery, and external-edit refusal.
