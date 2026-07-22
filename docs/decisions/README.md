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

## Proposed working-state decisions

- `0014` assigns desktop route mapping, drafts, focus, announcements, localisation, and safe disclosure to an Experience bounded context. It remains proposed until P03 and the P03D gate are accepted.
