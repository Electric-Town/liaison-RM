# Liaison RM design contract

Version: 1.1.0
Status: P03D candidate; binding only after `FG-B0-DESIGN-001` acceptance
Decision date: 2026-07-22
Owning context: Experience

This document is the candidate presentation authority for Liaison RM's Workplace Review Alpha. The design consultation and review material is preserved, but `T-B0-P03` is still current and `T-B0-P03D` is blocked; this file is therefore neither accepted P03D evidence nor permission to start P04. Product, domain, privacy, recovery, and storage rules remain owned by `SPEC.md`, accepted ADRs, application contracts, and tests. A visual treatment may not weaken those rules.

## 1. Experience outcome

Liaison RM should feel like a calm working ledger: legible, tangible, and clearly local. The interface helps one trusted workspace owner account for people, event cohorts, dietary readiness, and disclosure evidence without treating people as leads, scores, or productivity units.

The selected direction is **Editorial Ledger**:

- a warm note-paper canvas outside work surfaces;
- flat, high-legibility work surfaces;
- restrained hard-offset emphasis on at most one primary work surface per page;
- strong ink borders where a boundary carries meaning;
- Source Serif 4 for page titles only;
- Atkinson Hyperlegible Next for body copy, controls, forms, navigation, tables, dialogs, drawers, and operational headings;
- IBM Plex Mono only for provenance such as paths, revisions, hashes, operation identifiers, and receipts;
- no decorative handwriting font in operational UI;
- no remote fonts, remote textures, hidden assets, or runtime theme downloads.

The paper treatment is character, not noise. Texture stays on the canvas and never sits beneath long-form text, tables, forms, dialogs, or evidence content. Intentional irregularity must never alter target position, focus order, hit area, alignment of tabular data, or the meaning of state.

## 2. Scope and route contract

P04 preserves only the already implemented foundation through four capability-backed destinations:

1. **Overview** — presentation-only orientation over Application status and the currently held workspace session.
2. **Workspace** — create, open, resume, close, authority, and recovery state.
3. **People** — bounded create, list, search, and read-only Person reachability already supplied by the accepted pre-P04 surface.
4. **Health** — lock-free inspection, canonical-record findings, and safe recovery guidance already supplied by the accepted pre-P04 surface.

P11 later composes the complete B0 navigation from its accepted upstream capabilities: Overview, Directory, Events, Health, and Settings, including Details, Cohort, Attendees, Readiness, and Brief. P04 supplies semantic components and route-mapping seams but does not publish speculative Directory, Events, or Settings route IDs.

Navigation is capability-honest. A destination is visible only when its complete route contract is compiled and available. Liaison does not show disabled "coming soon" items.

At widths below 760 CSS pixels, the permanent rail becomes one labelled **Sections** control. It is never replaced by icon-only navigation or a horizontal scroller. The active destination, and the active Event stage once P11 owns that workflow, remain available to assistive technology through `aria-current`.

## 3. Application architecture

P04 replaces the disposable vanilla shell with a typed React adapter inside the existing Tauri application.

```text
React / TypeScript presentation
        ↓ generated request and result types
Tauri command adapter
        ↓
liaison-application
        ↓
Workspace Session and bounded-context application services
        ↓
context-owned ports and local adapters
```

Rules:

- React owns ephemeral presentation state, form drafts, focus restoration, and route state.
- Rust owns domain rules, authorization, canonical mutations, recovery, readiness derivation, and receipts.
- The frontend never opens canonical files directly.
- The frontend never calculates a second dietary outcome, maintenance status, cohort denominator, or Review Priority.
- Request and result types are generated or compile-checked from one versioned contract. Hand-maintained duplicate DTOs are prohibited.
- Every mutation has one operation identifier and one visible result state.
- Browser storage is not canonical. Persisted UI settings use the approved Settings application service.
- No interface may infer trust, affection, closeness, employee value, or relationship strength from interaction volume.

## 4. Semantic token contract

The canonical registry is `design/semantic-tokens.v1.json`. Components consume semantic roles rather than raw colours.

Required roles include:

```text
canvas
surface
surface-subtle
content
content-muted
border
border-strong
action
content-on-action
focus
highlight
content-on-highlight
current-step
current-step-content
surface-information
success
surface-success
warning
surface-warning
danger
surface-danger
```

Built-ins:

