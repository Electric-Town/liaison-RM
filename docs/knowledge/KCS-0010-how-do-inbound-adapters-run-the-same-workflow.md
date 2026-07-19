---
id: KCS-0010
title: How do the CLI and desktop run the same Liaison workflow?
state: Draft
owner: application
created: 2026-07-18
reviewed: 2026-07-18
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
  - LRM-WS-011
related_uat:
  - UAT-042
related_adrs:
  - ADR-0006
---

# How do the CLI and desktop run the same Liaison workflow?

## Problem

An inbound adapter needs to open or create a workspace, validate it, and work with People. Constructing context services or a Markdown repository inside each adapter makes the raw path an accidental identity, produces different errors and revisions, and leaves no single place for later writer locks, recovery, keys, or projection state.

## Resolution

Construct one `LiaisonApplication` for the adapter process and call its typed commands and queries. Opening or initialising returns a `WorkspaceSessionId`; later commands use that identifier instead of accepting another path.

The application result contains a contract version, command identifier, completion time, and typed value. A failure contains the same contract version, a stable code, display message, recovery action, safe details, and correlation identifier. Adapters may format that envelope, but they do not parse error strings, expose rejected sensitive input, or replace it with an unrelated message.

Workspace-relative paths inside Health findings are logical record identifiers, not host filesystem strings. The Markdown adapter converts path components to `/`-separated values before they enter the application contract. This keeps JSON, CLI, desktop, fixtures, and later APIs stable across Unix and Windows and avoids falling back to an absolute private path if a record cannot be relativised safely.

User-selected or operating-system-provided absolute folders are different: they remain native host paths because the OS must open them. Tests construct expected defaults through `PathBuf::join`; they do not impose Unix separators on a Windows Documents path.

The CLI opens a session for the command lifetime. Tauri keeps one managed application instance and its session map in native state; the current disposable UI holds only the opaque active session identifier returned by that native application. Browser fixtures may fake the typed bridge for interaction testing, but they are not storage or domain implementations.

Browser fixtures also cannot prove the native WebKit event lifecycle. Capture any form, button, or other event target that is needed after an `await` before yielding to the native command. A compiled P01 review exposed a false-failure path where the Person file was written successfully and WebKit then cleared `event.currentTarget`; the interface reported failure and made a dangerous retry look appropriate. The native request shape, success message, file result, and post-command UI state must be tested together.

## Why this works

The composition root owns cross-context orchestration while each bounded context retains its invariants and ports. The same application method therefore determines initial revision, tolerant reads, validation findings, error codes, and recovery guidance for every inbound adapter. `spec/fixtures/application-parity.json` records the stable subset both adapter-boundary tests must satisfy.

This first slice binds workspace identity and repository access. It does not yet claim writer-lock authority, operation recovery, key availability, projection health, Airgap isolation, or release readiness. Those status changes require their own implementation and evidence.

## Verify

Run:

```bash
cargo test -p liaison-application --locked
cargo test -p liaison-cli --locked
cargo test -p liaison-desktop --locked
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
- human and JSON failures retain the same stable code, recovery action, safe details, and correlation identifier;
- the rejected email or phone value is absent from errors and test output;
- the exact compiled Tauri bundle accepts the same request DTO names exercised by adapter tests;
- native Person creation shows success only after the readable file exists, updates the list/count, clears the form, and returns focus without exposing framework argument errors;
- hidden precondition or warning content is absent from the native accessibility tree after its condition becomes false.

Record the compiled artifact checksum, source commit, architecture, signature result, workflow steps, and remaining gates under `docs/evidence/macos/`. A Chromium fake-bridge pass is necessary interaction evidence, but it is not a substitute for compiled WebKit and filesystem proof.

Keep this article in Draft until the P01 exact head passes the remote macOS, Windows, Linux, policy, architecture, and interface matrices. Local reproduction alone is not release evidence.

## If the surfaces disagree

Add or correct an application contract test first. Do not patch the discrepancy independently in JavaScript or the CLI dispatcher. If the command genuinely needs a new domain concept, update the owning context, accepted decisions, requirement/UAT traceability, and this article before widening the adapter.
