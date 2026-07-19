# AI build instructions

This file gives coding agents an executable order of work. `AGENTS.md`, `PROJECT_CONTEXT.md`, `docs/product/working-state-delivery.md`, accepted decision records, bounded-context READMEs, schemas, requirements, feature gates, tests, and this file form the build contract. When they conflict, stop and reconcile the contract rather than choosing silently.

## 1. Do not start from the UI

Build a vertical slice through domain, application service, port, adapter, CLI, tests, and documentation before adding a desktop screen. React components must not become the first or only implementation of a business rule.

## 2. Read order

For each task:

1. `AGENTS.md`
2. `PROJECT_CONTEXT.md`
3. `docs/product/working-state-delivery.md`
4. `spec/traceability-ownership.json` and generated `docs/product/traceability.md`
5. `SPEC.md`
6. owning context README and domain tests
7. related decision records
8. related knowledge articles
9. machine-readable requirement, UAT, feature gate, and implementation task
10. adapter or application README
11. current changelog

State which sources were read in the pull request.

## 3. Task selection

Select one implementation task whose dependencies are complete. Do not combine unrelated contexts to appear productive. A task must identify:

- problem and persona;
- owning context;
- inputs and outputs;
- invariants;
- required ports;
- acceptance tests;
- privacy classification;
- accessibility effect;
- migration and rollback effect;
- knowledge article action.

## 4. Accepted delivery sequence

Work follows this order. A later item may be explored in isolation, but it cannot be merged as a competing authority or used to claim the earlier gate complete.

1. **P00 — contract truth:** reconcile accepted ADRs, formats, requirements, UAT, gates, tasks, versions, commands, evidence, and stale-branch claims.
2. **P01 — application composition:** add `liaison-application`, typed commands/DTOs/errors, common CLI/Tauri composition, tolerant Health, and correct validation/error semantics.
3. **P02 — workspace authority:** add `WorkspaceSession`, one advisory writer lock, read-only recovery, quiescence, and explicit schema handling.
4. **P03 — recoverable operations:** route every canonical mutation through staged multi-target operations with a durable commit decision and roll-forward recovery.
5. **P03 design gate:** run design consultation to create canonical `DESIGN.md`, then run plan design review against the complete B0 journey and amend P04 before implementation. G0 records this gate but does not create the file or preselect the direction.
6. **P04 — desktop inbound adapter:** migrate to React/TypeScript/Vite over typed Rust commands and prove Workspace, People, and Health parity before new event UI.
7. **P05-OKF/P06 — portable People foundation:** after P03 and P04, add the pinned OKF v0.1 Draft strict writer/schema port and tolerant Directory reader/domain-validity quarantine under `FG-B0-001`. Do not expand P01/P02.
8. **P06-REPAIR — guided canonical repair:** `T-B0-P06-REPAIR` runs after P03 and P06, previews invalid-record repair, creates an exact backup, applies through failure-atomic recoverable operations, records an exact receipt, supports exact rollback, and closes `UAT-040` under `FG-R1-002` before P09-OKF.
9. **P05/P07/P08 — separate B foundations:** keep G3 dietary/domain contracts in P05, sensitive types/policy exclusively in P07 under `FG-B0-002`, then prove checkpoint and encrypted clean-install recovery in P08.
10. **P09-OKF/P09–P11 — B product:** after P06-REPAIR, add required previewable, exact-backup-first, journaled, failure-atomic, idempotent and exactly reversible People normalization, then complete Directory onboarding import, Events/cohort/readiness/brief workflows, and the installed desktop experience.
11. **B0 qualification:** with one trusted local workspace owner, run scale, crash, repair, key, grant, leak, OKF, normalization/rollback, accessibility, offline, installed-app, and developer-journey evidence on the exact review artifact. Workplace domain types and outputs must structurally omit relationship allocation, ranking, and scoring.
12. **A0:** only after B0 acceptance, implement quick/full capture, the source-complete purpose-scoped profile, explicit fact states, reversible identity review, source/range timeline, stable custom-field layouts and user-organised profile tabs, interactions, bounded commitments, reason-only Review, and the personal-memory journey. Do not introduce a generic task engine, global person score, or automatic merge.

