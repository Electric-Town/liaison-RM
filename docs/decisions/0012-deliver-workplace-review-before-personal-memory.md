# 0012: Deliver Workplace Review before Personal Memory

- Status: accepted
- Date: 2026-07-18
- Deciders: Electric Town maintainer
- Contexts: product, directory, events, experience
- Requirements: LRM-EV-001, LRM-EV-002, LRM-EV-003, LRM-EV-004, LRM-EV-005
- Feature gates: FG-R3-001, FG-R3-002, FG-R3-003, FG-R3-004, FG-R3-005

## Context and problem

The specification supports personal, family, executive-assistant, networking, and workplace use. Attempting all of them before a complete outcome would produce a wide demo without a trustworthy recovery, privacy, or acceptance boundary.

## Decision

The first independently reviewable product is **B0 Workplace Review Alpha**. It takes a workplace operator from Directory import through event cohort finalization, exact dietary reconciliation, least-disclosure preview, immutable internal brief, and verified delivery.

**A0 Personal Memory Alpha** begins only after B0 acceptance. It adds profile editing, meaningful interactions, commitments, reason-only Review, last-interaction context, and open-loop views over the same session, storage, security, recovery, and desktop foundations.

Shared foundations land before B0. Personal Today/Review, mobile/PWA hosts, providers, AI/MCP, and broad integrations cannot become B0 prerequisites.

## Consequences

- Product navigation for B0 is Overview, Directory, Events, Health, and Settings.
- B0 Settings owns built-in theme choice and persistence only. Versioned settings bundle export/import and clean-device transfer begin in A0.
- Event Details uses Cohort, Attendees, Readiness, and Brief subviews.
- Personal-first PRs #27 and #29 are superseded as delivery plans; their useful audit findings may be selectively ported.
- B0 and A0 receive separate compiled design-review and native-QA evidence.

## Migration, rollback, or reversal conditions

Changing the order requires an explicit maintainer decision with updated requirements, UAT, gates, tasks, and release evidence. A broad roadmap feature does not silently change the next accepted product.

## Machine traceability authority group

The following identifier-exact anchors are the contracts whose execution
ownership is deliberately re-assigned by this accepted B0-before-A0 decision.
The machine registry binds this complete group to the digest of this decision;
adding, removing, or redirecting a member requires updating both sources.

### ADR0012-B0-before-A0

- `cross-release::LRM-EV-001`
- `cross-release::LRM-EV-002`
- `cross-release::LRM-EV-003`
- `cross-release::LRM-EV-004`
- `cross-release::LRM-EV-005`
- `cross-release::LRM-EV-007`
- `cross-release::LRM-EV-008`
- `cross-release::LRM-EV-009`
- `cross-release::LRM-EV-010`
- `cross-release::LRM-EV-011`
- `cross-release::LRM-EV-013`
- `cross-release::LRM-IN-001`
- `cross-release::LRM-L10N-001`
- `cross-release::LRM-L10N-002`
- `cross-release::LRM-L10N-003`
- `cross-release::LRM-L10N-004`
- `cross-release::LRM-L10N-005`
- `cross-release::LRM-L10N-006`
- `cross-release::LRM-L10N-008`
- `cross-release::LRM-OR-001`
- `cross-release::LRM-OR-002`
- `cross-release::LRM-OR-003`
- `cross-release::LRM-OR-004`
- `cross-release::LRM-PE-001`
- `cross-release::LRM-PE-002`
- `cross-release::LRM-PE-003`
- `cross-release::LRM-PE-004`
- `cross-release::LRM-PE-005`
- `cross-release::LRM-PE-006`
- `cross-release::LRM-PE-008`
- `cross-release::LRM-PE-009`
- `cross-release::LRM-PE-010`
- `cross-release::LRM-RE-001`
- `cross-release::LRM-RE-002`
- `cross-release::LRM-RE-003`
- `cross-release::LRM-RE-004`
- `cross-release::LRM-RE-005`
- `cross-release::LRM-UX-001`
- `cross-release::LRM-UX-002`
- `cross-release::LRM-UX-003`
- `cross-release::LRM-UX-004`
- `cross-release::LRM-UX-005`
- `cross-release::LRM-UX-006`
- `cross-release::LRM-UX-007`
- `cross-release::LRM-UX-008`
- `cross-release::LRM-UX-012`
- `cross-release::LRM-WS-001`
- `cross-release::LRM-WS-002`
- `cross-release::LRM-WS-003`
- `cross-release::LRM-WS-004`
- `cross-release::LRM-WS-005`
- `cross-release::LRM-WS-006`
- `cross-release::LRM-WS-008`
- `cross-release::UAT-001`
- `cross-release::UAT-002`
- `cross-release::UAT-003`
- `cross-release::UAT-004`
- `cross-release::UAT-008`
- `cross-release::UAT-009`
- `cross-release::UAT-010`
- `cross-release::UAT-011`
- `cross-release::UAT-012`
- `cross-release::UAT-017`
- `cross-release::UAT-021`
- `cross-release::UAT-023`
- `cross-release::UAT-025`
- `cross-release::UAT-040`
- `cross-release::UAT-073`
- `cross-release::UAT-LOC-001`
- `cross-release::UAT-LOC-002`
