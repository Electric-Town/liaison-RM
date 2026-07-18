# ADR 0005: Separate relationship intent, evidence, maintenance, and readiness

- Status: Proposed
- Date: 2026-07-18
- Owners: Relationships; Identity and Profiles; Review and Attention

## Context

Communication frequency is observable, but it does not establish closeness, trust, importance, or relationship quality. A universal relationship score would combine user intent, imported activity, open work, and profile coverage into an opaque value. That model would be misleading for low-frequency friendships, unwanted high-volume correspondence, executive-assistant operations, families, and workplace data.

## Decision

Liaison models four separate concepts:

1. Relationship intent is user-authored and owned by Relationships.
2. Relationship evidence is factual and owned by Interactions and Commitments or the relevant source context.
3. Maintenance status is explainable and owned by Review and Attention.
4. Profile readiness is purpose-specific and owned by Identity and Profiles.

Numeric Review Priority, when enabled, orders a queue. It is not named or represented as relationship strength. Reason-only mode is the default for personal workspaces.

Interfaces and plugins consume application results. They may not calculate maintenance, readiness, or priority independently.

## Consequences

- Sparse profiles do not imply low importance.
- A relationship with no cadence is not automatically overdue.
- Imported message volume cannot assign closeness or trust.
- Readiness can differ by purpose.
- Every surfaced review candidate carries factual reasons.
- Weighted policies require versioning and component explanations.
- More bounded-context queries are required than in a single-score design.

## Rejected alternatives

### Universal relationship score

Rejected because it communicates false precision and encourages value judgments from incomplete evidence.

### Interaction-frequency score

Rejected because operational email, unwanted contact, and automated messages can dominate volume while strong relationships may be low-frequency.

### Universal profile completeness

Rejected because it pressures users to collect irrelevant or sensitive information and cannot represent purpose-specific requirements.

### Per-interface calculations

Rejected because UI, plugins, providers, and AI adapters would drift and could bypass suppressions or privacy controls.
