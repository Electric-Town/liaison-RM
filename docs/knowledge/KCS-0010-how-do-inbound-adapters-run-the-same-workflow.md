---
id: KCS-0010
title: How do the CLI and desktop run the same Liaison workflow?
state: Draft
owner: application
created: 2026-07-18
reviewed: 2026-07-19
applies_to:
  - liaison-application
  - liaison-cli
  - liaison-desktop
search_terms:
  - composition root
  - command parity
  - CLI desktop drift
  - structured application error
  - workspace session
  - Tauri argument mismatch
  - WebKit event currentTarget
related_requirements:
  - LRM-AP-001
  - LRM-WS-002
  - LRM-WS-009
  - LRM-WS-011
related_uat:
  - UAT-042
related_adrs:
  - ADR-0006
---

# How do the CLI and desktop run the same Liaison workflow?

## Problem

An inbound adapter needs to open or create a workspace, validate it, and work with People. Constructing context services or a Markdown repository inside each adapter makes the raw path an accidental identity, produces different errors and revisions, and bypasses the single owner of writer authority, recovery, keys, and projection state.

## Resolution

Construct one `LiaisonApplication` for the adapter process and call its typed commands and queries. Opening or initialising returns a `WorkspaceSessionId`; later commands use that identifier instead of accepting another path.

The application result contains a contract version, command identifier, completion time, and typed value. A failure contains the same contract version, a stable code, display message, recovery action, safe details, and correlation identifier. Adapters may format that envelope, but they do not parse error strings, expose rejected sensitive input, or replace it with an unrelated message.

Workspace-relative paths inside Health findings are logical record identifiers, not host filesystem strings. The Markdown adapter converts path components to `/`-separated values before they enter the application contract. This keeps JSON, CLI, desktop, fixtures, and later APIs stable across Unix and Windows and avoids falling back to an absolute private path if a record cannot be relativised safely.

User-selected or operating-system-provided absolute folders are different: they remain native host paths because the OS must open them. Tests construct expected defaults through `PathBuf::join`; they do not impose Unix separators on a Windows Documents path.

The CLI opens a write-authoritative session for the command lifetime, except
that `workspace validate` runs one-shot lock-free Health. Tauri keeps one
managed application instance and its session map in native state; the current
disposable UI holds only the opaque active session identifier returned by that
native application. Replacing the selected workspace first opens the
replacement, then closes the previous session before accepting it; failure to
close the previous session keeps it selected and best-effort closes the
replacement. The disposable desktop admits one native operation at a time,
disables every native-operation field and action while it is active, and
captures both a monotonic operation generation and the starting session ID.
It applies a result only if both still own the current state and explicitly
closes an opened replacement that became superseded. Browser fixtures may fake
the typed bridge for interaction testing, but they are not storage or domain
implementations.

If closing the previous session fails, the desktop keeps that workspace
selected and attempts to close the unopened replacement. If both closes fail,
the replacement can retain invisible writer authority until process exit. The
interface must preserve that cleanup failure, tell the user to restart Liaison,
and disable further native operations rather than overwrite the recovery with
the ordinary switch error or create more hidden sessions.

Browser fixtures also cannot prove the native WebKit event lifecycle. Capture any form, button, or other event target that is needed after an `await` before yielding to the native command. A compiled P01 review exposed a false-failure path where the Person file was written successfully and WebKit then cleared `event.currentTarget`; the interface reported failure and made a dangerous retry look appropriate. The native request shape, success message, file result, and post-command UI state must be tested together.

## Why this works

The composition root owns cross-context orchestration while each bounded context retains its invariants and ports. The same application method therefore determines initial revision, tolerant reads, validation findings, error codes, and recovery guidance for every inbound adapter. `spec/fixtures/application-parity.json` records the stable subset both adapter-boundary tests must satisfy.

The P02 slice binds workspace identity/schema, one retained root capability,
path-free repository access, composite workspace-local and per-user
`WorkspaceId` operating-system authority for ordinary unconfined same-account
processes, and a quiescence barrier. The identity registry ignores process
`HOME`/XDG overrides and fails closed rather than selecting a fallback. The
manifest is re-read under authority and the post-lock value becomes the
session snapshot. Lock metadata is diagnostic only. Cross-container host/GUI
coordination requires a future shared broker/namespace and is not a current
claim. Recovery, key, and projection states remain explicit unavailable
values; recoverable operations, final mutation preconditions, Airgap isolation,
and release readiness require their own implementation and evidence.

`BoundMarkdownVault` deliberately does not implement `PersonRepository`.
Application code must hold a live `WorkspaceWorkGuard` and obtain the opaque
People repository with `people_repository(&work)`. Do not expose the private
adapter type, accept an unrelated guard as a token, or add a second path-taking
Person store. Compiler-boundary tests prove both the denied unguarded use and
the allowed guarded use.

Capability-bound manifest and Person reads reject a non-regular file before
opening it, use a no-follow nonblocking open, and validate the opened handle
again. The preflight provides a clear typed result; the nonblocking open and
post-check close the replacement race without letting a FIFO wedge one-shot
Health or an ordinary workspace command.

