# Remediation PR plan — personal-CRM parity core

**Date:** 2026-07-18 · **Companion to:** [`2026-07-18-app-parity-and-wcag-gap.md`](2026-07-18-app-parity-and-wcag-gap.md) · **Decision:** personal-CRM core before the dietary-readiness wedge (audit §8).

Each PR is a **bounded vertical slice** (domain rule → application service → port/adapter → CLI + desktop surface → tests → knowledge/changelog), per `AGENTS.md`. Every behavioural PR carries the accessibility, privacy, and evidence sections the repo's PR template requires. **The design system (PR 1) and the a11y CI gate (PR 2) are ratcheted in first so every later surface inherits them** — we do not bolt accessibility on at the end.

**Standing gates on every UI PR** (from roadmap R2 exit): keyboard operation, visible focus, screen-reader names + live status, 200% zoom / 320–390px reflow, contrast, target size, reduced motion, interruption-safe drafts, and text alternatives to colour/icons/graphs. CI must run an automated a11y check (axe/pa11y) once PR 2 lands.

---

## PR 1 — Design system + app shell (prototype IA)
**Why:** closes the design complaint and unblocks every later surface (audit §4). RICE 4.0.
**Owning surface:** `apps/desktop/ui` (+ a small shared token/style layer). No domain change.
**Scope**
- Define the hand-drawn/sketchbook design language as **tokens** (colour, type scale, spacing, radius, stroke, elevation) + a documented component kit (buttons, cards, list rows, nav, form fields, badges, empty/loading/error states).
- Replace the numbered setup-wizard IA with the prototype IA: left nav **Today / People / (Events) / (Network) / Settings**; ship **Today** and **People** shells now, stub the rest behind feature gates.
- Capture the aesthetic as a **buildable reference screen** (the prototypes never did — audit §4).
**Out of scope:** wiring real data beyond what already exists; new domain logic.
**Acceptance / evidence:** design-token file + component gallery; Today/People shells render with existing workspace/people data; visual reflow at 320/390/desktop; reduced-motion honoured; screenshots in `docs/evidence/ux/`.
**Replaces:** retire PR #27 and reopen it as this slice (its current diff is empty).

## PR 2 — WCAG 2.2 AA gate in CI + shell remediation
**Why:** turn "target" into "evidence"; ratchet from here (audit §5). RICE 4.5.
**Scope**
- Add automated a11y checks (axe-core/pa11y) over the desktop UI in CI, failing on new violations.
- Contrast audit of all token pairs (light + dark); fix failures. Verify target size (≥24px, 2.2 SC 2.5.8), focus-not-obscured (2.2 SC 2.4.11), 400% zoom reflow.
- Programmatically associate errors with fields (SC 3.3.1); audit name/role/value across states.
**Acceptance / evidence:** CI a11y job green; contrast report; keyboard + screen-reader walkthrough notes; zoom/reflow screenshots — all in `docs/evidence/ux/`.
**Dependency:** PR 1 (shared tokens/components to test against).

## PR 3 — Reminders + reason-only review (Today/Overdue/This Week)
**Why:** highest RICE (6.75); the domain already exists in `contexts/review-attention` (`ReasonOnlyPolicy`, `ReviewReason`, `Suppression`, `ReviewCandidate`) — this is **surfacing, not building**.
**Owning context:** Review & Attention (+ Reminders).
**Scope**
- Application service + ports to compute the reason-only review queue and Today/Overdue/This Week groupings from canonical records.
- CLI verbs (`liaison review today|week|overdue`, JSON + human output) and the desktop **Today** surface: each surfaced person shows a **factual reason** and actions (open / log / snooze / pause / archive).
- Honour hard suppressions (do-not-contact, archive, pause, snooze). No score shown (reason-only default).
**Out of scope:** weighted Review Priority (Plan step 7, later); calendar/CalDAV integration (separate PR).
**Acceptance / evidence:** reason shown for every candidate; suppressions override; no colour-only meaning; 30-second review actions; unit + UAT tests; recovery/interruption test.

