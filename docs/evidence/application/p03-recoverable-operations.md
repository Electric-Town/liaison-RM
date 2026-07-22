# P03 recoverable canonical-operation evidence

Date: 2026-07-19
Reviewed: 2026-07-22
Status: **invalidated as acceptance evidence; source candidate only; P03 remains current**

## Claim boundary

The candidate source routes canonical Person creation and revisioned Person updates through a Workspace-owned operation protocol. The following description records its intended mechanism and the behavior covered by source tests; it is not an accepted durability, recovery, platform, or installed-artifact claim.

The candidate intends to discard an operation without `COMMIT`, roll forward one with `COMMIT`, treat an already-published target with the committed digest as idempotent, and stop on an external conflict. Independent source review found unresolved publication, evidence-binding, completion/projection ordering, bootstrap/session, untrusted-manifest, bounded-history, and child-process qualification gaps. Until those gaps are resolved and the accepted matrix passes, this document must not state that every crash or external-edit boundary is safe.

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

Neither PR #65/
`3499a6e9278fc72d2498a9978df59f30d03722e6`, later main
`49ee419e30f2d71524dd6fa15badf1ec4b8d0e27`, the unsupported `vB0` tag, nor
the premature `c2f852c` observation material completes P03. Exact `49ee419`
fails Rust and Windows formatting checks even though local workspace tests and
strict Clippy pass. A future qualified head must pass:

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

The dedicated operation tests and the accepted real child-process fault/relaunch matrix must pass on Ubuntu, macOS, and Windows. Technical acceptance must bind pairwise-distinct qualified-code, merge-result, and attestation Git SHAs plus distinct qualification-receipt and executable-artifact SHA-256 identities. Only that accepted tuple completes P03 and makes `T-B0-P03-OBS` current. Until then, `T-B0-P03` is `current`, not complete, and no D1-B session may start.

## Limits

This slice does not claim encrypted recovery packages, key lifecycle, Directory projection, OKF normalization, event readiness, public release readiness, or protection from a hostile process that bypasses Liaison and writes files directly. P07 and P08 own cryptographic recovery. P06 owns the disposable Directory projection. P09-OKF owns canonical People normalization.
