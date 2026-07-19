# P02 Workspace Session authority evidence

Date: 2026-07-19
Status: local composite authority slice reviewed; exact-head remote runtime evidence pending

## Claim boundary

The P02 source state provides a write-authoritative
`WorkspaceSession` for the current Workspace/People/Markdown slice. A session owns workspace
identity/schema, one retained capability root, path-free repositories, one
composite operating-system writer authority, a quiescence barrier, and explicit
recovery, key, and projection states.

The composite authority retains the workspace-local lock and adds a per-user,
zero-data operating-system lock keyed only by `WorkspaceId`. Current
cooperating, ordinary unconfined Liaison processes on one OS account and
machine therefore deny a second writer when a manifest is copied or
file-synchronised to another path, even if their `HOME` or XDG environment
differs. The registry contains no path, PID, diagnostics, or relationship data.
`T-B0-P02` remains open until exact-head native Linux, macOS, and Windows
evidence passes.

This is source evidence, not installed-artifact or supported-release evidence.
The final P02 commit and its rebased integration head are reported with the
review handoff rather than embedded circularly in this file. Remote
Linux/macOS/Windows matrices, installed macOS requalification, signing,
notarisation, Airgap proof, and public distribution remain pending.

The authority coordinates ordinary unconfined cooperating Liaison processes. A
hostile or non-cooperating same-user process can still unlink the lock inode or
write canonical files directly. Liaison verifies the retained control
directory, identity registry, and both lock identities before issuing each new
work guard, but that is not a hostile-process atomicity guarantee. Older builds,
other user accounts, other machines, and sandbox/host process pairs remain
outside this P02 boundary.

P02 does not claim a cross-container authority namespace. Linux rejects a
Flatpak process with typed `Unsupported` while `/.flatpak-info` is present;
there is no fallback to its private XDG data directory. A macOS App Sandbox GUI
cannot be paired authoritatively with the current host CLI without an App Group
or broker, and Windows AppContainer/MSIX AppData redirection can isolate a GUI
from the host registry. The currently reviewed macOS app is unsandboxed and
the Windows workflow builds a full-trust NSIS artifact. Any future sandboxed
package remains blocked until it supplies and proves one shared authority
broker/namespace or fails closed before opening a writer session.

## Behaviours exercised

- `WorkspaceSession` is non-`Clone`, application-owned through `Arc`, and owns
  its authority-bearing handle for the full open lifetime.
- Authority and `BoundMarkdownVault` repositories are derived from the same
  already-open capability root; later operations receive no raw workspace path.
- `.liaison`, the stable lock file, the diagnostic sidecar, manifests, and
  People records are opened relative to retained capability directories with
  no-follow checks at governed boundaries.
- A same-path second process receives `workspace.writer-already-active`; a
  copied-path process with the same manifest identity receives
  `workspace.identity-writer-already-active`. The bounded
  JSON sidecar is untrusted diagnostic metadata and does not grant, steal, or
  release the lock.
- Missing, malformed, stale, and oversized diagnostics do not change authority.
- Symlinked or replaced authority paths fail with typed errors rather than
  falling through to another filesystem object.
- A real child process holding either the same path or a different path with
  the same identity denies the contender. Forced exit releases both operating-
  system locks through handle cleanup; no PID or age heuristic participates.
- Real child processes using production `bind` vary `HOME` and
  `XDG_DATA_HOME`, run both workspace-copy launch orders, and acquire only
  after the holder exits. The Windows native variant additionally varies
  `USERPROFILE` and `LOCALAPPDATA`.
- Linux/macOS derive the unconfined account home from the operating-system
  account database; Windows uses Known Folder APIs. An inaccessible canonical
  authority returns a typed safe error and no alternate registry is created.
- A synthetic Flatpak marker produces typed `Unsupported`; native Flatpak
  packaging remains a future broker/fail-closed gate rather than a current
  authority claim.
- Different Workspace IDs can hold authority concurrently. Stale empty
  registry entries neither grant nor deny authority.
- Registry roots and entries reject relative paths, symlinks/reparse points,
  replacement, unsafe ownership or permissions, unexpected bytes, and unsafe
  hard-link counts. Safe first use creates only missing owned local-data
  components with no-follow traversal.
- The manifest is validated before acquisition and re-read under authority.
  Identity/schema drift aborts and releases authority; the session retains the
  post-lock manifest.
- `begin_work` revalidates live authority, rejects work after quiescence starts,
  and lets issued work drain before `close` releases the lock.
- Read-only Health does not acquire or materialise writer artifacts and remains
  available during contention, malformed records, and parseable newer schemas.
  Desktop one-shot Health also labels its inspected folder independently of
  the active writer session.
- Person mutation/list ports are path-free and are reached only through a live
  session work guard. The public `BoundMarkdownVault` cannot implement
  `PersonRepository`; an opaque guard-borrowed adapter is the only production
  route, with compile-fail/pass fixtures covering the boundary. Session
  identity and schema are revalidated before use.
- New manifests explicitly publish the required `people` module and validate
  against the committed version-one JSON Schema. A P01 manifest without the
  later field opens with the same default, permits guarded People work, and is
  not rewritten.
- Replacing the ambient workspace path after open does not redirect repository
  operations away from the retained root capability.
- Desktop open/create switching closes the previous native session before
  accepting the replacement. If that close fails, the interface keeps the
  previous selection and best-effort closes the replacement. Rust tests prove
  each close releases the corresponding real lock; browser tests prove both
  switching paths and the rollback orchestration.

