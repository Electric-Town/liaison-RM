# 0014: Allow parallel preparation without transferring acceptance

- Status: accepted
- Date: 2026-07-19
- Deciders: Electric Town maintainer
- Contexts: governance, all
- Requirements: none added; the delivery rule in `spec/implementation-plan.yaml` is unchanged
- Feature gates: none added; no gate closes earlier because of this decision

## Context and problem

Liaison RM is built by more than one contributor at a time. One serialized chain owns the critical path (currently P00 → P01 → P02 → P03 → P03D → P04 → … → B0 acceptance), while additional capacity works in parallel. Two failure modes have already occurred:

- parallel implementation ran ahead of its owning milestone and fixed contracts those owners had not supplied, so it was closed with its branch preserved (PR #41);
- two contributors corrected the same machine-contract defect simultaneously, and the duplicate was closed (PR #45 duplicated merged PR #43 within the hour).

The accepted order (ADR-0012 and the delivery rule) says what ships first. It does not say what parallel capacity may *prepare* while the chain runs, so every parallel contribution currently relies on case-by-case judgement, and useful capacity idles once the plan-parallel lanes (pilot governance, design-contract evidence) are exhausted.

## Constraints and evidence

- The delivery rule is normative: B0 acceptance precedes every A0 task; A0 acceptance precedes provider, mobile, AI, MCP, and theme-package delivery. "Dependencies may establish seams but never transfer acceptance ownership."
- The PR #41 closure established the substantive limits: parallel work must not invent canonical formats, precedence orders, or delivered bytes ahead of the owning milestone, and structural claims must be literally true.
- The PR #43/#45 race established the coordination limit: machine-contract (`spec/`) surgery cannot be safely self-assigned by parallel capacity.
- The closed-but-preserved branch disposition already treats early work as recoverable input rather than waste; this decision formalises that lane instead of leaving it as an exception path.

## Alternatives considered

- **Keep parallel capacity idle until the chain unblocks it** — rejected: it discards capacity and was not the practice even before this decision (pilot governance and design evidence already ran in parallel by explicit selection).
- **Let parallel capacity implement later milestones normally** — rejected: it would move acceptance and contract authority ahead of their owners, which ADR-0012 and the PR #41 disposition explicitly prevent.
- **Whitelist specific tasks one at a time** — rejected: it recreates the case-by-case judgement problem this decision removes; conditions generalise better than lists.

## Decision

Parallel capacity may take unstarted work from the accepted sequence — preferring the latest lanes first, because distance from the active phase is the best predictor of low interference — or any other unclaimed lane, as **preparation**, under all of the following conditions:

1. **Acceptance never moves.** The accepted order is unchanged. No gate closes early, no milestone's evidence is claimed early, and nothing merges into a milestone out of order. Preparation matures on draft pull requests based on `main` that remain draft until their owning milestone opens; the parallel author keeps them rebased and current.
2. **Contract-complete only.** The work must be fully specified by accepted contracts (requirement and UAT language, accepted decisions). Where a contract detail is missing, the preparation records an open question; it does not invent the answer.
3. **No duplicate effort.** Before starting, the parallel contributor re-fetches and inspects open and recently merged pull requests, active worktrees, and the preserved-branch dispositions. Work covered by a preserved branch's disposition is harvested through that disposition, not rebuilt. Work inside the active phase owner's surface is not taken.
4. **Provisional formats.** No file format, schema, wire shape, precedence order, or delivered byte layout becomes canonical through preparation. Such artifacts are labelled provisional in the preparation itself and are reconciled — possibly with breaking changes — when the owning milestone runs. Domain invariants, validation logic, and tests written against accepted contract language are the durable output; formats are the disposable one.
5. **Chain right-of-way.** The serialized-chain owner has right-of-way over the machine contracts (`spec/`) and the active phase surface. Parallel machine-contract changes happen only on an explicit maintainer request.
6. **Same quality bar.** Preparation follows every existing rule: DDD boundaries, bounded-context READMEs, tests, synthetic fixtures, honest structural claims, changelog, and the pull-request template. Draft status is a sequencing statement, not a quality waiver.

Precedence when conflicts appear: active chain contracts, then preserved-branch dispositions, then parallel drafts. A parallel draft invalidated by later canonical contracts is amended or closed-with-branch-preserved without ceremony; that outcome is an accepted cost of the lane, not a defect.

## Consequences

- Later-lane domain logic, validation rules, and test suites mature early and arrive at their milestones reviewed and rebased, while acceptance ordering and gate evidence stay exactly as ADR-0012 and the delivery rule require.
- Some preparation will be reworked or discarded at reconciliation; the lane deliberately trades that risk for capacity utilisation, and the provisional-format rule keeps the rework surface small.
- Maintainer review of preparation drafts is deferred to each draft's owning milestone; the open-draft list becomes part of that milestone's intake.
- The closed-preserved disposition remains available for preparation that drifts out of alignment.

## Migration, rollback, or reversal conditions

No data or code migration is involved. The maintainer reverses this decision by superseding it with a new record; open preparation drafts are then closed with branches preserved under the existing disposition practice.
