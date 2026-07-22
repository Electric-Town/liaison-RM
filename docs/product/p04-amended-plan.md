# T-B0-P04 amended implementation plan

Status: reviewed and expanded for developer experience; P03 source/build changes are on remote `main`, but P04 remains blocked until exact P03 acceptance/attestation, the selected D1-B exact-artifact observation returns `continue`, P03D is accepted with its design/traceability amendments, and an isolated exact-head P04 execution branch is created
Owning contexts: Experience and Application
Primary gate: corrected P04 foundation gate derived from `FG-R2-001`; the current unsplit gate is not closable by P04
Design authority: `DESIGN.md` 1.0.0
Developer-experience mode: Expansion

## Outcome and claim boundary

Replace the disposable vanilla webview with a typed React/TypeScript client while preserving every working Workspace, People, Health, authority, and recovery behaviour. Establish the semantic design system, Application-to-Experience contract, and contributor evidence ratchet that P05-P11 can extend without route-specific styling, duplicate business rules, or private maintainer knowledge.

This document is an implementation plan, not implementation evidence. P04 does not start until the authoritative P03 head is accepted and attested, the user-selected D1-B observation has exercised that exact artifact and recorded a distinct `continue` decision, and the resulting P03D head is merged and verified. A passing Quick Tour proves only its declared synthetic checks; it does not prove P04 verification, installed qualification, an Airgap artifact, or a supported release.

At the refreshed 2026-07-22 review snapshot, remote `main` was `3499a6e` with P03 source/build workflows green; that fact is not the separate P03 acceptance/attestation receipt. P03D PR #58 was clean but unmerged at `fc2ae2c`. Local `main` remained at `04d8532`, ahead of `origin/main` by 58 commits, behind by 8 commits, and dirty in the desktop UI plus this plan. Those observations are diagnostic only. Draft PRs #62, #63, and #64 and the local Events/customisation/UI commits remain non-authoritative evidence. Do not merge, cherry-pick, or silently rebase that stack into the P04 implementation line.

P03D closure creates a clean isolated execution worktree from its accepted merge head, records the base/head/dirty-state manifest, and preserves the current worktree and its three user-edited legacy UI files byte-for-byte. Selective reuse requires an explicit source-commit inventory and ownership review; no wholesale transplant is allowed.

## Domain-driven design contract

P04 remains domain-driven by construction:

- each business bounded context owns its model, vocabulary, invariants, typed failures, fixtures, and tests;
- Application owns use-case orchestration, the public application-interface schema, capability and use-case identity, semantic error descriptors, correlation, and cross-context coordination; it does not own desktop route IDs;
- Experience owns navigation, interaction state, localized presentation, accessible recovery affordances, and exhaustive rendering of Application capabilities;
- Tauri, CLI, React, test harnesses, and contributor tooling are inbound adapters and may not recreate repositories, calculate domain state, or duplicate business rules;
- `xtask` is a thin contributor-tooling orchestrator. It invokes context-owned checks or public Application use cases and never becomes a business service;
- persistence records, generated TypeScript, transport DTOs, receipts, diagnostic bundles, and contributor catalogs are not domain entities;
- public error descriptors and private diagnostic records are separate contracts; an arbitrary adapter or Application details map never crosses into Experience;
- cross-context use is through explicit Application interfaces, events, or anti-corruption layers; no UI feature imports another context's internals;
- provider identity, arbitrary metadata, relationship allocation, relationship value, relationship strength, and workplace relationship ranking remain structurally absent.

Before frontend implementation, P03D establishes Experience formally with a focused decision, an updated context map and ubiquitous language, `apps/desktop/src/experience/README.md`, explicit Application/Experience interfaces, and architecture tests. Experience is a presentation bounded context, not a business-domain Rust crate. Do not create `contexts/experience/` unless it becomes a real workspace crate with both `README.md` and `Cargo.toml` under the repository architecture policy. Experience owns route IDs, navigation, operation disclosure, focus restoration, and draft navigation state. It does not own Workspace, People, Event, recovery, readiness, authority, or capability meaning.

## Primary developer persona

| Attribute | P04 persona |
|---|---|
| Role | Cross-stack open-source contributor implementing one B0 vertical slice |
| Goal | Make one DDD-correct Rust/Application/Tauri/React change and produce reviewable evidence |
| Starting knowledge | Comfortable with Rust or TypeScript, Git, tests, and CI; unfamiliar with Liaison's delivery history and vocabulary |
| Constraints | macOS or Windows, intermittent attention, no private maintainer context, no real personal data, no hosted account |
| Trust need | Know the authoritative branch, allowed task, owning context, exact command, result scope, and recovery path |
| Success | Prepared/offline Quick green within two minutes and a real native typed-status window within ten minutes after declared prerequisites; clean connected bootstrap is measured separately |

### Developer empathy narrative

> I want to contribute one coherent B0 slice, not reconstruct the project from old branches and thirty thousand words of contracts. First tell me whether this checkout is authoritative and which bounded context owns the rule. Give me one safe command that checks my machine, creates synthetic state, exercises a real Application use case through Tauri and React, and tells me exactly what passed. If it fails, show the semantic code, cause, recovery action, and a private local diagnostic path. If it passes, leave me a receipt and one next command so I can make a change without overstating what I proved.

## Competitive DX benchmark

The target combines the strongest relevant patterns rather than copying another product's architecture.

| Reference | Standard applied to P04 | Reviewed-plan target |
|---|---|---:|
| Vercel-style first run | One command produces a visible real result with continuous progress | 10/10 |
| GitHub CLI-style interface | Guessable noun/verb grammar, terminal discovery, human and machine output | 10/10 |
| Rust/Elm-style diagnostics | Stable code, exact problem, safe cause, recovery, and local learning path | 10/10 |
| Stripe-style contract discipline | Versioned structured interfaces and exhaustive compatibility rules | 10/10 |
| Liaison local-authoritative boundary | No account, hidden telemetry, undeclared network, or real-data fixture | 10/10 |

The competitive tier is **Champion by plan**, with three non-interchangeable start states:

1. **Prepared/offline Quick:** repository dependencies and toolchains are present and an exact-input `cargo xtask build --phase p04` receipt exists for the current worktree digest; terminal green is budgeted at no more than two minutes.
2. **Clean connected bootstrap:** toolchains and lockfile dependencies may be downloaded; measure and report it separately without a time claim until repeated platform evidence supports one.
3. **Prepared native:** the same exact-input build receipt and native prerequisites are present; the real native window is budgeted at no more than ten minutes.

Mandatory contributor reading is a contribution-readiness gate before selecting or editing a task, not hidden work inside the Quick timing claim. These targets remain plans until exact-head receipts from the claimed environment and surface prove them.

## Magical moment

The first meaningful result is a real `app_status` use case flowing through the complete supported boundary:

```text
Application-owned schema
  -> generated checked-in TypeScript
  -> Tauri command adapter
  -> Experience capability and route mapping
  -> React Overview status
```

`cargo xtask quick --phase p04` is the delivery vehicle. It implicitly runs the non-mutating doctor, validates the exact-input build receipt, uses owned synthetic scenarios, reaches terminal green within the prepared/offline Quick budget, optionally continues to a native window within the prepared-native budget, and records the exact evidence scope. A missing or stale build receipt fails with `NEXT cargo xtask build --phase p04`; Quick does not hide compilation inside its timing claim.

The focused native assertion uses WebdriverIO's maintained Tauri service with the embedded provider. It launches the real compiled review binary, invokes `app_status`, observes the rendered accessible status, submits the synthetic Workspace/People request canary, and asserts the localized typed failure in the native webview. Browser-mode tests remain fast supporting tests; a mock is never accepted for this real-bridge assertion. The optional WebDriver plugins are review-build/test dependencies only and are proved absent from production bundles.

`app_status` proves the read-only result path but cannot prove the risky request/session/mutation/failure seams by itself. Quick therefore also runs a synthetic request-bearing canary through the existing Workspace Session and People Application use cases: create and open an owned workspace, create one synthetic person, and provoke one typed validation or authority failure. Application owns the request and result schema; generation or parity checks cover adapter request shapes as well as result shapes. Tauri and React only translate and render the public interface.

Expected progress shape:

```text
P04 QUICK
PASS  authority and environment contract
PASS  generated Application interface parity
PASS  synthetic app-status scenario
PASS  synthetic request/session/mutation/failure canary
READY quick checks in 00:00-02:00; higher gates omitted: verify, qualify
OPEN  native typed-status window by 00:10:00
RECEIPT <local ignored path>
NEXT  cargo xtask dev --phase p04 --scenario app-status
```

## Developer journey

| Stage | Contributor does | Plan resolution | Claim |
|---|---|---|---|
| Discover | Runs `cargo xtask status --phase p04` or reads the P04 contributor hub | Offline local canonical task, gate, owner, contradiction, and evidence status; remote CI only with `--remote` | Discovery only |
| Install | Runs `cargo xtask doctor` directly or through Quick | Cross-platform non-mutating prerequisite diagnosis with exact recovery | Environment readiness only |
| Hello World | Runs `cargo xtask quick --phase p04` | Prepared/offline synthetic real-bridge `app_status` plus request/session/mutation/failure canary, progress, cleanup, receipt | Quick only |
| Real usage | Runs `cargo xtask next --phase p04`, then `cargo xtask dev --phase p04 --scenario <id>` | One dependency-complete slice and an owned scenario | Development evidence only |
| Debug | Expands the safe error, then uses `explain-error` or `reproduce-error` | Offline semantic help and previewed diagnostics | Diagnostic evidence only |
| Upgrade | Generates a dependency proposal and runs `cargo xtask rehearse-upgrade --phase p04` | Classified interface, dependency, fixture, and rollback diff | Upgrade rehearsal only |
| Review | Runs `cargo xtask verify --phase p04` and `cargo xtask prepare-pr --phase p04` | Full local matrix and evidence-grounded PR draft | Verification only |
| Qualify | Runs `cargo xtask qualify --phase p04` | Exact artifact, OS, accessibility, and installed evidence | P04 qualification only |

## First-time contributor confusion report

Every observed confusion point is assigned below; none is accepted as tribal knowledge.

| Time | Observed confusion | Resolution |
|---|---|---|
| T+0:45 | CLI and desktop have separate manual paths and no authoritative front door | `xtask`, Quick Tour, contributor hub |
| T+1:30 | Documented native artifact path differs from the root build output | environment contract discovers and reports paths from build metadata |
| T+2:15 | Tauri and future Node prerequisites are hidden or unpinned | doctor plus exact environment manifest |
| T+3:00 | Mandatory first reads exceed 29,000 words before context-specific material | retain the normative gate honestly; provide a generated ownership index and progressive post-read contributor hub without claiming to bypass it |
| T+6:30 | P04's accepted dependency does not match current PR lineage | fail-closed authority check and preservation-first restack |
| T+8:00 | Experience is named as an owner but has no bounded-context README | formal Experience context prerequisite |
| T+10:00 | No executable DDD reference shows schema-to-native-to-React ownership | shipping `app_status` golden reference slice and KCS guidance |

