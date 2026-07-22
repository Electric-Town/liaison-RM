# Changelog

All notable changes to Liaison RM are recorded here. The format follows Keep a Changelog principles and the project uses semantic versioning after the first tagged release.

## Unreleased

### Added

- A preserved P03D Editorial Ledger design candidate, semantic-token registry, consultation record, amended P04 plan, and deterministic design-contract validator. These remain candidate inputs because P03 technical acceptance, exact-artifact observation, and a Continue decision do not yet exist.
- Recoverable canonical multi-target operations with staged and flushed targets, exact digest/revision preconditions, a durable commit decision, per-target progress, roll-forward recovery, external-edit refusal, projection-stale marking, and fault-injection coverage.
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
- Preserved candidate-source history for semantic Person validation, duplicate-identity Health findings, safe workspace-path rejection, actionable human Health output, attempted desktop create/open actions, keyboard form submission, hidden-state rendering, and automated base/dark text-contrast gates. The packaged `49ee419` shell omits the required workspace and Person controls, so these remain source and prototype provenance rather than current user-visible desktop behavior.
- KCS-0010 and executable evidence for keeping inbound adapters on the same application workflow.
- Candidate B0 Events design-contract evidence covering the thirteen-step journey storyboard, all five stage state contracts with lightweight wireframes, a one-to-one canonical-to-label presenter table, accounted-exception presentation rules, deferred operation-state mapping shape, and reflow/zoom/target rules, prepared as input for the future `T-B0-P03D` reconciliation without creating `DESIGN.md` or authorising P04.
- ADR 0015 and the exactly owned `T-B0-P03-OBS`/`LRM-PK-010` checkpoint for D1-B: technical P03 acceptance first binds qualified-code, merge-result, attestation, qualification-receipt, and executable identities; OBS then observes that exact artifact with synthetic or redacted workplace scenarios and records a distinct Continue, Change, or Stop decision before P03D.
- ADR 0016 separating general and third-party post-A0 migration safety under `LRM-WS-007`/`T-R5-005`/`FG-R5-005` from the narrow B0 OKF People normalization under `LRM-WS-017`/`UAT-066`.
- Explicit P04/P11/A0 design ownership: P04 owns the semantic token/component foundation and installed Workspace/People/Health built-in-theme/recovery UAT-073; P11 owns the full desktop and every-built-in Details-to-Brief UAT-062; A0 relationship work owns UAT-022.
- Identifier-exact cross-release and task forward-evidence registries with default-deny checks: every intentional repository-release mismatch and non-blocking later-milestone task input is bound to its precise task, artifact, owner, milestones, authority, and rationale, while blocking feature-gate evidence is forbidden from pointing to a later or unreachable milestone. Generated JSON and Markdown expose the permitted task edges.
- An exact R5 Google Drive backup slice (`T-R5-010`, `FG-R5-007`, and `UAT-074`) that depends on the R4 provider-neutral foundation without moving Google Drive into R4 or treating upload success as restore or synchronisation evidence.
- A separate deferred `PILOT` milestone for real workplace data, outside every synthetic B0, A0, and provider dependency path; its privacy guardrail remains mandatory if a pilot is attempted without making pilot completion a product-acceptance prerequisite.
- Domain-separated D1-B evidence identities and an exact P05-to-P04 dependency, preventing a collapsed receipt tuple or accepted-P03/pending-observation state from bypassing OBS, P03D, or the typed desktop boundary.

