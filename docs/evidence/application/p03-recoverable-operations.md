# P03 recoverable canonical-operation evidence

Date: 2026-07-19
Status: source candidate under review; exact-head cross-platform evidence pending

## Claim boundary

This change routes canonical Person creation and revisioned Person updates through a Workspace-owned recoverable operation protocol. It stages every target, flushes staged bytes and the manifest, checks exact final preconditions, persists one durable `COMMIT` decision, publishes targets in stable order with per-target progress, persists `COMPLETE`, and marks the disposable projection stale.

An operation without `COMMIT` is discarded during session open. An operation with `COMMIT` rolls forward. Recovery treats an already-published target with the committed digest as idempotent. If a target differs from both its original precondition and the committed digest, recovery stops with a typed conflict and leaves the external bytes untouched.

The operation manifest contains paths, sizes, hashes, revisions, identifiers, timestamps, phases, and results. It does not contain staged Person content.

## Canonical layout

```text
.liaison/operations/<operation-id>/
├── manifest.yaml
├── COMMIT
├── COMPLETE
├── published/
│   └── 00000000.published
└── staged/
    └── 00000000.bin
```

`staged/` is removed after completion. The bounded manifest and phase evidence remain available for Health and incident diagnosis.

## Behaviours covered

- safe portable canonical paths;
- SHA-256 content identities;
- absent and exact-digest preconditions;
- optional expected record revision metadata;
- duplicate-target rejection and stable ordering;
- staged-byte and manifest durability before commitment;
- durable commit decision before publication;
- per-target progress evidence;
- no-clobber creation;
- exact-digest replacement;
- final precondition checks immediately before publication;
- projection-stale marker after completion;
- discard of pre-commit staging;
- roll-forward after commit;
- idempotent recovery after partial publication;
- conflict refusal after a non-cooperating external edit;
- application-provided operation identifiers and time;
- read-only People repositories that cannot mutate without an operation context;
- preservation of unknown front-matter fields and Markdown body sections.

## Fault matrix

The adapter tests inject failure:

1. after staged content and manifest are durable;
2. after the commit decision is durable;
3. after one target in a multi-target operation is published;
4. immediately before completion.

The enclosing workspace and application suites continue to cover writer authority, one-shot Health, malformed sibling records, stale revisions, and session quiescence.

## Remaining acceptance evidence

The PR remains draft until the exact head passes:

```text
cargo fmt --all --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
python3 scripts/check_architecture.py
python3 scripts/check_repository.py
python3 scripts/check_spec.py
python3 scripts/generate_traceability.py --check
```

The dedicated operation tests must pass on Ubuntu, macOS, and Windows. A final evidence amendment will bind the exact head and workflow run IDs. Until then, `T-B0-P03` is `current`, not complete.

## Limits

This slice does not claim encrypted recovery packages, key lifecycle, Directory projection, OKF normalization, event readiness, public release readiness, or protection from a hostile process that bypasses Liaison and writes files directly. P07 and P08 own cryptographic recovery. P06 owns the disposable Directory projection. P09-OKF owns canonical People normalization.