General and third-party migrations are excluded from B0 except for the required OKF People normalization. Mobile products, provider transports, multi-writer sharing, AI/MCP, Meitheal integration, broad platform support, and public notarized distribution remain later gates unless a prerequisite contract explicitly requires a narrow seam. Later providers have no hidden sync, refresh, or egress; later AI, MCP, plugins, providers, and imports can stage source-backed proposals but cannot write confirmed facts directly.

## 5. Vertical-slice implementation sequence

1. Add failing domain or contract tests.
2. Add or revise value objects, entities, aggregate behaviour, and domain events.
3. Add an application command/query and explicit ports.
4. Add an in-memory test adapter.
5. Add the production adapter.
6. Expose the use case through the CLI with human and JSON output.
7. Add the desktop/API/MCP surface only after the application service is stable.
8. Add integration and recovery tests.
9. Update context README, schema, requirement traceability, knowledge, and changelog.
10. Run repository and workspace checks.

## 6. Rust rules

- Use the pinned toolchain in `rust-toolchain.toml`.
- Avoid `unsafe`; an exception requires a decision record, safety invariants, and focused tests.
- Prefer explicit domain types over strings and booleans.
- Return typed errors; preserve source errors internally without exposing secrets or filesystem internals to users.
- Use deterministic serialisation for canonical and signed content.
- Inject clocks, identifiers, randomness, filesystems, secret stores, and network clients.
- Do not use global mutable state.
- Do not add a dependency without reviewing licence, maintenance, transitive surface, build profiles, and Airgap effect.
- Keep provider SDKs in provider adapters.

## 7. Canonical-file rules

- Implement format structs separately from domain entities.
- Preserve unknown fields and sections.
- Validate before replacing a file.
- Use revision preconditions and content hashes.
- Use the Workspace-owned recoverable multi-target operation protocol and test interruption before and after its durable commit decision.
- Make projection rebuild independent of the previous projection.
- Never write secrets, access tokens, private keys, or remote credentials into the workspace.
- Use synthetic fixtures.

## 8. Provider and plugin rules

A new provider implements an existing versioned contract or proposes a new contract in a separate architecture PR.

Required provider work:

- descriptor and configuration schema;
- anti-corruption layer;
- secret references;
- destination declaration;
- dry-run behaviour;
- conformance suite;
- consistency and limits statement;
- backup/sync mode evidence;
- failure, retry, idempotency, and revocation tests;
- knowledge article.

A plugin is denied every capability not declared in its manifest and approved by the user. Plugins do not receive a raw database handle, unrestricted filesystem access, or ambient network access.

## 9. UX implementation rules

Before implementing a screen, write the task flow and required states. Use domain language, not database or provider terms.

Every user-facing task supports:

- keyboard completion;
- visible focus;
- loading, empty, partial, stale, conflict, permission, success, undo, and recovery states;
- interruption-safe draft preservation;
- screen-reader names and status messages;
- reduced motion;
- 400% zoom/reflow;
- localisation and long-content handling;
- a semantic alternative to graph or drag interaction.

Do not hide important data state in colour, animation, hover, spatial position, or an icon without a text alternative.

## 10. AI and MCP rules

- Read tools return source references and the grant used.
- Write tools create a staged proposal by default.
- Proposal review lists affected records, fields, old values, new values, provenance, and consequences.
- AI output is untrusted input and passes normal validation.
- No personal data is sent to a model without an explicit provider, purpose, scope, and expiry grant.
- Local Ollama-compatible operation must not require a remote account.

## 11. Required checks

```bash
python3 scripts/check_repository.py
python3 scripts/check_spec.py
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

Run context, adapter, CLI, schema, provider conformance, migration, checkpoint/recovery, accessibility, and packaging checks when the task affects them.

Do not write that checks pass unless they were executed against the submitted commit. When a check cannot run, state the reason and keep the pull request in draft.

## 12. Completion report

At the end of a task, report:

- implemented behaviour;
- files and contexts changed;
- invariants added or changed;
- tests and exact results;
- requirements, UAT, and gates covered;
- knowledge and changelog updates;
- privacy, security, accessibility, migration, and rollback effects;
- remaining uncertainty and the next dependency-ready task.
