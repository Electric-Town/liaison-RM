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
- Relationship-memory contract separating intent, evidence, maintenance status, and purpose-specific readiness.
- Topic Pack, field-state, profile-readiness, and Review Policy examples, schemas, validation, and three review screens.
- Identity and Profiles domain types for Topic Packs, explicit information states, sealed sensitive values, Purpose Definitions, and purpose-specific readiness.
- Review and Attention reason-only policies, hard suppressions, explainable queue items, and capacity-bounded queue construction.
- Versioned locale catalogs, an expansion pseudolocale, Unicode and placeholder validation, and human-review gates for Irish, Japanese, and Brazilian Portuguese.

### Changed

- The repository README defines Liaison RM as a local-authoritative, open-file relationship manager.
- Reason-only review is the personal-workspace default; weighted Review Priority is explicitly queue ordering rather than relationship strength.
- Review queues preserve factual reasons and do not expose a relationship-strength score.

### Security

- Documented the prohibition on undeclared network requests, hidden telemetry, secret material in canonical files, and provider or plugin access without an explicit grant.
- Defined separate Airgap and Connected-local build profiles and least-disclosure handling for sensitive relationship and workplace data.
- Kept network, provider, SQL, Tauri, and secret-storage dependencies out of the initial Workspace and People domain crates.
- Provider registration remains inert without a purpose-bound grant, and the local adapter claims backup/single-writer modes only.
- Private assessments and sensitive Topic Pack values require explicit classification, purpose, and sharing grants.
- Sensitive and secret profile definitions require sealed values in the new domain contract.
