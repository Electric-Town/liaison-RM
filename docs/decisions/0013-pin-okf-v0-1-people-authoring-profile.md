# 0013: Pin OKF v0.1 Draft for People authoring

- Status: accepted
- Date: 2026-07-19
- Deciders: Electric Town maintainer
- Contexts: workspace, people, directory, migration, workspace-security
- Requirements: LRM-WS-017, LRM-PE-016, LRM-PE-017
- UAT: UAT-065, UAT-066
- Feature gate: FG-B0-001
- Strategy overlay SHA-256: `795a6e6751cd29a995478e254323f491e68a53ef7c35fa729d8627b87cd37089`

## Context and problem

Readable People files need a portable Markdown envelope without allowing an interoperability draft to replace Liaison's domain model. The [Open Knowledge Format v0.1 Draft at immutable source commit `ee67a5ca27044ebe7c38385f5b6cffc2305a9c1a`](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/ee67a5ca27044ebe7c38385f5b6cffc2305a9c1a/okf/SPEC.md) defines a useful frontmatter and Markdown envelope. The reviewed raw specification has SHA-256 `b9655e607346dbbdc6de21190e9a953313eda6a7eba68d4d272a65975940ad6e`. It does not define Liaison identity, purpose, revision, provenance, information state, sensitivity, disclosure, or event-readiness meaning.

The first B0 People writer must therefore make portability deterministic while remaining tolerant of ordinary Markdown tools, unknown extensions, broken links, and malformed sibling files. Sensitive facts cannot be copied into plaintext merely to satisfy an external format.

## Decision

Liaison pins its first People compatibility profile to **OKF v0.1 Draft**.

Compatibility evidence is pinned to source commit `ee67a5ca27044ebe7c38385f5b6cffc2305a9c1a` and raw `okf/SPEC.md` SHA-256 `b9655e607346dbbdc6de21190e9a953313eda6a7eba68d4d272a65975940ad6e`; an upstream `main` change cannot silently alter this decision.

- Every Liaison-authored non-reserved People Markdown file has a non-empty `type: person` and maps supported OKF `title`, `description`, `tags`, and `timestamp` fields.
- Liaison's versioned, namespaced domain extension remains authoritative for stable identity, purpose, revision, provenance, information state, classification, disclosure, and operational meaning.
- OKF-valid never means Liaison-valid. Domain-invalid, stale, unauthorised, unsupported, or unsealed facts remain inert or quarantined and cannot affect event readiness.
- Sealed sensitive values remain encrypted envelopes or references. They never enter plaintext frontmatter, body text, generated indexes, projections, errors, logs, or evidence for OKF compatibility.
- Readers tolerate missing optional fields, unknown document types and keys, ordinary broken links, unknown sections, and malformed sibling files while healthy People remain available.
- Read-modify-write preserves unknown safe keys and sections, original Markdown body bytes outside controlled regions, stable IDs, ordinary links, and curated reserved or `index.md` content. A generated index never overwrites a curated body.
- The required B0 legacy-People normalization uses `WorkspaceSession`: exact preview, exact backup, journaled staging, final preconditions, one durable commit decision, idempotent recovery, and exact rollback. No partial profile/index state may survive a failure.

The work remains split by one accountable gate and task per contract:

| Seam | Task | Contract |
|---|---|---|
| Strict profile/schema and writer port | `T-B0-P05-OKF` | `LRM-PE-016`, `UAT-065` |
| Tolerant Directory read and projection | `T-B0-P06` | `LRM-PE-017` |
| Legacy People normalization | `T-B0-P09-OKF` | `LRM-WS-017`, `UAT-066` |

All three belong to `FG-B0-001` and follow P03. P01 and P02 are unchanged.

## Scope boundary

The OKF People normalizer is a required B0 format migration for Liaison's own People records. B0 still excludes general and third-party migrations, including Meerkat, Monica, CRM-in-Markdown, broad CSV/vCard round trips, provider sync, and arbitrary format conversion. Those remain independently gated later work.

OKF compatibility does not introduce a global person score, generic task engine, automatic identity merge, direct AI write, hidden provider refresh, or hidden egress.

## Consequences

- The canonical examples, schemas, validators, CLI and desktop writers use the pinned profile.
- A0 reuses `UAT-065` as B0 regression evidence; it does not co-own the test or create another People writer.
- External Markdown tools may add unknown safe content without forcing Liaison to understand or delete it.
- A conforming OKF document may still be unavailable to operational workflows until Liaison validation, policy, revision, and sealing checks pass.
- The format remains explicitly labelled a draft compatibility profile, not a complete person ontology.

## Future OKF changes

Liaison may claim a newer OKF version only after an accepted decision records:

1. explicit read and write adapters for both the old and new profiles;
2. a compatibility matrix and canonical fixtures;
3. additive migration with preview, fault injection, recovery, downgrade decision, and exact rollback evidence;
4. reserved-name, index, link, extension, body-byte, and sealed-data behaviour;
5. the release and interoperability claim supported by exact-head evidence.

Silently changing the emitted OKF version or treating a draft update as backward compatible is prohibited.

## Rollback or reversal conditions

If OKF v0.1 cannot preserve Liaison identity, unknown safe content, curated Markdown, or sealed-data boundaries, the writer remains on the last proven Liaison profile. A replacement requires the version-adapter decision and evidence above; existing files are never destructively rewritten merely to update a compatibility claim.
