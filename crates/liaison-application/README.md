# Liaison application

This crate is the single composition root used by Liaison RM inbound adapters.
It composes the Workspace and People contexts with the Markdown vault adapter
and exposes serializable commands, queries, results, and structured errors.

## Current boundary

- A path is accepted only by workspace initialise and open commands.
- A successful open returns a typed `WorkspaceSessionId`.
- Validation and People commands resolve the workspace through that session.
- Opening a workspace keeps healthy People visible when another Person file is
  malformed and returns the validation finding for repair.
- Runtime time, identifiers, and randomness are injected through
  `RuntimePorts`, so tests do not depend on ambient time or identifiers.

The current session binds a workspace identity and repository root. It does not
yet claim writer-lock authority, operation recovery, key availability, or
projection health. Those capabilities belong to the Workspace Session delivery
phase and must be added here rather than independently in the CLI or Tauri.

## Status language

`app_status` deliberately reports a local-authoritative review build, no
configured connection, and release evidence that is not yet proven. A build
profile in a workspace manifest is not evidence that an installed artifact has
passed the Airgap release gate.