- `system` — a resolution rule that follows the operating-system light/dark preference;
- `light`;
- `dark`;
- `high_contrast`.

`system` is not a fourth palette. An OS preference change updates the resolved built-in without changing the saved choice.

Meaning must survive all built-ins. Colour is never the only indicator of selected, current, complete, warning, conflict, stale, unavailable, or successful state. Essential component boundaries use `border-strong`, text, shape, and structure. The lower-contrast routine `border` token is decorative and may not be the sole boundary for a control, selected row, dialog, drawer, current step, or actionable region.

The dark highlight correction is binding: `content-on-highlight` and `current-step-content` use light content on the approved dark olive highlight. Near-black content on that highlight is prohibited.

## 5. Typography and content

### Roles

| Role | Font | Use |
|---|---|---|
| Operational | Atkinson Hyperlegible Next | body, controls, labels, forms, tables, navigation, dialogs, drawers, operational headings |
| Editorial | Source Serif 4 | page titles and non-operational section titles only |
| Provenance | IBM Plex Mono | paths, IDs, hashes, revision tokens, receipts, technical evidence |

Text remains usable when a preferred font fails. Fallback metrics must be tested for clipping and reflow. Font versions, licences, hashes, and script coverage are release evidence.

### Voice

- Name the actor, action, state, and recovery path.
- Use neutral, no-guilt language.
- Say “overdue relative to quarterly cadence,” not “neglected relationship.”
- Say “Unknown — needs attention,” never an empty label or “No requirements.”
- Avoid promotional language, mystery states, and generic success messages.
- A receipt or error may expose safe identifiers; it must not repeat sealed or special-category values.

All natural-language strings use locale keys. No new user-facing string may exist only inside a component, test selector, or error mapping.

## 6. Component contract

P04 implements one versioned component library. Components use named variants, not page-specific class fragments.

### AppShell

Owns the top bar, navigation, main region, status region, and application-level recovery banner. It exposes workspace identity and authority without repeating a sensitive path in every view.

### PageHeader

Contains one page title, concise purpose statement, and no more than one primary action. Counts belong in reconciled sentences or tables, not KPI mosaics.

### LedgerPanel

Flat surface for ordinary content. `emphasis="primary"` may add the one hard-offset shadow allowed on a page. Panels do not rotate, jiggle, or overlap operational content.

### Notice and RecoveryBanner

Use an explicit heading, state label, message, and next safe action. A recovery banner persists until its condition is resolved or acknowledged through an application service.

### FormField

Provides label, optionality, description, input, validation message, and stable error association. Placeholder text is never a label. Invalid focus moves only on submit or explicit validation, not while typing.

### Button

Variants: primary, secondary, quiet, danger. Minimum target 48 by 48 CSS pixels for primary workflow actions. Disabled controls are used only for temporarily unavailable operations with visible explanation; unavailable capabilities are otherwise absent.

### StatusChip

Atomic, text-labelled state. One chip represents one canonical value. It is not an icon-only badge and never combines dietary readiness with attendee lifecycle.

### DataTable

Uses a caption or labelled relationship, column headers, keyboard-reachable row actions, and stable selected-row semantics. At 400% zoom or narrow width, it becomes a summary/detail representation with identical information and actions; horizontal scrolling is not required for the primary task.

### Stepper

For the Event stages only: Details, Cohort, Attendees, Readiness, Brief. The current stage uses `aria-current="step"`; unavailable future stages are labelled “not started” and are not interactive.

### Drawer

Used for evidence and source detail. Focus enters the title, Escape and Close return focus to the invoking row, and the selected row is programmatically connected to the drawer.

### Dialog

Used only when a decision cannot be safely made inline. Destructive and disclosure actions name the consequence. Initial focus avoids the destructive action. Closing returns focus to the invoker.

### EmptyState

Names what is empty, why that matters, and the one valid next action. Illustration is optional and carries no required information.

### OperationStatus

Maps one application operation state to visible copy, retry availability, focus behavior, receipt link, and interruption behavior. It never invents a local “saved” flag.

### ThemePicker

Presents `system`, `light`, `dark`, and `high_contrast` as text-labelled built-ins with a preview that contains content, focus, current step, warning, danger, success, table, and form samples. Saving a theme is an application operation with rollback on failure.

## 7. P03 operation presentation

The following mapping is binding for P04. P11 may add domain-specific copy but cannot change the durable semantics.