Windows file and directory security descriptors must be requested as
`SE_FILE_OBJECT`. Do not use `windows-permissions` 0.2.4's blanket
`WindowsSecure::security_descriptor` implementation here because it supplies
`SE_UNKNOWN_OBJECT_TYPE`; the identity-registry adapter calls the pinned
wrapper with the explicit object type before applying the same owner and DACL
checks.

Windows LocalAppData is operating-system traversal infrastructure, not a
Liaison-owned private object; Profile is not a second locator prerequisite.
Resolve `FOLDERID_LocalAppData` with an explicitly opened current-process user
token rather than process `USERPROFILE` or `LOCALAPPDATA`, then retain and
identity-check it without following reparse points. On creation only, normalise
the child Liaison registry and zero-byte locks to TokenUser ownership and the
protected user/System/Administrators ACL, then
require that exact shape on every use. Never auto-repair a pre-existing
noncanonical registry or lock, and preserve the typed `cause_issue` through an
initialisation wrapper so native failures remain diagnosable without
disclosing a path.

New version-one manifests publish `enabled_modules` and currently require the
`people` identifier. A P01 manifest without that later field is read as
`people` without rewriting its bytes; it remains invalid for new-writer schema
fixtures. One-shot Health may inspect a path without opening a writer session,
but its result must visibly name the inspected folder separately from the
active workspace.

## Verify

Run:

```bash
cargo test -p liaison-application --locked
cargo test -p liaison-cli --locked
cargo test -p liaison-desktop --locked
python3 scripts/check_workspace_manifest.py
python3 scripts/test_desktop_ui.py
python3 scripts/check_architecture.py
cargo tauri build --bundles app
```

Check that:

- the CLI and desktop depend on `liaison-application` rather than constructing People or Markdown services;
- initial creation with an email remains revision 1;
- a malformed sibling is reported by Health while healthy People remain visible;
- Health exposes the same `/`-separated workspace-relative record path on macOS, Linux, and Windows;
- semantic corruption and duplicate Person identities are findings rather than silently omitted data;
- invalid validation returns a deterministic non-zero CLI exit after emitting the report;
- a second writer receives the stable typed contention code while read-only
  Health remains available and does not create lock artifacts;
- a copied workspace with the same identity receives
  `workspace.identity-writer-already-active`, discloses neither path nor
  identity, and opens only after explicit close or process exit in both
  production launch orders despite divergent `HOME`/XDG values;
- different workspace identities remain independently writable, while a
  stale empty identity entry neither grants nor steals authority;
- `BoundMarkdownVault` fails the external-crate `PersonRepository` trait
  boundary, a repository cannot escape its work guard, and guarded use
  compiles;
- new manifests validate against the strict published schema, while the P01
  missing-module fixture opens with `people` and its manifest bytes remain
  unchanged;
- one-shot Health shows the exact inspected folder while the footer retains a
  different active workspace, and the inspection acquires no writer session;
- forced process exit releases the operating-system lock without a PID or age
  heuristic;
- registry first use and hostile owner, permission, symlink/reparse,
  replacement, data, and hard-link cases fail closed on the owning platforms;
- Windows identity-registry file and directory handles complete owner/DACL
  inspection through `GetSecurityInfo` with `SE_FILE_OBJECT` before a session
  opens;
- inaccessible canonical authority returns a typed safe error without fallback,
  Flatpak is explicitly unsupported, and no App Sandbox/AppContainer pairing
  is claimed before installed packaging proves a shared authority seam;
- opening and creating a replacement workspace close the previous session, and
  failed switching best-effort closes the replacement without changing the
  selected workspace;
- delayed or programmatic overlapping desktop operations result in at most one
  native command, one visible active session, no hidden replacement lock, no
  cross-workspace Person result, and restored controls/focus;
- if both old-session close and replacement cleanup fail, the prior selection
  remains visible, restart recovery remains announced, and every further native
  operation stays disabled until the process exits;
- manifest and `people/*.md` FIFOs or other special files return a typed result
  within the bounded regression timeout; healthy People remain listable and a
  Person special file becomes a Health finding;
- human and JSON failures retain the same stable code, recovery action, safe details, and correlation identifier;
- the rejected email or phone value is absent from errors and test output;
- the exact compiled Tauri bundle accepts the same request DTO names exercised by adapter tests;
- native Person creation shows success only after the readable file exists, updates the list/count, clears the form, and returns focus without exposing framework argument errors;
- hidden precondition or warning content is absent from the native accessibility tree after its condition becomes false.

Record the compiled artifact checksum, source commit, architecture, signature result, workflow steps, and remaining gates under `docs/evidence/macos/`. A Chromium fake-bridge pass is necessary interaction evidence, but it is not a substitute for compiled WebKit and filesystem proof.

Keep this article in Draft until the P02 exact head passes the remote macOS,
Windows, Linux, policy, architecture, and interface matrices. Local
reproduction alone is not release evidence, and P03 remains necessary for the
crash/recovery portions of UAT-042.

## If the surfaces disagree

Add or correct an application contract test first. Do not patch the discrepancy independently in JavaScript or the CLI dispatcher. If the command genuinely needs a new domain concept, update the owning context, accepted decisions, requirement/UAT traceability, and this article before widening the adapter.
