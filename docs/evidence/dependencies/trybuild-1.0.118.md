# `trybuild` 1.0.118 dependency review

Reviewed: 2026-07-19
Owner: Workspace and Markdown vault adapter
Decision: accepted as a development-only P02 boundary test dependency

## Why it is direct

P02 must prove a negative Rust API contract: `BoundMarkdownVault` is not a
`PersonRepository`, and the only production People repository cannot outlive
the `WorkspaceWorkGuard` from which it was created. Runtime tests cannot prove
that an external crate is unable to compile a bypass. Source-pattern checks
also miss aliases, re-exports, and trait-resolution changes.

`trybuild` compiles small external-crate fixtures and makes a test fail if an
expected misuse starts compiling. Rust documentation `compile_fail` blocks
were considered, but they do not compare the expected diagnostic and can pass
because of an unrelated compile error. The selected harness pairs two exact
compile-fail cases with a compile-pass case under a live guard.

## Version, provenance, and maintenance

- Declared version: `1.0.118`; registry checksum:
  `06649c6f63d86604ba0c8950d5a1829fc9a17afd70fc6629f481d75b6a624c78`.
- Upstream source and tag: <https://github.com/dtolnay/trybuild/tree/1.0.118>.
- Registry source and metadata: <https://crates.io/crates/trybuild/1.0.118>.
- API and source documentation: <https://docs.rs/trybuild/1.0.118/trybuild/>.
- Licence: `MIT OR Apache-2.0`, compatible with Liaison RM's
  `AGPL-3.0-only` distribution.
- Upstream minimum Rust version: 1.85; Liaison pins Rust 1.97.1.
- Maintenance observation: upstream commit `7ce4c26b` released 1.0.118 on
  2026-07-11. The repository was active and not archived at review time, and
  tags 1.0.109 through 1.0.118 showed continuing point releases.

Version changes require another lockfile, licence, compiler-diagnostic,
platform, advisory, and Airgap-effect review.

## Transitive and supply-chain boundary

The dependency is under `[dev-dependencies]`. The lockfile addition consists
of:

- `target-triple` 1.0.1, checksum
  `c3a6bfce3d99adfa72d24750a61f782f3036a81e7f86d8841ee1326deaebd171`,
  licensed `MIT OR Apache-2.0`;
- `termcolor` 1.4.1, checksum
  `06794f8f6c5c898b3275aebefa6b8a1cb24cd2c6c79397ab15774837a0bc5755`,
  licensed `Unlicense OR MIT`.

Its remaining dependencies were already present in the workspace lockfile.
The default feature set does not enable the optional diff dependency. The
package has no build script. During tests it launches the pinned local Rust
compiler against temporary fixtures; it adds no HTTP client, DNS client,
remote endpoint, account, telemetry, updater, database, or production file
format.

A GitHub Advisory Database query for `trybuild@1.0.118` returned no advisory
on 2026-07-19. This bounded check is not a substitute for a complete locked
dependency audit. `cargo-audit` and `cargo-deny` were unavailable on the local
host and remain release-automation gaps.

## Platform, binary, and Airgap effect

The fixtures use only Rust type checking and execute in the ordinary test
matrix. Exact diagnostics are pinned to Liaison's Rust toolchain. Linux,
macOS, and Windows CI must all run the workspace tests because compiler output
or target handling can differ.

The crate and its transitive additions do not enter normal dependency trees or
the Liaison desktop/CLI binaries. They therefore add no runtime network or
filesystem authority and no shipped Airgap capability. Fetching development
dependencies remains a build-environment network concern; offline reproducible
builds require the normal vendoring/cache gate and are not proven by this
review.

## Verification

```text
cargo info trybuild@1.0.118
cargo tree -p liaison-vault-markdown --target all --edges normal,dev
cargo test -p liaison-vault-markdown --all-features --locked
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
python3 scripts/check_architecture.py
```

The exact-head CI results belong in P02 evidence. This file records the
dependency decision and its limits; it does not substitute for those runs.
