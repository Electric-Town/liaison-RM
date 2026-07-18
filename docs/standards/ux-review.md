# User experience and interaction review standard

## Purpose

Liaison RM must remain usable when a person is interrupted, overloaded, using assistive technology, unfamiliar with CRM terminology, or working under event-day pressure. A visually polished happy path is not sufficient.

This checklist combines evidence-led discovery, ADHD and AuDHD considerations, AskTog interaction principles, Gestalt principles, Nielsen’s heuristics, relevant IxDF research, WCAG 2.2 Level AA, and applicable EN 301 549 requirements. It is a review framework, not a claim of legal certification.

## Evidence before design

Use past-behaviour questions rather than selling the proposed interface. Capture:

- the last time the person performed the task;
- what triggered it and what deadline applied;
- the tools, spreadsheets, notes, messages, or memory aids used;
- where they hesitated, switched tools, asked for help, or made a correction;
- what happened when information was missing or wrong;
- what they did after an interruption;
- the consequence of failure.

Do not treat compliments, hypothetical willingness, or feature voting as demand evidence.

## ADHD and AuDHD review

A flow should provide:

- a clear current location and current task;
- visible save, sync, validation, and completion state;
- short, named steps for long workflows;
- safe pause and resume without reconstructing context;
- draft preservation and recovery after interruption;
- progressive disclosure rather than a wall of equally weighted controls;
- stable navigation, wording, and control placement;
- optional compact and comfortable density;
- reduced motion and no unnecessary animation;
- no time pressure unless the domain truly requires it;
- reminders that explain the reason and next action;
- filters that show what is active and how to clear them;
- confirmation proportionate to consequence rather than repeated modal friction;
- summaries before committing imports, exports, disclosures, or destructive changes.

Avoid overstimulating dashboards, unexplained badges, surprise reordering, auto-advancing forms, and hidden state.

## AskTog interaction review

Review whether the interface:

- anticipates the likely next action without removing control;
- gives users autonomy and reversible choices;
- uses consistent objects and actions;
- reduces latency and acknowledges work immediately;
- protects data and makes destructive scope explicit;
- avoids invisible modes and distinguishes modes that must exist;
- makes learnable controls visible rather than depending on memory;
- supports keyboard use without requiring a slower second-class workflow;
- uses defaults based on evidence and makes them easy to inspect;
- preserves continuity across screens, tabs, and interruptions.

## Gestalt and information hierarchy

Check proximity, similarity, common region, continuity, closure, figure-ground separation, and connectedness. Group controls by task and consequence rather than implementation layer. Do not use visual similarity for controls with materially different risk.

Headings, spacing, order, and labels must communicate hierarchy without colour. Dense profile pages should support user-configurable sections while retaining a predictable reading order.

## Nielsen’s ten heuristics

Record evidence for all ten:

1. Visibility of system status
2. Match between system and the real world
3. User control and freedom
4. Consistency and standards
5. Error prevention
6. Recognition rather than recall
7. Flexibility and efficiency of use
8. Aesthetic and minimalist design
9. Recognition, diagnosis, and recovery from errors
10. Help and documentation

A checklist answer must name the screen, state, test, or observation. `Pass` without evidence is not acceptable.

## Accessibility requirements

User-facing changes require evidence for the relevant items:

- complete keyboard operation with visible focus;
- logical focus order and focus restoration after dialogs;
- semantic names, roles, states, relationships, errors, and instructions;
- screen-reader operation on at least one supported platform;
- 400% zoom and reflow without lost information or action;
- target size and spacing appropriate to WCAG 2.2;
- contrast that meets the applicable success criteria;
- no colour-only, shape-only, sound-only, or position-only meaning;
- reduced-motion behaviour;
- accessible authentication and no cognitive-function test as the only route;
- status messages announced without unexpected focus movement;
- plain-language errors tied to the relevant field;
- localisation-safe layout and long-name handling.

## Graphs, tables, and drag interactions

A relationship graph is an additional view, not the information architecture.

Every graph capability must have a semantic table or tree equivalent with the same filtering and actions. Graph nodes and edges require accessible names and inspectable details. Layout, colour, and distance cannot be the sole carriers of meaning.

Every drag-and-drop action requires keyboard and direct-action alternatives. Users must be able to undo reordering and restore defaults.

## Required states

Review at least:

- first use and empty state;
- loading and long-running progress;
- partial import or partial provider response;
- offline and Airgap behaviour;
- stale data and conflicting revisions;
- permission denied and grant expired;
- validation error and corrupt file;
- success and next action;
- undo and recovery;
- no results under active filters;
- very large names, lists, notes, and datasets.

## Pull-request evidence

Attach or link:

- task and persona;
- relevant past-behaviour evidence;
- annotated flow or prototype;
- keyboard route;
- screen-reader notes;
- interruption-resume observation;
- heuristic review;
- accessibility checks and unresolved defects;
- decision on graph/drag alternatives;
- evidence that copy uses domain language and discloses consequences.
