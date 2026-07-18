# Review and Attention specification review

Status: design evidence; no production usability or accessibility conformance claim.

## Reviewed workflows

- reason-only daily review;
- low-capacity mobile review;
- Topic Pack activation;
- purpose-specific profile readiness;
- missing, stale, conflicting, declined, and not-applicable field states;
- hard suppressions;
- transparent weighted-policy example.

## Cognitive and interruption controls

The concept limits daily review by capacity rather than exposing an unbounded backlog. Each candidate states why they appeared. Skip, snooze, pause, and archive are valid actions and do not reduce a relationship score. The mobile concept shows one candidate at a time.

Unfinished forms and review sessions must later persist their exact candidate, action state, and draft. This specification defines that requirement but does not demonstrate the runtime behavior.

## Nielsen heuristic review

| Heuristic | Applied decision |
|---|---|
| Visibility of system status | Policy, capacity, reasons, readiness gaps, and suppression state are visible |
| Match with the real world | Copy uses commitments, dates, cadence, and information states rather than an abstract strength score |
| User control and freedom | Skip, snooze, pause, archive, and field-state actions are explicit |
| Consistency and standards | Stable field IDs and policy vocabulary are shared across files, views, and plugins |
| Error prevention | Hard suppressions precede queue ordering; sensitive fields require classification |
| Recognition rather than recall | The current reason and last meaningful topic are visible on the review card |
| Flexibility and efficiency | Reason-only, tiered, and weighted modes serve different operational needs |
| Aesthetic and minimalist design | Low-capacity review presents a bounded candidate set and optional detail |
| Error recovery | Readiness results list the unresolved field and available resolution actions |
| Help and documentation | Purpose definitions and policy versions remain inspectable |

## Gestalt and interaction review

- Reasons are grouped with the person they explain.
- Readiness results are grouped by purpose, not merged into one completion gauge.
- Status is communicated by text as well as colour.
- Primary and secondary actions remain visually distinct.
- The graph or radar representation, if later added, requires an equivalent semantic table.

## ADHD and AuDHD considerations

- no permanent red overdue count by default;
- low, normal, and high capacity settings;
- bounded batches;
- explicit next action;
- no contact-count gamification;
- reduced-motion and low-stimulation requirements;
- optional hiding of irrelevant Topic Packs;
- exact-place interruption recovery requirement;
- no moral language around cadence.

## Sensitive-data review

Dietary, accessibility, travel, emergency, and private relationship-assessment information requires classification and purpose controls. A readiness calculation may reveal that information is unresolved without revealing the sensitive value itself. Least-disclosure output remains a separate application use case.

## Evidence still required

- unassisted task observation;
- screen-reader evaluation;
- keyboard walkthrough of the production UI;
- 200% zoom and reflow testing;
- interruption and draft-recovery tests;
- comprehension testing for `unknown`, `declined`, `not applicable`, and `stale`;
- review of private-assessment sharing boundaries;
- formal WCAG 2.2 and applicable EN 301 549 evidence.