- Accepted working-state decisions for one application composition root, Workspace Session authority, recoverable multi-target commits, workspace key hierarchy and local policy, disposable Directory projection, structurally least-disclosure event readiness, checkpoint/recovery separation, and B0-before-A0 delivery.
- A repository-hosted working-state delivery contract with current implementation truth, B0/A0 acceptance boundaries, active-branch disposition, and evidence-safe claim language.
- An evidence-qualified RICE model for B0/A0 work packages, a KCS article explaining dependency and safety overrides, and mandatory PR fields for priority assumptions and decision rationale.
- A bounded review of maintainer-supplied machine-signal and localization guidance, adopting observable editorial checks while rejecting detector evasion and unsupported authorship classification.
- Exact, deterministic ownership for every requirement, UAT case, feature gate, and implementation task, with milestone, status, evidence owner, zero-orphan validation, and generated human- and machine-readable reports.
- A post-technical-acceptance D1-B observation gate; only a same-artifact Continue receipt makes P03D eligible to run design consultation, create canonical design authority, and complete plan design review before P04 begins.
- Explicit B0 event-readiness contracts for ordered outcomes, attendee corrections and denominator reconciliation, real-pilot privacy approval, one trusted owner, and byte-identical least-disclosure recipient briefs.
- KCS guidance for changing product contracts without creating traceability orphans.
- ADR 0013 and KCS-0009 pinning the immutable OKF v0.1 Draft People authoring source, Liaison domain-extension authority, strict-write/tolerant-read behavior, sealed-plaintext denial, and required recoverable B0 People normalization.
- Atomic P05-OKF and P09-OKF ownership under `FG-B0-001`; future P06 acceptance remains owned by `FG-R1-003`, with sensitive work removed from G3 P05 and retained exclusively under `FG-B0-002`.
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
- Historical Tauri alpha source and prototype provenance for workspace creation, People capture, validation, reflow, and macOS review-bundle workflows. The reviewed packaged shell does not expose the required workspace or Person controls, rendered acceptance fails, and no universal macOS artifact is qualified.
- `PROJECT_CONTEXT.md` as a complete, repository-hosted product and engineering handoff for human and automated contributors.
- Agent entry points for GitHub Copilot, Claude, and Gemini that defer to the normative repository contract.
- KCS-0005 covering current-context discovery, source hierarchy, and agent handoff.
- A version-controlled public About-description and repository-topic recommendation.
- A project-context workflow that validates repository links/content policy and machine-readable product specifications together.
- Versioned locale-catalog architecture with an `en-IE` source catalogue, `en-XA` expansion pseudolocale, draft Irish, Japanese, and Brazilian Portuguese fixtures, Unicode and placeholder validation, human-review gates, and release-evidence guidance.
- Windows desktop packaging: an NSIS installer target (per-user install, embedded WebView2 bootstrapper) and a `windows-2022` CI workflow that checks, lints, tests, builds, and checksums the desktop bundle.
- A static public project site with semantic metadata, local assets, a custom 404 page, a social card, responsive layouts, and a GitHub Pages deployment workflow.
- A public-site validator covering metadata, local asset resolution, accessibility basics, structured data, language consistency, discovery files, and copy standards.
- A real workplace-data pilot governance record set under `docs/pilot/` — data controller and accountable operators, lawful purpose and legal basis, special-category condition, DPIA decision, participant notice, retention and rights plan, incident response plan, and an independent-review record — with a deferred post-B0 lifecycle and an explicit real-data-denied boundary, plus KCS-0011 describing the authorisation path.
- Structured dietary source facts in the People context: the eight distinguished dietary kinds, four orthogonal availability/freshness/conflict/disclosure axes validated across every combination, dated verified-none records so an absent field can never read as no restriction, legacy coverage migration that preserves the original value and provenance, a constrained operational instruction separated from the stricter-classified detailed note, and an authorised operational view for Events from which the note is structurally absent.
- An Organisations and Groups bounded context adapted from the preserved organisations branch: organisations, locations, and groups as stable named records, effective-dated memberships carrying typed role, department, cost centre, location, primary flag, required provenance source, and record date, plus as-of snapshot queries so a department move creates a new membership and historical reports keep the membership that applied at the time.
- A preserved Editorial Ledger candidate proposes a paper-canvas palette in light and dark with measured contrast arithmetic (the previously rejected dark highlight pairing measures 8.12:1), locally bundled OFL-licensed Atkinson Hyperlegible Next, Source Serif 4, and IBM Plex Mono subsets with recorded hashes, a dotted note-paper canvas, one hard-offset primary work surface per page, provenance typography, and 48-pixel primary controls. These static source choices are non-accepted design provenance until the P03, D1-B observation, P03D, installed-P04, accessibility, and no-egress gates pass.
- An Events bounded-context domain candidate: event and attendee lifecycle with superseding corrections and an exact active deduplicated denominator, orthogonal dietary facts, an ordered versioned fail-closed readiness decision table, and least-disclosure brief concepts. P10 and P11 remain blocked; the current application projection and static UI are not accepted B0 behavior.

