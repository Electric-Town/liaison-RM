# 0006: Automation emit and MCP disclosure boundary

- Status: proposed
- Date: 2026-07-18
- Contexts: Automation, Review and Attention, Relationships, Sharing

## Context and problem

The remediation plan ([docs/audits/2026-07-18-remediation-pr-plan.md](../audits/2026-07-18-remediation-pr-plan.md)) accepts two expansions that puncture the workspace's privacy membrane for the first time:

- **E6** — a local API and MCP surface exposing Review and Attention output (why-now reasons, cadence status, purpose readiness) to local agents and tools;
- **E4** — an emit-only bridge that publishes selected records to meitheal, a *household-level* execution hub, where other household members and Home Assistant automations can see them.

Without a hard boundary, a private relationship assessment, a sensitive note, a boundary ("paused", "do not contact"), or a classified profile field could leak into a shared household surface via an innocuous-looking "task". That failure is silent, outward-facing, and unrecoverable once another person has seen it.

## Decision

The Automation context enforces a **deny-by-default disclosure filter** on every emitted or MCP-served record. The rules:

1. **Allowlist, not blocklist.** Only these record types may cross the boundary, and only these fields of them:
   - *Commitment*: title, due date, linked person **display name only**, completion state.
   - *Cadence-due signal*: person display name, factual reason wording (e.g. "quarterly cadence; last interaction 112 days ago"), due date.
   - *Important date*: person display name, date label (e.g. "birthday"), date. Unknown-year dates never gain an invented year.
2. **Never emitted, no override:** private assessments and overlays (SPEC §10.2), relationship boundaries and do-not-contact/paused state, tier and importance values, any Topic Pack field classified `private` or `sensitive` (dietary detail, accessibility, family, health), interaction bodies and notes, review-priority numbers, and provenance chains.
3. **Suppression states also suppress emission.** A person who is archived, paused, do-not-contact, or snoozed produces **no** emitted records at all — the absence itself must not signal the state (an emitted "paused" marker would leak the boundary).
4. **Grant-gated like any provider.** The bridge and the MCP surface each require an explicit grant (purpose, data classes, fields, operations, retention, expiry, `approved_by`, `revocable: true`) per the existing Connections contract. No grant → the surface serves nothing, and in the Airgap build the listener and emitter are compiled out entirely.
5. **Auditable.** Every emission writes an audit record (what, to where, under which grant) to the workspace audit stream, inspectable by the user.
6. **One filter, one owner.** The filter lives in the Automation context's application service. No UI, plugin, importer, or the bridge itself may serialise a domain record for external consumption directly.

## Consequences

- E4/E6 implementation PRs must ship the filter, its deny-by-default tests (including a "new field types are denied until allowlisted" test), and the audit trail **in the same slice** — not as a follow-up.
- Two-way meitheal sync (deferred, see [TODOS.md](../../TODOS.md)) will require a second decision covering inbound trust; this ADR does not authorise it.
- The allowlist is intentionally narrow enough to be boring. Widening it is a reviewed schema change with a new grant version, never a code-only edit.
