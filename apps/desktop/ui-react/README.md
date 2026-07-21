# Liaison desktop typed React foundation

Status: stacked P04.1 draft. This directory is not the active production interface.

## Purpose

The foundation proves the React/TypeScript build, versioned application-response envelope, injected Tauri transport seam, stable B0 route identities, stable Events stage identities, and P03 operation phases before route parity begins.

It deliberately does not duplicate Workspace, People, Events, readiness, recovery, or disclosure rules. Those remain in Rust application and domain services.

## Authority boundary

- The local Workspace and Rust application services are authoritative.
- Browser storage is not canonical and is not used by this foundation.
- The frontend has no fetch, WebSocket, EventSource, XMLHttpRequest, or beacon authority.
- The Tauri invoke function is injected into one typed transport adapter.
- Every response must use `liaison/application-response@1`.
- An unversioned or structurally invalid response fails closed.

## Parallel migration

The existing `apps/desktop/ui` shell remains active while P04 is built in slices. This package builds to `apps/desktop/ui-react-dist`; Tauri is not pointed at that directory in P04.1.

The migration may switch the review build only after Workspace, People, Health, operation recovery, localization, keyboard, reflow, theme, and installed-platform parity pass. The legacy shell is removed in a separate reviewed change after that evidence exists.

## Commands

From this directory:

```text
npm ci
npm run typecheck
npm run test
npm run build
```

Repository-level validation also runs `python3 scripts/check_frontend_contract.py`.

## Adding a command

1. Add or change the Rust application contract first.
2. Record the versioning and compatibility decision.
3. Update `CommandPayloadMap` and `CommandResponseMap` without adding domain derivation.
4. Add a contract fixture and a transport test.
5. Add route behavior only in the later owning P04 slice.
6. Prove no browser storage or network authority was introduced.