## What already exists and must be reused

- `rust-toolchain.toml` pins Rust, rustfmt, and clippy.
- The `liaison` CLI already uses noun/verb commands, human/JSON output, stable exit classes, synthetic workspace examples, and the shared Application composition root.
- `ApplicationError` already carries a contract version, stable code, English message/recovery text, an arbitrary private details map, and a correlation ID. P04 must separate the safe public descriptor before generation; the existing details map is not safe Experience input.
- Workspace Session already owns writer authority and preserves read-only Health during conflicts.
- The current desktop shell already exercises native operation serialization, stale-result rejection, workspace switching, People reachability, Health, and local-only CSP.
- `DESIGN.md`, semantic-token validation, UX standards, localization standards, traceability generation, KCS practice, the changelog, and the pull-request template provide reusable governance.
- Existing P03 fault fixtures and exact-head evidence conventions remain the source for operation/recovery presentation.

Reuse does not mean copying current prose or DTOs into new layers. Generate or adapt at the owning boundary and keep legacy behaviour under parity tests until removal.

## Repository shape

```text
tooling/xtask/
├── Cargo.toml
├── src/
└── tests/

.cargo/
└── config.toml                 # cargo xtask alias only

tooling/phases/p04/
├── environment.v1.json
├── commands.v1.json
├── scenarios.v1.json
└── dx-scorecard.v1.json

apps/desktop/
├── package.json
├── package-lock.json
├── vite.config.ts
├── tsconfig.json
├── src/
│   ├── app/
│   │   ├── AppShell.tsx
│   │   ├── routes.tsx
│   │   ├── operation-state.ts
│   │   └── generated/
│   ├── experience/
│   │   └── README.md
│   ├── design-system/
│   ├── features/
│   ├── i18n/
│   └── test/
├── ui-legacy/                 # retained only until exact-head parity passes
└── src-tauri/

docs/contributing/p04/
├── README.md
├── golden-slice.md
├── errors.md                  # generated reference plus owned prose
└── upgrades.md

docs/evidence/dx/
└── README.md
```

`apps/desktop/` becomes the permanent frontend package root in P04.1. The new build output remains inactive until shell parity passes; `ui-legacy` remains selectable only through the bounded review fallback described below.

`tooling/xtask` is a real workspace member with its own manifest. `.cargo/config.toml` exposes the `cargo xtask` alias and contains no domain configuration. The public command namespace is permanent and phase-neutral; `--phase p04` selects phase metadata so later phases do not clone a disposable runner. Architecture checks prove that the runner invokes owner-provided commands and public Application interfaces rather than importing business-context internals.

## Contributor command contract

The initial public grammar is:

```text
cargo xtask doctor
cargo xtask quick --phase p04 [--no-cache]
cargo xtask dev --phase p04 --scenario <id>
cargo xtask build --phase p04
cargo xtask verify --phase p04
cargo xtask qualify --phase p04
cargo xtask status --phase p04 [--remote]
cargo xtask next --phase p04 [--task <id>]
cargo xtask where <concept>
cargo xtask explain-error <code>
cargo xtask reproduce-error <code>
cargo xtask rehearse-upgrade --phase p04
cargo xtask prepare-pr --phase p04
cargo xtask dx-report --phase p04
```

`commands.v1.json` is contributor-tooling metadata, not a domain model. It generates concise terminal help, checked-in help snapshots, README command tables, JSON metadata, runnable examples, and supported shell completions. CI rejects drift.

Every command renders human and `--output json` views from one versioned `ContributorCommandResult`. It records command version, requested evidence level, exact SHA and dirty state, environment and scenario versions, status, checks proved, checks omitted, stable tooling failure code, elapsed phases, receipt path, and next actions. Exit success means the requested command contract passed; it never implies a higher level.

Evidence levels are deliberately distinct:

- **Quick:** under-two-minute bounded checks only from the prepared/offline start state, synthetic state, explicit omissions;
- **Verify:** the full local source matrix for the candidate checkout;
- **Qualify:** installed/platform evidence bound to an exact artifact and head.

The runner auto-detects terminal presentation, remains non-interactive in CI, and offers explicit `--output json`, `--no-cache`, scenario selection, and preservation of labelled synthetic state. `status` is offline by default. Its explicit `--remote` mode may use `gh` to fetch remote CI and PR metadata, reports source and observation time, and fails as remote discovery rather than silently falling back. It never accepts an arbitrary canonical workspace for fault injection or verification.

## Environment, isolation, and receipts

`environment.v1.json` declares the exact Rust, Node, package-manager, Tauri CLI, supported OS/architecture, native prerequisite probes, lockfiles, artifact discovery rules, and reference timing assumptions. Doctor, CI, generated docs, and receipts consume the same contract. Workflow-local pins are checked against it.

Command catalog entries contain typed program identifiers and argument arrays, never shell source. `xtask` resolves a reviewed allowlist, fixes the repository-root working directory, passes an explicit environment allowlist, spawns with `std::process::Command` without `sh -c`/`cmd /C`, enforces per-phase deadlines, and terminates the owned process group on interruption. User-controlled task, scenario, error, or path values are validated identifiers and are never interpolated into executable text.

Doctor is read-only and installs nothing. Prepared/offline Quick performs no network access and does not install dependencies. Clean connected bootstrap is a separate command phase that may populate build caches and install only lockfile-pinned repository-local packages with lifecycle scripts disabled; it has separate timings and receipts. Neither phase installs system packages, invokes `sudo`, configures a provider, or creates an account.

Every contributor run receives an owned synthetic workspace, explicit task-specific state root, unique ports, bounded child processes, and deterministic interruption cleanup. `--keep-scenario` preserves only a labelled synthetic workspace and prints its expiry and exact removal command. Shared caches may contain dependencies and build outputs, never canonical records, secrets, diagnostics, or private paths.

Quick automatically writes human and JSON proof receipts to a declared gitignored local evidence directory. Receipts bind exact SHA, dirty state, platform, tool versions, scenario and contract versions, phase timings, results, omissions, cache reuse, and known base blockers. They contain only synthetic and contributor-environment data, print their path and deletion command, and are not uploaded.

Content-addressed phase receipts key reuse on exact SHA, the complete relevant worktree content digest, environment contract, lockfiles, generated-interface digest, scenario version, and command version. The digest obtains NUL-delimited tracked and non-ignored untracked files from Git under every declared input root, then streams a normalized path, mode, symlink target, and file contents through SHA-256. It never follows symlinks, never loads whole files into memory, rejects special files/submodules/path escapes, and fails closed if input completeness cannot be proved. Ignored build outputs and evidence roots are explicitly classified. Reuse is always visible, and `--no-cache` forces a fresh check. Qualify never reuses verification results. Cache removal requires a preview, an exact target list, and a confirmation token.

Legacy and React assets use explicit Tauri config overlays. The accepted legacy configuration remains the default until parity; `cargo xtask build --phase p04` passes a checked-in React overlay through Tauri's `--config` merge, with a fixed Vite `beforeBuildCommand` and `frontendDist`. P04.7 removes the legacy overlay only after parity and installed qualification; no environment-variable-only switch can change a release artifact silently.

## Application interface and generated TypeScript

Application is the single source for the public use-case interface. P04 pins and reviews the stable `ts-rs` generator as an optional Application build feature, derives checked-in TypeScript request, result, capability, public-error, and recovery-action types from the public Rust DTOs, and verifies deterministic regeneration in CI. A single Tauri `desktop_command_contract!` declaration generates the invoke-handler registry, command metadata, outer `{ request: ... }` envelope, command names, and typed TypeScript invoke wrappers. Adapter DTOs may add transport-only wrappers, but they cannot restate Application fields.

Every exposed boundary must **generate or compile-check** its request, result, command name, envelope, casing, and public-error shape from that owning declaration; a hand-maintained duplicate is not accepted as parity evidence.

Golden JSON fixtures compile and decode in both directions across Application, Tauri, and TypeScript. The bridge suite asserts exact command name, outer argument name, field casing, request schema, result schema, error schema, and interface version. Unsupported directional Serde attributes fail generation; warnings are not suppressed. `ts-rs`, WebdriverIO, the Tauri test plugins, React, Vite, TypeScript, and every transitive dependency require the repository's licence, maintenance, supply-chain, platform, Airgap, install-script, and binary-size review before admission.

The application-interface version follows an explicit lockstep policy:

- one current interface version is supported inside a built desktop artifact;
- additive changes retain the version only when compatibility tests prove old fixtures still decode and semantics do not change;
- breaking changes bump the version and regenerate Rust fixtures, TypeScript, help, and parity tests atomically;
- runtime mismatch fails closed with expected and observed versions plus a localized recovery action;
- the previous accepted fixture remains for diagnostics and upgrade rehearsal, not a rolling runtime support promise;
- future external API, MCP, provider, or plugin compatibility is separately gated and cannot expand P04.

Application publishes capability semantics in three explicit layers:

- `CapabilityDefinition` says whether the installed artifact structurally supports a use case and names only its stable Application use-case identity;
- `CapabilityAvailability` reports current availability, permission, authority, dependency, and blocking reason without redefining the capability;
- transition metadata states which recovery action can make a temporarily unavailable capability available.

Experience owns the explicit mapping from structurally supported Application use-case identities to stable desktop route IDs and components. Structurally absent capabilities are omitted. Supported but currently unavailable, denied, or blocked capabilities remain discoverable with accessible state, reason, and recovery; they are not fake routes and do not disappear while a contributor or user has a draft. A domain model change does not automatically become a route or transport change.

## Error and diagnostic contract

Each domain context retains its typed failures. Application exhaustively maps public failures to a stable `PublicApplicationError` containing a code, explicitly typed safe parameters, message and recovery keys, recovery-action identifier, correlation ID, and an opaque local diagnostic-record reference. Business contexts do not own translated prose. The existing arbitrary `ApplicationError.details` map remains private during migration and is removed from the public Tauri result before TypeScript generation; private diagnostics are stored behind the local reference and require preview before disclosure.

Localization migration is staged so existing CLI and local API behaviour does not break:

1. add semantic message/recovery keys and safe parameters while retaining the current English `message` and `recovery` compatibility fields;
2. migrate Tauri and React to Experience-owned locale catalogues, then migrate CLI and any local API adapter to adapter-owned presentation catalogues built from the same semantic keys;
3. prove every public adapter renders every descriptor before a breaking Application-interface version removes legacy prose;
4. keep shared catalogue generation and validation in presentation/tooling infrastructure, never in a business context or the shared kernel.

Error presentation has two layers:

1. a safe localized problem and recovery action that is always visible and accessible;
2. an accessible disclosure containing stable code, correlation ID, contract versions, synthetic scenario, and local receipt link.

Private diagnostic fields appear only after explicit local bundle preview. Review builds may expose additional affordances, never additional domain meaning or unpreviewed private data. Stack traces and raw adapter errors do not enter the interface.

