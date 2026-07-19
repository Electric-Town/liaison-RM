---
id: KCS-0011
title: How do we authorise a real workplace-data pilot?
state: Draft
owner: repository-governance
created: 2026-07-19
reviewed: 2026-07-19
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

B0 development and UAT run entirely on synthetic fixtures, but at some point a real organisation will want to trial the workplace review workflow with real employees' dietary and accessibility data. That data is special-category (health, belief, disability) about people in an employment power imbalance. Requirement `LRM-EV-012` therefore blocks any real-data pilot until governance responsibilities are recorded and independently reviewed; feature gate `FG-B0-PILOT-001` holds the `real-workplace-data` capability closed.

Contributors need to know what has to exist before a pilot is allowed, who must act, and what an agent may and may not do.

## Environment and preconditions

- The governance record set lives in `docs/pilot/`, with the checklist in its README.
- The identifiers `T-B0-PILOT`, `LRM-EV-012`, and `FG-B0-PILOT-001` are defined in the machine-readable contracts introduced by the P00 contract reconciliation.
- No installed-application capability is required to complete the records; this path is deliberately parallel to the P01–P11 implementation chain.

## Resolution

1. Read the checklist in `docs/pilot/README.md`. It lists the eight records, their current states, and who must act.
2. Complete the records in dependency order: data controller and accountable operators → lawful purpose and Article 6 basis → Article 9 special-category condition → DPIA decision (and assessment if required) → participant notice → retention and rights plan → incident plan.
3. Every decision field needs a value, a named decider, and a date. Use an explicit "not applicable — reason" rather than leaving a field empty; an empty field is an open decision.
4. Obtain the independent legal and privacy review in `docs/pilot/independent-review-record.md`. The reviewer must be independent of the pilot's operators and of the records' authors, and every condition they set must reach `resolved`.
5. Update the checklist states, record the exact commit in the feature-gate evidence, and only then allow real data into the pilot workspace.
6. Issue the completed participant notice before entering any person's real data.

## Why it works

The gate turns "we should be careful with real data" into named, dated, blocking evidence. Splitting the records keeps each decision small and reviewable, and the independent review exists because the people building the pilot are the wrong people to certify it. The records also bind the pilot to the product's own boundaries — least disclosure, no scoring, unknown-never-means-none — so a pilot cannot quietly become monitoring.

## Verification

- Every record in `docs/pilot/` shows a completed state with names and dates.
- The independent-review record has a signed conclusion and zero unresolved conditions.
- The feature-gate evidence for `FG-B0-PILOT-001` cites the exact commit of the completed set.
- The issued participant notice matches the completed records.

## Recovery or rollback

- Any material change (new data class, operator change, purpose change, incident, scope growth) reopens the independent-review record and returns the gate to closed.
- A participant's withdrawal triggers the deletion path in the retention and rights plan.
- At pilot end, execute and verify the pilot-end deletion evidence in the retention plan.
- If an incident occurs, follow `docs/pilot/incident-plan.md`; report product defects with synthetic data only.

## Known limitations

- The records structure the decisions; they are not legal advice. Adequacy for the controller's jurisdiction is exactly what the independent review confirms.
- The software-side denial of real-data import is a separate technical control owned by the import and event surfaces and verified at B0 acceptance; while those surfaces are unimplemented, the only control is that authorisation is withheld.
- Coding agents and contributors may improve record structure and drafts but cannot make controller decisions, issue notices, or perform the independent review.

## Related decisions, tests, and articles

- `docs/pilot/` — the record set and checklist.
- `SPEC.md` sections 6.1 (dietary model) and 9 (facilities limits); `AGENTS.md` product boundary.
- `docs/security/threat-model.md` — general threat context the DPIA can cite.
- KCS-0008 (zero-orphan product contracts) — how ownership of `LRM-EV-012` and `FG-B0-PILOT-001` is recorded in `spec/traceability-ownership.json`.
