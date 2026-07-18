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
- Event Details uses Cohort, Attendees, Readiness, and Brief subviews.
- Personal-first PRs #27 and #29 are superseded as delivery plans; their useful audit findings may be selectively ported.
- B0 and A0 receive separate compiled design-review and native-QA evidence.

## Migration, rollback, or reversal conditions

Changing the order requires an explicit maintainer decision with updated requirements, UAT, gates, tasks, and release evidence. A broad roadmap feature does not silently change the next accepted product.
