# B0 Events design contract — candidate

Status: candidate design-contract evidence for `T-B0-P03D`; no production usability or accessibility conformance claim.

This document specifies the five B0 Event stages — Details, Cohort, Attendees, Readiness, Brief — as screen-state contracts against the merged P00 machine contracts. It exists so the future `T-B0-P03D` reconciliation and the P10/P11 implementation start from recorded state coverage instead of inventing it during coding.

What this document is not:

- It is not canonical `DESIGN.md`, and it is never promoted into one. After exact-head P03 evidence, the `T-B0-P03D` design consultation creates canonical `DESIGN.md` with this file as one of its inputs; the plan design review then checks that direction and amends the P04 plan.
- It is not P04 or P11 implementation authority, and it does not authorise any Events surface to ship.
- It does not bind exact operation, error, or recovery states; those belong to P03 and are mapped at the P03D reconciliation.
- It does not amend machine contracts, gates, or task ownership.

Layering rule: Event presentation remains a design contract against P00 requirements until P05/P10 finalise Event DTOs and errors. Any added, removed, merged, or semantically changed Event state triggers a documented delta before P11 implementation and does not reopen unrelated shell aesthetics.

## Sources

- [ADR-0010: Deliver event dietary readiness with structural least disclosure](../../decisions/0010-deliver-event-dietary-readiness-with-structural-least-disclosure.md)
- `LRM-EV-001` – `LRM-EV-013` in [spec/requirements.json](../../../spec/requirements.json); `UAT-009` – `UAT-012` in [spec/uat-cases.json](../../../spec/uat-cases.json)
- [docs/product/working-state-delivery.md](../../product/working-state-delivery.md) — B0 boundary and claim language
- Personas `P-RECEPTION` and `P-EVENTS`; the conditional plan design review of 2026-07-19 names the receptionist persona "Jess" and its decisions D-P03D-01 to D-P03D-16 are directional inputs pending P03D reconciliation
- The preserved branch `claude/events-readiness-domain` (pull request #41, closed pending its owning P05/P10 milestones) prototypes the same canonical axes and outcomes; requirement language is the authority wherever they differ

All names, organisations, and values below are synthetic.

## The B0 journey

The Event stages sit inside this complete storyboard. The intended emotional outcome is calm confidence: every attendee is accounted for, and the operator can prove why. The interface stays calm and ordinary; nothing simulates magic, scores people, or ranks relationships.

| # | Step | Surface | Outcome the operator can verify |
|---|---|---|---|
| 1 | First launch | Onboarding (pre-shell) | Sees what the application will and will not do before any data exists |
| 2 | Create, open, or recover workspace | Onboarding and recovery | Knows where canonical files live and that recovery is possible |
| 3 | Import or enter Directory records | Directory | Every imported row is accepted, quarantined, or visibly rejected |
| 4 | Reconcile identities | Directory | No destructive merge; duplicates are superseded with history |
| 5 | Define Event Details | Events — Details | A named, dated event draft exists and survives interruption |
| 6 | Construct the Cohort | Events — Cohort | Selection is previewed with exact counts before finalisation |
| 7 | Resolve Attendees | Events — Attendees | The active denominator reconciles after every correction |
| 8 | Reconcile Readiness | Events — Readiness | Every attendee shows exactly one explicit outcome with evidence |
| 9 | Preview the Brief | Events — Brief | The preview is exactly what will be emitted, byte for byte |
| 10 | Generate the internal artifact | Events — Brief | A sealed immutable brief with checksum and input revisions exists |
| 11 | Deliver externally | Events — Brief | Delivery is a separate explicit operation with its own evidence |
| 12 | Retain the receipt | Events — Brief | The receipt records what was disclosed, to whom, and why |
| 13 | Relaunch or resume | All stages | Interrupted work resumes at the named stage without loss |

## Shell context assumed, not owned here

- Navigation is capability-honest: the Events destination ID and order are reserved by the shell contract, but the destination is not visible until P11 delivers the working workflow. No disabled, placeholder, or "coming soon" navigation exists.
- Below 760 px the shell uses a visible labelled Sections control; Event stages inherit that pattern and never introduce icon-only navigation, a horizontal five-item scroller, or a bottom bar.
- This document owns only the presentation contract of the five Event stages and their states.

## Canonical language and the presenter table

The interface consumes canonical values — availability, freshness, conflict, and disclosure facts, the attendee lifecycle, and the derived outcome — and never persists or computes a parallel readiness state. Visible labels are copy only, mapped one-to-one by a versioned presenter table. The canonical value, decision-table version, and policy identifier remain inspectable in the evidence drawer and in receipts.

Dietary-outcome presenter table, version `presenter-b0-0.1-candidate`, covering exactly the eight derived outcomes ADR-0010 names (labels are en-IE source copy and localisable; canonical values are stable):

| Canonical value | Visible label | One-line meaning shown on demand |
|---|---|---|
| `verified_none` | No dietary requirements — verified | This person confirmed they have no dietary requirements |
| `provided` | Instruction recorded | A catering instruction is recorded and current |
| `pending` | Asked — awaiting reply | The person was asked and has not yet answered |
| `declined` | Declined to share | The person chose not to disclose; default provision applies |
| `unreachable` | Could not be reached | Contact was attempted and failed |
| `excluded` | Catering use not permitted | Disclosure for catering is not permitted for this person |
| `conflicting` | Sources disagree | Two active sources conflict and need resolution |
| `unknown` | Unknown — needs attention | Nothing usable is recorded; this never means "no requirements" |

Attendee-lifecycle presenter (separate mapping — unresolved identity is a lifecycle and denominator state, not a dietary-readiness outcome):

| Canonical lifecycle state | Visible label | One-line meaning shown on demand |
|---|---|---|
| unresolved identity | Identity not confirmed | This attendee row is not yet matched to one person |

Staleness is P10-bound: it derives from verified time, purpose policy, and source revision, and whether it is a final P10 outcome or an orthogonal presenter modifier stays open until P10's exact derived-state contract exists. The "Needs re-checking" copy used in the Readiness stage below is a copy placeholder for whichever form P10 fixes, not a ninth canonical outcome.

Rules:

- one label per canonical value and one canonical value per label, within each presenter; the table version appears wherever outcomes are shown in evidence;
- `unknown` and `verified_none` labels must never be visually interchangeable;
- outcome chips are atomic: one chip communicates one state, with a text label in the accessibility tree; colour alone never carries the distinction;
- prototype wording with no canonical P10 value remains copy only and never becomes a persisted or computed state.

## Stage contracts

Every stage uses the Editorial Ledger pattern: exact counts appear first as one reconciled sentence and a semantic table, never a KPI-card mosaic. Each stage names its current position with `aria-current="step"` in a visible stepper: Details → Cohort → Attendees → Readiness → Brief. Stages the event has not reached are labelled "not started" and are not interactive. Reduced motion removes all non-functional animation.

State-table columns: **State** — when it applies; **The operator sees** — visible behaviour; **Actions** — permitted next steps; **Announcement and focus** — assistive behaviour.

### Stage 1 — Details

Purpose: a named, dated event exists as a draft the operator can leave and resume.

```text
┌ Events ────────────────────────────────────────────────┐
│ Prepare event                                          │
│ [Details]─[Cohort]─[Attendees]─[Readiness]─[Brief]     │
│                                                        │
│ Event name      [ Summer all-hands lunch          ]    │
│ Event date      [ 2026-08-14                      ]    │
│ Location label  [ Dublin canteen (optional)       ]    │
│                                                        │
│ [P03-bound saved-state line] · Continue to Cohort →    │
└────────────────────────────────────────────────────────┘
```

| State | The operator sees | Actions | Announcement and focus |
|---|---|---|---|
| Empty (no event exists) | "Prepare the first Event" entry point with a one-sentence explanation | Create event | Focus moves to the name field on entry |
| Draft | Fields with a P03-bound saved-state line; its exact wording and promise bind only to proven P03 states | Edit; continue; leave | Status line is a polite live region; no focus theft |
| Invalid | Field-level message: name required, or date not a real date | Correct the field | Error text is programmatically associated; focus moves to the first invalid field on submit |
| Interrupted, then reopened | "Resume: Summer all-hands lunch — Details" with the draft intact | Resume; discard draft (confirm) | Resume control is the first focusable element |
| Success | Stepper advances to Cohort with Details marked complete | Continue; return to Details | `aria-current` moves; change announced once |

Guardrails: no other stage is reachable before a valid Details draft; the date field accepts keyboard entry, not only a picker.

### Stage 2 — Cohort

Purpose: construct and finalise the attendee selection with exact counts, knowing finalisation freezes selection and every later change becomes a visible correction.

```text
│ Cohort                                                 │
│ Selection: Location is Dublin office                   │
│                                                        │
│ 41 people match. 39 resolved, 2 unresolved identities. │
│ ┌ Name              Team        Identity ┐             │
│ │ Aoife Byrne       Reception   resolved │             │
│ │ badge 0417        —           needs match │          │
│ │ …                                       │            │
│ └─────────────────────────────────────────┘            │
│ [ Finalise cohort… ]  Finalising freezes selection.    │
```

| State | The operator sees | Actions | Announcement and focus |
|---|---|---|---|
| Empty | "No people selected yet" with the two entry paths: choose from Directory or import | Add people; import | Explanation is visible text, not a tooltip |
| Building (partial) | The reconciled sentence: matched count, resolved count, unresolved count; the semantic table below | Adjust selection; continue | Count sentence updates as one polite announcement |
| Unresolved identities present | Unresolved rows labelled "Identity not confirmed" with their source label | Resolve now or carry into Attendees | Row state is in the row's accessible name |
| Stale inputs | "The Directory changed since this preview" with the changed count | Refresh preview | Refresh is explicit; the table never silently reorders |
| Finalise (confirmation) | A confirmation naming the exact count and the rule: selection freezes; corrections, walk-ins, and identity work continue with history | Finalise; cancel | Confirmation is a focus-trapped dialog; focus returns to the trigger on cancel |
| Finalised | Cohort marked immutable with its revision number; stepper advances | Continue to Attendees | Success announced once with the frozen count |
| Interrupted | Draft selection resumes exactly as left | Resume | Same resume pattern as Details |

Guardrails: finalisation shows the count it freezes; there is no bulk silent re-selection after finalisation; the unresolved count is never hidden to make the cohort look ready.

### Stage 3 — Attendees

Purpose: resolve every membership question — participation, duplicates, identity, walk-ins, removals — while the active denominator visibly reconciles after every change.

```text
│ Attendees                        Cohort revision 7     │
│ 41 rows · 39 active · 2 superseded — totals reconcile. │
│ ┌ Row  Person        Participation  Status    ┐        │
│ │ 3    Aoife Byrne   confirmed      active    │        │
│ │ 9    badge 0417    unknown        needs match │      │
│ │ 12   Tomás Ó Sé    invited        removed   │        │
│ └──────────────────────────────────────────────┘       │
│ History: every correction listed with date and reason  │
```

| State | The operator sees | Actions | Announcement and focus |
|---|---|---|---|
| Loading | A labelled progress indicator naming what is loading | Wait; leave stage | Progress has an accessible name; no spinner-only state |
| Populated | The reconciled sentence (rows, active, superseded) and the table with participation and status per row | Record participation; correct; remove; resolve identity; add walk-in | Table has a caption and labelled headers |
| Invalid transition | "Attended cannot return to invited without a correction" beside the control | Use the correction action | Error is announced from the row, focus stays on the control |
| Duplicate detected | The superseding row shows "superseded as duplicate of row N"; the denominator sentence updates | Inspect history | The sentence update is one polite announcement |
| Identity resolved onto an active person | The row is superseded automatically and the merge is recorded in history | Inspect history; undo is not offered — a new correction is | History entry names both rows |
| Walk-in added | New active row with origin walk-in and participation attended | Correct if wrong | Row addition announced with the new active count |
| Removal | Row status removed with reason and date; row remains visible in history | Reinstate via correction | Removal never deletes the row from view |
| Recovery after interruption | The stage reopens with all corrections intact and the history complete | Continue | Focus lands on the reconciled sentence |

Guardrails: corrected and superseded rows stay inspectable; the denominator sentence and the table can never disagree; no correction happens without a dated history entry.

### Stage 4 — Readiness

Purpose: every active attendee resolves to exactly one explicit outcome, with the evidence to prove it, before any brief exists. This ledger is the principal visual anchor of B0.

```text
│ Readiness                 Policy b0-baseline v1        │
│ 39 active attendees: 24 instruction recorded,          │
│ 6 no requirements (verified), 4 asked — awaiting       │
│ reply, 2 needs re-checking, 1 sources disagree,        │
│ 1 unknown — needs attention, 1 identity not confirmed. │
│ Totals reconcile to 39 of 39.                          │
│ ┌ Person        Outcome                    ┐ ┌Evidence┐│
│ │ Aoife Byrne   Instruction recorded       │ │ source ││
│ │ Niamh Kelly   Unknown — needs attention  │ │ dates  ││
│ │ badge 0417    Identity not confirmed     │ │ policy ││
│ └───────────────────────────────────────────┘ └───────┘│
```

| State | The operator sees | Actions | Announcement and focus |
|---|---|---|---|
| Deriving | Progress named "Calculating readiness" | Wait | Announced start and finish, no focus theft |
| Reconciled | The one-sentence exact count per outcome and the ledger table; the reconciliation line "Totals reconcile to N of N" | Open evidence; resolve gaps | The reconciliation line is always rendered, never implied |
| Missing information | Rows labelled "Unknown — needs attention"; unknown is visually and semantically distinct from verified none | Ask the person; record an accounted exception | Row label carries the state |
| Stale | "Needs re-checking" rows show the verification date and the policy that aged them | Re-verify; record exception | Evidence drawer shows the dates |
| Conflict | "Sources disagree" rows name both sources | Resolve in Directory; cannot be excepted away | Drawer lists both sources |
| Catering use not permitted | Counted and labelled; instruction content is never displayed for these rows | None within this event | Exclusion reason scope shown in drawer |
| Unresolved identity | Counted separately in the sentence and table | Return to Attendees to resolve | Link names the target stage |
| Accounted exception (create) | A confirmation requiring reason, owner, source, and review or expiry date; refuses invalid identity and conflicting-source rows | Confirm; cancel | Dialog focus pattern; on confirm the row shows the exception distinctly |
| Evidence drawer open | Source, capture and verification dates, canonical value, policy identifier and version, revision used | Close; act on the row | Drawer opens with focus on its title and returns focus to the row on close |
| Empty cohort edge | "This event has no active attendees" with the path back to Cohort or Attendees | Return | Plain statement, no error styling |

Ready versus accounted exception: an accounted exception is neutral but never looks equivalent to a ready outcome. It always exposes its event-specific reason, responsible owner, source, review or expiry date, and next safe action. Creating one requires explicit confirmation, and it cannot bypass an invalid identity or conflicting active sources.

Guardrails: no percentage, score, gauge, or progress ring summarises readiness; the sentence and table carry the truth; the selected table row and the evidence drawer are programmatically connected.

### Stage 5 — Brief

Purpose: preview exactly what would be disclosed, generate the sealed internal artifact, and treat every external delivery as its own explicit, evidenced operation.

```text
│ Brief                                                  │
│ Recipient: campus caterer   Purpose: summer lunch      │
│ Identifiers: names absent (default)                    │
│ Expires: 2026-08-21                                    │
│ ┌ Preview — this is the exact emitted content ┐        │
│ │ liaison-catering-brief v1                    │       │
│ │ denominator: 39 · provided: 24 · …           │       │
│ │ - 12 x no gluten; separate preparation       │       │
│ └──────────────────────────────────────────────┘       │
│ [ Generate internal brief ]  [ Deliver… (separate) ]   │
```

The content lines inside the preview box are illustrative layout only. The canonical delivered format, its header fields, and its exact rendering are owned by the P05 schema and P10 disclosure/delivery contracts; this document fixes the presentation behaviour around the preview, not the bytes inside it.

| State | The operator sees | Actions | Announcement and focus |
|---|---|---|---|
| Preview | The byte-exact content that would be emitted, with recipient, purpose, expiry, and identifier mode above it | Generate; adjust inputs | Preview region is labelled; identifier mode is stated in text |
| Names-absent default | No person names or identifiers anywhere in the content | Switch to opaque tokens only via an explicitly named approved policy | The default is stated, not assumed |
| Opaque tokens (approved policy named) | Event-local tokens beside grouped instructions; the approving policy name shown in the header | Generate | Policy name is part of the preview header |
| Disclosure denied | Token mode unavailable with the reason: no approved policy is recorded | Record the policy elsewhere; continue names-absent | The control is absent, not greyed mystery |
| Generated (sealed) | The sealed brief with checksum, creation date, input revisions, and the statement that preview and emitted bytes matched | Open receipt; start delivery | Success announced once with the checksum available in the receipt |
| Delivery (separate operation) | Its own explicit confirmation naming transport and destination; a failed or retried delivery leaves the sealed brief valid and never overwrites an earlier export | Retry; cancel delivery | Delivery result announced separately from generation |
| Stale after source change | The sealed brief marked "no longer reflects its sources" with the changed inputs listed; its content and checksum unchanged | Generate a new revision | Stale marking never rewrites history |
| Recovery | Reopening shows the sealed brief and its receipt exactly as before the interruption | Continue | Receipt is reachable from the stage without regenerating |

Guardrails: there is no way to edit sealed content; regeneration creates a new revision beside the old one; the receipt uses provenance typography (monospace) for checksums and identifiers only. The only text that can reach the Brief presenter is the constrained, validated operational instruction exposed by the authorised `DietaryOperationalView`; profile notes, diagnoses, restricted detail, and arbitrary source text have no path into it. Within that boundary the validated instruction reaches the Brief verbatim, and a length bound is still not least disclosure by itself: the surface where instructions are authored must carry permanent guidance copy — describe what catering must do, not the person's condition — and the preview is the last human review point for instruction content before disclosure.

## Route empty-state minimums

- No workspace → Create or Open workspace.
- Empty Directory → Import or add the first record.
- No Event → Prepare the first Event.
- Resumable draft → Resume the named stage.
- Healthy Health → last validation time plus "no active findings".

## Operation-state mapping shape — binding deferred

Every recoverable operation reached from these stages (finalise, correction, generation, delivery) will be mapped at the P03D reconciliation, one row per exact P03 application state or error, with these eight fields:

1. visible plain-language copy;
2. permitted primary and secondary actions;
3. whether the action is safe to retry;
4. accessible announcement behaviour;
5. focus destination and focus return;
6. durable operation receipt or saved-state evidence;
7. interruption, resume, and last-known-good behaviour;
8. appearance persistence or rollback when a theme change fails mid-operation.

This document deliberately leaves those rows empty. No copy in the stage tables above promises "saved locally", "recoverable", "busy", or retry semantics beyond what P00 contracts already require; exact wording binds only to proven P03 states. One committed rule is fixed now because it is a P00 contract: before the durable commit decision, Cancel leaves canonical files unchanged; after it, cancellation is unavailable, the interface explains that the committed operation is being recovered or finished, and the interface never reports a committed operation as cancelled.

## Reflow, zoom, and input targets

- Supported native window floor: 360×640 CSS pixels; 320 CSS pixels remains a required reflow and zoom condition, not a mobile claim.
- No stage task depends on horizontal scrolling at 400% zoom.
- At narrow widths every stage table becomes a semantic summary and detail presentation with the same information, selection, actions, counts, and evidence-drawer relationship; the reconciled sentence stays first.
- Every primary control meets a 48 px target; the previously previewed 40 px filter control is rejected.
- Labels never leave the accessibility tree at compact widths; workspace-authority and saved-state information stays perceivable.

## Evidence obligations for the owning milestones

Nothing below is claimed by this document; it is the checklist the owning slices must satisfy on the exact installed build:

- `aria-current` for the active stage; captioned, labelled tables; programmatic selected-row and drawer connection; deterministic drawer focus entry and return; status announcements without focus theft.
- Keyboard completion of all five stages; visible unobscured focus at 200% and 400% zoom; reduced-motion behaviour.
- en-XA expansion and localisation-safe layout for every label in the presenter table and every state row above.
- System, light, dark, and high-contrast coverage of the full workflow, including forced colours, on the installed universal Mac build with named VoiceOver evidence.
- A narrow-width receptionist-persona proof of the five stages; the existing 390 px artifact demonstrates a future personal-profile surface and does not satisfy this.
- UAT alignment: `UAT-009` (find every dietary gap), `UAT-010` (least-disclosure brief), `UAT-011` (stale brief detection), `UAT-012` (attendee list from filters).

## Open evidence-bound inputs

1. The exact P03 command, result, error, and recovery-to-presentation mapping — required before P04; owned by the P03D reconciliation.
2. The exact P05/P10 Event DTO and state delta against this contract — required before P11; recorded as a documented delta to the canonical `DESIGN.md` the consultation creates.

Neither input is an invitation to guess ahead of its gate.