Generate an offline error catalog from Application descriptors. Each entry records the code, originating context, safe parameters, localization and recovery keys, retry safety, allowed diagnostic fields, synthetic reproduction scenario, and local help reference. CI proves every published error and recovery action is mapped exactly once in every migrated adapter. `p04 explain-error` renders the entry; `p04 reproduce-error` runs only safe owned scenarios and compares the observed Application and Experience results. Unsynthesizable errors state why.

Contexts publish only safe, typed diagnostic facts. Application correlates them; adapters may add bounded surface facts such as bridge and contract versions. A diagnostic bundle is local, previewable, synthetic by default, and never uploaded automatically.

## Scenario catalog

`scenarios.v1.json` is a versioned contributor projection assembled from context-owned synthetic fixtures and Application use cases. Each scenario names its owning context, purpose, allowed state, setup, expected capability/result/error, cleanup, and production-exclusion test. `p04 dev --scenario <id>` is the only interactive scenario entry point.

No scenario contains real personal data, secrets, provider credentials, arbitrary host paths, or a copied production workspace. Test-only scenario and fault-injection symbols are excluded from production bundles and proved absent.

## Documentation and contribution loop

`docs/contributing/p04/README.md` is the one P04 landing page. Its five-step fast path is Quick Tour, understand the golden slice, make one bounded-context change, verify, and prepare evidence. It deep-links to authoritative sources instead of duplicating them. Pages declare audience, authority, implemented/planned status, contract version, and last verified command.

Commands and expected output derive from the command and scenario catalogs and execute in CI. Rust and TypeScript boundary examples compile against their owners. `cargo xtask where <concept>` searches a rebuildable local ownership index and returns the bounded context, ubiquitous-language definition, Application interface, tests, requirements, gate status, and authoritative guide. The index cannot redefine context ownership.

`cargo xtask status --phase p04` and `cargo xtask next --phase p04` derive eligible, blocked, and completed slices from local machine-readable traceability, the current checkout, canonical accepted-evidence records, and declared source precedence. They surface contradictions instead of choosing between prose and machine sources. Remote CI and PR metadata are fetched only through explicit `--remote`, labelled with source and observation time, and never become accepted gate evidence by themselves. The commands show reasons, owners, contribution-readiness reading, expected evidence, and explicit task overrides; they never mutate planning state or replace the mandatory contributor reading contract.

Add issue forms for bugs, DX friction, and architecture clarification, plus the existing security-report route. They request task/scenario IDs, exact SHA, semantic error code, receipt summary, and synthetic reproduction. Attaching diagnostics requires explicit preview and scrub confirmation. No maintainer-response SLA is invented.

`p04 prepare-pr` inspects the selected task, exact diff, receipts, upgrade manifest, KCS/changelog state, accessibility evidence, risks, rollback, and open gates. It generates a local preview of the repository PR template. It cannot stage, commit, push, open a PR, contact GitHub, or convert missing evidence into a completion claim.

## Dependency and upgrade policy

Dependency automation creates a local previewable proposal; CI verifies but never writes a branch. Every proposed dependency or upgrade records licence compatibility, maintenance history, transitive and supply-chain surface, supported platforms/toolchains, Airgap and bundle-size effect, and why a smaller existing option is insufficient.

Every candidate generates an exact-head upgrade manifest covering:

- Application-interface compatibility and generated TypeScript diff;
- dependency, provenance, licence, install-script, network, and bundle effects;
- route, capability, error, diagnostic, scenario, locale, and documentation changes;
- canonical-format impact or an explicit proof of no format change;
- legacy-shell availability, removal gate, rollback, and irreversible boundaries;
- KCS, changelog, context-owner, and evidence updates.

`p04 rehearse-upgrade` compares the candidate against checked-in fixtures and receipts from the last accepted interface. It does not check out or execute arbitrary historical branch code. It replays synthetic scenarios, classifies semantic changes, proves the available rollback path, and fails on an unclassified breaking change.

## Programme scope, estimates, and dependency graph

The canonical P04 phase is estimated at seven to ten calendar weeks once amended P03D is accepted. That elapsed estimate assumes a two-to-three-person cross-stack team and covers the serial P04 core: React/toolchain setup, generated Application/Tauri interface, `app_status` plus the request-bearing native canary, semantic components, parity for already implemented P02 surfaces, the safe public error boundary, core synthetic isolation/receipts, installed-Mac qualification, and gated legacy removal.

The accepted DX Expansion recommendations remain committed work, but they form a separately governed `DXE-P04` programme and are not hidden P04 closure requirements. Plan on fourteen to twenty-one elapsed team weeks for the full core-plus-DXE programme, subject to platform evidence queues and dependency review; the CC estimates are planning aids, not delivery evidence or promises.

```text
G0 accepted P03 + amended P03D (design + traceability + Experience boundary)
└─ C1 isolated repository shape + preserved legacy source
   ├─ C2 React toolchain + generated Application/Tauri interface
   │  └─ C3 app_status + request/session/mutation/failure canary
   │     └─ C4 semantic system + existing P02 shell/error/recovery parity
   │        └─ C5 installed-Mac qualification + gated legacy removal
   └─ X1 minimal phase-neutral xtask/environment/result envelope
      ├─ X2 scenarios + receipts + diagnostics + contributor knowledge
      │  └─ X3 upgrade rehearsal + contribution loop
      └─ X4 measurement + clean/warm benchmarks + safe cache/resume
```

`C1-C5` are P04 core. `X1` contributes only the minimal commands and evidence needed by the core; advanced command discovery, community, upgrade, measurement, and cache features remain `DXE-P04` deliverables. All tasks will be implemented, but an unfinished DXE deliverable cannot be used to keep an otherwise accepted P04 core gate open.

## Delivery slices

### P03D closure - Authority, traceability, and Experience prerequisite

- re-inventory the authoritative base, exact P03/P03D heads, open PRs, worktrees, dirty state, and CI;
- replace `FG-R2-001` as P04's acceptance gate with a narrow `FG-B0-P04-001` foundation gate and audit every inherited evidence dependency, including `UAT-021` and `LRM-L10N-008`;
- re-own `UAT-021` and relationship-reminder `UAT-022` to their A0 tasks; re-own `UAT-062`, `LRM-UX-009`, and the complete-journey acceptance of `LRM-UX-012` to P11/`FG-B0-003`; add narrow P04 requirement/UAT records for existing-P02 React parity, semantic primitives, transient theme rendering, and 400% shell accessibility;
- amend `DESIGN.md` so P04 does not own persisted theme save/rollback, Directory/Event code splitting, Directory virtualization, or the complete B0 journey; those acceptance clauses remain P11/P06/P10-owned while P04 supplies the semantic foundation;
- assign P04 only semantic foundations, existing-P02 React parity, Application use-case/capability contracts, Experience-owned route mapping, and generic component seams; retain the complete Event Details-to-Brief workflow, persisted theme preference, and long-workflow interruption/resume in P11, and relationship reminders in A0;
- regenerate traceability and prove every requirement, UAT case, gate, task, and milestone has one computable owner;
- preserve non-authoritative draft work and restack only after the accepted P03D merge;
- establish the Experience bounded context, context-map edge, vocabulary, interfaces, and architecture tests inside P03D closure;
- update the generated traceability status before authorizing P04.1.

Acceptance: P03D's accepted merge head contains the corrected design and machine ownership, the candidate lineage is dependency-complete, Experience ownership is explicit, and no draft or prototype is represented as merged authority. Only then may P04.1 begin.

### P04.1 - Contributor foundation and generated contract

- add the environment, command, scenario, and scorecard contracts;
- add the repository-owned cross-platform, phase-neutral `xtask` front door, doctor, evidence levels, core isolation, and receipts;
- establish `apps/desktop/` as the frontend package root and pin Node/package-manager/Tauri versions;
- add React, TypeScript, Vite, testing-library, axe integration, and no runtime network dependency;
- generate and drift-check TypeScript from the Application DTOs and the complete Tauri command declaration;
- ship the real `app_status` golden slice, request/session/mutation/failure canary, focused embedded-WebDriver native assertion, and prepared/offline one-command Quick Tour.

Acceptance: an exact-input build receipt exists; a prepared/offline supported environment then produces Quick green within two minutes and a prepared native environment opens and drives the typed-status window within ten minutes, with exact receipts and explicit omitted gates. Clean connected bootstrap and compilation are timed separately and carry no target until evidence supports one.

### P04.2 - Semantic foundation

- implement `design/semantic-tokens.v1.json` as CSS custom properties and typed token names;
- implement system/light/dark/high-contrast resolution;
- implement Atkinson, Source Serif, and Plex Mono roles with existing local assets;
- create the versioned component contract from `DESIGN.md`;
- add Storybook-free local component fixtures rendered by the test app.

Acceptance: automated contrast, focus, token completeness, forced-colour, reduced-motion, `en-XA`, long-content, localization, and no-external-request checks pass.

### P04.3 - Shell parity and capability routing

- migrate top bar, Sections navigation, status region, error boundary, Workspace create/open/switch, People create/list/profile reachability, and Health;
- preserve native-operation serialization, stale-result rejection, and default-workspace resume;
- derive available routes from Application capabilities and map them exhaustively in Experience;
- distinguish structurally supported routes from current availability, permission, authority, and blocking state; preserve drafts across state changes;
- run legacy and React parity tests against the same fake and native command boundaries.

Acceptance: every existing browser and Rust workflow passes against React; structurally absent routes are omitted, supported-but-unavailable routes remain stable with accessible reasons, and no behaviour or draft disappears when the legacy shell is later removed.

### P04.4 - Errors, operations, diagnostics, and recovery

- implement the `DESIGN.md` operation binding table;
- separate `PublicApplicationError` from private diagnostic records, then implement typed recovery actions and two-layer safe error rendering for existing P02 surfaces;
- expose core receipts, safe operation identifiers, and opaque local diagnostic references;
- implement committed-recovery and conflict banners;
- prove cancel availability before commit and absence after commit;
- prove focus restoration, announcements, and interruption/restart parity for the already implemented P02 operations only.

Acceptance: P03 fault fixtures render the correct localized copy, actions, disclosures, announcements, opaque diagnostic references, and focus behaviour without serializing the private details map or leaking private facts. The searchable catalog, previewed bundle, and safe replay commands remain DXE work.

### P04.5 - Existing routes, components, and generic composition seams

- implement capability-backed routes only for surfaces already implemented by P02 and required for React shell parity;
- keep route IDs in Experience and map only accepted Application use-case identities;
- add generic layout/component composition seams without naming or freezing P05-P11 domain routes before those contracts exist;
- add semantic table/summary-detail, Stepper, Drawer, Dialog, ThemePicker, NoticeBanner, and OperationStatus components;
- provide representative component-state fixtures with presentation-only data; downstream contexts add their own route fixtures when their domain contracts exist;
- publish the minimal core contributor guide and golden-slice reference.

Acceptance: synthetic fixtures render every P04 component state in all built-in themes and locales at 320/360 CSS pixels, every contributor example executes at exact head, and no speculative Directory/Event/P11 route contract exists.

### DXE-P04.1 - Upgrade, community, and DX evidence

