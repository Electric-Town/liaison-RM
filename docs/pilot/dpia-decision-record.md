# DPIA decision record

- Record: DPIA decision
- Gate evidence for: `FG-B0-PILOT-001`
- Record state: **open — screening not answered, decision not made**
- Last updated: 2026-07-19

This record holds the controller's decision on whether the pilot requires a data protection impact assessment under Article 35, and the assessment itself if required. The decision must be made and dated even if the answer is "not required" — an absent decision is not a negative decision.

## Screening

Answer each question for the actual pilot. "Yes" answers weigh towards a DPIA being required; special-category data about employees will usually make one the prudent choice.

| Question | Answer | State |
|---|---|---|
| Does the pilot process special-category data (health, belief, disability)? | Expected: yes | Open |
| Are data subjects in a dependency relationship with the controller (employees)? | Expected: yes | Open |
| Is processing systematic (ongoing coverage records rather than one-off)? | — | Open |
| What is the scale (number of participants, sites, events)? | — | Open |
| Is any evaluation, scoring, or profiling performed? | Expected: no — structurally excluded | Open |
| Is any automated decision-making with legal or similar effect performed? | Expected: no | Open |
| Is data transferred outside the controller's estate? | Expected: no — local-authoritative storage | Open |
| Does the supervisory authority's DPIA blacklist cover this processing? | — | Open |

## Decision

| Field | Value | State |
|---|---|---|
| DPIA required? | — | Open |
| Rationale | — | Open |
| Decided by (name, role) | — | Open |
| Date | — | Open |

## Assessment (complete if required)

Each section must be filled against the actual pilot configuration, not the product's general documentation. The systematic description can cite [SPEC.md](../../SPEC.md) and [docs/security/threat-model.md](../security/threat-model.md) but must state what this pilot concretely does.

### 1. Systematic description

- Processing operations and their purpose: —
- Data classes and data subjects: —
- Storage location, devices, and any backups: —
- Recipients of the least-disclosure brief: —
- Retention: reference [retention-and-rights-plan.md](retention-and-rights-plan.md)

### 2. Necessity and proportionality

- Why each data class is necessary for the recorded purpose: —
- Why the least-disclosure design is the minimum viable disclosure: —
- Legal basis and special-category condition: reference the completed records

### 3. Risks to individuals

Assess likelihood and severity for at least:

- disclosure of belief or health information beyond catering need;
- inference of belief from an operational instruction by brief recipients;
- device loss or theft exposing the workspace files;
- unauthorised operator access or purpose creep towards monitoring;
- inaccurate coverage data leading to a harmful catering decision (allergy);
- pressure on employees to participate or disclose.

| Risk | Likelihood | Severity | Assessment state |
|---|---|---|---|
| — | — | — | Open |

### 4. Mitigations

Candidate mitigations to confirm or extend: named-operator access only, least-disclosure briefs without names, explicit information states preventing unknown-as-none, purpose limits with no-scoring rules, device encryption, pilot-bounded retention with verified deletion, incident plan, participant withdrawal path.

### 5. Residual risk and sign-off

| Field | Value | State |
|---|---|---|
| Residual risk acceptable? | — | Open |
| Prior consultation with the supervisory authority needed? | — | Open |
| Signed off by (name, role, date) | — | Open |

## Completion

This record is complete when the screening is answered, the decision is dated, and — if a DPIA is required — the assessment is signed off. The independent review checks this record's adequacy, not merely its presence.
