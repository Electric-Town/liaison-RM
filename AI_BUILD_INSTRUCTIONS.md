# AI build instructions

This file gives coding agents an executable order of work. `AGENTS.md`, accepted decision records, bounded-context READMEs, schemas, requirements, feature gates, tests, and this file form the build contract. When they conflict, stop and open a decision rather than choosing silently.

## 1. Do not start from the UI

Build a vertical slice through domain, application service, port, adapter, CLI, tests, and documentation before adding a desktop screen. React components must not become the first or only implementation of a business rule.

## 2. Read order

For each task:

1. `AGENTS.md`
2. `SPEC.md`
3. owning context README and domain tests
4. related decision records
5. related knowledge articles
6. machine-readable requirement, UAT, feature gate, and implementation task
7. adapter or application README
8. current changelog

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

## 4. Implementation sequence

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

## 5. Rust rules

- Use the pinned toolchain in `rust-toolchain.toml`.
- Avoid `unsafe`; an exception requires a decision record, safety invariants, and focused tests.
- Prefer explicit domain types over strings and booleans.
- Return typed errors; preserve source errors internally without exposing secrets or filesystem internals to users.
- Use deterministic serialisation for canonical and signed content.
- Inject clocks, identifiers, randomness, filesystems, secret stores, and network clients.
- Do not use global mutable state.
- Do not add a dependency without reviewing licence, maintenance, transitive surface, build profiles, and Airgap effect.
- Keep provider SDKs in provider adapters.

## 6. Canonical-file rules

- Implement format structs separately from domain entities.
- Preserve unknown fields and sections.
- Validate before replacing a file.
- Use revision preconditions and content hashes.
- Journal writes and test interrupted replacement.
- Make projection rebuild independent of the previous projection.
- Never write secrets, access tokens, private keys, or remote credentials into the workspace.
- Use synthetic fixtures.

## 7. Provider and plugin rules

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

## 8. UX implementation rules

Before implementing a screen, write the task flow and required states. Use domain language, not database or provider terms.

Every user-facing task supports:

- keyboard completion;
- visible focus;
- loading, empty, partial, stale, conflict, permission, success, undo, and recovery states;
- interruption-safe draft preservation;
- screen-reader names and status messages;
- reduced motion;
- 200% zoom/reflow;
- localisation and long-content handling;
- a semantic alternative to graph or drag interaction.

Do not hide important data state in colour, animation, hover, spatial position, or an icon without a text alternative.

## 9. AI and MCP rules

- Read tools return source references and the grant used.
- Write tools create a staged proposal by default.
- Proposal review lists affected records, fields, old values, new values, provenance, and consequences.
- AI output is untrusted input and passes normal validation.
- No personal data is sent to a model without an explicit provider, purpose, scope, and expiry grant.
- Local Ollama-compatible operation must not require a remote account.

## 10. Required checks

```bash
python scripts/check_repository.py
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Run context, adapter, CLI, schema, provider conformance, migration, backup/restore, accessibility, and packaging checks when the task affects them.

Do not write that checks pass unless they were executed against the submitted commit. When a check cannot run, state the reason and keep the pull request in draft.

## 11. Completion report

At the end of a task, report:

- implemented behaviour;
- files and contexts changed;
- invariants added or changed;
- tests and exact results;
- requirements, UAT, and gates covered;
- knowledge and changelog updates;
- privacy, security, accessibility, migration, and rollback effects;
- remaining uncertainty and the next dependency-ready task.
