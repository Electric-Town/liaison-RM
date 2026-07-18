# TODOS — deferred scope ledger

Deliberate deferrals with context, so "later" is written down instead of lost.
Source decisions: [remediation PR plan](docs/audits/2026-07-18-remediation-pr-plan.md)
(CEO review outcome, 2026-07-18) and the [parity audit](docs/audits/2026-07-18-app-parity-and-wcag-gap.md).

## Deferred from the CEO review (2026-07-18)

- **Two-way meitheal sync.** E4 ships emit-only. Inbound trust (meitheal task completion
  becoming interaction evidence) needs its own decision record extending
  [ADR 0006](docs/decisions/0006-automation-emit-and-mcp-disclosure-boundary.md).
- **Weighted Review Priority (Plan step 7).** Reason-only review stays the default until
  users demonstrably trust the reasons. Requires transparent, versioned, exportable policies.
- **Relationship graph rendering.** Semantic table/tree ships first (LRM-RE-004); bounded
  graph layout (LRM-RE-005) follows.
- **Importers beyond vCard/CSV + CRM-in-Markdown.** Meerkat and Monica importers follow
  once the field-mapping model is frozen (E3 ships the first two paths).
- **Remote AI providers behind disclosure grants.** Local-only (Ollama-compatible) first;
  remote providers per SPEC §14.3 later.

## Open product decisions

- **Debts / money-owed tracking** (Monica has it): in or out of scope? Flagged in the
  parity audit §3b; undecided.
- **Message/outreach templates** (CRM-in-Markdown parity): likely a thin slice after the
  interactions surface; not yet scheduled.

## Repository hygiene

- **Retire PR #27** (empty diff: payload chunks + worktree-export workflow) and reopen it
  as the PR 1 design-system slice; review the `.handdrawn-payload` mechanism before trusting
  that branch.
- **Fold backend DRAFTs into vertical slices** so domain stops landing unsurfaced:
  #21 → People/profile slice, #22 → orgs/groups before events wedge, #25 → export/backup
  slice, #28 → workspace lifecycle.
- **Ratify RICE scores** from the audit §8 with the team and record them in `spec/`.
