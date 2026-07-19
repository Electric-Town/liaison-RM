# Special-category condition record

- Record: special-category condition
- Gate evidence for: `FG-B0-PILOT-001`
- Record state: **open — required decisions are not made**
- Last updated: 2026-07-19

Pilot data is not ordinary contact data. This record establishes why special-category rules apply and which Article 9 condition the controller relies on.

## Why the pilot handles special-category data

- An allergy, intolerance, or medical dietary restriction is **health data**.
- A religious dietary restriction reveals **religious belief**; an ethical pattern such as veganism can reveal **philosophical belief**.
- Accessibility and sensory needs are **health or disability data**.

The product's least-disclosure design reduces exposure but does not change the data's character:

- B0 stores an operational instruction (for example, "no gluten; separate preparation required"), not diagnoses, medical history, or diagnostic narrative — storing those is structurally excluded from B0;
- briefs group instructions and omit names by default;
- an empty dietary field means unknown, never "no restriction", so the pilot cannot silently invent a "verified none".

Even a bare operational instruction like "halal; no alcohol in preparation" reveals belief, so the condition below is still required.

## Article 9 condition

The controller must choose and record the condition. Points the decision must address:

- **Explicit consent (9(2)(a))** — the usual fit for a voluntary pilot, with the same employment-context caveats as Article 6 consent: genuinely voluntary, refusal without consequence, easy withdrawal, and consent captured per person before their dietary or accessibility data is entered.
- **Employment, social security and social protection law (9(2)(b))** — only if the controller identifies a specific legal obligation (for example, a duty to accommodate) that actually covers catering data; must be named, not assumed.
- Other conditions are unlikely to fit and, if claimed, need specific justification.

A participant who declines is recorded with the product's explicit `declined` state and is excluded from dietary processing; catering for them follows the controller's default provision. Declining must not disadvantage them.

| Field | Value | State |
|---|---|---|
| Chosen Article 9 condition | — | Open |
| How and where explicit consent (or the named legal obligation) is evidenced | — | Open |
| Per-person consent state tracked where in the workspace | — | Open |
| Handling for declined participants confirmed | — | Open |

## Minimisation commitments

Adopted as pilot rules, matching `SPEC.md` section 6.1:

- record the coverage state and operational instruction only; no diagnoses or medical detail;
- stricter classification for any optional detailed note, and no such note enters a brief;
- dietary and accessibility fields are visible only to named accountable operators;
- exports beyond the least-disclosure brief are out of scope for the pilot.

| Field | Value | State |
|---|---|---|
| Minimisation commitments adopted? | — | Open |

## Completion

| Field | Value |
|---|---|
| Recorded by | — |
| Date | — |
| Controller confirmation (name, role) | — |