## Traceability disposition

| Record | P02 evidence | Remaining work |
|---|---|---|
| `T-B0-P02` | Workspace Session, environment-independent composite path/identity OS authority for ordinary unconfined processes, quiescence, session-bound repositories, lock-free Health, copy denial, and process-exit release are implemented and locally exercised. | Exact-head remote Linux, macOS, and Windows runtime qualification remains pending; sandboxed packaging needs one shared broker/namespace or explicit fail-closed proof. |
| `LRM-WS-009` | Typed same-path and copied-path second-writer exclusion, read-only Health, explicit states, rename checks, both production launch orders, and process-exit release are covered for ordinary unconfined same-account processes without weakening identity authority. | Exact-head native Linux/Windows runtime evidence and the enclosing P02 gate remain pending; cross-container host/GUI authority is explicitly unsupported, not treated as a second valid namespace. |
| `UAT-042` | The malformed-record, healthy-record, active-writer, typed-lock, and process-release portions are covered. | P03 must cover injected crashes, durable commit decisions, roll-forward, staging cleanup, and external-edit final preconditions. |
| `FG-B0-001` | The application/session/Health portion is advanced. | The gate remains open until P03 recoverable operations and the complete fault-injection matrix pass. |

Neither `UAT-042` nor `FG-B0-001` is closed by P02.

The production-path regression was demonstrated before the locator change: on
macOS, a contender launched with a different `HOME`/`XDG_DATA_HOME` reported
`LIAISON_CHILD_PRODUCTION_ACQUIRED` while the first process still held the same
`WorkspaceId`. The same exact test passes after the change and reports typed
contention in both launch orders, followed by acquisition only after holder
exit.

## Reproducible local checks

The following checks passed in the source worktree:

```text
cargo fmt --all --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
cargo check -p liaison-workspace-session-local -p liaison-workspace -p liaison-application -p liaison-cli --all-targets --all-features --locked --target x86_64-apple-darwin
cargo clippy -p liaison-workspace-session-local -p liaison-workspace -p liaison-application -p liaison-cli --all-targets --all-features --locked --target x86_64-apple-darwin -- -D warnings
cargo check -p liaison-workspace-session-local -p liaison-workspace -p liaison-application -p liaison-cli --all-targets --all-features --locked --target x86_64-pc-windows-gnu
cargo clippy -p liaison-workspace-session-local -p liaison-workspace -p liaison-application -p liaison-cli --all-targets --all-features --locked --target x86_64-pc-windows-gnu -- -D warnings
python3 scripts/check_repository.py
python3 scripts/check_spec.py
python3 scripts/check_architecture.py
python3 scripts/check_providers.py
python3 scripts/check_wit_contract.py
python3 scripts/check_desktop_shell.py
python3 scripts/check_workspace_manifest.py
python3 scripts/check_localization.py
python3 scripts/check_relationship_model.py
python3 scripts/check_public_site.py
python3 scripts/generate_traceability.py --check
node --check apps/desktop/ui/app.js
python3 scripts/test_desktop_ui.py
```

The Workspace Session adapter test also passed inside a Linux
`rust:1.97.1-bookworm` container copied from this worktree: 21 passed and the
two child-helper entry points were intentionally ignored. That run exercised
the Linux production child-process test, including divergent environment
values, both launch orders, contention, and release after exit. It is useful
local Linux runtime evidence, but it does not replace the required exact-head
remote workflow result.

The browser test used the workflow-pinned Pillow `11.3.0`, Playwright `1.57.0`,
and Chromium build `1200`, wrote its report/screenshots outside the worktree,
and passed workspace create/open switching, rollback, Person, Health, focus,
mobile reflow, dark mode, and zero-external-request assertions.

The workflow's generated-asset byte check did not pass on this macOS host:

```text
$ python3 scripts/generate_desktop_assets.py --check
Desktop assets differ from generator output: 32x32.png, 128x128.png, 128x128@2x.png, icon.icns
```

`icon.ico` matched. P02 changed neither the generator nor any icon. The check
byte-compares host-generated PNG/ICNS encodings and this is recorded as a
pre-existing host-portability issue; the checked-in assets were deliberately
not regenerated. The semantic desktop-shell check passed.

The local Rust target inventory was extended with `x86_64-pc-windows-gnu`.
`cargo check` and Clippy with warnings denied passed for the Workspace Session,
Workspace, application, and CLI crates on that Windows target, including
`windows-permissions` 0.2.4. This is compile evidence only: no Windows runtime
or native filesystem result is claimed until the `windows-2022` job passes.

## Dependency and release limits

The capability-filesystem dependency decision is recorded in
[`../dependencies/cap-std-4.0.2.md`](../dependencies/cap-std-4.0.2.md). The
identity registry dependency decision is recorded in
[`../dependencies/workspace-identity-registry.md`](../dependencies/workspace-identity-registry.md).
The development-only compile-boundary dependency is recorded in
[`../dependencies/trybuild-1.0.118.md`](../dependencies/trybuild-1.0.118.md).
The local host did not have `cargo-deny` or `cargo-audit`, so their licence and
advisory gates were not reproduced locally; the repository does not yet
provide those jobs, so this is an unimplemented release gate. No P02 installed
application was built or substituted for the reviewed P01 installed-app
evidence.

P03 remains responsible for durable operation journals, a durable commit
decision, directory durability, multi-target roll-forward, staged-output
cleanup, external-edit final preconditions, and projection-stale handling.
