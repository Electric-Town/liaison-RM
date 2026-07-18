# Liaison application

This crate is the single composition root used by Liaison RM inbound adapters.
It composes the Workspace and People contexts with the Markdown vault adapter
and exposes serializable commands, queries, results, and structured errors.

## Current boundary

- A path is accepted only by workspace initialise and open commands.
- A successful open returns a typed `WorkspaceSessionId`.
- Workspace inspection and People commands resolve the workspace through a
  write-authoritative session; one-shot Health is separately path-selected,
  read-only, and lock-free.
- Each session owns one retained root capability, its operating-system writer
  authority, and path-free repositories derived from that exact root handle.
- Close rejects new work, drains issued work guards, releases authority, and
  invalidates the old session identifier.
- Opening a workspace keeps healthy People visible when another Person file is
  malformed and returns the validation finding for repair.
- Runtime time, identifiers, and randomness are injected through
  `RuntimePorts`, so tests do not depend on ambient time or identifiers.

The current session claims writer authority only while its operating-system
path-local lock handle is live. An independently copied workspace with the same
identity at another path is not yet excluded, so `LRM-WS-009` and `T-B0-P02`
remain blocked. Recovery, key, and projection states are explicitly
unavailable until P03, workspace security, and Directory projection provide
their real implementations. P03 also owns durable multi-target commit decisions
and final mutation preconditions; this P02 boundary does not imply them.

## Status language

`app_status` deliberately reports a local-authoritative review build, no
configured connection, and release evidence that is not yet proven. A build
profile in a workspace manifest is not evidence that an installed artifact has
passed the Airgap release gate.
