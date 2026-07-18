# Changelog

All notable changes to Liaison RM are recorded here. The format follows Keep a Changelog principles and the project uses semantic versioning after the first tagged release.

## Unreleased

### Added

- Repository governance, contribution, knowledge, content-quality, architecture, and user-experience review standards.
- Pull-request evidence requirements and repository policy checks.
- Product, domain, security, platform, sharing, provider-neutral connection, and release specifications.
- Machine-readable requirements, persona UAT cases, feature gates, and implementation tasks.
- Interactive desktop and mobile concept with committed review screens and browser-level interaction and accessibility smoke tests.
- Rust Workspace and People bounded contexts, typed shared identifiers and revisions, a Markdown/YAML vault adapter, and a local `liaison` CLI vertical slice.
- Cross-platform Rust formatting, checking, Clippy, domain, adapter, CLI, architecture, repository, and specification tests.
- Provider-neutral Connections context, `object-store@1`, local-folder reference adapter, WIT contract, and cross-platform conformance suite.
- Relationship-memory contract separating intent, evidence, maintenance status, purpose-specific readiness, and queue ordering.
- Topic Pack, field-state, profile-readiness, and Review Policy examples, schemas, validation, and relationship-review screens.
- Identity and Profiles domain types for Topic Packs, explicit information states, sealed sensitive values, Purpose Definitions, and purpose-specific readiness.
- Review and Attention reason-only policies, hard suppressions, explainable queue items, deterministic ordering, and capacity-bounded queue construction.
- Local Tauri desktop alpha for workspace creation, People capture, validation, accessible reflow, and universal macOS review bundles.
- Comprehensive public README, project handoff, status, development, documentation index, repository metadata contract, and coding-agent entry points.

### Changed

- The repository README now documents the product model, personas, architecture, storage, release profiles, provider and sharing boundaries, implemented runtime, development commands, status, and claim discipline.
- Reason-only review is the personal-workspace default; weighted Review Priority is explicitly queue ordering rather than relationship strength.
- Review queues preserve factual reasons and do not expose a relationship-strength score.
- Agent instructions now require the project handoff and current status before task selection.

### Fixed

- Desktop alpha now compiles and lints cleanly across Linux, macOS, and Windows: Tauri command arguments acknowledge required ownership, the default workspace path uses `map_or_else`, and a deterministic Windows `icon.ico` resource is generated for `tauri-build`.

### Security

- Documented the prohibition on undeclared network requests, hidden telemetry, secret material in canonical files, and provider or plugin access without an explicit grant.
- Defined separate Airgap and Connected-local build profiles and least-disclosure handling for sensitive relationship and workplace data.
- Kept network, provider, SQL, Tauri, and secret-storage dependencies out of Workspace, People, Profiles, and Review and Attention domain crates.
- Provider registration remains inert without a purpose-bound grant, and the local adapter claims backup/single-writer modes only.
- Private assessments and sensitive Topic Pack values require explicit classification, purpose, and sharing grants.
- Sensitive and secret profile definitions and values require sealed storage in the domain contract.
- Desktop alpha compiles no network client, uses a local-only CSP, and separates ad-hoc review artifacts from notarized release artifacts.
