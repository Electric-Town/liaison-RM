# Changelog

All notable changes to Liaison RM are recorded here. The format follows Keep a Changelog principles and the project uses semantic versioning after the first tagged release.

## Unreleased

### Added

- A write-authoritative, `Arc`-owned Workspace Session that retains one
  capability root, workspace identity/schema, path-free repositories, an
  operating-system writer lock, quiescence, and explicit unavailable recovery,
  key, and projection states.
- A local Workspace Session adapter with no-follow capability traversal,
  typed writer contention, diagnostic-only bounded sidecars, process-exit lock
  release, retained-root identity checks, and lock-free read-only Health.
- Per-user `WorkspaceId` writer exclusion for copied or file-synchronised
  workspaces, with zero-data registry entries, safe first-use creation, typed
  identity contention, hostile-registry checks, post-lock manifest validation,
  Windows owner/DACL verification, and focused native Windows workflow tests.
- Safe desktop workspace switching that closes the previous session before
  accepting its replacement and best-effort closes the replacement if the
  previous session cannot close.
- A published version-one Workspace manifest schema with explicit
  `enabled_modules`, strict new-writer fixtures, lossless legacy reads for the
  pre-field P01 manifest, and a pinned schema-validation workflow.
- An opaque People repository borrowed from a live Workspace work guard, plus
  compiler-boundary tests proving an unguarded Markdown vault cannot implement
  or retain the production Person repository.
- One-shot read-only Health from the selected folder even when no writer
  session opens, with the inspected folder shown separately from the active
  workspace.
- One `liaison-application` composition root shared by the CLI and desktop, with typed command results, structured recoverable errors, deterministic runtime ports, and typed workspace session identifiers.
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

- A Customisation bounded-context domain core preparing the A0 G2c settings work: classified settings entries, an adversarial bundle review that lists rejected record-identifier, secret-like, and absolute-path content instead of silently dropping it, unknown-safe-key preservation, diff preview with mandatory conflict choices, and pure apply with exact revisioned rollback — with the bundle format explicitly provisional pending its owning milestone.

### Changed

- Windows first-use registry creation now uses a cross-process initialisation
  lock so concurrent desktop or CLI launches cannot observe the directory
  between creation and canonical ACL verification.
- Traceability now records merged P00/P01 completion and active P02/G1 execution, closes the composite workspace gate only at downstream P09-OKF, proves gate and cross-milestone reachability, places P05's prerequisite domain contracts in G1, and reserves compiled-out Airgap evidence for UAT-024/FG-R2-005.
- Mandatory build-order guidance and semantic specification checks now enforce P06-REPAIR between tolerant Directory reads and OKF normalization, together with the corrected P02, P03, repair, B0 acceptance, and A0 settings ownership edges.
- Corrected machine-contract ownership so P02 owns session authority rather than installed/offline round-trip acceptance, B0 acceptance owns workspace creation and `UAT-001`, final A0 acceptance closes the full round-trip gate, settings-only export/import and `UAT-050` belong to A0 configuration, and post-P06 guided repair owns `UAT-040` through backup-first recoverable operations.
- The repository README now serves as a detailed public product overview and routes contributors to canonical product, architecture, security, planning, and status sources.
- `AGENTS.md` now requires branch, exact-head, source-hierarchy, relationship-domain, provider, AI, accessibility, and handoff checks before completion claims.
- Reason-only review is the personal-workspace default; weighted Review Priority is explicitly queue ordering rather than relationship strength.
- Review queues preserve factual reasons and do not expose a relationship-strength score.
- The R2 roadmap now includes locale-catalog, expansion, Unicode, accessible-name, formatting, and named human-review evidence gates.
- The README and GitHub About contract now lead with the product outcome, state the pre-alpha boundary plainly, and route readers to current evidence, contribution guidance, and the public site.
- Public site, README, project context, roadmap, and agent guidance now put B0 before A0, distinguish the tested CLI from the broken installed desktop Person-create path, and use 400% reflow as the accessibility evidence target.
- Requirements and UAT task arrays now contain only atomically owned contracts; reused prerequisite or regression coverage is labelled `evidence_dependencies` rather than implying a second owner.

