# UI Design Contract Specification: B0 Workplace Review Alpha (7-Screen Editorial Ledger)

Date: 2026-07-21
Status: Approved Design Spec
Owning Context: Experience / Desktop Inbound Adapter

## 1. Objective

Implement the complete 7-screen presentation design for Liaison RM Workplace Review Alpha (`apps/desktop/ui/`) following the Editorial Ledger design contract (`DESIGN.md`) and Domain-Driven Design (DDD) principles.

The screens to be implemented with high visual polish, modularity, and accessibility are:
1. **01 TODAY / OVERVIEW**: Workspace dashboard, date strip calendar, Prepare alert card, Upcoming events, Commitments, Preparation stepper preview, and Recent interactions.
2. **02 EVENTS**: Calendar header bar, interactive week date strip, event timeline cards with readiness tags and action buttons, floating filter drawer toggle.
3. **03 EVENT ATTENDEE READINESS**: Event header, 5-stage progress Stepper, Attendee reconciliation table, Least-disclosure catering brief panel, and Notes panel.
4. **04 PEOPLE / DIRECTORY**: Directory search and filter bar, batch operations card, paginated people data table, and footer pagination controls.
5. **05 PERSON**: Person header, tab navigation bar, contact info list, recent interactions timeline, "Working with me" guidance notes, important dates, and commitments checklist.
6. **06 HEALTH / RECOVERY**: Workspace health status header, Last safe checkpoint card with audit evidence panel, Recoverable change notice banner, and Local safety backup export buttons.
7. **07 SETTINGS**: Tabbed settings interface (`Appearance`, `Profile tabs`, `Custom fields`, `Portable`, `Data & backup`, `Accessibility`), theme selector cards (`Paper`, `High contrast`, `Night`), text size controls, reduce motion switch, density selector, and local backup panel.
8. **360PX - TODAY (NARROW)**: Mobile / 360px viewport responsive reflow layout with bottom navigation drawer.

---

## 2. Architecture & Domain-Driven Design (DDD) Boundaries

- **Rust Core Authority**: `crates/liaison-application` and Tauri IPC commands remain the single source of truth for canonical records, event cohorts, dietary readiness, attendance reconciliation, and health status.
- **Frontend Presentation Adapter**: `apps/desktop/ui/` contains only presentation code, view routing, ephemeral form/draft state, and DOM rendering.
- **Modular View Architecture**:
  - `apps/desktop/ui/index.html`: Semantic HTML5 document shell.
  - `apps/desktop/ui/styles.css`: CSS custom properties, Editorial Ledger tokens, typography, component styles, and 360px responsive reflow rules.
  - `apps/desktop/ui/app.js`: Application orchestrator, theme manager, and IPC event router.
  - `apps/desktop/ui/modules/`: Modular view components (`appShell.js`, `todayView.js`, `eventsView.js`, `readinessView.js`, `peopleView.js`, `personView.js`, `healthView.js`, `settingsView.js`).
  - `apps/desktop/ui/modules/components/`: Shared UI components (`stepper.js`, `dataTable.js`, `statusChip.js`, `noticeBanner.js`).

---

## 3. Design System & Accessibility Invariants

- **Typography**:
  - Operational: Atkinson Hyperlegible Next for controls, forms, tables, body copy, and UI headings.
  - Editorial: Source Serif 4 for page titles.
  - Technical Provenance: IBM Plex Mono for file paths, revision hashes, and operation IDs.
- **Color Palette & Themes**:
  - Note-paper canvas (`#eee8dc`) with radial paper dot pattern.
  - Built-in theme presets: `Paper`, `High contrast`, `Night` (Editorial Dark), `Nordic Midnight`, `Warm Amber`, `Emerald Forest`.
- **Accessibility**:
  - WCAG 2.2 AA and EN 301 549 compliance.
  - Minimum hit targets >= 48px for primary actions.
  - Full keyboard focusability (`3px solid var(--focus)`) and ARIA roles (`aria-current="page"`, `aria-current="step"`, `role="tablist"`, etc.).
  - 400% zoom and 360px mobile reflow without horizontal scrolling or text clipping.

---

## 4. Verification Plan

1. **Modular Code Inspection**: Verify clean separation of view modules in `apps/desktop/ui/modules/`.
2. **Visual & Parity Check**: Ensure exact visual matching for all 7 screens plus the 360px narrow reflow view.
3. **Repository Checks**: Run `python3 scripts/check_repository.py` and `python3 scripts/check_spec.py` to ensure repository invariants pass.
