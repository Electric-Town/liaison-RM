# B0 7-Screen Editorial Ledger UI Redesign Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the complete 7-screen presentation design for Liaison RM Workplace Review Alpha (`apps/desktop/ui/`) following the Editorial Ledger design contract (`DESIGN.md`), Domain-Driven Design (DDD) principles, and WCAG 2.2 AA accessibility standards.

**Architecture:** Frontend UI modules in `apps/desktop/ui/modules/` act as presentation adapters consuming domain data and dispatching user intentions via Tauri IPC (`crates/liaison-application`). Reusable components (`stepper.js`, `dataTable.js`, `statusChip.js`, `noticeBanner.js`) encapsulate presentation logic, while CSS custom properties in `styles.css` enforce Editorial Ledger tokens and responsive 360px reflow layout.

**Tech Stack:** Vanilla JavaScript (ES Modules), HTML5 semantic markup, Vanilla CSS (CSS Variables), Tauri IPC integration, Atkinson Hyperlegible Next, Source Serif 4, and IBM Plex Mono fonts.

## Global Constraints

- Domain rules belong to Rust domain crates; UI modules only display domain state and call application services.
- Accessible color contrast (WCAG 2.2 Level AA).
- Minimum hit area 48x48 CSS pixels for primary actions.
- Touch/keyboard/screen-reader friendly with explicit focus styles (`3px solid var(--focus)`).
- 400% zoom and 360px viewport reflow support.

---

### Task 1: CSS Design System Tokens & Responsive Reflow (`styles.css`)

**Files:**
- Modify: `apps/desktop/ui/styles.css`

**Interfaces:**
- Consumes: `DESIGN.md` Editorial Ledger semantic token specification.
- Produces: CSS custom properties, utility classes, theme presets, component styles, and `@media (max-width: 760px)` / `@media (max-width: 360px)` reflow rules.

- [ ] **Step 1: Inspect and enhance `styles.css` token definitions**
  Ensure `:root` and theme data-attributes (`data-theme="paper"`, `data-theme="high-contrast"`, `data-theme="night"`, `data-theme="nordic"`, `data-theme="amber"`, `data-theme="emerald"`) contain all required semantic tokens (`--canvas`, `--surface`, `--ink`, `--muted`, `--border`, `--action`, `--highlight`, `--content-on-highlight`, `--success`, `--warning`, `--danger`).

- [ ] **Step 2: Add 360px narrow reflow CSS rules**
  Add media queries `@media (max-width: 760px)` and `@media (max-width: 360px)` to collapse navigation rail into a mobile bottom/header bar, stack cards into a single column, adjust touch targets to 48px, and prevent horizontal scroll.

- [ ] **Step 3: Commit CSS changes**
  Run: `git add apps/desktop/ui/styles.css && git commit -m "style: enhance Editorial Ledger CSS tokens and 360px responsive reflow rules"`

---

### Task 2: Shared UI Components (`stepper.js`, `dataTable.js`, `statusChip.js`, `noticeBanner.js`)

**Files:**
- Create: `apps/desktop/ui/modules/components/stepper.js`
- Create: `apps/desktop/ui/modules/components/dataTable.js`
- Create: `apps/desktop/ui/modules/components/statusChip.js`
- Create: `apps/desktop/ui/modules/components/noticeBanner.js`

**Interfaces:**
- Consumes: DOM nodes and structured state objects.
- Produces: Modular render functions for progress steppers, accessible data tables, status chips, and notice banners.

- [ ] **Step 1: Create `stepper.js`**
  Implement 5-stage stepper renderer (`Details`, `Cohort`, `Attendees`, `Readiness`, `Brief`) with `aria-current="step"` support.

- [ ] **Step 2: Create `dataTable.js`**
  Implement accessible tabular data renderer with row selection, status badges, action links, and mobile card reflow.

- [ ] **Step 3: Create `statusChip.js`**
  Implement text-labelled status chip component (`Ready`, `Action needed`, `Confirmed`, `Pending`, `Not started`).

- [ ] **Step 4: Create `noticeBanner.js`**
  Implement alert and recovery banner component with clear headings, messages, and next actions.

- [ ] **Step 5: Commit shared components**
  Run: `git add apps/desktop/ui/modules/components/ && git commit -m "feat(ui): add modular shared UI components"`

---

### Task 3: Overview & Today View Module (`todayView.js` - Screen 01 & 08)

**Files:**
- Create: `apps/desktop/ui/modules/todayView.js`

**Interfaces:**
- Consumes: Application IPC `get_overview` service payload.
- Produces: Rendered Screen 01 (Today / Overview) with date strip, Prepare alert card, Upcoming events, Commitments list, Stepper overview, and Recent interactions table.

- [ ] **Step 1: Write `todayView.js` module renderer**
  Render current date header ("Today · Friday, 24 July 2026"), interactive week strip calendar (20-26 with THU 24 active), Quick Capture button, Prepare event card with red `Continue readiness →` action, Upcoming calendar card, Commitments checklist, Event preparation stepper, and Recent interactions table.

- [ ] **Step 2: Verify desktop & 360px layout**
  Ensure view responds cleanly to narrow viewports <= 360px.

- [ ] **Step 3: Commit `todayView.js`**
  Run: `git add apps/desktop/ui/modules/todayView.js && git commit -m "feat(ui): add Today/Overview view module (Screen 01 & 08)"`

---

### Task 4: Events & Attendee Readiness View Modules (`eventsView.js` & `readinessView.js` - Screens 02 & 03)

