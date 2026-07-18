# AI build instructions

This file gives coding agents an executable order of work. `AGENTS.md`, `docs/PROJECT_CONTEXT.md`, accepted decisions, bounded-context READMEs, schemas, requirements, feature gates, tests, and this file form the build contract. When they conflict, stop and record the conflict instead of choosing silently.

## 1. Establish the baseline

Before editing:

1. read `README.md`;
2. read `AGENTS.md`;
3. read `docs/PROJECT_CONTEXT.md`;
4. read `docs/STATUS.md`;
5. inspect the current pull-request stack and exact branch base;
6. run or inspect baseline checks;
7. state the owning bounded context and dependency-ready task.

Do not assume the default branch contains the newest accepted work while the repository uses stacked pull requests.

## 2. Do not start from the UI

Build a vertical slice through domain, application service, port, adapter, CLI, tests, and documentation before adding a desktop screen. UI components, provider adapters, shell scripts, and MCP tools must not become the first or only implementation of a business rule.

## 3. Task read order

For each task:

1. `SPEC.md`;
2. owning context README and domain tests;
3. related decision records;
4. related knowledge articles;
5. machine-readable requirement, UAT, feature gate, schema, and implementation task;
6. adapter or application README;
7. current changelog and status;
8. relevant workflow definitions and release evidence.

State which sources were read in the pull request.

## 4. Task selection

Select one implementation task whose dependencies are complete. Do not combine unrelated contexts to appear productive. A task identifies:

- problem and persona;
- owning context;
- inputs and outputs;
- invariants and prohibited outcomes;
- required commands, queries, events, and ports;
- acceptance and recovery tests;
- data classification and disclosure policy;
- accessibility and localization effects;
- migration and rollback effects;
- knowledge article action;
- feature gate and release effect.

If the task needs an unresolved architecture choice, open a decision PR before implementation.

## 5. Implementation sequence

1. Add failing domain or contract tests.
2. Add or revise value objects, entities, aggregate behaviour, policies, and domain events.
3. Add an application command or query and explicit context-owned ports.
4. Add an in-memory test adapter.
5. Add the production adapter.
6. Expose the use case through the CLI with human and JSON output where practical.
7. Add desktop, API, MCP, job, or plugin surfaces after the application service is stable.
8. Add integration, interruption, migration, and recovery tests.
9. Update context README, schemas, traceability, knowledge, status, and changelog.
10. Run repository, workspace, context, provider, accessibility, and packaging checks relevant to the change.
11. Report exact results against the submitted commit.

## 6. Rust rules

- Use the pinned toolchain in `rust-toolchain.toml`.
- Avoid `unsafe`; an exception requires a decision record, safety invariants, and focused tests.
- Prefer explicit domain types over strings, booleans, and unowned maps.
- Return typed errors; preserve source errors internally without exposing secrets or unnecessary filesystem detail.
- Use deterministic serialisation for canonical and signed content.
- Inject clocks, identifiers, randomness, filesystems, secret stores, and network clients.
- Do not use global mutable state.
- Do not add a dependency without reviewing licence, maintenance, transitive surface, build profiles, supply-chain effect, and Airgap effect.
- Keep provider SDKs in provider adapters.
- Keep Tauri, CLI parsing, HTTP, WIT, and persistence types outside domain models.

## 7. Canonical-file rules

- Implement format structs separately from domain entities.
- Preserve unknown fields and supported body sections.
- Validate before replacing a file.
- Use revision preconditions and content hashes.
- Journal writes and test interrupted replacement.
- Make projection rebuild independent of the previous projection.
- Keep invalid records discoverable by validation and repair.
- Never write secrets, access tokens, private keys, remote credentials, or unredacted personal fixtures into the workspace or repository.
- Use synthetic fixtures.

## 8. Product-model rules

- Relationship intent is manually configured.
- Relationship evidence is factual and source-linked.
- Maintenance status is explainable and relative to the configured relationship policy.
- Profile readiness is purpose-specific; there is no universal profile-completeness score.
- Review Priority orders attention and does not measure human worth, affection, trust, employee value, or relationship quality.
- Reason-only review ships before weighted review.
- Empty, unknown, stale, declined, conflicting, and not-applicable values remain distinguishable.
- Topic Pack field IDs are stable and separate from displayed labels.

## 9. Provider and plugin rules

A new provider implements an existing versioned contract or proposes a new contract in a separate architecture PR.

Required provider work:

- descriptor and configuration schema;
- anti-corruption layer;
- secret references;
- destination declaration;
- dry-run behaviour;
- conformance suite;
- consistency and limits statement;
- backup/single-writer/multi-writer evidence;
- failure, retry, idempotency, expiry, and revocation tests;
- knowledge article;
- exact-head evidence record.

A plugin is denied every capability not declared in its manifest and approved by the user. Plugins do not receive a raw database handle, unrestricted filesystem access, ambient network access, or unscoped relationship data.

## 10. UX implementation rules

Before implementing a screen, write the task flow and required states. Use domain language, not database or provider terms.

Every user-facing task supports:

- keyboard completion and visible focus;
- interruption-safe draft preservation and return to the previous place;
- loading, empty, partial, stale, conflict, permission, success, undo, and recovery states;
- screen-reader names and status messages;
- reduced motion and low-stimulation use;
- 200% zoom, reflow, long text, and localization expansion;
- semantic alternatives to graph or drag interaction;
- text alternatives for colour, icons, animation, hover, and spatial position;
- non-shaming wording and valid skip, snooze, pause, archive, and decline actions.

## 11. AI and MCP rules

- Read tools return source references and the grant used.
- Write tools create a staged proposal by default.
- Proposal review lists affected records, fields, old values, new values, provenance, disclosure, and consequences.
- AI output is untrusted input and passes normal validation.
- No personal data is sent to a model without an explicit provider, purpose, scope, and expiry grant.
- Local Ollama-compatible operation must not require a remote account.
- Prompt or tool injection cannot expand authority.

## 12. Required checks

```bash
python scripts/check_repository.py
python scripts/check_spec.py
python scripts/check_architecture.py
python scripts/check_providers.py
python scripts/check_wit_contract.py

cargo fmt --all --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

Run context, adapter, CLI, schema, provider conformance, migration, backup/restore, accessibility, browser, packaging, signature, and platform checks when the task affects them.

Do not state that checks pass unless they ran against the submitted commit. When a check cannot run, state why and keep the pull request in draft.

## 13. Completion report

Report:

- implemented behaviour;
- files and contexts changed;
- invariants added or changed;
- tests and exact results;
- requirements, UAT, schemas, and feature gates covered;
- knowledge, status, and changelog updates;
- privacy, security, accessibility, localization, migration, compatibility, and rollback effects;
- incomplete evidence and the next dependency-ready task.
