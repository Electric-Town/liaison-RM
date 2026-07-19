# Changelog

All notable changes to Liaison RM are recorded here. The format follows Keep a Changelog principles and the project uses semantic versioning after the first tagged release.

## Unreleased

### Added

- One `liaison-application` composition root shared by the CLI and desktop, with typed command results, structured recoverable errors, deterministic runtime ports, and identity-bound workspace sessions.
- Shared-fixture CLI and Tauri parity for workspace initialise/open/validate and Person create/list, including versioned envelopes, tolerant malformed-sibling reads, and initial Person revision 1.
- Semantic Person validation, duplicate-identity Health findings, safe workspace-path rejection, actionable human Health output, retryable desktop create/open actions, native-safe keyboard form submission, correct hidden-state rendering, and automated base/dark text-contrast gates.
- KCS-0010 and executable evidence for keeping inbound adapters on the same application workflow.

- Accepted working-state decisions for one application composition root, Workspace Session authority, recoverable multi-target commits, workspace key hierarchy and local policy, disposable Directory projection, structurally least-disclosure event readiness, checkpoint/recovery separation, and B0-before-A0 delivery.
- A repository-hosted working-state delivery contract with current implementation truth, B0/A0 acceptance boundaries, active-branch disposition, and evidence-safe claim language.
- An evidence-qualified RICE model for B0/A0 work packages, a KCS article explaining dependency and safety overrides, and mandatory PR fields for priority assumptions and decision rationale.
- A bounded review of maintainer-supplied machine-signal and localization guidance, adopting observable editorial checks while rejecting detector evasion and unsupported authorship classification.
- Exact, deterministic ownership for every requirement, UAT case, feature gate, and implementation task, with milestone, status, evidence owner, zero-orphan validation, and generated human- and machine-readable reports.
- A post-P03 design gate that runs design consultation to create `DESIGN.md`, then plan design review before P04 begins; G0 records the gate without pre-empting the design artifact.
- Explicit B0 event-readiness contracts for ordered outcomes, attendee corrections and denominator reconciliation, real-pilot privacy approval, one trusted owner, and byte-identical least-disclosure recipient briefs.
- KCS guidance for changing product contracts without creating traceability orphans.
- ADR 0013 and KCS-0009 pinning the immutable OKF v0.1 Draft People authoring source, Liaison domain-extension authority, strict-write/tolerant-read behavior, sealed-plaintext denial, and required recoverable B0 People normalization.
- Atomic P05-OKF, P06, and P09-OKF ownership under `FG-B0-001`, with sensitive work removed from G3 P05 and retained exclusively under `FG-B0-002`.
- A0 source-complete purpose-scoped profile, fact-state, reversible identity-review, and source/range timeline contracts plus later provider-operation, staged-enrichment, and privacy-bounded spatial contracts.
- Semantic specification checks and negative mutation coverage for the B0 migration exception, global-score/task-engine/auto-merge/direct-write/hidden-sync prohibitions, and canonical task ownership versus explicit evidence dependencies.

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
- A static public project site with semantic metadata, local assets, a custom 404 page, a social card, responsive layouts, and a GitHub Pages deployment workflow.
- A public-site validator covering metadata, local asset resolution, accessibility basics, structured data, language consistency, discovery files, and copy standards.

### Changed

- The repository README now serves as a detailed public product overview and routes contributors to canonical product, architecture, security, planning, and status sources.
- `AGENTS.md` now requires branch, exact-head, source-hierarchy, relationship-domain, provider, AI, accessibility, and handoff checks before completion claims.
- Reason-only review is the personal-workspace default; weighted Review Priority is explicitly queue ordering rather than relationship strength.
- Review queues preserve factual reasons and do not expose a relationship-strength score.
- The R2 roadmap now includes locale-catalog, expansion, Unicode, accessible-name, formatting, and named human-review evidence gates.
- The README and GitHub About contract now lead with the product outcome, state the pre-alpha boundary plainly, and route readers to current evidence, contribution guidance, and the public site.
- Public site, README, project context, roadmap, and agent guidance now put B0 before A0, distinguish the tested CLI from the broken installed desktop Person-create path, and use 400% reflow as the accessibility evidence target.
- Requirements and UAT task arrays now contain only atomically owned contracts; reused prerequisite or regression coverage is labelled `evidence_dependencies` rather than implying a second owner.

### Fixed

- Desktop alpha now compiles and lints cleanly across Linux, macOS, and Windows: Tauri command arguments acknowledge required ownership, the default workspace path uses `map_or_else`, and a deterministic Windows `icon.ico` resource is generated for `tauri-build`.
- Health findings now expose portable `/`-separated workspace-relative paths across macOS, Linux, and Windows instead of leaking host-specific path separators into the shared application contract.
- Default-workspace tests now compare the operating system's native Documents path composition instead of imposing Unix separators on Windows.

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