## PR 4 — People / profile surface (contact points, dates, first Topic Packs)
**Why:** RICE 4.5; domain largely exists (`profiles`: `TopicPackId`/`FieldType`/`InformationState`/`Classification`; `people`: `PartialDate`). Requirements LRM-PE-001..005.
**Owning context:** Identity & Profiles.
**Scope**
- Surface typed contact points (email/phone/URL/handle, preferred flags), aliases, pronouns, addresses.
- Important dates incl. **unknown-year birthdays** (never invent an age — LRM-PE-003).
- First Topic Packs (identity/communication + one more) with **explicit information states** (known/verified/unknown/…); profile field detail shows provenance + age without exposing value to unauthorised roles.
- Configurable field visibility/order (LRM-PE-007) as a later toggle, not blocking.
**Acceptance / evidence:** round-trip preserves order/type/preferred/unknown fields/Unicode (LRM-PE-002); stable field IDs survive label rename (LRM-PE-004); a11y gates.

## PR 5 — Interactions + notes + 30-second logging + timeline
**Why:** table-stakes for a personal CRM (audit §3a); RICE 4.0. Requirement LRM-IN-001.
**Owning context:** Interactions & Commitments.
**Scope**
- Record note/interaction: date, direction, channel, participants (multi-person), summary, next action, provenance, source ref.
- **30-second logging flow** that also updates last-contacted / next-due / last-topic.
- Chronological **timeline** per person (and general notes / journaling as first-class — audit §3b.7).
**Acceptance / evidence:** 30-second flow updates summaries without duplicate entry (LRM-IN-001); timeline renders with keyboard operation; a11y gates.

## PR 6 — Relationships + cadence editor + circles
**Why:** RICE 4.0; delivers relationship type/tier/intent + **user-facing cadence setting (req #5)** and circles/groups.
**Owning context:** Relationships (+ Groups).
**Scope**
- Typed person↔person / person↔org links with direction, status (VIP/maintain/watch/…), priority, **cadence**, provenance (LRM-RE-001/002).
- Cadence editor feeding the review queue (PR 3). Circles/groups membership on profiles.
- Summary of last interaction / last meaningful note / next action / due cadence (LRM-RE-003).
**Out of scope:** relationship **graph** + semantic table (Plan R2, separate PR); CalDAV reminders (separate, req #5 second half).
**Acceptance / evidence:** changing relationship priority does not mutate either profile (LRM-RE-001); label customisation keeps stable machine values (LRM-RE-002); a11y gates.

## PR 7 — Search + filtering + saved views
**Why:** parity floor (every comparator has it); RICE 4.5.
**Scope**
- Fast people search (name/handle/email) + filtering across relationship type/tier, maintenance status, review reason, groups, missing/stale fields.
- Saved views that show their active predicates and reproduce later.
**Acceptance / evidence:** search + filter over a synthetic workspace; saved-view round-trip; a11y gates.

---

## After the core (not in this plan, tracked in audit §8)
- **Export + Google Drive backup adapter** (req #2, RICE 3.2) — land alongside PR 4–7 if capacity allows.
- **Organisations/groups/memberships** — land DRAFT PR #22 (prereq for the events wedge).
- **Events + dietary readiness** — the Plan's operational wedge, after the core.
- **Mobile/adaptive** (req #1) — after the **PWA-vs-Tauri-mobile decision** (open decision b).
- **Business-card scan / OCR** (req #4), **AI follow-up/network discovery** (Ozzy parity), **meitheal MCP interop spike** (req #6) — later per RICE.

## Backend DRAFTs to fold in
- **#21 profile values persistence** → into PR 4.
- **#22 organisations/groups** → land after core, before events wedge.
- **#25 backup/restore** → into the export/backup workstream.
- **#28 vault write journal** → merge into workspace lifecycle when wired.
- **#27** → retire; reopen as PR 1 (design system).

## Governance
- **RICE** (audit §8) recorded here per PR; ratify scores with the team and store in `spec/`.
- **DDD:** no UI/CLI computes domain rules; all logic stays in the owning context's application services.
- **KCS:** each PR links/creates a knowledge article for its workflow.
