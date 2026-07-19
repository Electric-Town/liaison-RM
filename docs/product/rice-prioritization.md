# B0 and A0 RICE prioritization

Last scored: 2026-07-19

RICE helps compare useful slices. It does not override a safety gate, accepted dependency, or the maintainer decision to qualify B0 before starting A0.

## Scoring model

The project has no production adoption data, so invented user counts would create false precision. This pre-alpha score uses:

- **Reach:** the number of accepted B0/A0 acceptance paths or downstream work packages directly unblocked, capped at 12.
- **Impact:** 3 for an alpha-blocking outcome, 2 for a major outcome improvement, 1 for a meaningful support improvement, and 0.5 for a minor improvement.
- **Confidence:** evidence-weighted certainty from 0.50 to 0.95, based on existing code, tests, prototypes, research, and unresolved technical risk.
- **Effort:** estimated engineer-weeks for a dependency-complete slice, including tests, KCS, documentation, accessibility, migration, and review evidence.

`RICE = Reach × Impact × Confidence ÷ Effort`

Scores rank work only when its dependencies and safety prerequisites are already satisfied. A low-scoring key-recovery task still blocks sensitive dietary data. A high-scoring personal feature cannot start before B0 acceptance.

## Current scores

| Work package | Reach | Impact | Confidence | Effort | Score | Ordering rule |
|---|---:|---:|---:|---:|---:|---|
| P00 Contract and truth reconciliation | 12 | 1 | 0.95 | 1.5 | 7.60 | First because later formats depend on accepted contracts. |
| P01 Application composition root | 12 | 3 | 0.90 | 3 | 10.80 | Highest current value; enables one CLI/Tauri command model. |
| P02 Workspace Session authority | 12 | 3 | 0.85 | 4 | 7.65 | Must precede recoverable writes, security, and checkpoints. |
| P03 Recoverable multi-target operations | 12 | 3 | 0.80 | 5 | 5.76 | Required before new canonical formats or imports. |
| P04 Typed React/Tauri design system | 8 | 2 | 0.80 | 4 | 3.20 | Starts after stable application/session commands; parity precedes Events UI. |
| P05 G1 Directory/profile/Event contracts | 10 | 3 | 0.75 | 5 | 4.50 | Establishes the domain prerequisite consumed by P06/P07 and later G3 work. |
| P05-OKF strict People schema/port | 10 | 2 | 0.90 | 3 | 6.00 | Runs after P03/P04; owns strict authoring and UAT-065 under FG-B0-001. |
| P06 Tolerant Directory projection | 7 | 2 | 0.75 | 4 | 2.63 | Required before 10,000-person cohort workflows. |
| P06-REPAIR Guided canonical repair | 7 | 3 | 0.85 | 3 | 5.95 | Runs after P03/P06; owns UAT-040 and closes FG-R1-002 before P09-OKF. |
| P07 Workspace Security and local policy | 9 | 3 | 0.65 | 7 | 2.51 | Safety gate; score cannot defer it behind user-facing dietary work. |
| P08 Checkpoint and encrypted recovery | 7 | 3 | 0.65 | 6 | 2.28 | Safety gate; B0 cannot accept sensitive data without clean recovery. |
| P09-OKF required legacy-People normalization | 8 | 3 | 0.80 | 4 | 4.80 | Runs after P03/P05-OKF/P06/P06-REPAIR; owns exact backup, recovery, rerun, rollback, and UAT-066. |
| P09 Directory onboarding and import | 5 | 3 | 0.75 | 5 | 2.25 | Begins after the required OKF normalizer and projection foundations; general/third-party migration remains later. |
| P10 Events core and brief delivery | 6 | 3 | 0.70 | 7 | 1.80 | Product wedge; depends on Directory, security, and recovery contracts. |
| P11 B0 compiled interface | 6 | 3 | 0.75 | 6 | 2.25 | Uses stable commands; it is not a mock-first screen project. |
| B0 installed-app qualification | 4 | 3 | 0.90 | 3 | 3.60 | Mandatory acceptance gate before A0. |
| A0 Personal Memory Alpha | 5 | 2 | 0.70 | 8 | 0.88 | Explicitly starts after B0 acceptance. |
| A0 source-complete profile qualification | 5 | 3 | 0.75 | 5 | 2.25 | Composes P01/P02/P03 and owns UAT-067–069 under FG-A0-001. |

P06-REPAIR is a three-engineer-week integrity slice: P03 supplies recoverable operations, P06 supplies tolerant findings, and the task adds guided preview, exact backup and receipt, failure-atomic application, exact rollback, fault evidence, and recovery knowledge. Its 5.95 score does not let it bypass those dependencies; it makes the previously hidden repair acceptance cost explicit before P09-OKF.

## PR use

Every behavioural PR records:

- its work package and current RICE values;
- which observed user outcome and accepted gate it advances;
- dependencies that override the numeric score;
- evidence that would change reach, impact, confidence, or effort;
- the score after scope changes.

Mechanical corrections may state `RICE: not applicable`, but must still explain why the change is needed. Product and architecture decisions cannot use a score as a substitute for causal reasoning, safety review, or user observation.