- add the offline error catalog, previewed private diagnostic bundle, and safe owned-scenario reproduction commands;
- generate the upgrade manifest and fixture-based rehearsal;
- add issue forms and local PR-draft preparation;
- add governed scorecards, selected-receipt reporting, and regression-waiver rules;
- run clean connected bootstrap, prepared/offline, and warm onboarding benchmarks on fresh supported macOS and Windows build runners with start state and claim surface labelled;
- add resumable content-addressed phase receipts with adversarial invalidation tests.

Acceptance: upgrade, handoff, and DX claims are exact-head, synthetic, privacy-safe, and reproducible; no contributor telemetry or activity ranking exists. This work is required by the full DX programme but does not expand the P04 core gate.

### P04.7 - Installed qualification and legacy removal

- build a universal macOS review artifact where the supported architecture contract requires it, plus Windows review artifacts;
- verify architecture, checksums, review signature state, zero external requests, all built-in theme rendering, transient preview cancellation/fail-closed fallback, keyboard, 400% reflow, reduced motion, and long localized content;
- run installed macOS/WKWebView plus VoiceOver evidence for the installed P04 claim; treat browser/axe results as supporting evidence only;
- label Windows as build-only until an installed WebView2 plus Narrator matrix passes; do not infer installed accessibility or supported-Windows status from compilation;
- record exact head, run IDs, browser/runtime version, OS version, assistive technology, and artifact hashes;
- remove the legacy switch and `ui-legacy` only after every parity and installed gate passes.

Acceptance: P04 evidence is bound to the exact merge candidate and claimed surface. It is not public-release, notarization, supported-download, supported-Windows, or Airgap evidence.

## State ownership

| State | Owner |
|---|---|
| relationship, workspace, event, recovery, and readiness meaning | owning business context |
| use-case result, capability, semantic error, and correlation | Application |
| active route, drawer/dialog state, unsaved field draft, announcement, and error disclosure | Experience |
| native bridge lifecycle and OS integration | Tauri adapter |
| transient theme preview and OS-derived appearance | Experience in P04 |
| persisted built-in appearance preference | P11 Settings Application use case; Experience renders it |
| resolved system light/dark mode | Experience from OS preference |
| workspace authority, key, recovery, and projection state | Workspace Session/Application |
| localized visible copy | Experience locale catalogues |
| contributor environment, scenario orchestration, receipts, and DX scorecard | contributor tooling/repository governance |

## Verification matrix

`cargo xtask quick|verify|qualify --phase p04` are the canonical entry points. The runner may orchestrate the following owner-provided checks but cannot copy their logic:

