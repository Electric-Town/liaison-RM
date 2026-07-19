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
- `winsafe` 0.0.28 supplies safe wrappers for opening the current process token
  and resolving `FOLDERID_LocalAppData` with that explicit token. Only the
  `advapi` and `shell` features are enabled; Profile is not a second locator
  prerequisite. The crate is MIT-licensed, has Rust 1.87 as its minimum
  supported version, and adds no third-party dependency of its own.
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
| `winsafe` 0.0.28 | MIT | <https://github.com/rodrigocfd/winsafe> | `7d3922ac07e168376bc6799e7896fc737ddb9538be7533ca44bc3463091db1ad` |
| `rustix` 1.1.4 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | <https://github.com/bytecodealliance/rustix> | `b6fe4565b9518b83ef4f91bb47ce29620ca828bd32cb7e408f0062e9930ba190` |
| `windows-permissions` 0.2.4 | MIT | <https://github.com/danieldulaney/windows-permissions-rs> | `9e2ccdc3c6bf4d4a094e031b63fadd08d8e42abd259940eb8aa5fdc09d4bf9be` |

The exact versions and checksums are pinned by `Cargo.lock`. Any change needs a
new licence, provenance, platform, security, and Airgap review.

`winsafe` remains a `0.0.x` single-maintainer wrapper. Its known-folder helper
returns a UTF-8 `String`; invalid UTF-16 becomes a diagnostic string, which the
adapter's absolute-path and existing-directory checks reject. The helper frees
the returned Shell allocation on success but not on an HRESULT failure with a
non-null output. Microsoft assigns that allocation to the caller in either
case. Native qualification must exercise the locator; the residual dependency
issue remains bounded to one call and should be raised upstream rather than
worked around with repository-local unsafe code.

## Security boundary

On Linux and macOS, Liaison obtains the effective account's home from the OS
account database and ignores `HOME`; Linux also ignores `XDG_DATA_HOME`.
Windows passes an explicitly opened current-process user token to
`SHGetKnownFolderPath(FOLDERID_LocalAppData)` and does not use the tested
`HOME`, `XDG_DATA_HOME`, `USERPROFILE`, or `LOCALAPPDATA` overrides as
authority. On Unix, Liaison
safely creates only missing components beneath the owned account home, one
component at a time with no-follow capability handles. On Windows,
the returned LocalAppData Known Folder is operating-system traversal
infrastructure: it must already exist, is opened without following a reparse
point, and is retained by file identity. Profile is not a second locator
prerequisite. Liaison does not require an elevated account's LocalAppData owner
to equal its token-user SID and does not manufacture a missing Known Folder. A
missing or inaccessible canonical location fails closed with a typed error;
there is no fallback.

On Unix, registry ownership must match the effective UID, the registry mode is
exactly `0700`, and every zero-byte lock entry is owned, mode `0600`, and has
one hard link. On Windows, creation-only hardening changes the Liaison-owned
registry and lock owner to the current token-user SID and installs a protected
three-entry full-control DACL for that user, LocalSystem, and Builtin
Administrators. Validation requires that exact owner and canonical ACL rather
than blacklisting only familiar broad groups; an arbitrary account ACE or an
inherit-only extra ACE therefore fails. A pre-existing noncanonical object is
never repaired automatically. Reparse points are rejected, retained handles
omit delete sharing, and the created-directory handle is identity-matched to
the retained directory before use. An inspection failure is typed; Liaison
does not downgrade to an assumed-safe LocalAppData path.

The current capability directory builder cannot attach a Windows security
descriptor to `CreateDirectoryW` atomically. After a successful create,
Liaison immediately opens the new path without following a reparse point and
without delete sharing, accepts only TokenUser, LocalSystem, or Builtin
Administrators as the initial owner, installs the canonical descriptor, and
identity-matches that handle to the retained registry directory. A different
owner, replacement, non-empty created registry, or hardening failure fails
closed. This is evidence for the stated ordinary cooperating, unconfined
same-account contract; it is not a claim against a hostile same-user,
administrator, or LocalSystem process. A future broader local-adversary claim
would require creation with an atomic security descriptor or an equivalent
brokered authority primitive.

`windows-permissions` is an older, narrowly scoped wrapper over Win32 security
APIs. Its blanket `WindowsSecure` handle implementation passes
`SE_UNKNOWN_OBJECT_TYPE` to `GetSecurityInfo`; that is not the file/directory
contract required by Liaison. The adapter therefore calls the crate's public
`GetSecurityInfo` and `SetSecurityInfo` wrappers with `SE_FILE_OBJECT`
explicitly, then uses the crate's SID, DACL, ACE, and SDDL types for the
bounded canonical checks above.
Microsoft documents both the object-type parameter and `SE_FILE_OBJECT` in the
[GetSecurityInfo contract](https://learn.microsoft.com/en-us/windows/win32/api/aclapi/nf-aclapi-getsecurityinfo)
and [`SE_OBJECT_TYPE` enumeration](https://learn.microsoft.com/en-us/windows/win32/api/accctrl/ne-accctrl-se_object_type).
The dependency's target-specific surface remains confined to this adapter. The
locally installed pinned Rust toolchain compiled and linted the exact call for
`x86_64-pc-windows-gnu`; native semantics still require the repository's
`windows-2022` runtime tests before P02 can close.

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
