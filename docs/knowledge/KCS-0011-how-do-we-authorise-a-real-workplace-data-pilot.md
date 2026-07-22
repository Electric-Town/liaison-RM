---
id: KCS-0011
title: How do we authorise a real workplace-data pilot?
state: Draft
owner: repository-governance
created: 2026-07-19
reviewed: 2026-07-22
applies_to:
  - maintainers
  - pilot host organisations
  - contributors
  - coding agents
search_terms:
  - pilot
  - real data
  - workplace data
  - DPIA
  - data controller
  - legal basis
  - special category
  - dietary data privacy
  - participant notice
  - retention
  - incident
  - independent review
  - FG-B0-PILOT-001
  - LRM-EV-012
related_requirements:
  - LRM-EV-012
related_gates:
  - FG-B0-PILOT-001
---

# KCS-0011: How do we authorise a real workplace-data pilot?

## Problem

B0 development and UAT run entirely on synthetic fixtures, but at some point a real organisation will want to trial the workplace review workflow with real employees' dietary and accessibility data. That data is special-category (health, belief, disability) about people in an employment power imbalance. Requirement `LRM-EV-012` therefore blocks any real-data pilot until governance responsibilities are recorded and independently reviewed; feature gate `FG-B0-PILOT-001` keeps the `real-workplace-data` capability denied.

Contributors need to know what has to exist before a pilot is allowed, who must act, and what an agent may and may not do.

## Environment and preconditions

- The governance record set lives in `docs/pilot/`, with the checklist in its README.
- The identifiers `T-B0-PILOT`, `LRM-EV-012`, and `FG-B0-PILOT-001` are defined in the machine-readable contracts introduced by the P00 contract reconciliation.
- Governance records may be drafted while P01–P11 proceeds, but `T-B0-PILOT`, `LRM-EV-012`, and `FG-B0-PILOT-001` remain deferred until B0 is accepted. Record completion is necessary but cannot authorise a pre-B0 pilot.

## Resolution

1. Read the checklist in `docs/pilot/README.md`. It lists the eight records, their current states, and who must act.
2. Complete the source records in dependency order: data controller and accountable operators → lawful purpose and Article 6 basis → Article 9 special-category condition → DPIA decision (and assessment if required) → retention and rights plan → incident plan.
3. Every decision field needs a value, a named decider, and a date. Use an explicit "not applicable — reason" rather than leaving a field empty; an empty field is an open decision.
4. Complete the participant-notice draft without issuing it and bind its exact content artifact to the source records. Before B0, all of these materials remain drafts; no final independent conclusion or pilot execution evidence may be recorded.
5. After B0 is accepted and pilot work begins, move `T-B0-PILOT` and `LRM-EV-012` to current and `FG-B0-PILOT-001` to blocked. Produce the post-B0 pilot evidence showing that real-data import remains technically denied until this exact governance set is authorised, and that activation is scoped to the reviewed pilot.
6. Record `reviewed_notice_content_sha256` and obtain the independent legal and privacy review in `docs/pilot/independent-review-record.md` over that exact notice content, the source records, B0 receipt, and technical-control evidence. The reviewer must be independent of the pilot's operators and of the records' authors. The final conclusion must be exactly `authorise`, and every condition must be `resolved`; `authorise with conditions`, `do not authorise`, defer, or any unresolved condition keeps real data denied.
7. Issue the exact participant-notice content artifact that received the affirmative review. Record `issued_notice_content_sha256` and prove it equals `reviewed_notice_content_sha256`; a mismatch or content change reopens review. Record issuance evidence before completing the pilot gate.
8. Update the checklist states and `FG-B0-PILOT-001` evidence with the exact governance, notice-issuance, and technical-control artifacts, then complete `T-B0-PILOT` and `FG-B0-PILOT-001` through the machine-owned lifecycle. A drafted, current, or merely activated task or gate is still denied.
9. Only after the task and gate are both `complete` may any person's real data enter the pilot workspace.

## Why it works

The gate turns "we should be careful with real data" into named, dated, blocking evidence. Splitting the records keeps each decision small and reviewable, and the independent review exists because the people building the pilot are the wrong people to certify it. The records also bind the pilot to the product's own boundaries — least disclosure, no scoring, unknown-never-means-none — so a pilot cannot quietly become monitoring.

## Verification

- Every record in `docs/pilot/` shows a completed state with names and dates.
- The independent-review record has a signed, explicit `authorise` conclusion and zero unresolved conditions; every other conclusion is a denial.
- B0 is accepted on its exact review artifact before the separate PILOT milestone begins.
- Post-B0 technical evidence proves that real-data import is denied before pilot authorisation and enabled only for the reviewed scope.
- `T-B0-PILOT` and `FG-B0-PILOT-001` are both `complete`, not merely drafted, current, or activated.
- The feature-gate evidence for `FG-B0-PILOT-001` cites the exact commits and artifact identities for the completed governance set and technical control.
- The issued participant notice matches the completed records.

## Recovery or rollback

- Any material change (new data class, operator change, purpose change, incident, scope growth) reopens the independent-review record, returns `T-B0-PILOT` and `LRM-EV-012` to current, returns `FG-B0-PILOT-001` to blocked, and returns the `real-workplace-data` capability to denied until the changed scope is affirmatively reviewed and requalified.
- A participant's withdrawal triggers the deletion path in the retention and rights plan.
- At pilot end, execute and verify the pilot-end deletion evidence in the retention plan.
- If an incident occurs, follow `docs/pilot/incident-plan.md`; report product defects with synthetic data only.

## Known limitations

- The records structure the decisions; they are not legal advice. Adequacy for the controller's jurisdiction is exactly what the independent review confirms.
- The software-side denial of real-data import is a separate post-B0 PILOT control. B0 proves only the synthetic product boundary; completed records do not substitute for pilot-specific technical denial and authorisation evidence.
- Coding agents and contributors may improve record structure and drafts but cannot make controller decisions, issue notices, or perform the independent review.

## Related decisions, tests, and articles

- `docs/pilot/` — the record set and checklist.
- `SPEC.md` sections 6.1 (dietary model) and 9 (facilities limits); `AGENTS.md` product boundary.
- `docs/security/threat-model.md` — general threat context the DPIA can cite.
- KCS-0008 (zero-orphan product contracts) — how ownership of `LRM-EV-012` and `FG-B0-PILOT-001` is recorded in `spec/traceability-ownership.json`.
