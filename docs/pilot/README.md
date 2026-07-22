# Real workplace-data pilot governance

- Owning task: `T-B0-PILOT` (introduced by the P00 contract reconciliation)
- Owning requirement: `LRM-EV-012`
- Owning feature gate: `FG-B0-PILOT-001`
- Machine state: **deferred until after B0; the `real-workplace-data` capability remains denied unless and until `T-B0-PILOT` and `FG-B0-PILOT-001` are both complete**
- Last updated: 2026-07-22

This directory is the governance record set that must be complete, dated, and independently reviewed before any real workplace data enters a Liaison RM pilot. It exists so that authorisation is a recorded decision with named accountability, not an assumption. Records may be drafted before B0, but the separate PILOT task, requirement, and gate remain deferred until B0 is accepted.

Synthetic-fixture development and UAT continue unaffected. Before B0 acceptance, `T-B0-PILOT`, `LRM-EV-012`, and `FG-B0-PILOT-001` remain deferred and the `real-workplace-data` capability remains denied.

After B0 acceptance, starting pilot work makes `T-B0-PILOT` and `LRM-EV-012` current while `FG-B0-PILOT-001` is blocked; the capability remains denied. It may be enabled only after an independent final conclusion of exactly `authorise`, every review condition is resolved, the reviewed notice is issued unchanged, and both the task and gate are complete. A later material change returns the task and requirement to current, the gate to blocked, and the capability to denied until the changed scope is affirmatively reviewed and requalified.

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

1. **Human decisions are necessary but do not complete the gate.** Repository contributors and coding agents may prepare and improve these records, but they cannot make the controller decisions, issue the notice, or perform the independent review. An unfilled field is an open decision, never an implied approval; governance decisions do not replace the technical evidence or machine-owned task and gate lifecycle.
2. **Only an affirmative final review qualifies.** The final independent-review conclusion must be exactly `authorise` and every condition must be `resolved`. `Authorise with conditions`, `do not authorise`, defer, an unresolved condition, or no conclusion keeps real data denied.
3. **Dates and names are evidence.** Every decision field requires a date and a named decider. "Approved" without both is not evidence.
4. **The software denial is a separate post-B0 control.** When pilot work begins after B0, `T-B0-PILOT` and `LRM-EV-012` become current, `FG-B0-PILOT-001` becomes blocked, and the `real-workplace-data` capability remains denied. Pilot evidence must prove real-data import remains denied until the reviewed scope is authorised. B0 acceptance and these records alone do not establish that technical control.
5. **Records are necessary, not sufficient.** Completing this set does not authorise PILOT. B0 must first be accepted, the post-B0 denial and scoped authorisation must be evidenced, and both `T-B0-PILOT` and `FG-B0-PILOT-001` must be `complete`. Drafted, current, or merely activated states remain denied.
6. **Records describe this pilot, not the product's compliance.** Completing this set contributes to authorising one bounded pilot. It is not a product certification claim and must not be cited as one.
7. **Later pilots copy, not reuse.** A subsequent pilot copies this record set into a dated subdirectory and completes it independently. Authorisation does not carry over.
8. **Material change removes authorisation.** A new data class, operator, purpose, incident, or scope expansion returns `T-B0-PILOT` and `LRM-EV-012` to current, `FG-B0-PILOT-001` to blocked, and the capability to denied until the changed scope receives a new affirmative review and requalification.

## Sequence

The intended completion order, reflecting which decisions depend on which:

1. Record the data controller, jurisdiction, and accountable operators.
2. Record the purpose, purpose limits, and Article 6 legal basis.
3. Record the special-category condition for dietary and accessibility data.
4. Complete the DPIA screening and, if required, the assessment.
5. Adopt the retention schedule and rights procedures.
6. Adopt the incident response plan.
7. Complete the participant-notice draft without issuing it, create its immutable content artifact, and bind it to the source-record drafts. Before B0, every governance artifact remains a draft; no final independent conclusion or pilot execution evidence may be recorded.
8. After B0 is accepted and pilot work begins, record `T-B0-PILOT` and `LRM-EV-012` as current and `FG-B0-PILOT-001` as blocked; prove that the capability remains denied and that any later authorisation is limited to the reviewed scope.
9. Record `reviewed_notice_content_sha256` and obtain the dated, scoped independent review of that exact notice content artifact, digest, source-record set, B0 receipt, and technical-control evidence; the final conclusion must be `authorise` and every condition must be resolved.
10. Issue the exact reviewed participant-notice content artifact, record `issued_notice_content_sha256`, and prove it equals `reviewed_notice_content_sha256`; a mismatch or material content change reopens review.
11. Update this checklist and the feature-gate evidence with the exact governance commits, notice-issuance evidence, and technical-control artifact identities. Complete `T-B0-PILOT` and `FG-B0-PILOT-001`; only then may real data enter the pilot workspace.

## Relationship to repository rules

These records operationalise existing repository positions: the product boundary in [AGENTS.md](../../AGENTS.md) (no productivity, attendance-compliance, performance, or risk scoring), the dietary and facilities constraints in [SPEC.md](../../SPEC.md), the threat model in [docs/security/threat-model.md](../security/threat-model.md), and the delivery boundaries in [docs/product/working-state-delivery.md](../product/working-state-delivery.md). The requirement `LRM-EV-012`, the task `T-B0-PILOT`, and the gate `FG-B0-PILOT-001` are defined in [spec/requirements.json](../../spec/requirements.json), [spec/implementation-plan.yaml](../../spec/implementation-plan.yaml), and [spec/feature-gates.yaml](../../spec/feature-gates.yaml), with ownership recorded in [spec/traceability-ownership.json](../../spec/traceability-ownership.json).
