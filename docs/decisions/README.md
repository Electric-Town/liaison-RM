# Decision records

Durable product, architecture, privacy, security, compatibility, and licensing decisions are recorded here.

Filename format:

```text
NNNN-short-decision-title.md
```

Template:

```markdown
# NNNN: Decision title

- Status: proposed | accepted | superseded | rejected
- Date: YYYY-MM-DD
- Deciders:
- Contexts:
- Requirements:
- Feature gates:

## Context and problem

## Constraints and evidence

## Alternatives considered

## Decision

## Consequences

## Migration, rollback, or reversal conditions

## Related knowledge, tests, and evidence
```

A decision record explains why a durable choice was made. It does not duplicate implementation detail that belongs in code or an operational article.

## Accepted working-state decisions

- `0001`–`0005` establish the shared Rust/Tauri core, open canonical files, provider-neutral contracts, separate build profiles, and honest relationship/readiness language.
- `0006` establishes one application composition root and `WorkspaceSession`.
- `0007` defines recoverable multi-target canonical commits.
- `0008` defines the workspace key hierarchy and honest local authorization boundary.
- `0009` defines the disposable Directory projection.
- `0010` defines event dietary readiness and structural least disclosure.
- `0011` separates local checkpoints from encrypted recovery packages.
- `0012` fixes the delivery order as B0 Workplace Review before A0 Personal Memory.
- `0013` pins the OKF v0.1 Draft People authoring profile, strict-write/tolerant-read boundary, sealed-data rule, and recoverable B0 normalization seam.
- `0015` requires exact technical P03 qualification and attestation, then D1-B observation and a separate Continue decision before design authority advances.
- `0016` scopes general and third-party post-A0 migration safety to R5 while preserving the narrow B0 OKF normalization contract.

Number `0014` appears only in preserved, non-authoritative unmerged proposals, including closed PR #46 and the premature `c2f852c` P03O branch. No ADR 0014 file belongs to current authority: the P03O branch inferred P03 completion from `3499a6e` and introduced competing `T-B0-P03O`/`FG-B0-P03-OBS-001` identifiers. Current main intentionally continues at accepted ADR 0015, whose only canonical chain is `LRM-PK-010` -> `T-B0-P03-OBS` (D9) -> `FG-B0-DESIGN-001`. Preserve the old refs, but do not merge, recreate, or cite ADR 0014/KCS-0015 as acceptance.
