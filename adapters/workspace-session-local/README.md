# Local Workspace Session authority

This adapter owns Liaison's local operating-system writer authority. The open
file lock is the authority. `.liaison/workspace-writer.json` is bounded,
best-effort diagnostic metadata only; it is never used to grant, steal, or
release authority.

The adapter opens the workspace and `.liaison` as capability directories,
opens the stable lock file relative to the retained control-directory handle,
and refuses symlink or replaced-path seams. On Unix it compares device/inode
identity. On Windows it opens the retained directory and lock handles without
delete sharing, so the governed paths cannot be replaced while authority is
live. The lock is released by RAII and by operating-system handle cleanup on
process termination. Every session work lease revalidates the retained control
directory and lock-file identity before exposing repositories.

Unix advisory locks coordinate cooperating Liaison processes. They cannot stop
a hostile or non-cooperating same-user process from unlinking the lock inode or
writing canonical files directly. Liaison detects an already-replaced inode
before issuing more work, but there is no hostile-process atomicity guarantee;
P03 final preconditions and recoverable operations remain required.

The lock is currently path-local. A copied or file-synchronised workspace at a
different path has an independent lock inode even when its manifest carries the
same workspace identifier. Until identity-scoped cross-path exclusion is
implemented, `LRM-WS-009` and `T-B0-P02` remain blocked.

## Dependency review

- `cap-std` and `cap-fs-ext` are Bytecode Alliance capability-filesystem
  crates, version `4.0.2`, licensed as `Apache-2.0 WITH LLVM-exception OR
  Apache-2.0 OR MIT`.
- The selected version is newer than the RustSec Windows device-name issue
  fixed in `cap-std` 3.4.1. The local review host did not have `cargo-deny` or
  `cargo-audit` installed, and the repository does not yet provide those jobs;
  a licence/advisory release gate remains to be implemented and passed.
- The full dependency decision and lockfile checksums are recorded in
  `docs/evidence/dependencies/cap-std-4.0.2.md`.
- The lock itself uses Rust's standard-library file-lock API; no PID or age
  heuristic participates in authority.

Focused Unix tests cover independent-handle contention, stale and malformed
diagnostics, symlink rejection, bound-directory replacement, and forced child
process termination. Windows-specific source and runtime tests are compiled
only on Windows; this P02 evidence does not claim a Windows runtime run from a
non-Windows host. Airgap artifact contents and final binary-size impact remain
release-qualification work, not claims made by this adapter.
