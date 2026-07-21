# P03D design consultation record

Date: 2026-07-19
Work package: `T-B0-P03D`
Status: consultation complete; exact-head repository validation required before task completion
Facilitator and accountable product owner: Ryan until delegated
Selected direction: Editorial Ledger
Canonical output: `DESIGN.md` version 1.0.0

## Question

What presentation and interaction contract lets Liaison deliver the complete B0 workplace event-readiness journey while preserving local authority, recoverable operations, accessibility, localization, and a calm human character?

## Inputs reviewed

- merged P00 product truth, ADRs, requirements, UAT, and gate ownership;
- P01 application composition root;
- P02 Workspace Session and exact-head platform evidence;
- P03 recoverable-operation states and failure boundaries;
- the accepted Editorial Ledger palette, local font provenance, and contrast evidence;
- the candidate five-stage Events design contract;
- the current installed review-shell defects and parity tests;
- receptionist, events-manager, elder-user, workplace-owner, and recovery personas;
- WCAG 2.2 AA target, relevant EN 301 549 requirements, Nielsen heuristics, Gestalt grouping, AskTog interaction principles, and interruption-recovery needs;
- `en-XA`, Irish, Japanese, and Brazilian Portuguese localization requirements.

## Alternatives considered

### Keep the vanilla HTML/CSS/JavaScript shell

Rejected as P04 authority. It proved Tauri packaging and current workflows, but it lacks typed component contracts, generated DTOs, scalable state composition, route-level boundaries, and a maintainable accessibility/localization ratchet.

### Adopt a generic enterprise dashboard

Rejected. KPI mosaics and dense navigation obscure the reconciled denominator and make an event-readiness system look like sales or employee analytics.

### Adopt a highly irregular hand-drawn interface

Rejected for operational surfaces. Paper texture and a tangible ledger character support the product, but rotated controls, wobbly data tables, decorative handwriting, and moving targets harm scanning, localization, zoom, focus predictability, and error recovery. The accepted direction keeps the human paper character without making interaction geometry unpredictable.

### Ship a single light theme

Rejected by `LRM-UX-012` and `UAT-062`. B0 requires system, light, dark, and high-contrast built-ins through one semantic contract.

### Let each route define its own state styling

Rejected. State meaning, operation recovery, dietary outcomes, and focus behavior would drift across Directory and Events.

## Decisions

| ID | Decision |
|---|---|
| D-P03D-01 | `DESIGN.md` is the canonical presentation contract from P04 through P11. |
| D-P03D-02 | P04 uses React and TypeScript inside the existing Tauri shell and calls only `liaison-application` commands. |
| D-P03D-03 | Editorial Ledger is selected: warm canvas, flat work surfaces, restrained hard emphasis, bundled accessible fonts, no remote assets. |
| D-P03D-04 | Operational geometry remains stable. Texture and character do not rotate, overlap, or animate controls, tables, forms, dialogs, or status regions. |
| D-P03D-05 | Overview, Directory, Events, Health, and Settings are the B0 route identities. |
| D-P03D-06 | Events uses Details, Cohort, Attendees, Readiness, and Brief with `aria-current="step"`. |
| D-P03D-07 | Exact reconciled sentences and semantic tables replace KPI mosaics and readiness percentages. |
| D-P03D-08 | The canonical semantic-token registry is `design/semantic-tokens.v1.json`; system is a resolution rule. |
| D-P03D-09 | `content-on-highlight` and `current-step-content` are mandatory independent roles. |
| D-P03D-10 | Essential boundaries use `border-strong` or multiple non-colour cues; routine low-contrast borders are decorative only. |
| D-P03D-11 | P03 operation states map through one `OperationStatus` contract. Before COMMIT, Cancel changes no canonical file; after COMMIT, Cancel is absent and recovery rolls forward. |
| D-P03D-12 | The frontend never persists or derives a parallel dietary state, cohort denominator, maintenance status, or Review Priority. |
| D-P03D-13 | Directory is a semantic table/summary-detail workspace, not an avatar card grid. |
| D-P03D-14 | At 400% zoom the primary task needs no horizontal scrolling; 320 CSS pixels is a required reflow condition. |
| D-P03D-15 | All natural-language strings use locale keys; en-XA expansion is a merge gate. |
| D-P03D-16 | P04 proves parity with the current shell before deleting it. |
| D-P03D-17 | Theme preview is reversible; failed persistence restores the last saved built-in. |
| D-P03D-18 | Screenshots are visual evidence only. Exact installed builds own final accessibility, persistence, recovery, and platform claims. |

## Persona walkthrough findings

### Workplace receptionist

The operator needs one denominator, explicit unknowns, safe filtering, and a clear way back to the attendee source. The chosen design keeps counts in one reconciled sentence and one table, with evidence in a focus-managed drawer.

### Events manager

Generation and delivery must be separate. The Brief stage shows byte-exact output, identifier mode, recipient, purpose, expiry, source revisions, and separate receipts.

### Elder and low-vision user

Large stable targets, predictable layout, strong focus, text-labelled states, 400% reflow, and high contrast outrank visual novelty. The selected direction retains character without moving or rotating interaction targets.

### Neurodivergent operator

One stage at a time, explicit saved/recovery state, bounded choices, no hidden autosave, and return-to-stage behavior reduce working-memory demand. The interface avoids permanent red backlog counters and noisy dashboards.

### Local workspace owner

The UI distinguishes local authority, operation phase, recovery state, key state, projection state, and provider state. A generic green “safe” badge is insufficient.

## Evidence reviewed

- Candidate token validator reproduces the rejected dark highlight pair at 1.77:1 and measures the selected content-on-highlight pair at 8.12:1.
- Current shell browser evidence demonstrates zero external requests, local fonts, reduced motion, and narrow reflow, but does not satisfy installed P04/P11 evidence.
- P03 contracts provide the exact pre-commit, committed, complete, discarded, and recovery-conflict meanings adopted in `DESIGN.md`.

## Outcome

The consultation selects Editorial Ledger and approves `DESIGN.md` 1.0.0 plus `design/semantic-tokens.v1.json` as the inputs to P04. No P04 implementation or B0 acceptance claim is made by this record.
