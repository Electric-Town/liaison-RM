# P02 Workspace Session authority evidence

Date: 2026-07-18
Status: local path-bound authority slice reviewed; workspace-identity and exact-head remote evidence pending

## Claim boundary

The P02 source state provides a path-bound write-authoritative
`WorkspaceSession` for the current Workspace/People/Markdown slice. A session owns workspace
identity/schema, one retained capability root, path-free repositories, one
operating-system writer authority, a quiescence barrier, and explicit recovery,
key, and projection states.

The current lock lives inside the selected workspace. A file-synced or copied
workspace at another path contains an independent lock inode and can still
obtain writer authority for the same workspace identifier. This leaves
`LRM-WS-009` and `T-B0-P02` blocked until identity-scoped cross-path exclusion
and native platform evidence exist. Rename and retained-root checks do not
substitute for that missing identity registry.

This is source evidence, not installed-artifact or supported-release evidence.
The P02 work began from P01 commit `807071e`; the final P02 commit is reported
with the review handoff rather than embedded circularly in this file. Remote
Linux/macOS/Windows matrices, installed macOS requalification, signing,
notarisation, Airgap proof, and public distribution remain pending.

On Unix, the advisory lock coordinates cooperating Liaison processes. A
hostile or non-cooperating same-user process can still unlink the lock inode or
write canonical files directly. Liaison verifies the retained control
directory and lock inode before issuing each new work guard, but that is not a
hostile-process atomicity guarantee.

## Behaviours exercised

- `WorkspaceSession` is non-`Clone`, application-owned through `Arc`, and owns
  its authority-bearing handle for the full open lifetime.
- Authority and `BoundMarkdownVault` repositories are derived from the same
  already-open capability root; later operations receive no raw workspace path.
- `.liaison`, the stable lock file, the diagnostic sidecar, manifests, and
  People records are opened relative to retained capability directories with
  no-follow checks at governed boundaries.
- A second process receives `workspace.writer-already-active`. The bounded
  JSON sidecar is untrusted diagnostic metadata and does not grant, steal, or
  release the lock.
- Missing, malformed, stale, and oversized diagnostics do not change authority.
- Symlinked or replaced authority paths fail with typed errors rather than
  falling through to another filesystem object.
- A forced child-process exit releases the operating-system lock through handle
  cleanup; no PID or age heuristic participates.
- `begin_work` revalidates live authority, rejects work after quiescence starts,
  and lets issued work drain before `close` releases the lock.
- Read-only Health does not acquire or materialise writer artifacts and remains
  available during contention, malformed records, and parseable newer schemas.
- Person mutation/list ports are path-free and are reached only through a live
  session work guard. Session identity and schema are revalidated before use.
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
| `T-B0-P02` | Workspace Session, path-local OS writer lock, quiescence, session-bound repositories, and lock-free Health are implemented in the current slice. | Identity-scoped cross-path exclusion plus exact-head remote platform qualification remain pending. |
| `LRM-WS-009` | Typed same-path second-writer exclusion, read-only Health, explicit states, rename checks, and forced process-exit release are covered locally. | A file-synced copy at another path can still acquire an independent lock; identity-scoped denial and Windows runtime evidence remain pending. |
| `UAT-042` | The malformed-record, healthy-record, active-writer, typed-lock, and process-release portions are covered. | P03 must cover injected crashes, durable commit decisions, roll-forward, staging cleanup, and external-edit final preconditions. |
| `FG-B0-001` | The application/session/Health portion is advanced. | The gate remains open until P03 recoverable operations and the complete fault-injection matrix pass. |

Neither `UAT-042` nor `FG-B0-001` is closed by P02.

## Reproducible local checks

The following checks passed in the source worktree:

```text
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
python3 scripts/check_repository.py
python3 scripts/check_spec.py
python3 scripts/check_architecture.py
python3 scripts/check_providers.py
python3 scripts/check_wit_contract.py
python3 scripts/check_desktop_shell.py
node --check apps/desktop/ui/app.js
python3 scripts/test_desktop_ui.py
```

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

The local Rust target inventory contained only `aarch64-apple-darwin` and
`x86_64-apple-darwin`; no Windows runtime or cross-target result is claimed.

## Dependency and release limits

The capability-filesystem dependency decision is recorded in
[`../dependencies/cap-std-4.0.2.md`](../dependencies/cap-std-4.0.2.md). The
local host did not have `cargo-deny` or `cargo-audit`, so their licence and
advisory gates were not reproduced locally; the repository does not yet
provide those jobs, so this is an unimplemented release gate. No P02 installed
application was built or substituted for the reviewed P01 installed-app
evidence.

P03 remains responsible for durable operation journals, a durable commit
decision, directory durability, multi-target roll-forward, staged-output
cleanup, external-edit final preconditions, and projection-stale handling.