- Parked A0 Profiles candidate work for typed custom-field values and reserved canonical namespaces. It does not advance the B0 sequence and remains subject to A0 schema and migration review.

- Parked A0 Customisation candidate work for classified settings, adversarial bundle review, conflict choices, and revisioned rollback. Its bundle format remains provisional pending its owning milestone.

### Changed

- PR #65 established execution baseline `3499a6e9278fc72d2498a9978df59f30d03722e6` without accepting P03. All seven ordinary-push workflows succeeded in runs `29899084738`, `29899084740`, `29899084741`, `29899084751`, `29899084753`, `29899084769`, and `29899084789`, including Windows; separately dispatched notarized run `29899498005` failed for missing Apple credentials and is not release evidence. P03 remains current and P03D/P04 remain blocked.
- Later history through `49ee419e30f2d71524dd6fa15badf1ec4b8d0e27` is preserved as candidate implementation and design provenance, not acceptance. Its exact-head Rust and Windows checks fail formatting; no distinct P03 qualification, merge-result, attestation, artifact, observation, or Continue receipt exists.
- The unsigned annotated `vB0` tag is preserved at its original object and target but is an unsupported historical shipment claim. It has no GitHub Release or bound installed-artifact evidence and does not advance B0.
- The `.planning` vB0 archive and P04 static review/UAT/verification files are superseded as active status sources. They now point back to machine authority instead of claiming complete requirements, WCAG conformance, installed qualification, or shipment.
- Generic migration safety `LRM-WS-007` and `migration-dry-run` moved from P03 to R5 `T-R5-005`/`FG-R5-005`; B0 P09-OKF remains exclusively the narrow pinned-OKF normalization under `LRM-WS-017`/`UAT-066`, and P06-REPAIR has no forward generic-migration evidence edge.
- Windows first-use registry creation now uses a cross-process initialisation
  lock so concurrent desktop or CLI launches cannot observe the directory
  between creation and canonical ACL verification.
- Traceability now records merged P00/P01/P02 completion and active P03/G1 execution, keeps OBS/P03D/P04/P05-P11/B0 blocked and PILOT deferred, closes the composite workspace gate only at downstream P09-OKF, proves gate and cross-milestone reachability, places P05's prerequisite domain contracts in G1, and reserves compiled-out Airgap evidence for UAT-024/FG-R2-005.
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

- Recoverable canonical mutations now avoid Windows handle failures: directory handles open with flush support and release before staged cleanup or uncommitted-operation discard; sequential mutations also retain an existing regular projection-stale marker.
- P03 validation workflows are read-only again, and the obsolete one-shot source-mutation workflow has been removed.
- The earlier static Events route guard is not acceptance evidence: `49ee419` exposes an Events-labelled destination through the `readiness` alias and therefore remains a source defect while `T-B0-P11` is blocked. Runtime removal and a semantic checker repair belong to the downstream source/tooling correction slice.
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
- Earlier desktop-alpha fixes addressed Tauri ownership, default-path linting, and deterministic Windows icon generation. The reviewed pre-reconciliation main at 2026-07-22, `49ee419`, fails Rust and Windows formatting checks and is not qualified or released. The C3 commit and any merge result are distinct identities requiring their own exact-head receipt; absent that receipt they do not advance acceptance.
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
