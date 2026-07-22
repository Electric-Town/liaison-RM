# Living Retrospective: Liaison RM

> **Invalidated completion record — 2026-07-22:** B0 was not shipped. The
> statements below are preserved as historical claims, not current evidence.
> The eight-screen surface was prototype/sample UI; P04–P11 and B0 acceptance
> remained blocked.

## Milestone: vB0 — Workplace Review Alpha

**Historical shipped claim:** 2026-07-22 — **invalidated**
**Phases:** 5 (P00–P04) | **Plans:** 5 | **Tasks:** 79

### What the invalidated record claimed was built
- B0 Workplace Review Alpha desktop application shell using Tauri and Vanilla Web Component architecture.
- Reconciled UI authority (`approved-atlas-authority-and-ux-audit-20260722.md`), exact recovery language (`Local checkpoint intact`), built-in system theme support, and offline request template badges.
- 8 fully functional screens (Today Overview, Events, Cohort Readiness, People Directory, Person Detail, Health & Recovery, Settings, Edit Profile & Customisation).

### What the invalidated record claimed worked
- Domain-driven isolation keeping business logic in Rust crates (`liaison_people`, `liaison_events`, `liaison_workspace`) and rendering logic in clean Web Components.
- Continuous automated verification (`check_desktop_shell.py`, `check_repository.py`, `check_spec.py`) enforcing accessibility and security gates.

### Key Lessons
- Clear separation of delivery boundaries (B0 Workplace Review vs A0 Personal Memory) prevents scope creep and keeps audit contracts crisp.

### Unsupported quality claims
- 100% Rust unit test suite pass across all workspace crates.
- 0 clippy warnings under strict `-D warnings` settings.
- WCAG 2.2 AA compliant contrast and 100% explicit `<label for="...">` accessibility coverage.

Those command, WCAG, and functional-screen statements had no accepted
exact-head or exact-artifact evidence. A token contrast check is not WCAG or
EN 301 549 conformance, and a passing unit suite would not qualify B0.
