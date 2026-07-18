# Review and Attention bounded context

## Purpose

Review and Attention helps a user decide where attention may be useful without treating people as leads or converting communication frequency into a relationship-value score.

## Separate concepts

- **Relationship intent:** user-authored importance, desired cadence, boundaries, and desired future state.
- **Relationship evidence:** interactions, dates, commitments, notes, events, and Resources.
- **Maintenance status:** an explainable state relative to the user’s rules.
- **Profile readiness:** purpose-specific coverage of required information.
- **Review Priority:** optional queue ordering. It is not relationship strength or human value.

## Default mode

Personal workspaces default to **reason-only** review. The queue groups people by factual reasons and displays no numeric score. Typical reasons are an open commitment, an approaching important date, a cadence becoming due, a selected readiness gap, or a manual pin.

Tiered and weighted modes are opt-in. A weighted policy exposes every component, weight, normalized input, suppression, policy version, and final explanation.

## Hard suppressions

The following states override every queue score:

- archived;
- do not contact;
- relationship ended;
- paused until a future date;
- snoozed;
- excluded from the active policy.

## Guardrails

The context cannot:

- infer affection, trust, reciprocity, or importance from message volume;
- rank employees or support employment decisions;
- become a social-credit system;
- shame a user for overdue contact;
- send a message automatically;
- expose private assessments to a subject or shared workspace without an explicit disclosure decision;
- assume every relationship needs a cadence.

## Explainability contract

Every queue item returns a structured explanation containing the reason ID, human-readable explanation key, source fact IDs, contribution where applicable, active policy version, and available actions. Interfaces localize the explanation but cannot invent or omit reasons.

Example:

```text
Quarterly cadence is 18 days overdue.
One commitment remains open.
Birthday is in 11 days.
```

Bad output:

```text
Relationship strength: 42%.
```

## Capacity and interruption recovery

A review policy defines a bounded batch, time budget, social-capacity setting, quiet periods, and permitted contexts. The application stores unfinished review state locally and returns to the same item after interruption. Skip, snooze, pause, archive, and do-not-contact are valid outcomes.

## Published inputs

The context consumes stable IDs and read-only facts from Identity and Profiles, Organizations and Groups, Relationships, Interactions and Commitments, Events and Calendar, and Knowledge and Resources. It does not access their persistence implementations.

## Outputs

- reason-only queue;
- tiered or weighted queue when enabled;
- maintenance-status explanation;
- purpose-specific readiness report;
- daily review session;
- monthly Markdown review record;
- policy simulation diff;
- audit-safe policy and explanation metadata.
