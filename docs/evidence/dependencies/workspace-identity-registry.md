# Workspace identity-registry dependency review

Reviewed: 2026-07-19
Owner: Workspace
Decision: accepted for P02, subject to exact-head native platform evidence

## Why these dependencies are direct

The per-user `WorkspaceId` authority must resolve one account-stable location
independent of process environment overrides, verify Unix ownership, and
inspect real Windows owner/DACL state. Treating `HOME`, `XDG_DATA_HOME`, or an
AppData-looking string as authority would make the copied-workspace exclusion
claim false.

- `uzers` 0.12.2 safely queries the Unix account database with `getpwuid_r` for
  the effective UID. Default cache, mock, and logging features are disabled;
  its only normal dependency is `libc`.
- `dirs` 6.0.0 is now Windows-only and resolves the current account's
  `FOLDERID_LocalAppData` and `FOLDERID_Profile` Known Folders.
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
| `uzers` 0.12.2 | MIT | <https://github.com/rustadopt/uzers-rs> | `0b8275fb1afee25b4111d2dc8b5c505dbbc4afd0b990cb96deb2d88bff8be18d` |
| `dirs` 6.0.0 | MIT OR Apache-2.0 | <https://github.com/soc/dirs-rs> | `c3e8aa94d75141228480295a7d0e7feb620b1a5ad9f12bc40be62411e38cce4e` |
| `rustix` 1.1.4 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | <https://github.com/bytecodealliance/rustix> | `b6fe4565b9518b83ef4f91bb47ce29620ca828bd32cb7e408f0062e9930ba190` |
| `windows-permissions` 0.2.4 | MIT | <https://github.com/danieldulaney/windows-permissions-rs> | `9e2ccdc3c6bf4d4a094e031b63fadd08d8e42abd259940eb8aa5fdc09d4bf9be` |

The exact versions and checksums are pinned by `Cargo.lock`. Any change needs a
new licence, provenance, platform, security, and Airgap review.

## Security boundary

On Linux and macOS, Liaison obtains the effective account's home from the OS
account database and ignores `HOME`; Linux also ignores `XDG_DATA_HOME`.
Windows Known Folder calls do not use the tested `HOME`, `XDG_DATA_HOME`,
`USERPROFILE`, or `LOCALAPPDATA` overrides as authority. Liaison safely creates
only missing components beneath the owned account home, one component at a
time with no-follow capability handles. A missing root outside that boundary
or inaccessible canonical location fails closed with a typed error; there is
no fallback. The final registry is owned and private.

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

Container-local data roots are deliberately not accepted as equivalent
account authority. [Flatpak documents](https://docs.flatpak.org/en/latest/flatpak-command-reference.html)
its `/.flatpak-info` marker and app-private XDG rewrites; the adapter returns
typed `Unsupported` until a host-shared broker/namespace exists. [Apple
documents](https://developer.apple.com/documentation/security/protecting-user-data-with-app-sandbox)
that App Sandbox uses an application container rather than unrestricted account
home access, and [Microsoft
documents](https://learn.microsoft.com/en-us/windows/msix/desktop/desktop-to-uwp-behind-the-scenes)
per-package AppData redirection for packaged/AppContainer processes. P02
therefore claims only ordinary unconfined macOS processes and unvirtualised
full-trust Windows processes. Future sandboxed packages must add a reviewed
shared authority seam or prove fail-closed behavior in installed-artifact
tests; app-private fallback authority is prohibited.

The dependency set adds no HTTP/DNS client, endpoint, account, telemetry,
updater, database, or persistence payload. Registry files contain no path,
PID, diagnostic, or relationship data.

## Verification

```text
cargo metadata --format-version 1 --locked --offline
cargo tree -p liaison-workspace-session-local --edges normal --target all
cargo test -p liaison-workspace-session-local --all-features --locked
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
