# `cap-std` and `cap-fs-ext` 4.0.2 dependency review

Reviewed: 2026-07-18
Owner: Workspace
Decision: accepted for the P02 local Workspace Session adapter

## Why they are direct

`cap-std` lets the adapter retain an already-open workspace directory and keep
all later manifest, Health, and Person access relative to that capability.
`cap-fs-ext` supplies the extension APIs used for no-follow and platform-aware
filesystem opening. Together they keep raw ambient paths out of a live
Workspace Session and make the same root handle the source of both writer
authority and bound repositories.

The crates do not replace the operating-system lock. Liaison uses Rust's
standard-library file-lock API on the retained stable lock file.

## Version, provenance, and licence

- Declared versions: `cap-std = 4.0.2` and `cap-fs-ext = 4.0.2`.
- Upstream: <https://github.com/bytecodealliance/cap-std>
- Documentation: <https://docs.rs/cap-std/4.0.2/cap_std/> and
  <https://docs.rs/cap-fs-ext/4.0.2/cap_fs_ext/>.
- Licence expression reported by Cargo metadata:
  `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT`.
- `cap-std` registry checksum:
  `7281235d6e96d3544ca18bba9049be92f4190f8d923e3caef1b5f66cfa752608`.
- `cap-fs-ext` registry checksum:
  `d78e5a3368ae89b7cb68186411452b4b9fac8b41be9c19bf3f47c2d2c8e36e6b`.

The exact versions and checksums are retained in `Cargo.lock`. Version changes
require another lockfile, licence, platform, advisory, and Airgap review.

## Security and platform boundary

RustSec advisory
[`RUSTSEC-2024-0445`](https://rustsec.org/advisories/RUSTSEC-2024-0445.html)
describes a Windows superscript device-name issue in `cap-primitives` and lists
`>= 3.4.1` as patched. The selected 4.0.2 family is newer than that patched
boundary. This observation is not a substitute for a current complete advisory
scan.

The normal dependency tree adds the Bytecode Alliance capability filesystem
family and its platform support (`cap-primitives`, `io-extras`, `io-lifetimes`,
`rustix`, and target-specific Windows support). It adds no HTTP client, DNS
client, remote endpoint, account, telemetry, updater, or database.

The local review compiled and exercised macOS/Unix behavior. Windows-specific
share-mode code and runtime tests are guarded by `cfg(windows)` and require a
Windows runner. Only `aarch64-apple-darwin` and `x86_64-apple-darwin` targets
were installed on the review host, so this decision does not claim a Windows
runtime result.

## Airgap and audit limits

Capability filesystem access is local operating-system I/O and does not grant
network access. This review supports that dependency boundary only. It is not
proof that a compiled Airgap artifact has no socket capability.

Neither `cargo-deny` nor `cargo-audit` was installed on the local review host.
The repository does not currently provide those jobs. A licence/advisory
release gate therefore remains to be implemented and passed before a release
claim.

## Verification

```text
cargo metadata --locked --format-version 1
cargo tree -p liaison-workspace-session-local --edges normal
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

The exact-head results belong in P02 CI evidence. This file records the
dependency decision and its limits; it does not substitute for those runs.
