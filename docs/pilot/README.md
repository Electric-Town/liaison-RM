# Real workplace-data pilot governance

- Owning task: `T-B0-PILOT` (introduced by the P00 contract reconciliation)
- Owning requirement: `LRM-EV-012`
- Owning feature gate: `FG-B0-PILOT-001`
- Gate state: **closed — no real workplace-data pilot is authorised**
- Last updated: 2026-07-19

This directory is the governance record set that must be complete, dated, and independently reviewed before any real workplace data enters a Liaison RM pilot. It exists so that authorisation is a recorded decision with named accountability, not an assumption.

Synthetic-fixture development and UAT continue unaffected. Only real data is blocked.

## What counts as real workplace data

Any information about identifiable people collected from or about an actual workplace, including:

- employee dietary requirements and coverage states;
- accessibility and sensory needs;
- email or communication metadata;
- facilities and access records;
- Working With Me and similar profile content;
- names, contact details, memberships, and event attendance drawn from a real organisation.

Synthetic fixtures with invented people are not real workplace data and remain the required basis for development, tests, screenshots, and UAT.

## Governance checklist

The gate requires every record below to be complete. A record is complete when its required decisions are made, dated, and attributed to a named person. No record in this directory is complete yet.

| Record | File | State | Who must act |
|---|---|---|---|
| Data controller and accountable operators | [data-controller-record.md](data-controller-record.md) | Open | Pilot host organisation |
| Lawful purpose and legal basis | [lawful-purpose-record.md](lawful-purpose-record.md) | Open | Data controller |
| Special-category condition | [special-category-condition.md](special-category-condition.md) | Open | Data controller |
| DPIA decision | [dpia-decision-record.md](dpia-decision-record.md) | Open | Data controller |
| Participant notice | [participant-notice.md](participant-notice.md) | Open — draft text prepared | Data controller |
| Retention and rights plan | [retention-and-rights-plan.md](retention-and-rights-plan.md) | Open — structure prepared | Data controller |
| Incident response plan | [incident-plan.md](incident-plan.md) | Open — structure prepared | Data controller and accountable operators |
| Independent legal and privacy review | [independent-review-record.md](independent-review-record.md) | Open — not started | Independent reviewer |

## Rules

1. **The gate closes only on human decisions.** Repository contributors and coding agents may prepare and improve these records, but they cannot make the controller decisions, issue the notice, or perform the independent review. An unfilled field is an open decision, never an implied approval.
2. **Unresolved conditions block the pilot.** If the independent review records any unresolved condition, the pilot remains blocked regardless of the other records.
3. **Dates and names are evidence.** Every decision field requires a date and a named decider. "Approved" without both is not evidence.
4. **The software denial is a separate control.** `FG-B0-PILOT-001` blocks the `real-workplace-data` capability. The technical denial of real-data import belongs to the import and event surfaces when they are implemented, and is verified at B0 acceptance. Nothing in this directory claims that the current application enforces the denial; the current safeguard is that these records gate authorisation and the pilot does not begin.
5. **Records describe this pilot, not the product's compliance.** Completing this set authorises one bounded pilot. It is not a product certification claim and must not be cited as one.
6. **Later pilots copy, not reuse.** A subsequent pilot copies this record set into a dated subdirectory and completes it independently. Authorisation does not carry over.

## Sequence

The intended completion order, reflecting which decisions depend on which:

1. Record the data controller, jurisdiction, and accountable operators.
2. Record the purpose, purpose limits, and Article 6 legal basis.
3. Record the special-category condition for dietary and accessibility data.
4. Complete the DPIA screening and, if required, the assessment.
5. Complete and issue the participant notice.
6. Adopt the retention schedule and rights procedures.
7. Adopt the incident response plan.
8. Obtain the dated, scoped independent review.
9. Update this checklist and the feature-gate evidence, and record the exact commit.

## Relationship to repository rules

These records operationalise existing repository positions: the product boundary in [AGENTS.md](../../AGENTS.md) (no productivity, attendance-compliance, performance, or risk scoring), the dietary and facilities constraints in [SPEC.md](../../SPEC.md), the threat model in [docs/security/threat-model.md](../security/threat-model.md), and the delivery boundaries in [docs/product/working-state-delivery.md](../product/working-state-delivery.md). The requirement `LRM-EV-012`, the task `T-B0-PILOT`, and the gate `FG-B0-PILOT-001` are defined in [spec/requirements.json](../../spec/requirements.json), [spec/implementation-plan.yaml](../../spec/implementation-plan.yaml), and [spec/feature-gates.yaml](../../spec/feature-gates.yaml), with ownership recorded in [spec/traceability-ownership.json](../../spec/traceability-ownership.json).
