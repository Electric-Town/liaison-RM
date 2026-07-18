# R1 Rust vertical-slice evidence

Status: implementation evidence for the initial Workspace, People, Markdown adapter, and CLI slice. This document does not open the R1 release gate.

## Implemented boundary

- Shared typed identifiers and record revisions.
- Workspace manifest invariants and validation service.
- Person aggregate, typed contact points, partial dates, archive behaviour, and repository port.
- Markdown/YAML adapter with stable IDs, revision preconditions, unknown-front-matter preservation, and readable records.
- CLI commands for workspace initialization, inspection and validation, and person creation and listing.
- Human-readable and JSON output modes.

## Automated checks

The pull-request workflow runs on Ubuntu, macOS, and Windows:

```text
cargo fmt --all --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

A separate architecture job runs:

```text
python scripts/check_architecture.py
python scripts/check_repository.py
python scripts/check_spec.py
```

The current pull-request head is acceptable only when every matrix job and architecture job succeeds. The pull-request description must be updated with the exact successful head SHA before the draft is marked ready.

## Deliberately unopened gates

- Crash-safe journal and fault injection.
- Complete migration and read-only recovery modes.
- Projection rebuild.
- Encryption and secret-store integration.
- Airgap binary inspection.
- Signed packages for Linux, macOS, and Windows.
- Desktop UI and formal accessibility evidence.
- Provider, sharing, API, MCP, and plugin execution.

## Manual review focus

- Domain crates must not import filesystem, provider, network, database, Tauri, or UI mechanisms.
- The CLI must call application services rather than edit canonical files directly.
- Invalid records must be reported rather than silently deleted.
- User-authored Markdown body content and unknown front-matter keys must survive supported save paths.
- Test fixtures must remain synthetic and must not contain real personal information.
