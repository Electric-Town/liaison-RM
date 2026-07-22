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

The current session claims writer authority only while both its workspace-local
lock and per-user `WorkspaceId` lock handles are live. An independently copied
workspace with the same identity is denied to cooperating Liaison processes on
the same user account and machine. `T-B0-P02` is complete with accepted exact-head
Linux, macOS, and Windows evidence. Baseline
`3499a6e9278fc72d2498a9978df59f30d03722e6` also contains the candidate P03
durable multi-target commit decisions and final mutation preconditions. Those
operations are implemented candidate source, not accepted P03 capability, until
hardening, exact qualification, the three-identity attestation, and executable
artifact receipt complete. Key and Directory projection states remain explicitly
unavailable until their owning security and Directory tasks provide them.

Later source through `49ee419e30f2d71524dd6fa15badf1ec4b8d0e27`
adds out-of-order Event and desktop candidates, but exact-head formatting checks
are red and no accepted P03/OBS identity exists. Its in-memory Event projection,
static fixtures, `DESIGN.md`, P04 review claims, installed app, and preserved
`vB0` tag are not application-contract or release evidence. P03 remains current;
P03D, P04, P05-P11, and B0 acceptance remain blocked; PILOT remains deferred by
machine authority.

## Status language

`app_status` deliberately reports a local-authoritative review build, no
configured connection, and release evidence that is not yet proven. A build
profile in a workspace manifest is not evidence that an installed artifact has
passed the Airgap release gate.