| P03 state | User-facing meaning | Permitted actions | Interruption and focus |
|---|---|---|---|
| `staged` before commit decision | “Preparing changes. Nothing has been committed.” | Cancel | Cancel leaves canonical files unchanged; focus returns to the initiating control |
| precondition failure | “The file changed before Liaison could commit.” | Review current file; retry after review | No canonical target is overwritten; focus moves to the conflict summary |
| contract failure | “This change could not be prepared safely.” | Return to form | Field or operation summary names a safe correction |
| storage failure before commit | “Liaison could not prepare the change.” | Retry; open Health | Canonical targets remain unchanged |
| `commit-decided` or `publishing` | “The change was committed and is being finished.” | Wait; open operation details | Cancel is absent; relaunch rolls forward |
| `complete` | “Changes saved.” | Open receipt; continue | Announce once; focus moves to the next workflow heading or returns to the invoking control |
| discarded before commit during recovery | “An interrupted uncommitted change was discarded. Canonical files were unchanged.” | Dismiss; inspect Health | Recovery summary receives focus only when opened by the user |
| rolled forward during recovery | “Liaison finished a committed change after interruption.” | Open receipt; continue | Announce once after workspace open |
| already complete during recovery | No warning by default; operation remains in evidence history | Open receipt | No focus movement |
| recovery conflict | “A committed change could not finish because a target changed outside Liaison.” | Preserve both sources; open guided recovery | No automatic overwrite; conflict banner persists |

Every operation detail shows the safe operation ID, phase, target count, start/completion time, and receipt. It does not expose staged content or sealed values.

## 8. Directory contract

Directory is a data workspace, not a card gallery.

- One search field covers display name and approved non-sensitive directory fields.
- Filters are visible as removable text chips and represented in the URL/route state where safe.
- The default result is a semantic table with stable row selection.
- People, Organisations, Groups, Locations, and Memberships remain distinct records.
- Current and historical Memberships are visibly different; “latest” never silently erases history.
- Import has Preview → Map → Validate → Commit → Receipt states.
- Invalid siblings do not hide healthy records.
- Repair always begins with preview and exact backup evidence.
- Sealed dietary detail never appears in the general Directory table.
- A dietary operational outcome is visible only to an authorized purpose surface.

## 9. Events contract

Events implement the five-stage contract:

```text
Details → Cohort → Attendees → Readiness → Brief
```

The canonical stage and state definitions are in `docs/evidence/ux/b0-events-design-contract-candidate.md`; P03D promotes its state coverage into this contract with these rules:

- exact reconciled counts appear as a sentence before a semantic table;
- no score, gauge, percentage, or progress ring summarizes readiness;
- every active attendee appears in exactly one dietary outcome;
- unresolved identity is an attendee lifecycle state, not a dietary outcome;
- unknown never means verified none;
- an accounted exception is visually and semantically distinct from ready;
- Brief preview is byte-identical to emitted content;
- names are absent by default;
- opaque tokens require an approved policy;
- generation and delivery are separate operations and receipts;
- a sealed brief is immutable; regeneration creates a new revision;
- source changes mark an earlier brief stale without rewriting it.

The presenter table version, policy ID, decision-table version, and source revisions remain inspectable.

## 10. Responsive, zoom, and input contract

- Native window floor: 360 by 640 CSS pixels.
- 320 CSS pixels is a required reflow/zoom condition, not a mobile product claim.
- At 400% zoom, the primary task requires no horizontal scrolling.
- Tables convert to equivalent summary/detail views where necessary.
- Labels remain visible and in the accessibility tree.
- Primary controls meet 48-pixel target size; all controls meet WCAG 2.2 target-size requirements or documented exceptions.
- Drag and drop always has a keyboard file-selection or reorder equivalent.
- Sticky elements may not obscure focused content at 200% or 400% zoom.

## 11. Accessibility evidence matrix

P04 establishes the automated ratchet; P11 and B0 acceptance prove the installed workflow.

Required evidence:

- complete keyboard operation and deterministic focus return;
- VoiceOver evidence on the installed universal Mac app;
- accessible names, descriptions, errors, status announcements, table semantics, step semantics, dialog/drawer behavior, and selected-row relationships;
- 200% text scaling and 400% zoom/reflow;
- reduced motion and no motion-dependent meaning;
- system, light, dark, high-contrast, and forced-colours behavior;
- non-colour state communication;
- 48-pixel primary workflow targets;
- en-XA 45% expansion and long unbroken-value handling;
- Irish, Japanese, and Brazilian Portuguese structural fixtures;
- interruption, restart, conflict, permission denial, stale state, success, undo/rollback, and recovery.