### Fixed

- Desktop native operations now share one synchronous busy boundary with
  generation/session checks, stale-result rejection, and explicit cleanup of
  superseded workspace sessions, preventing overlapping switches or Person
  results from leaking authority or state across workspaces. If both the old
  session close and replacement cleanup fail, the interface retains explicit
  restart recovery and disables further native operations until exit.
- Capability-bound manifest and Person reads now preflight and post-validate
  regular files around a nonblocking no-follow open, so FIFOs and other special
  files cannot wedge one-shot Health or normal workspace access.
- Windows identity-registry security inspection now queries file and directory
  handles as Win32 `SE_FILE_OBJECT` values instead of using the dependency's
  generic unknown-object helper, allowing owner/DACL checks to run natively.
- Windows now resolves `FOLDERID_LocalAppData` with an explicit current-process
  user token, without making Profile or mutable process environment a second
  authority prerequisite, and treats that retained, no-reparse Known Folder as
  traversal infrastructure rather than a Liaison-owned private object. Newly
  created Liaison registry directories and lock files are normalised to the
  token user with one protected user/System/Administrators ACL; any existing
  noncanonical registry or lock still fails closed. Initialisation and Windows
  API errors retain their typed recovery category without exposing a host path.
- Desktop asset verification now compares rendered PNG and ICNS content across
  platforms while retaining byte-exact checks for the uncompressed Windows ICO,
  avoiding false drift failures from host-specific compression libraries.
- Downloadable Mac and Windows review artifacts now carry checksum manifests
  with artifact-relative paths and verify those manifests before upload.
- Desktop alpha now compiles and lints cleanly across Linux, macOS, and Windows: Tauri command arguments acknowledge required ownership, the default workspace path uses `map_or_else`, and a deterministic Windows `icon.ico` resource is generated for `tauri-build`.
- Health findings now expose portable `/`-separated workspace-relative paths across macOS, Linux, and Windows instead of leaking host-specific path separators into the shared application contract.
- Default-workspace tests now compare the operating system's native Documents path composition instead of imposing Unix separators on Windows.
- Workspace identity writer authority no longer forks when ordinary
  same-account processes use different `HOME` or XDG values. Production
  launch-order and process-exit tests now exercise the canonical locator, and
  inaccessible or Flatpak-isolated authority fails closed without fallback.

### Security

- Documented the prohibition on undeclared network requests, hidden telemetry, secret material in canonical files, and provider or plugin access without an explicit grant.
- Defined separate Airgap and Connected-local build profiles and least-disclosure handling for sensitive relationship and workplace data.
- Kept network, provider, SQL, Tauri, and secret-storage dependencies out of the initial Workspace and People domain crates.
- Pinned and documented target-specific `uzers` 0.12.2, `winsafe` 0.0.28,
  `rustix` 1.1.4, and `windows-permissions` 0.2.4 for environment-independent,
  fail-closed per-user identity-registry resolution and ownership checks;
  registry entries contain no path, PID, diagnostic, or relationship data.
- Provider registration remains inert without a purpose-bound grant, and the local adapter claims backup/single-writer modes only.
- Private assessments and sensitive Topic Pack values require explicit classification, purpose, and sharing grants.
- Sensitive and secret profile definitions require sealed values in the new domain contract.
- Desktop alpha compiles no network client, uses a local-only CSP, and separates ad-hoc review artifacts from notarized release artifacts.
- Agent handoff guidance prohibits private prompt history, personal data, credentials, and unsupported implementation or compliance claims in repository context.
- Localization guidance prohibits private workspace data from entering public semantic metadata and does not treat machine-assisted text as human-approved language.
