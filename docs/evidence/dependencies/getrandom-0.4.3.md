# `getrandom` 0.4.3 dependency review

Reviewed: 2026-07-18
Owner: Application
Decision: accepted for the P01 runtime port

## Why it is direct

`RuntimePorts::random_bytes` is the application-owned boundary for operating-system entropy. A direct dependency keeps that boundary explicit and fallible; it avoids coupling application code to UUID generation or to a higher-level pseudo-random generator. Production code calls only `getrandom::fill`. Deterministic tests replace the port and do not read host entropy.

## Version and provenance

- Declared version: `0.4.3`; the exact registry checksum is retained in `Cargo.lock`.
- Upstream: <https://github.com/rust-random/getrandom/tree/v0.4.3>
- API and target documentation: <https://docs.rs/getrandom/0.4.3/getrandom/>
- Licence: MIT OR Apache-2.0, compatible with Liaison RM's AGPL-3.0-only distribution.
- Maintenance observation: 0.4.3 was the current published release at review time. Version changes require another lockfile, licence, target, and Airgap review.

## Runtime and transitive boundary

For the native targets Liaison currently builds, the crate delegates to the operating-system randomness source: Linux system facilities, Windows `ProcessPrng`, and macOS `getentropy`. Its normal native dependency path is limited to target support crates already present in the workspace lockfile. It adds no HTTP client, DNS client, remote endpoint, account, telemetry, updater, or persistence layer.

WASI targets are supported by the upstream crate. Browser-style `wasm32-unknown-unknown` is not enabled here, and Liaison does not enable the optional JavaScript backend. Any future browser-WASM use must make that platform and dependency expansion explicit rather than inheriting this native approval.

## Airgap and failure behavior

Reading operating-system entropy is a local kernel/platform operation; it does not require or grant network access. The port returns a typed failure instead of substituting predictable bytes. This review supports the dependency's local execution boundary only. It is not proof that a compiled Airgap artifact has no socket capability; that remains a separate binary and runtime release gate.

## Verification

```text
cargo tree -p liaison-application --edges normal
cargo tree -i getrandom@0.4.3 --edges normal
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

The exact-head results belong in the P01 pull-request and CI evidence. This file records the review decision and its limits; it does not substitute for those runs.