This is an evidence target, not a claim of certification.

## 12. Localization contract

- `en-IE` is the source catalogue.
- `en-XA` is mandatory in CI and expands visible strings by approximately 45%.
- `ga-IE`, `ja-JP`, and `pt-BR` remain human-review-gated catalogues.
- Japanese uses appropriate `line-break` and `word-break` behavior.
- Dates, times, numbers, and currency use locale-aware formatters.
- Stable IDs, route IDs, field IDs, policy IDs, and test selectors are not translated.
- Human approval is recorded by locale, source revision, reviewer, and date.
- Automated stylometry is not an authorship detector or acceptance substitute.

## 13. Theme preview, persistence, and rollback

P04 renders `system`, `light`, `dark`, and `high_contrast` through one semantic contract. Its preview is transient: cancel, navigation, reload, or a failed preview restores the prior rendering, and `system` follows operating-system changes. P04 neither writes an appearance preference nor claims relaunch persistence.

P11 Settings later persists one built-in selection per workspace owner:

- preview remains temporary until a Settings application operation succeeds;
- a failed save restores the last persisted built-in and announces the failure;
- relaunch restores the saved choice before the first meaningful paint;
- appearance records contain no relationship, dietary, or identity data; and
- theme changes serialize with other native mutations.

Third-party theme packages remain outside B0 and A0.

## 14. Performance budget

P04 measures and enforces for its four-route foundation:

- first meaningful shell render without network access;
- bounded rendering for the existing People and Health result sets;
- no layout shift from font loading after interactive controls appear;
- no synchronous parsing of the canonical workspace in the webview;
- no remote font, icon, texture, analytics, or telemetry request.

P06 owns Directory pagination, projection, and scale evidence. P10/P11 own Events data-route and complete-workflow performance. Their later budgets may build on P04 components but cannot be claimed from the P04 shell baseline. Exact numeric budgets are recorded from the first owning-phase baseline and then may tighten, not silently loosen.

## 15. Prohibited patterns

- KPI-card mosaics for reconciled operational counts.
- Icon-only primary navigation.
- A generic “Contact” aggregate in domain-facing copy where Person or Organisation is known.
- Disabled mystery capabilities.
- Soft blurred shadows as the main depth language.
- Decorative rotation or animation on operational tables, forms, dialogs, drawers, or status controls.
- Universal profile-completeness or relationship-strength scores.
- Employee ranking, productivity inference, attendance-compliance scoring, or social-credit behavior.
- Silent auto-save claims not backed by an application receipt.
- UI-local canonical state, browser-authoritative records, or direct file writes.
- User-facing strings outside the localization catalogue.
- Remote assets in the installed application.

## 16. Definition of done for P04

P04 is merge-ready only when:

1. React/TypeScript replaces the vanilla shell without duplicating domain rules.
2. Generated or compile-checked DTOs cover every exposed command and result.
3. The component contract and semantic-token registry are versioned and tested.
4. All four built-in selections render transiently through one component contract without a persistence claim.
5. The current Workspace, People, and Health behavior has parity tests before the old shell is removed.
6. Every normal pull-request matrix passes on Linux, macOS, and Windows.
7. Browser tests cover keyboard, focus, 320/360 reflow, 400% zoom, reduced motion, en-XA, and zero external requests.
8. The universal Mac review build proves the installed shell, transient preview cancellation, operating-system appearance resolution, and fail-closed fallback; P11 owns persistence and save rollback.
9. Font provenance, licences, hashes, fallbacks, and supported script coverage are recorded.
10. No closed P05–P11 or A0 behavior is represented as implemented.

## 17. Change control

A change to route identity, component semantics, operation-state meaning, Event stage meaning, token role, or accessibility behavior requires:

- a design-contract version change;
- a documented reason and affected UAT/gates;
- updated automated evidence;
- installed-build evidence when the change affects B0 acceptance.

Raw palette adjustment that preserves semantic roles and passes all evidence is a minor design-contract change. Removing or reinterpreting a semantic role is a major change.
