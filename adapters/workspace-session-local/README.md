# Local Workspace Session authority

This adapter owns Liaison's local operating-system writer authority. Two open
file locks form the authority: the workspace-local lock preserves same-path
coordination, and a per-user zero-byte lock keyed only by `WorkspaceId`
coordinates copied or synchronised paths. `.liaison/workspace-writer.json` is bounded,
best-effort diagnostic metadata only; it is never used to grant, steal, or
release authority.

The per-user registry is rooted beneath the operating system's local-data
directory at `io.github.electric-town.liaison-rm-writer-authority-v1`. Its
entries contain no workspace path, person data, process identifier, or
diagnostic. Missing local-data components beneath the owned home directory are
created one component at a time through no-follow capability handles; a
missing custom root outside that boundary fails closed.

The adapter opens the workspace, `.liaison`, registry, and lock files through
retained capability handles and refuses symlink, reparse, replacement,
unexpected-data, ownership, permissions, and hard-link seams. Unix requires an
owned `0700` registry and owned `0600`, single-link entries. Windows queries
the actual owner SID and DACL, rejects broad write/control rights, compares
handle-backed file identities, and omits delete sharing. The identity lock is
released before the path-local lock on explicit close, RAII drop, or operating-
system process-handle cleanup. Every session work lease revalidates both
authorities before exposing repositories.

Unix advisory locks coordinate cooperating Liaison processes. They cannot stop
a hostile or non-cooperating same-user process from unlinking the lock inode or
writing canonical files directly. Liaison detects an already-replaced inode
before issuing more work, but there is no hostile-process atomicity guarantee;
P03 final preconditions and recoverable operations remain required.

The manifest is read and validated before authority acquisition, then read and
validated again under composite authority. A changed identity or schema aborts
the session and releases both handles. The second manifest becomes the session
snapshot. For current cooperating Liaison processes on the same user account
and machine, a copied workspace therefore cannot transfer a live writer lease.

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
- Platform-local registry resolution, Unix identity checks, and Windows owner
  and DACL inspection use pinned `dirs`, `rustix`, and target-only
  `windows-permissions`; their decision and checksums are recorded in
  `docs/evidence/dependencies/workspace-identity-registry.md`.
- The lock itself uses Rust's standard-library file-lock API; no PID or age
  heuristic participates in authority.

Focused tests cover same-path and copied-path contention, different identities,
safe first use, hostile registry shapes, stale and malformed diagnostics,
symlink and hard-link rejection, retained-directory replacement, and forced
child-process termination. The Windows code and dependency compile under the
pinned GNU cross-target locally; native owner/DACL, replacement, and process
tests run in the pinned `windows-2022` workflow. Until that exact-head job
passes, no Windows runtime result is claimed. Airgap artifact contents and
final binary-size impact remain release-qualification work.

This authority coordinates current cooperating Liaison processes for one OS
user on one machine. It does not stop older builds, hostile same-user writes,
other user accounts, or other machines. P03 still owns final mutation
preconditions, durable commit decisions, and recoverable operations.