```text
npm ci --ignore-scripts
npm run typecheck
npm run lint
npm run test
npm run test:axe
npm run test:locales
npm run test:native:quick
npm run build
cargo test -p liaison-desktop bridge_contract
cargo test -p liaison-application --features typescript-bindings export_typescript_contract
git diff --exit-code -- apps/desktop/src/app/generated
python3 scripts/check_design_tokens.py
python3 scripts/check_design_contract.py
python3 scripts/check_desktop_shell.py
python3 scripts/check_repository.py
python3 scripts/check_spec.py
cargo fmt --all --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

Quick runs only the exact bounded bridge, generated-contract, core component, and synthetic lifecycle checks declared in its receipt. Verify runs the full local source matrix. Qualify runs Linux source checks, macOS and Windows build matrices, and installed macOS/WKWebView plus VoiceOver evidence. Windows compilation and browser/axe results are supporting build evidence, not installed accessibility evidence; a Windows accessibility or support claim requires an installed WebView2 plus Narrator matrix. Fresh and warm onboarding benchmarks report their own start states and surfaces. A known base failure such as the observed Windows `people.storage-error` must remain classified as a base blocker unless exact evidence proves the candidate introduced it.

## DX measurement and feedback

Repository governance owns a machine-readable DX scorecard and generated report. It uses only synthetic CI receipts, explicitly selected local receipts, and deliberate contributor feedback. Nothing uploads automatically.

Initial budgets and invariants:

| Measure | P04 target |
|---|---:|
| Doctor on supported prepared environment | <=15 seconds |
| Prepared/offline Quick terminal result after exact-input build receipt | <=2 minutes |
| Prepared native typed-status window after exact-input build receipt | <=10 minutes |
| Clean connected bootstrap | measured separately; no target until evidence exists |
| Published errors with code, cause, recovery, and local help | 100% |
| Command/docs/catalog drift | 0 |
| Synthetic state cleanup after normal or interrupted run | 100% |
| Hidden telemetry, contributor ranking, or individual productivity measures | 0 |

Review the scorecard at P04.1, every delivery slice, and final qualification. A threshold change requires a reviewed decision. A regression fails the owning gate or receives a named, reasoned, expiring waiver. No metric may measure contributor worth, activity volume, attendance, performance, or productivity.

Clean connected bootstrap benchmarks execute on fresh supported macOS and Windows build runners, followed by build preparation, prepared/offline, and warm reruns. Every result labels network availability, dependency-cache state, build-receipt digest, hardware/runner class, installed native surface, and claimed accessibility surface. Absolute budgets gate only the start states they name. Noisy hosted-runner deltas are reported with context and do not become fabricated precision.

## Migration and rollback

- Keep `ui-legacy` until exact-head parity and installed evidence pass.
- The React shell reads no new canonical format.
- A build-time switch permits one review artifact to select the legacy shell before P04 acceptance.
- Remove the switch and legacy shell only in P04.7 after all gates pass.
- Reverting P04 before canonical-format work restores the old shell without a workspace migration.
- A dependency or Application-interface change must pass proposal, compatibility classification, fixture rehearsal, documentation, and rollback checks.
- Semantic error localization retains the legacy English compatibility fields until Tauri/React, CLI, and every local API adapter prove complete key-based rendering; their removal is a reviewed breaking interface version, not a P04 shortcut.
- No destructive migration, canonical repair, or user-data operation is introduced by contributor tooling.

## NOT in scope

- starting P04 before accepted P03/P03D exact-head authority;
- merging current draft P04 branches merely to preserve their work;
- P05-P11 business capability implementation, A0 Personal Memory, mobile, providers, AI, MCP, plugins, or Meitheal;
- real P11 Directory, Events, Event Details-to-Brief, Health, and Settings destinations; persisted theme preference; and long-workflow interruption/resume;
- A0 relationship reminders, dashboard customization, settings transfer, personal interactions, commitments, and Review;
- speculative typed route IDs, fixtures, or domain interfaces for P05-P11 before the owning context publishes a real contract;
- relationship allocation, value, strength, employee, attendance, productivity, or communication-volume scoring;
- arbitrary real-workspace fault injection, fixture capture, or diagnostic upload;
- hosted documentation, accounts, analytics, hidden telemetry, remote logging, or automatic issue/PR creation;
- a prebuilt downloadable Quick Tour before packaging, provenance, and signing gates exist;
- a rolling two-version desktop runtime or unversioned Application contract;
- a task runner, route, DTO, database schema, or generic metadata map becoming a domain model;
- weakening the mandatory contributor reading contract merely to improve a timing metric; the hub may index authority but cannot replace it;
- public-release, notarized-distribution, supported-Windows, or Airgap claims from P04 source checks.

## Review gates

P04 cannot merge with:

- unresolved P03/P03D lineage or a non-authoritative base;
- P03D that has not absorbed the design/traceability/Experience amendments or an execution branch not rooted at its accepted merge head;
- P04 ownership of `UAT-021`, `UAT-022`, `UAT-062`, `LRM-UX-009`, complete-journey `LRM-UX-012`, `LRM-L10N-008`, or the unsplit `FG-R2-001` umbrella;
- a missing Experience bounded-context contract;
- handwritten duplicate DTOs or domain rules outside their owner;
- a generated DTO set that omits Tauri command names, outer request envelopes, field casing, or public/private error separation;
- a task runner that directly constructs repositories or context services;
- a command catalog that executes shell text, inherits ambient environment authority, or accepts unvalidated identifiers as executable input;
- new React user-facing strings outside Experience catalogues, or removal of legacy Application prose before every public adapter proves semantic-key rendering;
- raw colours outside the token adapter;
- route-local operation, recovery, readiness, or relationship calculations;
- a fake later-phase route, a structurally absent route rendered as available, or a supported route that disappears and loses a draft when availability changes;
- an unmapped capability, error, recovery action, locale key, command, scenario, or help entry;
- arbitrary canonical workspaces accepted by contributor fault or verification commands;
- remote assets, undeclared network requests, telemetry, secrets, or real-data fixtures;
- unresolved keyboard, focus, screen-reader, reflow, contrast, reduced-motion, long-content, or `en-XA` failures;
- a dependency change without the required proposal and provenance record;
- a Quick/Verify receipt represented as qualification;
- a cached result represented as qualification;
- a Quick timing claim without an exact-input build receipt, hardware/start-state label, and a focused real native WebDriver assertion;
- failure of `check_repository.py`, `check_design_contract.py`, or `check_spec.py`;
- a missing exact-head installed macOS/WKWebView plus VoiceOver review artifact at P04.7;
- a Windows installed-accessibility or support claim without installed WebView2 plus Narrator evidence.

## Decision ledger

| Decisions | Accepted direction |
|---|---|
| D1-D6 | Contributor platform; cross-stack OSS persona; Champion targets; real native `app_status`; DX Expansion |
| D7-D14 | Generated status; preserve/restack gate; formal Experience context; two onboarding lanes; doctor; `xtask`; permanent desktop package root; Quick/Verify/Qualify |
| D15-D22 | Application-generated TypeScript; golden reference slice; capability routes; semantic localized errors; safe diagnostics; owned scenarios; previewed dependency proposals; lockstep interface versioning |
| D23-D25 | Resolve every role-play confusion; one-command Quick Tour; automatic local proof receipt |
| D26-D30 | Versioned contributor result; generated command catalog; two-layer errors; offline error catalog; synthetic error replay |
| D31-D35 | Progressive executable docs; local ownership navigator; exact-head upgrade manifest; fixture-based rehearsal |
| D36-D38 | Shared environment contract; per-run isolation; content-addressed resumable phases |
| D39-D41 | Exact-head task discovery; safe support intake; local evidence-grounded PR draft |
| D42-D44 | Synthetic/opt-in measurement; governed scorecard; fresh cross-platform onboarding benchmark |
| D45-D46 | Split over-broad UAT/gate ownership; P04 owns contracts and existing-surface parity while P11 owns real B0 workflows/persistence and A0 owns relationship reminders |
| D47-D50 | Separately governed P04 core and DXE programme; explicit start-state timings; complete dirty-worktree cache key; `app_status` plus request/session/mutation/failure canary |
| D51-D53 | Staged semantic-key localization migration; Experience under the desktop source tree and a real xtask crate; structural capability separated from availability and blocking state |
| D54-D56 | Offline-first authority discovery with explicit remote mode; claim-specific native accessibility evidence; current-state facts and repository gates treated honestly |
| D57-D60 | P03D absorbs the prerequisite governance split; audit the full inherited gate including UAT-021/LRM-L10N-008; amend DESIGN.md's phase ownership; execute from an isolated accepted-head worktree while preserving user UI edits |
| D61-D64 | Application publishes use-case identity while Experience owns routes; remove speculative P05-P11 route contracts; split public errors from private diagnostics; generate the entire Tauri invoke seam from Rust-owned declarations |
| D65-D68 | Use maintained embedded WebDriver native proof; require exact-input build receipts for timing claims; make xtask phase-neutral and shell-free; stream a fail-closed Git-scoped SHA-256 worktree digest |
| D69-D72 | Move core scenarios/receipts and safe error rendering into P04; leave advanced diagnostics/replay in DXE; revise the core estimate to 7-10 weeks and full programme to 14-21 weeks; retain mandatory reading honestly |
| D73-D85 | Phase-correct four-route IA; deterministic launch and hierarchy; stable capability routes and current-window drafts; complete visible states and exact recovery language; transient theme review only; explicit trust journey; Editorial Ledger app grammar; redlined mockup authority; labelled narrow navigation; installed accessibility evidence; no deferred design debt |

## DX scorecard

Scores describe the amended plan, not implemented reality.

| Dimension | Before | Amended plan | Evidence required to claim reality |
|---|---:|---:|---|
| Getting Started | 3/10 | 9/10 | prepared/offline Quick, separate bootstrap, and native timing receipts |
| API/CLI/SDK | 6/10 | 9/10 | request/result generation, help snapshots, JSON contract, exit and claim tests |
| Error Messages | 6/10 | 9/10 | staged adapter migration, exhaustive catalog, three traced failures, accessible disclosure tests |
| Documentation | 4/10 | 9/10 | executable examples, generated offline status, link and drift checks |
| Upgrade Path | 7/10 | 9/10 | candidate manifest, compatibility fixtures, rehearsal receipt |
| Dev Environment | 5/10 | 9/10 | doctor parity and labelled macOS/Windows start-state runs |
| Community | 6/10 | 9/10 | task discovery, safe issue forms, PR draft checks |
| DX Measurement | 2/10 | 9/10 | governed exact-head scorecard and benchmark receipts |
| **Overall** | **4.9/10** | **9.0/10** | all owning gates complete |

TTHW is currently unmeasured for a clean P04 contributor environment. The observed warm CLI walkthrough is not native P04 proof. The amended targets apply only to prepared/offline Quick (<=2 minutes) and prepared native launch (<=10 minutes); clean connected bootstrap remains measured and unclaimed.

DX principle coverage: zero friction, learn by doing, fight uncertainty, opinionated defaults with explicit escape hatches, code in real context, and a real magical moment are all covered by plan. Implementation and boomerang `/devex-review` evidence remain open.

## DX implementation checklist

- [ ] Time to Quick green is <=2 minutes on prepared/offline supported evidence runners with an exact-input build receipt.
- [ ] Native typed-status window opens and passes the focused embedded-WebDriver assertion within <=10 minutes on a prepared native environment.
- [ ] Installation and first run use one canonical command after declared prerequisites.
- [ ] The magical moment traverses the real Application/Tauri/Experience boundary.
- [ ] A request-bearing synthetic canary proves session continuity, mutation, adapter request parity, and typed failure.
- [ ] Every published error has problem, safe cause, recovery action, and offline help.
- [ ] CLI naming, defaults, output, exits, and evidence claims are contract-tested.
- [ ] Documentation examples execute and represent real use cases.
- [ ] Upgrade classification, rehearsal, deprecation, and rollback are documented.
- [ ] Generated TypeScript is deterministic, checked in, and covers Application DTOs plus Tauri command names, envelopes, casing, results, and public errors.
- [ ] Private diagnostic details never enter the generated/public Experience contract.
- [ ] Contributor commands are phase-neutral, shell-free, environment-allowlisted, bounded, and process-group-cleaned.
- [ ] Local and CI environments consume one versioned manifest.
- [ ] Cache reuse keys every declared tracked and untracked input, fails closed when completeness is unknown, and is disabled for qualification.
- [x] No account, credit card, hosted service, or telemetry is required.
- [x] `CHANGELOG.md` and KCS-informed contribution rules exist.
- [ ] Local documentation ownership search is available.
- [ ] Safe community intake is available and its ownership is named.

## Engineering review findings

The 2026-07-22 full engineering review retained the full user-requested programme but corrected its dependency boundary. Recommended options were accepted under the standing instruction to implement every recommendation autonomously.

| # | Section | Finding and motivating evidence | Accepted resolution | Confidence |
|---:|---|---|---|---:|
| 1 | Architecture | `DESIGN.md:298` says “B0 persists one built-in selection” and `DESIGN.md:310` says “P04 measures” Directory/Event performance, while P04 excludes persistence and those routes. | Amend DESIGN.md in P03D so P04 owns transient themes and shell performance; P06/P10/P11 own downstream persistence/data-route evidence. | 0.99 |
| 2 | Architecture | This plan formerly blocked P04 on a traceability correction while placing that correction inside P04.0. | Move the correction, Experience decision, and design amendment into P03D closure; start P04 only afterward. | 0.99 |
| 3 | Architecture | `feature-gates.yaml:99` pulls `UAT-021`, `UAT-022`, and `UAT-062` into `FG-R2-001`; P04 also inherits public-site/vault `LRM-L10N-008`. | Replace the umbrella with `FG-B0-P04-001`, audit all inherited records, and re-own A0, P11, and public-site evidence explicitly. | 0.99 |
| 4 | Architecture | Application was asked to name “route/use-case identity,” although routing is Experience vocabulary. | Application publishes stable use-case/capability IDs; Experience alone maps them to desktop route IDs. | 0.98 |
| 5 | Architecture | Later Directory/Event route IDs and fixtures would freeze P05-P11 guesses before their owning contracts exist. | Keep only generic composition seams; add named routes with their owning downstream vertical slice. | 0.98 |
| 6 | Architecture | `ApplicationError.details` is an arbitrary `BTreeMap<String, Value>` at `crates/liaison-application/src/lib.rs:344`, while `apps/desktop/README.md:27` calls it private. | Introduce a typed safe public descriptor and opaque diagnostic reference before binding generation. | 0.99 |
| 7 | Architecture | Core acceptance depended on scenario/receipt and error work classified as DXE. | Put minimal scenarios, isolation, receipts, and safe error rendering in core; keep catalog/bundle/replay expansion in DXE. | 0.97 |
| 8 | Architecture | Local `main` is 58 ahead/8 behind with three user-edited legacy UI files; remote P03 is authoritative and P03D remains unmerged. | Execute from an isolated accepted-P03D worktree and preserve the current worktree byte-for-byte. | 0.99 |
| 9 | Code quality | Data-driven commands could become executable shell strings or inherit ambient credentials. | Use typed program IDs/argv arrays, no shell, fixed cwd, explicit environment allowlist, deadlines, and owned process groups. | 0.97 |
| 10 | Code quality | Fourteen `p04`-namespaced commands would be disposable or invite one runner per phase. | Make the command namespace permanent and select phase metadata with `--phase p04`. | 0.96 |
| 11 | Code quality | The contributor hub claimed to resolve the first-read cliff although `AGENTS.md` still makes the full read mandatory. | Retain the governance contract honestly; position the hub and ownership index after the required read. | 0.99 |
| 12 | Code quality | The legacy/React build switch had no exact Tauri activation mechanism. | Use checked-in config overlays passed through Tauri `--config`; keep legacy default until the removal gate. | 0.96 |
| 13 | Test | Private Tauri request structs at `apps/desktop/src-tauri/src/lib.rs:26-90` can drift in command name, outer envelope, casing, and fields. | Generate DTOs with pinned `ts-rs` and generate wrappers/handler metadata from one `desktop_command_contract!` declaration; retain golden JSON parity fixtures. | 0.97 |
| 14 | Test | Launching a native window alone does not prove React received and rendered real Tauri results. | Drive the compiled binary with the maintained embedded WebdriverIO Tauri provider and assert accessible rendered outcomes. | 0.98 |
| 15 | Test | No test proved arbitrary private details cannot enter generated TypeScript, DOM, receipts, or diagnostics. | Add negative schema, serialization, DOM, receipt, and bundle-leak fixtures. | 0.99 |
| 16 | Test | No adversarial test covered tracked/untracked content, symlinks, path escapes, ignored roots, or special files in cache keys. | Add table-driven digest invalidation and fail-closed tests. | 0.97 |
| 17 | Test | No integration test covered timeout/interruption cleanup of child processes and synthetic workspaces. | Add process-tree and interruption E2E tests on each supported build OS. | 0.96 |
| 18 | Test | Capability availability changes could hide routes or lose drafts without a state-transition oracle. | Add model-based Experience tests and native draft-preservation E2E coverage. | 0.95 |
| 19 | Test | Legacy parity could pass mocked browser tests while native command envelopes still failed. | Run the same scenario assertions against legacy, React browser mode, and the real native bridge before removal. | 0.98 |
| 20 | Test | Installed accessibility evidence could be inferred from axe/browser output. | Keep axe supporting only; require installed WKWebView/VoiceOver evidence bound to the artifact. | 0.99 |
| 21 | Performance | A two-minute Quick budget formerly omitted compiled-output state and a hardware baseline. | Require an exact-input build receipt and report hardware/runner class; measure bootstrap/build separately. | 0.99 |
| 22 | Performance | A complete dirty-tree digest could load large files or traverse symlinked/ignored trees. | Use Git-scoped NUL-delimited enumeration and streaming SHA-256 with fail-closed file-kind handling. | 0.98 |
| 23 | Performance | `app_status` and capability publication could accidentally scan canonical records during each render. | Require O(1) build/session metadata reads and prohibit canonical workspace traversal in shell status. | 0.94 |
| 24 | Performance | Unbounded contributor subprocesses could hang Quick, leak ports, or keep writer authority alive. | Apply phase deadlines, process-group termination, bounded output capture, and cleanup receipts. | 0.98 |

No LLM or prompt surface is introduced, so no evaluation suite is required. No separate `TODOS.md` item was selected: every accepted finding is implementation scope, while rejected scope stays in `NOT in scope`.

## Test coverage review

```text
CODE PATHS                                                USER FLOWS
[+] Application/Tauri/Experience contract                [+] Existing legacy P02 workflow
  ├── [★★ TESTED] Application app_status unit path          └── [★★★ TESTED] workspace/create-person/list/Health
  ├── [★★★ TESTED] WorkspaceSession + People services     [+] Prepared Quick
  ├── [★★ TESTED] current Tauri request deserialization     ├── [GAP] [→E2E] exact build-receipt validation
  ├── [GAP] generated DTO + command/envelope parity         ├── [GAP] [→E2E] native app_status rendered
  ├── [GAP] public/private error separation                  └── [GAP] [→E2E] request/mutation/failure canary
  ├── [GAP] capability-to-route exhaustive mapping        [+] Capability transitions
  ├── [GAP] locale/error/recovery exhaustive mapping        └── [GAP] [→E2E] unavailable→available keeps draft
  ├── [GAP] shell-free allowlisted command execution      [+] Error and recovery
  ├── [GAP] timeout/process-group cleanup                   └── [GAP] [→E2E] announce, act, restore focus
  ├── [GAP] tracked/untracked/symlink digest              [+] Interruption
  ├── [GAP] receipt invalidation and no-cache               └── [GAP] [→E2E] child/session/scenario cleanup
  ├── [GAP] legacy/React Tauri config selection           [+] Themes/localisation
  └── [GAP] test-plugin production exclusion                └── [GAP] [→E2E] 320/360/400%, en-XA, reduced motion
                                                         [+] Installed qualification
                                                           └── [GAP] [→E2E] WKWebView + VoiceOver + no-egress
                                                         [+] Upgrade and contribution preview
                                                           └── [GAP] additive/breaking/rollback/no-mutation

