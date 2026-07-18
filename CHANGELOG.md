# Changelog

All notable changes to Liaison RM are recorded here. The format follows Keep a Changelog principles and the project uses semantic versioning after the first tagged release.

## Unreleased

### Added

- A centralized hand-drawn desktop design system with paper texture, wobbly borders, hard offset shadows, handwriting-oriented local font stacks, reduced-motion behavior, and responsive accessibility checks.
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
- Local Tauri desktop alpha for workspace creation, People capture, validation, accessible reflow, and universal macOS review bundles.
- `PROJECT_CONTEXT.md` as a complete, repository-hosted product and engineering handoff for human and automated contributors.
- Agent entry points for GitHub Copilot, Claude, and Gemini that defer to the normative repository contract.
- KCS-0005 covering current-context discovery, source hierarchy, and agent handoff.
- A version-controlled public About-description and repository-topic recommendation.
- A project-context workflow that validates repository links/content policy and machine-readable product specifications together.
- Versioned locale-catalog architecture with an `en-IE` source catalogue, `en-XA` expansion pseudolocale, draft Irish, Japanese, and Brazilian Portuguese fixtures, Unicode and placeholder validation, human-review gates, and release-evidence guidance.
- Windows desktop packaging: an NSIS installer target (per-user install, embedded WebView2 bootstrapper) and a `windows-2022` CI workflow that checks, lints, tests, builds, and checksums the desktop bundle.

### Changed

- The repository README now serves as a detailed public product overview and routes contributors to canonical product, architecture, security, planning, and status sources.
- `AGENTS.md` now requires branch, exact-head, source-hierarchy, relationship-domain, provider, AI, accessibility, and handoff checks before completion claims.
- Reason-only review is the personal-workspace default; weighted Review Priority is explicitly queue ordering rather than relationship strength.
- Review queues preserve factual reasons and do not expose a relationship-strength score.
- The R2 roadmap now includes locale-catalog, expansion, Unicode, accessible-name, formatting, and named human-review evidence gates.

### Fixed

- Desktop alpha now compiles and lints cleanly across Linux, macOS, and Windows: Tauri command arguments acknowledge required ownership, the default workspace path uses `map_or_else`, and a deterministic Windows `icon.ico` resource is generated for `tauri-build`.

### Security

- Documented the prohibition on undeclared network requests, hidden telemetry, secret material in canonical files, and provider or plugin access without an explicit grant.
- Defined separate Airgap and Connected-local build profiles and least-disclosure handling for sensitive relationship and workplace data.
- Kept network, provider, SQL, Tauri, and secret-storage dependencies out of the initial Workspace and People domain crates.
- Provider registration remains inert without a purpose-bound grant, and the local adapter claims backup/single-writer modes only.
- Private assessments and sensitive Topic Pack values require explicit classification, purpose, and sharing grants.
- Sensitive and secret profile definitions require sealed values in the new domain contract.
- Desktop alpha compiles no network client, uses a local-only CSP, and separates ad-hoc review artifacts from notarized release artifacts.
- Agent handoff guidance prohibits private prompt history, personal data, credentials, and unsupported implementation or compliance claims in repository context.
- Localization guidance prohibits private workspace data from entering public semantic metadata and does not treat machine-assisted text as human-approved language.
