# 0006: Use one application composition root and workspace session

- Status: accepted
- Date: 2026-07-18
- Deciders: Electric Town maintainer
- Contexts: workspace, all record-owning contexts, CLI, desktop
- Requirements: LRM-WS-001, LRM-WS-004, LRM-WS-006, LRM-WS-008
- Feature gates: FG-R1-001, FG-R1-002, FG-R1-004

## Context and problem

The current CLI and desktop construct repositories independently and pass raw workspace paths through context ports. That makes the filesystem path an accidental application identity, permits UI-specific orchestration, and leaves no process-lifetime owner for locks, recovery, key state, or projection health.

## Decision

`liaison-application` is the sole cross-context workflow coordinator. CLI, Tauri, local services, imports, automation, and tests call its typed commands and queries.

Opening a workspace creates a `WorkspaceSession`. The session binds the canonical root, workspace identity and schema, an operating-system advisory writer-lock handle, recovery state, session-bound repositories, key availability, and projection status. Mutations require a write-authoritative session. Health may inspect a workspace without accepting write authority.

Paths remain an adapter concern. Context repository ports and application commands do not accept arbitrary raw workspace paths after session migration. Tauri holds a managed session; CLI mutations acquire the same authority for the command lifetime.

Command, session, and job identifiers are typed. Clocks, identifier generation, and randomness enter through explicit application ports so tests can be deterministic.

## Consequences

- CLI and desktop cannot drift into separate domain implementations.
- A second writer receives a typed lock error instead of racing canonical files.
- Process death releases the authority lock. A sidecar may carry diagnostic metadata, but PID age is never authority to steal a lock.
- Session migration is a prerequisite for sensitive storage, checkpoints, imports, and event workflows.

## Migration, rollback, or reversal conditions

Existing path-based ports are migrated one bounded context at a time behind the composition root. Reversal requires an alternative that retains one command model, one writer authority, and equivalent recovery and test seams.
