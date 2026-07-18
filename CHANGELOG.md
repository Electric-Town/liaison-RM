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
- Review and Attention and Topic Pack contracts that separate relationship intent, factual evidence, maintenance status, purpose-specific readiness, and optional Review Priority.
- Machine-validated profile-configuration and review-policy schemas with an open YAML example.
- Versioned locale-catalogue schema, source and pseudolocale fixtures, draft Irish, Japanese, and Brazilian Portuguese catalogues, and automated key, placeholder, Unicode, and expansion checks.

### Changed

- The repository README defines Liaison RM as a local-authoritative, open-file relationship manager.
- Localization review now requires stable keys, human approval for production language, locale-aware formatting, and explicit public/private metadata boundaries.

### Fixed

- Desktop alpha now compiles and lints cleanly across Linux, macOS, and Windows: Tauri command arguments acknowledge required ownership, the default workspace path uses `map_or_else`, and a deterministic Windows `icon.ico` resource is generated for `tauri-build`.

### Security

- Documented the prohibition on undeclared network requests, hidden telemetry, secret material in canonical files, and provider or plugin access without an explicit grant.
- Defined separate Airgap and Connected-local build profiles and least-disclosure handling for sensitive relationship and workplace data.
- Kept network, provider, SQL, Tauri, and secret-storage dependencies out of the initial Workspace and People domain crates.
- Provider registration remains inert without a purpose-bound grant, and the local adapter claims backup/single-writer modes only.
- Desktop alpha compiles no network client, uses a local-only CSP, and separates ad-hoc review artifacts from notarized release artifacts.
- Review and Attention prohibits employee ranking, social-credit behavior, hidden relationship-value inference, and private-assessment disclosure without an explicit decision.
- Localization tooling may not transmit private vault text or insert private relationship data into public structured metadata without an explicit approved boundary.