**Files:**
- Create: `apps/desktop/ui/modules/eventsView.js`
- Create: `apps/desktop/ui/modules/readinessView.js`

**Interfaces:**
- Consumes: Application IPC `list_events` and `get_event_readiness` payloads.
- Produces: Rendered Screen 02 (Events timeline & date strip) and Screen 03 (Event Attendee Readiness table & catering brief).

- [ ] **Step 1: Write `eventsView.js` module renderer**
  Render calendar nav header, date strip bar (MON 20 to SUN 26), event timeline cards (Team coffee, All-hands lunch, Client dinner, Team offsite) with readiness tags, and floating filter drawer trigger.

- [ ] **Step 2: Write `readinessView.js` module renderer**
  Render event header, 5-stage progress Stepper, Attendee reconciliation table (Aisling Byrne, Liam Lynch, John Hale, Adriana Cerry), Least-disclosure catering brief panel, and Notes panel.

- [ ] **Step 3: Commit event view modules**
  Run: `git add apps/desktop/ui/modules/eventsView.js apps/desktop/ui/modules/readinessView.js && git commit -m "feat(ui): add Events and Attendee Readiness view modules (Screens 02 & 03)"`

---

### Task 5: Directory & Person View Modules (`peopleView.js` & `personView.js` - Screens 04 & 05)

**Files:**
- Create: `apps/desktop/ui/modules/peopleView.js`
- Create: `apps/desktop/ui/modules/personView.js`

**Interfaces:**
- Consumes: Application IPC `list_people` and `get_person_detail` payloads.
- Produces: Rendered Screen 04 (Directory table, batch sidebar, pagination) and Screen 05 (Person profile detail, tabs, working-with-me guidance).

- [ ] **Step 1: Write `peopleView.js` module renderer**
  Render directory search & action bar, batch operations card, paginated people data table (214 people), and pagination footer.

- [ ] **Step 2: Write `personView.js` module renderer**
  Render person detail header (Aisling Byrne), tab navigation bar (`Overview`, `Notes`, `Important dates`, `Commitments`, `Workspace`, `Custom fields`), contact info panel, recent interactions timeline, Working with me guidance notes, important dates, and commitments checklist.

- [ ] **Step 3: Commit directory view modules**
  Run: `git add apps/desktop/ui/modules/peopleView.js apps/desktop/ui/modules/personView.js && git commit -m "feat(ui): add Directory and Person view modules (Screens 04 & 05)"`

---

### Task 6: Health & Settings View Modules (`healthView.js` & `settingsView.js` - Screens 06 & 07)

**Files:**
- Create: `apps/desktop/ui/modules/healthView.js`
- Create: `apps/desktop/ui/modules/settingsView.js`

**Interfaces:**
- Consumes: Application IPC `get_workspace_health` and `get_settings` payloads.
- Produces: Rendered Screen 06 (Health status, audit evidence, recovery actions) and Screen 07 (Settings tabs, appearance themes, backup export/import).

- [ ] **Step 1: Write `healthView.js` module renderer**
  Render local workspace health header, Last safe checkpoint card with audit evidence panel, Recoverable change notice banner, and Local safety copy export buttons.

- [ ] **Step 2: Write `settingsView.js` module renderer**
  Render settings tabs (`Appearance`, `Profile tabs`, `Custom fields`, `Portable`, `Data & backup`, `Accessibility`), theme selector cards (`Paper`, `High contrast`, `Night`), text size controls, reduce motion switch, density selector, and backup import/export panel.

- [ ] **Step 3: Commit health & settings view modules**
  Run: `git add apps/desktop/ui/modules/healthView.js apps/desktop/ui/modules/settingsView.js && git commit -m "feat(ui): add Health and Settings view modules (Screens 06 & 07)"`

---

### Task 7: Shell Orchestration, Navigation Integration & HTML Wireup (`index.html`, `app.js`, `appShell.js`)

**Files:**
- Modify: `apps/desktop/ui/index.html`
- Modify: `apps/desktop/ui/app.js`
- Create: `apps/desktop/ui/modules/appShell.js`

**Interfaces:**
- Consumes: View modules from Tasks 3-6.
- Produces: Integrated single-page application routing, navigation rail, topbar, theme switching, and IPC coordination.

- [ ] **Step 1: Update `index.html` semantic structure**
  Wire up skip links, app-shell layout container, navigation rail buttons, and main container `<main id="main-content">`.

- [ ] **Step 2: Create `appShell.js` and update `app.js` router**
  Wire up route switching (`today`, `events`, `readiness`, `people`, `person`, `health`, `settings`), theme switching, and Tauri IPC event handlers.

- [ ] **Step 3: Commit shell integration**
  Run: `git add apps/desktop/ui/index.html apps/desktop/ui/app.js apps/desktop/ui/modules/appShell.js && git commit -m "feat(ui): integrate application shell, modular router, and theme manager"`

---

### Task 8: Verification & Repository Checks

**Files:**
- Verification only

- [ ] **Step 1: Run repository invariants check**
  Run: `python3 scripts/check_repository.py`
  Expected: Clean pass with 0 errors.

- [ ] **Step 2: Run specification traceability check**
  Run: `python3 scripts/check_spec.py`
  Expected: Clean pass with 0 errors.

- [ ] **Step 3: Run Rust clippy & tests**
  Run: `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` and `cargo test --workspace --all-features --locked`
  Expected: PASS

- [ ] **Step 4: Commit final verification confirmation**
  Run: `git commit --allow-empty -m "chk: verify B0 7-screen desktop UI implementation and repository invariants"`
