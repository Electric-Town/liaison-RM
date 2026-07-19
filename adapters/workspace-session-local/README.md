# Local Workspace Session authority

This adapter owns Liaison's local operating-system writer authority. Two open
file locks form the authority: the workspace-local lock preserves same-path
coordination, and a per-user zero-byte lock keyed only by `WorkspaceId`
coordinates copied or synchronised paths. `.liaison/workspace-writer.json` is bounded,
best-effort diagnostic metadata only; it is never used to grant, steal, or
release authority.

The per-user registry has one environment-independent location for ordinary,
unconfined processes on an operating-system account. Linux and macOS resolve
the effective account's home directory through the operating-system account
database rather than `HOME`; Linux ignores `XDG_DATA_HOME` and uses
`~/.local/share`, while macOS uses `~/Library/Application Support`. Windows
uses the current account's `FOLDERID_LocalAppData` and `FOLDERID_Profile`
Known Folder results rather than environment strings. The final directory is
named `io.github.electric-town.liaison-rm-writer-authority-v1`.

Registry entries contain no workspace path, person data, process identifier,
or diagnostic. Missing Unix local-data components beneath the owned account
home are created one component at a time through no-follow capability handles.
Windows LocalAppData must already exist as an operating-system Known Folder;
its retained ancestors are traversal infrastructure, while the child Liaison
registry and locks are the private security boundary. An inaccessible
canonical location or a missing custom root returns a typed authority error;
production does not select a fallback registry.

The adapter opens the workspace, `.liaison`, registry, and lock files through
retained capability handles and refuses symlink, reparse, replacement,
unexpected-data, ownership, permissions, and hard-link seams. Unix requires an
owned `0700` registry and owned `0600`, single-link entries. Windows
normalises newly created Liaison objects to the token-user owner and a
protected user/System/Administrators ACL, requires that exact canonical shape
thereafter, compares handle-backed file identities, and omits delete sharing.
Existing unsafe registry or lock objects are rejected rather than repaired.
The identity lock is
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
  and DACL inspection use pinned target-specific `dirs`, `uzers`, `rustix`, and
  `windows-permissions`; their decision and checksums are recorded in
  `docs/evidence/dependencies/workspace-identity-registry.md`.
- The lock itself uses Rust's standard-library file-lock API; no PID or age
  heuristic participates in authority.

Focused tests cover same-path and copied-path contention, different identities,
safe first use, hostile registry shapes, stale and malformed diagnostics,
symlink and hard-link rejection, retained-directory replacement, and forced
child-process termination. Native Windows coverage additionally exercises a
broad traversal parent, canonical creation of the registry and lock, and
rejection of extra direct or inherit-only account ACEs. Real child processes
using production `bind`
exercise divergent `HOME`/`XDG_DATA_HOME` values in both launch orders and
prove release after process exit; the Windows variant also changes
`USERPROFILE` and `LOCALAPPDATA`. The Windows code and dependency compile under
the pinned GNU cross-target locally; native owner/DACL, replacement, and
process tests run in the pinned `windows-2022` workflow. Until that exact-head
job passes, no Windows runtime result is claimed. Airgap artifact contents and
final binary-size impact remain release-qualification work.

This is an unconfined-host authority contract, not a cross-container contract.
Flatpak is rejected with typed `Unsupported` while `/.flatpak-info` is present;
its XDG paths are app-private and it cannot coordinate with a host CLI until a
shared authority broker or namespace exists. A macOS App Sandbox container and
a host CLI likewise have no supported shared authority today. Windows
AppContainer/MSIX redirection can make AppData package-private, so P02 supports
the current unvirtualised full-trust/NSIS process model only. A future
sandboxed GUI package must either use a reviewed host-shared broker/namespace
or fail closed, with native packaging tests proving that boundary. No current
source or packaging evidence claims host-CLI coordination across those
sandboxes.

This authority coordinates current cooperating, ordinary unconfined Liaison
processes for one OS account on one machine. It does not stop older builds,
hostile same-user writes, sandbox/host pairs, other user accounts, or other
machines. P03 still owns final mutation preconditions, durable commit
decisions, and recoverable operations.