COVERAGE: 4/22 paths tested (18%) | Code paths: 3/13 (23%) | User flows: 1/9 (11%)
QUALITY: ★★★:2 ★★:2 ★:0 | GAPS: 18 (9 require native/integration E2E)
```

Legend: ★★★ behaviour + edge + error; ★★ happy path; ★ smoke; `[→E2E]` spans the real native or process boundary. Every gap above is assigned to T3-T16 below; none is deferred without an owner.

## Failure modes

| Path | Realistic failure | Planned test | Handling | User/contributor signal |
|---|---|---|---|---|
| Generated bridge | command/envelope/casing drifts | golden Rust/JSON/TS compile fixtures | generation fails | exact binding and diff |
| Public error | private path/value serializes | negative leak corpus | fail generation/serialization | safe code and local reference only |
| Native Quick | app launches but React never renders result | embedded-WebDriver assertion | timeout and capture safe logs | named native checkpoint failure |
| Capability transition | authority changes during a draft | model + native transition test | stable route, disable action, preserve draft | reason and recovery announcement |
| Command runner | catalog injects shell metacharacters | argv/property tests | reject identifier; no shell | stable tooling error |
| Child process | test hangs or leaves writer lock/port | forced timeout/interruption E2E | kill owned process group; cleanup | receipt lists cleanup result |
| Worktree digest | untracked or symlink target changes | adversarial invalidation table | disable reuse on uncertainty | cache miss reason |
| Receipt reuse | build receipt belongs to another digest | tampered/stale receipt test | refuse Quick | exact build next action |
| Legacy switch | release silently selects wrong assets | overlay/artifact manifest test | explicit config only | asset digest in receipt |
| Localisation | missing key falls back to unsafe prose | exhaustive locale matrix | fail build; retain compatibility field only during migration | stable missing-key code |
| Synthetic scenario | cleanup targets non-owned data | ownership/path-escape tests | reject outside state root | exact preserved target list |
| Qualification | axe result mistaken for native evidence | claim-schema tests | refuse qualification | omitted installed/AT gate |

No silent unhandled critical gap remains in the reviewed plan. Implementation evidence is still absent until these tests pass.

## Performance contract

```text
status/app_status       O(1) build + retained-session metadata; no workspace scan
Quick input validation O(number of declared input files + input bytes), streamed
command execution       bounded phase deadline + bounded captured output
component rendering     bounded P02 result sets; no downstream Directory claim
Verify/Qualify          intentionally outside Quick timing and cache promises
```

Numeric UI budgets are established from the first exact native baseline and may tighten only through the DX scorecard. Directory pagination/virtualization and Events code splitting cannot be measured or claimed until P06/P10/P11 provide the owning routes.

## Worktree parallelization strategy

| Step | Modules touched | Depends on |
|---|---|---|
| P03D governance closure | `spec/`, `docs/product/`, `docs/architecture/`, `DESIGN.md` | accepted P03 |
| Contract lane | `crates/liaison-application/`, `apps/desktop/src-tauri/` | P03D closure |
| Experience lane | `apps/desktop/src/`, `apps/desktop/design-system/` | P03D closure; generated contract before integration |
| Tooling lane | `tooling/xtask/`, `tooling/phases/`, `.cargo/` | P03D closure |
| Evidence lane | `docs/contributing/`, `docs/evidence/`, `.github/` | core contract and tooling receipts |

Lane A: P03D governance closure (sequential prerequisite).

After A merges, launch B (contract) + C (Experience foundations) + D (tooling) in isolated worktrees. C may build presentation-only primitives in parallel but waits for B before bridge integration. Merge B, then integrate C's native golden slice. D can continue through core scenario/receipt work, then consume B/C outputs. Launch E only after exact receipts exist. Root `Cargo.toml`, `Cargo.lock`, `apps/desktop/package*.json`, and generated bindings are deliberate merge points; one integration owner resolves them sequentially. This strategy is for team execution; this autonomous run remains preservation-first and must not mutate the current dirty worktree.

Inline ASCII diagrams belong beside the non-obvious implementation in `apps/desktop/src-tauri/src/bridge_contract.rs` (one declaration to Rust/TS), `crates/liaison-application/src/errors.rs` (typed failure to public/private split), `apps/desktop/src/experience/capabilities.ts` (definition/availability/draft transitions), `tooling/xtask/src/runner.rs` (spawn/timeout/cleanup), and `tooling/xtask/src/digest.rs` (Git enumeration to streamed digest).

## Design plan review

This review applies the app-UI rules to the exact P04 slice. The approved Editorial Ledger artifacts remain visual input, not implementation authority. The 12-screen atlas explicitly sets `implementation_authority: false`, includes P11 and A0 surfaces, and contains recovery, transport, export, appearance, route, and maturity claims that P04 cannot copy literally. The separate authority audit blocks literal parity while preserving the typography, warm local-file surfaces, restrained teal, exact counts, visible provenance, and non-scoring language.

The design generator could not produce a new P04 redline because no external OpenAI API key is configured. No replacement mockup is invented or presented as approved. P03D must turn the existing candidate and audit into a phase-correct textual contract before P04 source work.

### Pass 1 - Information architecture: 6.0/10 to 9.5/10

The earlier plan named routes and capabilities but did not say what occupies the first screen, how pre-workspace launch differs from an active session, or which region wins attention. The binding P04 shell is:

```text
Installed desktop window
├─ Skip link
├─ Application header
│  ├─ Liaison RM
│  ├─ local-authority / workspace state
│  └─ one application-level recovery banner when required
├─ Primary navigation
│  ├─ Overview      presentation-only orientation over Application status
│  ├─ Workspace     existing P02 create/open/switch/check capability
│  ├─ People        existing P02 create/list capability
│  └─ Health        existing P02 read-only inspection/validation capability
├─ Main region
│  ├─ one PageHeader and no more than one primary action
│  ├─ one dominant work surface
│  └─ secondary context/evidence after the primary task
└─ Status region
   ├─ latest operation announcement
   └─ safe correlation / receipt link when one exists
