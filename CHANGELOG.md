# Changelog

All notable changes to Liaison RM are recorded here. The format follows Keep a Changelog principles and the project uses semantic versioning after the first tagged release.

## Unreleased

### Added

- Repository governance, contribution, knowledge, content-quality, architecture, and user-experience review standards.
- Pull-request evidence requirements and repository policy checks.
- Product, domain, security, platform, sharing, provider-neutral connection, and release specifications.
- Machine-readable requirements, persona UAT cases, feature gates, and implementation tasks.
- Interactive desktop and mobile concept with six committed review screens and browser-level interaction and accessibility smoke tests.
- Rust Workspace and People bounded contexts, typed shared identifiers and revisions, a Markdown/YAML vault adapter, and a local `liaison` CLI vertical slice.
- Cross-platform Rust formatting, checking, Clippy, domain, adapter, CLI, architecture, repository, and specification tests.
- Provider-neutral Connections context, `object-store@1`, local-folder reference adapter, WIT contract, and cross-platform conformance suite.
- Local Tauri desktop alpha for workspace creation, People capture, validation, accessible reflow, and universal macOS review bundles.
- Comprehensive public README, project handoff, status, development, documentation index, repository metadata contract, and coding-agent entry points.

### Changed

- The repository README now documents the product model, personas, architecture, storage, release profiles, provider and sharing boundaries, development commands, status, and claim discipline.
- Agent instructions now require the project handoff and current status before task selection.

### Fixed

- Desktop alpha now compiles and lints cleanly across Linux, macOS, and Windows: Tauri command arguments acknowledge required ownership, the default workspace path uses `map_or_else`, and a deterministic Windows `icon.ico` resource is generated for `tauri-build`.

### Security

- Documented the prohibition on undeclared network requests, hidden telemetry, secret material in canonical files, and provider or plugin access without an explicit grant.
- Defined separate Airgap and Connected-local build profiles and least-disclosure handling for sensitive relationship and workplace data.
- Kept network, provider, SQL, Tauri, and secret-storage dependencies out of the initial Workspace and People domain crates.
- Provider registration remains inert without a purpose-bound grant, and the local adapter claims backup/single-writer modes only.
- Desktop alpha compiles no network client, uses a local-only CSP, and separates ad-hoc review artifacts from notarized release artifacts.
