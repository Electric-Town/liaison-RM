# Workspace identity-registry dependency review

Reviewed: 2026-07-19
Owner: Workspace
Decision: accepted for P02, subject to exact-head native platform evidence

## Why these dependencies are direct

The per-user `WorkspaceId` authority must resolve the operating system's local
data directory, verify Unix ownership, and inspect real Windows owner/DACL
state. Treating a conventional path as secure without those checks would make
the copied-workspace exclusion claim false.

- `dirs` 6.0.0 resolves the platform-local data and home directory contracts.
- `rustix` 1.1.4 supplies the effective Unix user identity used for owner
  checks. It is a Unix-only direct edge.
- `windows-permissions` 0.2.4 supplies safe wrappers for owner SID, DACL, and
  effective-rights inspection. It is a Windows-only direct edge.

None of these crates grants writer authority. The live standard-library file
locks remain authority, and `cap-std`/`cap-fs-ext` retain no-follow handles and
handle-backed file identities.

## Version, provenance, licence, and lock identity

| Crate | Licence | Upstream | `Cargo.lock` checksum |
|---|---|---|---|
| `dirs` 6.0.0 | MIT OR Apache-2.0 | <https://github.com/soc/dirs-rs> | `c3e8aa94d75141228480295a7d0e7feb620b1a5ad9f12bc40be62411e38cce4e` |
| `rustix` 1.1.4 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | <https://github.com/bytecodealliance/rustix> | `b6fe4565b9518b83ef4f91bb47ce29620ca828bd32cb7e408f0062e9930ba190` |
| `windows-permissions` 0.2.4 | MIT | <https://github.com/danieldulaney/windows-permissions-rs> | `9e2ccdc3c6bf4d4a094e031b63fadd08d8e42abd259940eb8aa5fdc09d4bf9be` |

The exact versions and checksums are pinned by `Cargo.lock`. Any change needs a
new licence, provenance, platform, security, and Airgap review.

## Security boundary

`dirs` returns a location; it does not establish trust. Liaison safely creates
only missing components beneath the owned home directory, one component at a
time with no-follow capability handles. A missing custom local-data root
outside that boundary fails closed. The final registry is owned and private.

On Unix, registry ownership must match the effective UID, the registry mode is
exactly `0700`, and every zero-byte lock entry is owned, mode `0600`, and has
one hard link. On Windows, the actual handle owner must match the current
process SID; a present DACL must not grant Everyone, Authenticated Users, or
Builtin Users write, delete, owner, or DACL-control rights. Reparse points are
rejected and retained handles omit delete sharing. An inspection failure is a
typed authority-unavailable result; Liaison does not downgrade to an assumed
safe LocalAppData path.

`windows-permissions` is an older, narrowly scoped wrapper over Win32 security
APIs. Its target-specific surface is confined to this adapter. The locally
installed pinned Rust toolchain compiled and linted the exact dependency and
call sites for `x86_64-pc-windows-gnu`; native semantics still require the
repository's `windows-2022` runtime tests before P02 can close.

The dependency set adds no HTTP/DNS client, endpoint, account, telemetry,
updater, database, or persistence payload. Registry files contain no path,
PID, diagnostic, or relationship data.

## Verification

```text
cargo metadata --format-version 1 --locked --offline
cargo tree -p liaison-workspace-session-local --edges normal --target all
cargo check -p liaison-workspace-session-local -p liaison-workspace -p liaison-application -p liaison-cli --all-targets --all-features --locked --target x86_64-pc-windows-gnu
cargo clippy -p liaison-workspace-session-local -p liaison-workspace -p liaison-application -p liaison-cli --all-targets --all-features --locked --target x86_64-pc-windows-gnu -- -D warnings
```

The Windows workflow additionally runs the Workspace Session adapter,
Workspace context, application, and CLI runtime tests on `windows-2022` before
building the desktop review bundle. That remote result is not claimed by this
source review until the exact head runs.

Neither `cargo-deny` nor `cargo-audit` is currently a repository gate. Complete
licence/advisory automation, native platform runtime evidence, Airgap binary
inspection, and installed-app qualification remain release work.