```

`Events`, `Directory`, `Settings`, personal profile/customisation, commitments, interactions, and Review are absent from P04 navigation. P04 may supply generic components that later contexts compose, but it does not publish their route IDs or imply that their workflows exist.

Launch and hierarchy are exact:

| Situation | Initial route | First | Second | Third |
|---|---|---|---|---|
| no workspace session | Workspace | choose or open a local folder | exact checks and one blocking reason | local-authority/no-account context |
| healthy open session | Overview | workspace identity and authority | one next safe action | latest operation or Health evidence |
| recovery required | current stable route with persistent banner | exact recovery condition | one safe review action | unaffected read-only context |
| explicit deep link to a supported route | requested route | route purpose and state | primary work surface | evidence/help |

Overview is not a dashboard. It contains no KPI mosaic, reminder queue, Event preview, relationship metric, or future capability teaser. It orients the user to the current local session and sends them to one real capability. Before a workspace opens, Workspace remains the default rather than forcing an empty Overview detour.

### Pass 2 - Interaction-state coverage: 5.0/10 to 9.5/10

Every state below is a presentation of an Application or context-owned result. Experience owns visibility, focus, draft handling, and localized wording; it never calculates the state.

| Feature | Loading/checking | Empty | Error/denied | Success | Partial/stale/conflict/recovery |
|---|---|---|---|---|---|
| application shell | labelled status `Checking this local app`; navigation remains stable | no workspace is an expected state, not an error | safe public code, problem, recovery action, diagnostic reference | version and local-only authority are available to assistive technology | persistent banner names the exact retained capability and the one safe next action |
| Workspace | selected folder and check currently running; repeated activation is blocked | warm choice between create and open, with no account language | keep the path input; name permission/structure/owner problem; focus the summary after submit | announce the opened workspace once and expose Overview | distinguish unsupported structure, partial bootstrap, concurrent owner, degraded recovery, and cancelled check; never say all data is safe |
| People | bounded list skeleton or text status; create form stays legible | `No people in this workspace yet` plus `Add a person` | preserve field values; associate field errors; show safe retry or Health route | announce the created Markdown profile and put it in the bounded list | invalid siblings do not hide healthy records; stale projection is named; authority loss blocks commit without deleting the draft |
| Health | `Checking workspace health` with scope | `No workspace inspected` plus Workspace action | read-only failure names scope and retry; raw paths stay private | exact checked scope and finding count | projection stale, canonical conflict, recoverable operation, controlled recovery, and unavailable capability remain separate states |
| capability-backed navigation | route stays in place while availability is refreshed | structurally absent capability is omitted | supported denied/blocked capability stays discoverable with reason and recovery | supported available route activates normally | availability changes do not discard drafts or move focus automatically |
| mutation/operation banner | staged text says nothing committed | no banner when no operation needs attention | pre-commit denial, publishing failure, and recovery conflict use their distinct Application outcome | one announcement plus receipt; no local `saved` guess | committed/publishing cannot be cancelled; recovery describes exact outcome and never promises universal rollback |
| theme review fixture | preview change announced without persistence | system-resolved theme is the default | invalid token/theme fixture fails closed to last rendered built-in | all semantic component states render under the selected preview | Cancel restores the entry palette; relaunch persistence and save rollback are not P04 claims |

Draft rules are deterministic:

1. Availability, authority, or permission changes never remove the active supported route.
2. An unsubmitted Person draft remains in Experience memory for the current window, becomes non-committable when authority is lost, and shows the blocking reason plus a safe copy/discard choice.
3. Workspace switching with a non-empty draft opens a consequence-naming Dialog. `Keep current workspace` is the initial action; `Discard draft and switch` is explicit.
4. Closing or reloading with an unsubmitted draft names that the draft is not persisted. Cross-relaunch draft recovery remains outside P04 and is never implied.
5. Background availability refresh never steals focus. User-initiated navigation moves focus to the destination heading; Dialog, Drawer, and Sections overlays return focus to their invoker.

### Pass 3 - User journey and emotional arc: 5.0/10 to 9.2/10

| Horizon / step | User does | Intended feeling | Contract that supports it |
|---|---|---|---|
| first 5 seconds | opens the installed app | oriented, not watched | Liaison RM, local authority, workspace state, no account, and one next action are visible without scrolling |
| first minute | chooses or opens a folder | in control | checks say what is being inspected; failure preserves the chosen path and gives one recovery action |
| first 5 minutes | opens the workspace, adds a synthetic or real local profile, then checks Health | confident without false reassurance | every mutation comes from an Application use case, every result is announced, and Health remains read-only during conflict |
| interruption | authority changes or an operation is incomplete | able to resume without panic | stable route, preserved draft, persistent exact recovery banner, and no blind retry |
| return visit | app resumes the accepted current workspace behavior | recognises the place | same four-route shell, workspace identity, current authority, and latest operation evidence; no surprise future destinations |
| long-term trust | inspects Markdown and receipts outside Liaison | ownership is credible | canonical files remain user-controlled, projections rebuild, and the UI never overstates checkpoint, backup, or portable recovery |

The emotional contract is calm operational confidence, not delight through animation. Copy is neutral, concrete, and non-moralising. Unknown, declined, stale, conflicting, and verified none remain distinct wherever those domain states later appear.

### Pass 4 - AI-slop risk: 7.0/10 to 9.6/10

Classifier: **APP UI**.

The P04 screen uses one dominant work surface, layout and dividers before containers, and at most one restrained hard-offset emphasis. Cards exist only for an independently actionable object. No three-column feature grid, centred hero, ornamental icon circle, decorative gradient, floating blob, universal rounded container, KPI mosaic, or generic welcome copy is admitted.

| Litmus check | Result after amendment |
|---|---|
| brand/product unmistakable in the first screen | YES - Liaison RM, local authority, and workspace state are the first reading order |
| one strong visual anchor | YES - the current work surface and its single primary action |
| understandable by scanning headings | YES - route purpose, exact state, and next action use utility language |
| each section has one job | YES - orientation, work, evidence, or recovery; never a mixed dashboard |
| cards actually necessary | YES only for independently actionable objects; ordinary grouping uses spacing and dividers |
| motion improves hierarchy or atmosphere | NO decorative motion is needed; state meaning is immediate and reduced-motion equivalent |
| feels intentional with decorative shadows removed | YES - typography, semantic boundaries, source context, and hierarchy carry the design |

### Pass 5 - Design-system alignment: 7.0/10 to 9.5/10

P03D amends `DESIGN.md` before P04 so its phase ownership matches the machine plan. P04 reuses the Editorial Ledger fonts, semantic tokens, operation-state mapping, and named component vocabulary. It does not copy the atlas's future routes or unsupported text.

| P04 need | Required design-system primitive | Constraint |
|---|---|---|
| shell and route hierarchy | `AppShell`, `PageHeader` | one main landmark, one page title, no more than one primary action |
| ordinary grouping / primary work | `LedgerPanel` | flat by default; one `emphasis="primary"` maximum |
| state and recovery | `Notice`, `RecoveryBanner`, `OperationStatus` | explicit heading, text state, next safe action, no icon/colour-only meaning |
| forms | `FormField`, `Button` | visible label; stable description/error; named variants; no placeholder-only label |
| bounded People/Health results | `DataTable` plus equivalent summary/detail | same information and actions at 320/360 and 400% zoom |
| no-result state | `EmptyState` | what is empty, why it matters, one valid next action |
| contextual evidence / decisions | `Drawer`, `Dialog` | documented focus entry, Escape/Close, and invoker return |
| phase-only appearance proof | `ThemePicker` rendered as a review fixture | `system`, `light`, `dark`, `high_contrast`; preview/cancel only; no P04 save claim |

All natural language uses semantic `en-IE` catalogue keys. Required state families include orientation, checking, empty, blocked, denied, stale, conflict, committed, complete, recovery, and retry. The safe public error owns typed parameters; locale code formats them. A component test selector, route ID, capability ID, correlation ID, receipt ID, or diagnostic reference is never translated.

### Pass 6 - Responsive and accessibility: 6.0/10 to 9.4/10

This is installed-desktop narrow-window support, not a mobile-product claim.

| Effective width | Intentional transformation | Navigation | Data / overlays |
|---|---|---|---|
| 760 CSS px and above | fixed rail plus fluid main region; primary/secondary columns only when both remain legible | labelled rail, current route via `aria-current="page"` | bounded tables; Drawer may be side-aligned |
| 480-759 CSS px | single-column main region; secondary context follows the task | one labelled `Sections` control; no icon-only bar or horizontal destination scroller | table becomes summary/detail when columns would force primary horizontal scroll; Drawer becomes full-width overlay |
| 320-479 CSS px and 400% zoom | one reading column; no sticky element obscures focus; long values wrap or use a labelled disclosure | same Sections interaction and visible current destination | equivalent summary/detail actions; Dialog fits viewport and scrolls internally without trapping page focus |

Keyboard and assistive-technology sequence:

1. Skip link targets the main heading.
2. Rail navigation is an ordinary labelled list. The Sections control opens a labelled menu: Enter/Space toggles, Arrow keys move, Home/End jump, Escape closes, and focus returns to the control.
3. Explicit route activation sets `aria-current` and focuses the destination `h1`; passive state refresh does neither.
4. Submit-time validation focuses the error summary or first invalid field and retains programmatic label/error association.
5. Dialog initial focus is the safest action; Drawer focus begins at its title; both return to the invoker.
6. `role="status"` announces non-blocking progress/result once. A blocking action error uses an assertive announcement without repeating on rerender.
7. Every primary workflow target is at least 48 by 48 CSS pixels; all other targets meet WCAG 2.2 target-size requirements or a documented exception.
8. Body text is at least 16 CSS pixels with 4.5:1 contrast; large text and non-text UI meet their applicable ratios. Meaning survives forced colours and every built-in.

Browser unit tests and axe checks are supporting evidence. P04 qualification requires the installed macOS/WKWebView keyboard and VoiceOver journey, 320/360 reflow, 400% zoom, reduced motion, high contrast, forced colours where supported, long content, `en-XA`, and zero external requests. A Windows build is not Windows accessibility evidence.

### Pass 7 - Decisions: 13 resolved, 0 deferred in this review

The user's standing instruction to take every recommended choice resolves the following two-way choices. The external D1-B observation outcome remains a prerequisite, not a design choice this review can fabricate.

| Decision | Accepted direction |
|---|---|
| D73 | Use the four-route P04 shell: Overview, Workspace, People, Health; omit future destinations completely. |
| D74 | Default to Workspace without a session and Overview with a healthy session; deep links remain capability-checked. |
| D75 | Give each screen one dominant task, one primary action, and secondary evidence after it. |
| D76 | Keep supported routes stable across availability/authority changes and preserve current-window drafts. |
| D77 | Specify visible loading, empty, denied, success, partial, stale, conflict, and recovery states for every P04 feature. |
| D78 | Use exact checkpoint, archive, operation, receipt, and conflict language; never collapse them into `safe` or universal rollback claims. |
| D79 | Make theme work a transient review fixture in P04; P11 owns saved preference, relaunch restoration, and save rollback. |
| D80 | Adopt the first-5-seconds, first-5-minutes, interruption, return, and long-term-trust journey above. |
| D81 | Enforce the app-UI Editorial Ledger grammar and reject card mosaics, ornamental UI, decorative gradients, and KPI summaries. |
| D82 | Treat approved mockups as phase-redlined visual input only; P03D text and machine ownership govern implementation. |
| D83 | Use the labelled Sections overlay below 760 CSS pixels and equivalent summary/detail data below the table threshold. |
| D84 | Bind the keyboard, focus, announcement, target, contrast, zoom, and installed-evidence matrix above. |
| D85 | Build every accepted design gap in T1/T2/T8/T9/T10; create no deferred `TODOS.md` design debt. |

### Design-review NOT in scope

- Literal parity with atlas Overview, Events, Directory, Settings, profile, customisation, readiness, Brief, or mobile-looking screens; they belong to later phase authority.
- Relationship reminders, commitments, interactions, personal user manuals, profile customisation, or relationship scoring; these remain A0 or prohibited as applicable.
- Persisted theme preference, density, product text scale, portable settings, or theme save rollback in P04; machine ownership remains P11/A0.
- PDF/print delivery, provider messaging, or an `Ask privately` transport; no accepted B0/P04 contract authorizes them.
- Claiming WCAG, EN 301 549, Airgap, macOS support, Windows support, mobile support, or recovery portability from static images, source checks, browser automation, or build success alone.

### Design-review completion summary

```text
+====================================================================+
|         DESIGN PLAN REVIEW - COMPLETION SUMMARY                    |
+====================================================================+
| System Audit         | DESIGN.md needs P03D phase correction;      |
|                      | approved atlas is visual input only          |
| Step 0               | 6.0/10; IA, states, hierarchy, evidence     |
| Pass 1  (Info Arch)  | 6.0/10 -> 9.5/10 after fixes               |
| Pass 2  (States)     | 5.0/10 -> 9.5/10 after fixes               |
| Pass 3  (Journey)    | 5.0/10 -> 9.2/10 after fixes               |
| Pass 4  (AI Slop)    | 7.0/10 -> 9.6/10 after fixes               |
| Pass 5  (Design Sys) | 7.0/10 -> 9.5/10 after fixes               |
| Pass 6  (Responsive) | 6.0/10 -> 9.4/10 after fixes               |
| Pass 7  (Decisions)  | 13 resolved, 0 deferred                    |
+--------------------------------------------------------------------+
| NOT in scope         | written (5 boundary groups)                 |
| What already exists  | DESIGN.md, P02 shell, P03 contracts,        |
|                      | Editorial Ledger candidate and atlas audit   |
| TODOS.md updates     | 0 proposed; accepted work remains in plan   |
| Approved Mockups     | 0 generated in this run; generator blocked  |
| Decisions made       | 13 added to plan                            |
| Decisions deferred   | 0 design decisions                         |
| Overall design score | 6.0/10 -> 9.4/10                            |
+====================================================================+
```

The plan is design-complete. A rendered `/design-review` and installed QA remain required after implementation; this score is not implementation or accessibility evidence.

## Implementation tasks

Synthesized from the review findings. Priorities describe P04 plan sequencing; none authorizes frontend work before amended P03D closure passes.

Design-review findings are owned by T1 (phase authority and `DESIGN.md` correction), T2 (Experience vocabulary and anti-corruption boundary), T8 (safe state/error/recovery presentation), T9 (information architecture, states, journey, semantic components, responsiveness, drafts, and localization), and T10 (rendered and installed evidence). No design finding is parked in an unowned TODO.

P03D closure is T1-T2. P04 core is T3-T10, including the minimal scenario/receipt and safe public-error work it actually depends on. T11-T16 are DXE. The dependency graph above, not list order alone, controls execution.

- [ ] **T1 (P1, human: 2-4d / CC: 5-8h)** - P03D governance - Close the design, traceability, authority, and preservation prerequisites before P04.
  - Surfaced by: Architecture findings 1-3 and 8; `FG-R2-001` also inherits `UAT-021` and `LRM-L10N-008`.
  - Files: `DESIGN.md`, `spec/implementation-plan.yaml`, `spec/uat-cases.json`, `spec/requirements.json`, `spec/localization-requirements.json`, `spec/feature-gates.yaml`, `spec/traceability-ownership.json`, generated traceability, `PROJECT_CONTEXT.md`, working-state delivery, `AI_BUILD_INSTRUCTIONS.md`, P03D evidence.
  - Verify: exact accepted P03/P03D base/head inventory, design and traceability generation/checks, one computable owner per record, contradiction report, and preserved-current-worktree manifest.
- [ ] **T2 (P1, human: 2-4d / CC: 5-8h)** - Experience - Establish the bounded context and Application anti-corruption boundary inside P03D closure.
  - Surfaced by: Architecture findings 4-5 and the missing Experience owner documentation.
  - Files: `apps/desktop/src/experience/README.md`, context map, ubiquitous language, focused decision, architecture checks.
  - Verify: context dependency tests, route/use-case ownership tests, and domain-language review.
- [ ] **T3 (P1, human: 5-8d / CC: 1-2d)** - Contributor tooling - Add the phase-neutral environment contract, shell-free runner, doctor, evidence levels, output envelope, and generated help.
  - Surfaced by: Code-quality findings 9-10 and performance finding 24.
  - Files: `tooling/xtask/Cargo.toml`, `tooling/xtask/`, workspace manifest, `.cargo/config.toml`, `tooling/phases/p04/environment.v1.json`, `tooling/phases/p04/commands.v1.json`, CI and docs generators.
  - Verify: `cargo xtask doctor`, help snapshots, JSON/exit tests, injection tests, timeout/process cleanup, and macOS/Windows parity.
- [ ] **T4 (P1, human: 3-5d / CC: 1d)** - Desktop toolchain - Establish the pinned React/Vite/TypeScript/WebdriverIO package and explicit inactive React Tauri overlay.
  - Surfaced by: code-quality finding 12 and native-test finding 14.
  - Files: `apps/desktop/package.json`, lockfile, Vite/TypeScript/WDIO configuration, Tauri config overlay, dependency evidence.
  - Verify: locked script-disabled install, reviewed dependency exceptions, licence/network/bundle checks, legacy-default and inactive-React assertions.
- [ ] **T5 (P1, human: 5-8d / CC: 1-2d)** - Application/Tauri interface - Generate the complete invoke contract and enforce lockstep compatibility.
  - Surfaced by: architecture finding 6 and test findings 13 and 15.
  - Files: `crates/liaison-application`, optional pinned `ts-rs` feature, `apps/desktop/src-tauri/src/bridge_contract.rs`, fixtures, `apps/desktop/src/app/generated/`.
  - Verify: deterministic regeneration; exact command/envelope/casing/request/result/public-error parity; additive/breaking fixtures; private-detail leak negatives; mismatch failure.
- [ ] **T6 (P1, human: 4-7d / CC: 1-2d)** - Golden slice - Deliver `app_status` plus a request/session/mutation/failure canary through Application, Tauri, Experience, and React.
  - Surfaced by: missing magical moment and executable DDD reference.
  - Files: Application schema, Tauri adapter, Experience routes, Overview, KCS guide.
  - Verify: focused embedded-WebDriver prepared Quick; generated request parity; session-scoped synthetic mutation; typed failure; accessible status/capability rendering; native receipt.
- [ ] **T7 (P1, human: 5-9d / CC: 1-2d)** - Core scenarios and evidence - Add owned synthetic scenarios, isolation, cleanup, exact build receipts, and local Quick receipts.
  - Surfaced by: architecture finding 7 and test findings 16-17.
  - Files: context fixtures, `tooling/phases/p04/scenarios.v1.json`, `xtask` runner, ignored evidence roots.
  - Verify: production-exclusion, interruption/process-tree cleanup, stale build receipt, dirty-head, symlink/path-escape, and omission receipts.
- [ ] **T8 (P1, human: 5-8d / CC: 1-2d)** - Public errors and recovery - Split private diagnostics, migrate semantic localization by adapter, and render typed recovery for P02 surfaces.
  - Surfaced by: architecture finding 6 and test findings 15 and 18.
  - Files: context error maps, Application public descriptors/private diagnostic records, Experience catalogues/components, CLI/Tauri compatibility adapters.
  - Verify: compatibility-field fixtures, exhaustive React/Tauri then CLI/local-API mappings, accessibility/privacy tests, three end-to-end error paths, breaking-version removal gate.
- [ ] **T9 (P1, human: 10-15d / CC: 3-5d)** - Experience UI - Implement semantic foundation, existing-P02 shell parity, capability semantics, and extension component states without P11 workflows.
  - Surfaced by: original P04 outcome and design-review contract.
  - Files: `apps/desktop/src/`, design-system contracts, locale catalogues, parity tests.
  - Verify: P04.2-P04.5 acceptance matrices, stable drafts during availability changes, no speculative later routes, and no persisted theme preference.
- [ ] **T10 (P1, human: 5-10d / CC: 2-3d)** - Qualification - Run Verify/Qualify, installed evidence, and gated legacy removal.
  - Surfaced by: claim-boundary and rollback review.
  - Files: workflows, installed evidence, legacy build selection, release notes.
  - Verify: exact-head Linux source and macOS/Windows build matrices, installed macOS/WKWebView plus VoiceOver evidence, Windows build-only claim check, and legacy-removal gate.
- [ ] **T11 (P2, human: 6-10d / CC: 1-2d)** - Contributor knowledge - Publish the post-read progressive hub, executable docs, error reference, and ownership navigator.
  - Surfaced by: code-quality finding 11 and Documentation pass.
  - Files: `docs/contributing/p04/`, generators, command/scenario/error catalogs.
  - Verify: docs execution, links, generated drift, mandatory-read honesty, and `cargo xtask where` acceptance.
- [ ] **T12 (P2, human: 6-10d / CC: 1-2d)** - Upgrade safety - Add dependency proposals, upgrade manifest, accepted fixtures, and rehearsal.
  - Surfaced by: Upgrade pass.
  - Files: dependency evidence, compatibility fixtures, changelog/KCS hooks, `xtask`.
  - Verify: classified additive/breaking/rollback cases and no branch-writing automation.
- [ ] **T13 (P2, human: 5-8d / CC: 1-2d)** - Contribution loop - Add task discovery, safe issue intake, and preview-only PR drafting.
  - Surfaced by: Community pass.
  - Files: traceability projection, `.github/ISSUE_TEMPLATE/`, PR-draft tooling.
  - Verify: blocked/eligible task fixtures, privacy scrub, no Git/network mutation tests.
- [ ] **T14 (P2, human: 6-10d / CC: 2-3d)** - DX evidence - Add scorecard governance and clean/build/prepared/warm cross-platform onboarding benchmarks.
  - Surfaced by: DX Measurement pass.
  - Files: `dx-scorecard.v1.json`, DX evidence docs, CI benchmark workflows.
  - Verify: budget, waiver, no-telemetry, and no-person-ranking tests.
- [ ] **T15 (P2, human: 5-9d / CC: 1-2d)** - Fast feedback - Add streamed Git-scoped content digests, resumable phases, and safe cache removal.
  - Surfaced by: performance findings 21-22 and Environment fast-feedback expansion.
  - Files: `xtask` cache model, receipt schema, adversarial invalidation tests.
  - Verify: exact-key reuse; tracked/untracked/symlink/special-file invalidation; `--no-cache`; interruption resume; previewed cleanup.
- [ ] **T16 (P2, human: 4-7d / CC: 1-2d)** - Diagnostic expansion - Add the offline catalog, previewed private bundle, and safe owned-scenario replay.
  - Surfaced by: core/DXE architecture finding 7.
  - Files: Application error catalog projection, `tooling/xtask/`, `docs/contributing/p04/errors.md`, privacy fixtures.
  - Verify: exhaustive catalog mapping, unsynthesizable-error explanation, bundle preview/scrub, no-upload proof, and safe replay parity.

## Engineering review completion summary

- Step 0 Scope Challenge: full user-requested core-plus-DXE scope retained; false phase ownership and self-blocking prerequisites corrected.
- Architecture Review: 8 issues found and folded into D57-D64.
- Code Quality Review: 4 issues found and folded into D65-D72.
- Test Review: combined code/user-flow diagram produced; 18 gaps identified and assigned.
- Performance Review: 4 issues found and assigned.
- NOT in scope: written and expanded for A0, speculative downstream contracts, governance shortcuts, and claim boundaries.
- What already exists: written with the private `ApplicationError.details` correction and authoritative-base distinction.
- TODOS.md updates: 0 items proposed; accepted work is in T1-T16.
- Failure modes: 12 enumerated; 0 silent unhandled critical gaps remain in the plan.
- Outside voice: Codex ran; 13 findings were accepted or reconciled, with no surviving cross-model tension.
- Parallelization: 5 lanes; B/C/D can run as 3 parallel lanes after sequential A, then E/integration is sequential.
- Lake Score: 24/24 review recommendations chose the complete option.

Retrospective: the local branch contains prior review-driven P03D, Event, customisation, and UI work that repeatedly crossed authority and route gates. This plan touches the same seams more aggressively: it begins from accepted authority, preserves the divergent work, generates the whole bridge rather than only DTOs, and refuses speculative Event/P11 routes.

## TODOS.md disposition

No separate `TODOS.md` entries are created by this review. Every accepted recommendation has an owner and task in this plan. Rejected alternatives are recorded under `NOT in scope`; they are not silent debt.

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 2 | ISSUES OPEN (stale review metadata) | 8 proposals, 4 accepted, 4 deferred; D1-B was later selected but its external observation outcome is still open |
| Codex Review | `/codex review` | Independent 2nd opinion | 4 | ISSUES FOUND AND INCORPORATED | Independent reviews exposed scope, authority, estimate, bridge, cache, localization, repository-shape, capability, platform, and gate risks |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 3 | CLEAR (PLAN) | Latest review: 24 issues, 0 unresolved, 0 critical gaps after amendment |
| Design Review | `/plan-design-review` | UI/UX gaps | 3 | CLEAR (FULL) | Latest review: 6.0/10 to 9.4/10, 13 decisions, 0 unresolved design choices |
| DX Review | `/plan-devex-review` | Developer experience gaps | 3 | CLEAR | Latest review: 4.9/10 to 9.0/10; 12 outside findings resolved in the plan |

**CODEX:** The independent review forced an honest P04/P11/A0 ownership split, generated whole-invoke contract, claim-specific native proof, fail-closed cache key, and request-bearing golden canary.

**CROSS-MODEL:** The current engineering, design, and DX reviews agree on explicit DDD ownership, phase-honest routes, exact evidence, preserved drafts, and a narrow Workspace/People/Health P04 surface.

**VERDICT:** ENG + DESIGN + DX CLEARED AT PLAN STAGE; the selected D1-B exact-artifact human observation and resulting P03/P03D authority receipts still gate P04 source implementation.

**UNRESOLVED DECISIONS:**

- D1-B outcome: representative workplace operators must observe the exact accepted P03 artifact and the distinct decision receipt must record Continue, Change, or Stop; no such human evidence exists yet, so P03D/P04 remain blocked.
