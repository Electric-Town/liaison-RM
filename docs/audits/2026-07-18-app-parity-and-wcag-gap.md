# App parity & WCAG gap audit

**Date:** 2026-07-18 · **Branch head at audit:** `8ecde07` · **Author:** engineering review

> Purpose: measure the distance between what Liaison RM's **Plan** promises, what the **repository actually ships**, what the **prototype design** specifies, and what **comparable products** in the field offer — then define what we still need for full coverage, sequenced by RICE.

## 0. Executive verdict

Three things are true at once:

1. **The Plan is strong and is a deliberate superset** of every comparator (Ozzy, Meerkat, CRM in Markdown, Monica). The gap is **not** in the specification.
2. **The shipped product is a thin slice of the Plan.** The desktop app is a 3-screen setup wizard (workspace → name+email people → validate); the CLI does `workspace init/show/validate` and `person create/list`. That is roughly **3 of ~8 core surfaces**, and even those are shallow.
3. **The domain layer is *ahead* of the surfaces.** Rust types for Topic Packs, field/information states, review reasons, suppressions, and reason-only policies already exist in `contexts/` but are **not wired to any CLI command or UI**. The main lever is *surfacing existing domain work*, not inventing it.

The user complaints are all correct: **no baseline parity with the README/comparator examples, the prototype ("Sketch") design is not honoured, and WCAG 2.2 AA is a foundation-only aspiration with no conformance evidence.** The one open PR scoped to fix this ([#27](https://github.com/Electric-Town/liaison-rm/pull/27)) currently contains **zero product code**.

## 1. Method & evidence base

| Source | What it establishes |
|---|---|
| `SPEC.md`, `spec/requirements.json`, `docs/product/roadmap.md`, `spec/feature-gates.yaml`, `preview.md` | The **Plan** (authoritative internal feature set + release order) |
| `apps/cli/src/main.rs`, `apps/desktop/ui/{index.html,app.js,styles.css}`, `contexts/*/src` | The **Repo** (what actually ships vs. domain-only) |
| `docs/prototypes/screens/*.svg`, `docs/prototypes/README.md` | The **design** contract |
| Open PRs #21, #22, #25, #27, #28 (all DRAFT) | Work in flight |
| tryozzy.xyz, Meerkat CRM, CRM in Markdown, Monica | **Competitive field** |
| `Coolock-Village/meitheal` (private, GH metadata) | Future interop target |

---

## 2. Part A — Surface parity: Plan vs. what actually ships

Legend: ✅ shipped & user-reachable · 📐 domain types exist but **not surfaced** · 🟡 partial/shallow · 🧪 in open DRAFT PR · ⛔ absent

| Core surface (Plan / prototype) | CLI | Desktop | Domain (Rust) | Verdict |
|---|---|---|---|---|
| Workspace create/open/validate | ✅ | ✅ | ✅ `contexts/workspace` | **Shipped** |
| Person: name + email | ✅ | ✅ | ✅ `contexts/people` | **Shipped (shallow)** |
| Person: typed contact points, aliases, pronouns, addresses, URLs | ⛔ | ⛔ | 🟡 `EmailAddress`/`PhoneNumber` only | Gap |
| Important dates / birthdays (unknown-year) | ⛔ | ⛔ | 📐 `PartialDate` | **Domain-only** |
| Topic Packs + custom fields + information states | ⛔ | ⛔ | 📐 `TopicPackId`,`FieldType`,`InformationState`,`Classification` | **Domain-only** |
| Profile readiness (purpose-specific) | ⛔ | ⛔ | 🟡 `PurposeId` scaffold | Gap |
| Relationships: type/tier/intent/cadence | ⛔ | ⛔ | ⛔ | **Absent** |
| Interactions / notes / 30-sec logging / timeline | ⛔ | ⛔ | ⛔ | **Absent** |
| Reminders + Today/Overdue/This Week | ⛔ | ⛔ | 📐 `ReviewReason`,`Suppression`,`ReasonOnlyPolicy`,`ReviewCandidate` | **Domain-only** |
| Search & filtering / saved views | ⛔ | ⛔ | ⛔ | **Absent** |
| Organisations / groups / memberships | ⛔ | ⛔ | 🧪 PR #22 | In DRAFT |
| Events + dietary readiness (the stated wedge) | ⛔ | ⛔ | ⛔ | **Absent** |
| Relationship network graph + semantic table | ⛔ | ⛔ | ⛔ | **Absent** |
| Import/export (vCard/CSV) | ⛔ | ⛔ | ⛔ | Absent (roadmap R1 deliverable, not built) |
| Backup / isolated restore | ⛔ | ⛔ | 🧪 PR #25 | In DRAFT |
| Health / validation view | ✅ | ✅ | ✅ | **Shipped** |

**Key finding — "domain ahead of surface":** the highest-ROI work is not greenfield. `review-attention` and `profiles` already model the hard parts (reason-only policy, suppressions, information states, topic packs). They need **application-service wiring + CLI verbs + UI**, not new domain design. This is a much cheaper path to parity than the surface count implies.

---

## 3. Part B — Competitive feature parity ("ensure we have everything we need")

Legend: ✅ has it · 🟡 partial/planned-only · ⛔ absent. **Liaison-Plan** = specified; **Liaison-Repo** = actually shipped.

| Capability | Liaison-Plan | Liaison-Repo | Ozzy | Meerkat | CRM-in-MD | Monica |
|---|---|---|---|---|---|---|
| People + fast search | ✅ | 🟡 (list, no search) | ✅ | ✅ | ✅ | ✅ |
| Circles / groups / labels | ✅ | ⛔ | 🟡 | ✅ | ✅ (tier) | ✅ |
| Relationship type & tier | ✅ | ⛔ | 🟡 | ✅ | ✅ | ✅ |
| Cadence / follow-up rhythm | ✅ | ⛔ | ✅ (AI) | 🟡 | ✅ | 🟡 |
| Last/next contact + reason + last topic | ✅ | ⛔ | ✅ | 🟡 | ✅ | 🟡 |
| Interaction / communication log | ✅ | ⛔ | ✅ (voice) | ✅ | ✅ | ✅ |
| Timeline view | ✅ | ⛔ | ✅ | ✅ | 🟡 | ✅ |
| Reminders + Today/Overdue/This Week | ✅ | ⛔ | ✅ | ✅ | ✅ | ✅ |
| Birthdays / important dates | ✅ | 📐 | 🟡 | ✅ | 🟡 | ✅ |
| Notes / journaling / diary | ✅ | ⛔ | 🟡 | ✅ | ✅ | ✅ (diary) |
| Monthly review workflow | ✅ | ⛔ | ⛔ | ⛔ | ✅ | ⛔ |
| Relationships between contacts (spouse/child) | ✅ | ⛔ | ⛔ | ✅ | ⛔ | ✅ |
| Organisations / memberships | ✅ | 🧪 | 🟡 | ⛔ | 🟡 (company tpl) | 🟡 |
| Events + attendance + dietary coverage | ✅ | ⛔ | ⛔ | ⛔ | ⛔ | 🟡 |
| Gifts / favourites | ✅ | ⛔ | ⛔ | ⛔ | 🟡 | ✅ |
| Pets | ✅ | ⛔ | ⛔ | ⛔ | ⛔ | ✅ |
| Tasks / commitments | ✅ | ⛔ | 🟡 | ⛔ | 🟡 | ✅ |
| Message / outreach templates | 🟡 | ⛔ | ⛔ | ⛔ | ✅ | ⛔ |
| CardDAV two-way sync | ✅ | ⛔ | ⛔ | ✅ | ⛔ | 🟡 |
| Calendar (CalDAV) integration | ✅ | ⛔ | ✅ | ⛔ | ⛔ | ⛔ |
| Email/social integrations | ✅ (grants) | ⛔ | ✅ (Gmail/Outlook/WhatsApp/X/LinkedIn) | ⛔ | ⛔ | ⛔ |
| Business-card scan / OCR import | 🟡 → **ADD** | ⛔ | ✅ | ⛔ | ⛔ | ⛔ |
| Voice-to-text interaction capture | 🟡 | ⛔ | ✅ | ⛔ | ⛔ | ⛔ |
| AI follow-up / "who to contact for goal X" | ✅ (R6) | ⛔ | ✅ | ⛔ | ⛔ | ⛔ (refuses AI) |
| Local-first / self-hosted / data ownership | ✅ | ✅ | ⛔ (cloud) | ✅ | ✅ | ✅ |
| Export (open formats) | ✅ | ⛔ | 🟡 | ✅ | ✅ (files) | ✅ |
| Google Drive / provider backup | ✅ | ⛔ | ⛔ | ⛔ | 🟡 (Obsidian sync) | 🟡 |
| Debts / money-owed | ⛔ **consider** | ⛔ | ⛔ | ⛔ | ⛔ | ✅ |
| Native mobile app | 🟡 (reflow only) → **ADD** | ⛔ | ✅ (mobile-first) | 🟡 (CardDAV) | 🟡 (Obsidian mobile) | 🟡 (web) |
| Desktop app | ✅ | 🟡 | ⛔ | 🟡 (web) | ➖ | 🟡 (web) |
| i18n / light-dark | ✅ | 🟡 (dark CSS only) | ⛔ | ✅ | ➖ | ✅ (27 langs) |

### 3a. Table-stakes we have *not* shipped (every calm-CRM comparator has these)
Fast **search**, **relationship type/tier**, **cadence + last/next contact**, **interaction log**, **reminders + Today/Overdue/This Week**, **notes/journaling**, **birthdays surfaced**. These define "a personal CRM" and are the parity floor. Our domain already covers most of the hard logic (see Part A) — they are unsurfaced, not unbuilt.

### 3b. Plan-completeness gaps — features to *add* so coverage is total
Folding in the six additions requested plus comparator scan:

1. **Native mobile + desktop, dynamic/adaptive UI** *(new req #1)* — roadmap currently ships **desktop-only Tauri with 320/390px reflow**; there is **no native mobile track**. Add an explicit adaptive/responsive requirement and a mobile delivery decision (Tauri-mobile vs. PWA — note meitheal is a PWA, see Part F).
2. **Local-first + export + Google Drive, provider-agnostic** *(new req #2)* — already a Plan principle (provider neutrality, R1 export, R4 transport); **but nothing is shipped**. Elevate vCard/CSV export and a Google Drive backup adapter to near-term, keeping the capability-contract/agnostic design intact.
3. **DDD + KCS + RICE governance** *(new req #3)* — DDD ✅ (context-owned domain) and KCS ✅ (README is KCS-informed). **RICE is new**: adopt RICE scoring for the backlog (applied in Part G) and record it in `spec/`.
4. **Business-card scanning / OCR import** *(new req #4)* — add as a first-class import path (Connections/Automation), local OCR to respect Airgap.
5. **Calendar reminders + cadence setting** *(new req #5)* — cadence exists in the Plan/domain; ensure a user-facing cadence editor **and** CalDAV reminder integration (Connected-local) are tracked distinctly.
6. **meitheal interoperability** *(new req #6, future)* — see Part F.
7. **Message/outreach templates** (CRM-in-MD) and **journaling/diary as first-class** (Monica/Meerkat) — currently only loosely implied.
8. **Debts/money-owed** (Monica) — *decision needed*: in-scope for personal use or explicitly out-of-scope? Flag, don't assume.

---

## 4. Part C — Design delta vs. the prototype ("Sketch") contract

The prototypes ([`docs/prototypes/screens/`](docs/prototypes/screens)) define the intended IA: left nav **Home / People / Events / Network / Settings**, a **Today dashboard** (stat cards → reason-based review → event readiness), plus People/profile, dietary readiness, network+semantic-table, review, and mobile screens.

| Dimension | Prototype intent | Shipped app | Delta |
|---|---|---|---|
| Information architecture | Home/People/Events/Network/Settings | Numbered wizard: 1 Workspace / 2 People / 3 Health | **Different product** |
| Primary screen | Today dashboard | Workspace setup form | Missing |
| Visual language | "Sketchbook / hand-drawn" (per PR #27 intent) | Flat corporate (`--primary:#3157c8` cards) | Not honoured |
| Design system | (intended) | None — plain 246-line CSS | **Absent** |
| Mobile | Dedicated mobile dashboard + low-capacity review screens | Single 760px reflow breakpoint | Shallow |

**Two-layer finding:** (a) the app doesn't match the prototypes; (b) the prototypes themselves are **flat corporate SVGs and do not express the intended hand-drawn aesthetic** — so the "Sketch design" was never captured as a buildable artifact. A design-system definition is a prerequisite, not a polish step.

---

## 5. Part D — WCAG 2.2 AA conformance (shipped shell only)

The shell has a **genuine foundation** but covers only 3 screens and has **no automated conformance evidence**. `scripts/check_desktop_shell.py` checks landmarks/labels/boundaries — it is **not** a WCAG test (no contrast, keyboard, or axe run).

| SC (2.2 AA) | Status | Evidence / gap |
|---|---|---|
| 1.1.1 Non-text content | 🟡 | Decorative marks `aria-hidden`; no meaningful images to alt yet |
| 1.3.1 Info & relationships | ✅ | Landmarks, `<dl>`, `<label>`, headings |
| 1.4.3 Contrast (min) | ❓ | **Unverified** — no measurement of token pairs |
| 1.4.10 Reflow | 🟡 | One 760px breakpoint; **320px / 400% zoom unverified** |
| 1.4.11 Non-text contrast | ❓ | Unverified (focus ring, borders) |
| 1.4.12 Text spacing | ❓ | Unverified |
| 2.1.1 Keyboard | 🟡 | Native controls likely OK; **not tested** |
| 2.4.1 Bypass blocks | ✅ | Skip link present |
| 2.4.7 Focus visible | ✅ | `:focus-visible` 3px outline |
| 2.4.11 Focus not obscured *(2.2 new)* | ❓ | Unverified |
| 2.5.8 Target size (min 24px) *(2.2 new)* | 🟡 | Buttons `min-height:44px`; nav/inputs unverified |
| 3.2.6 Consistent help *(2.2 new)* | ➖ | No help mechanism yet |
| 3.3.1 Error identification | 🟡 | Status messages exist; not programmatically associated to fields |
| 3.3.7 Redundant entry *(2.2 new)* | ➖ | N/A currently |
| 3.3.8 Accessible authentication *(2.2 new)* | ➖ | No auth (local-first) |
| 4.1.2 Name/role/value | 🟡 | `aria-current`, roles present; not audited across states |
| 4.1.3 Status messages | ✅ | `role="status"` live region |
| Reduced motion | ✅ | `prefers-reduced-motion` block |
| Dark mode | 🟡 | `prefers-color-scheme: dark` tokens; contrast unverified |

**Meta-gap:** no CI a11y gate, no contrast report, no keyboard/screen-reader/zoom evidence. "2.2 AA" is a target with **0 conformance artifacts**, and it covers 3 of the eventual ~8+ surfaces. Roadmap R2 exit *does* list the right evidence — it just isn't produced yet.

---

## 6. Part E — Cross-cutting principle conformance

| Principle | State | Note |
|---|---|---|
| Local authority / local-first | ✅ | Enforced in code (no-network CSP, Airgap build) |
| Open formats + export | 🟡 | Markdown/YAML canonical ✅; **export not shipped** |
| Provider-agnostic / modular | ✅ (design) | Capability contracts + `SafeMode` in `contexts/connections`; no adapter shipped |
| Google Drive option | ⛔ | Specified, not built |
| Domain-driven design | ✅ | Context-owned aggregates; UI holds no domain rules |
| Knowledge-Centered Support | ✅ | KCS-informed repo governance |
| **RICE scoring** | ⛔ **new** | Not yet used — adopt (Part G) |
| Accessibility (2.2 AA) | 🟡 | Foundation only (Part D) |
| Mobile + desktop, dynamic | 🟡 | Desktop reflow only; **no mobile track** |

---

## 7. Part F — meitheal interoperability / merge assessment *(future)*

Reviewed from the local checkout (`code/meitheal-onedev-control-plane-20260717`, monorepo `v0.1.80`). Meitheal — *"the Home Assistant-native execution hub for your house, your homelab, and the plans that keep both moving"* — is a **local-first** (Astro SSR, **SQLite**, HA ingress, **installable PWA with offline sync**) planning/execution hub. It exposes **MCP** three ways (Astro MCP surface, HA custom-component MCP, **A2A discovery**, experimental **WebMCP**), exports **JSON/CSV/SQLite**, and integrates HA todo/calendar entities, automations, notifications, Grocy, Node-RED, and n8n. Same org family as Electric-Town/pobal-os.

**Verdict: interoperate, do not merge — and guard the domain boundary.**

- **Different domains, deliberately.** Meitheal answers *"what should happen next, who owns it, how do we keep it moving"* (tasks, projects, goals, habits, routines). Liaison answers *"who is this person and how do I maintain the relationship"* (relationship memory & attention). Merging blurs two clean bounded products.
- **Codebases don't merge cleanly anyway:** meitheal is TypeScript/Astro/SQLite/HA-add-on; Liaison is Rust/Tauri/Markdown. The right integration layer is **MCP + open formats**, not shared code.
- **Overlap risk to manage (DDD boundary):** both ship *Today / Calendar / Review / Timeline / Search / custom fields / saved filters / recurrence / labels*. Liaison must **not** rebuild a task engine. Clean split: **Liaison computes *why now* (relationship reasons, cadence, commitments, important dates); meitheal handles *task execution, scheduling, and notification plumbing*** when the two are connected.
- **Concrete interop wedges (via Liaison's Automation/MCP context):**
  1. Liaison emits *commitments / reminders / important dates / cadence-due* as MCP resources or A2A-discoverable tools → meitheal materialises them as **tasks / HA notifications / routines**.
  2. Liaison consumes meitheal **task completion** back as *interaction evidence* ("followed up — done").
  3. `Household` is a first-class entity in both models — a natural shared key.
  4. Both are **PWA-capable and offline-first** — aligned delivery model.
- **Mobile alignment (req #1):** meitheal's **PWA-with-offline-sync** is a proven, in-family precedent. A Liaison **PWA** path would align technology with the sibling product and is the pragmatic answer to the mobile decision — weigh against Tauri-mobile.
- **Scope:** keep as an explicit **R6+ interop spike** (MCP/A2A handshake first), never a near-term dependency. A shared "village/household" shell, if ever, belongs at the *presentation* layer only.

---

## 8. Part G — RICE-scored remediation backlog

RICE = (Reach × Impact × Confidence) ÷ Effort. Reach 1–5 (share of users), Impact {0.5,1,2,3}, Confidence 0.5–1.0, Effort in person-weeks. *Initial estimates — to be ratified with the team and recorded in `spec/`.*

| # | Workstream | R | I | C | Eff | **RICE** | Notes |
|---|---|---|---|---|---|---|---|
| 1 | **Design system + prototype IA** (hand-drawn tokens/components, Today/People shell) | 5 | 3 | 0.8 | 3 | **4.0** | Unblocks every later surface; fixes design complaint |
| 2 | **People/profile full surface** (contact points, dates, Topic Packs) — *surface existing domain* | 5 | 3 | 0.9 | 3 | **4.5** | Cheap: domain exists (Part A) |
| 3 | **Reminders + reason-only review** (Today/Overdue/This Week) — *surface `review-attention`* | 5 | 3 | 0.9 | 2 | **6.75** | Highest ROI; domain done |
| 4 | **Interactions + notes + 30-sec logging + timeline** | 5 | 3 | 0.8 | 3 | **4.0** | Table stakes |
| 5 | **Relationships: type/tier/intent/cadence + circles** | 5 | 2 | 0.8 | 2 | **4.0** | Cadence editor = req #5 |
| 6 | **Search + filtering + saved views** | 5 | 2 | 0.9 | 2 | **4.5** | Parity floor |
| 7 | **WCAG 2.2 AA remediation + CI a11y gate** | 5 | 2 | 0.9 | 2 | **4.5** | Do alongside #1–#4, not after |
| 8 | **Import/export (vCard/CSV) + Google Drive backup adapter** | 4 | 2 | 0.8 | 2 | **3.2** | req #2; keep agnostic |
| 9 | **Responsive/adaptive + mobile (PWA or Tauri-mobile) decision + build** | 4 | 2 | 0.6 | 4 | **1.2** | req #1; needs platform decision first |
| 10 | **Organisations/groups/memberships** (land PR #22) | 3 | 2 | 0.8 | 2 | **2.4** | Prereq for events wedge |
| 11 | **Events + dietary readiness** (Plan's stated first wedge) | 2 | 3 | 0.7 | 4 | **1.05** | High impact, narrow reach for personal users |
| 12 | **Business-card scan / OCR import** (local OCR) | 3 | 1 | 0.6 | 2 | **0.9** | req #4; nice-to-have |
| 13 | **AI follow-up / network-goal discovery** (R6, grant-gated) | 3 | 2 | 0.5 | 4 | **0.75** | Ozzy parity; later |
| 14 | **meitheal MCP interop spike** | 1 | 1 | 0.5 | 2 | **0.25** | req #6; future |

**Sequencing tension — RESOLVED (2026-07-18, owner decision):** the Plan names **event dietary readiness** as the first wedge (#11, RICE 1.05), but RICE for a **personal-use** launch favours **Reminders/Review → People → Interactions → Search** (#3,#2,#4,#6). **Decision: ship the personal-CRM parity core first**, then the dietary wedge. This de-risks the design system and a11y and matches RICE. The dietary wedge remains the Plan's stated operational wedge and follows the core.

**Decisions still open:** (b) mobile platform — PWA (aligns with meitheal, see Part F) vs Tauri-mobile; (c) debts/money-owed in or out of scope. Neither blocks the core PR plan below.

### Suggested PR sequence (each a bounded vertical slice)
1. **Design system + app shell** honouring prototype IA (design tokens, Home/Today/People nav) — closes the design complaint, unblocks the rest.
2. **Reminders + reason-only review UI** wired to `review-attention` (Today/Overdue/This Week).
3. **People/profile surface** wired to `profiles`/`people` (contact points, dates, first Topic Packs).
4. **Interactions + timeline + 30-sec logging.**
5. **Relationships + cadence editor + circles.**
6. **Search + saved views.**
7. **WCAG 2.2 AA gate** (axe/pa11y in CI + contrast/keyboard/zoom evidence) — ratcheted from PR 1 onward.
8. **Export + Google Drive backup adapter.**
9. **Mobile/adaptive** after the platform decision.
Land the backend DRAFTs (#21, #22, #25, #28) into these slices; **retire PR #27** and re-open it as the design-system slice (its current diff is empty).

---

## 9. Appendix — evidence references

- Shipped desktop: [`apps/desktop/ui/app.js`](apps/desktop/ui/app.js), [`index.html`](apps/desktop/ui/index.html), [`styles.css`](apps/desktop/ui/styles.css)
- Shipped CLI: [`apps/cli/src/main.rs`](apps/cli/src/main.rs)
- Domain-only capability: [`contexts/profiles/src`](contexts/profiles/src), [`contexts/review-attention/src`](contexts/review-attention/src), [`contexts/people/src`](contexts/people/src), [`contexts/connections/src`](contexts/connections/src)
- Plan: [`SPEC.md`](SPEC.md), [`spec/requirements.json`](spec/requirements.json), [`docs/product/roadmap.md`](docs/product/roadmap.md), [`preview.md`](preview.md)
- Design: [`docs/prototypes/screens`](docs/prototypes/screens), [`docs/prototypes/README.md`](docs/prototypes/README.md)
- Open PRs: #21 profile values, #22 orgs/groups, #25 backup/restore, #27 (empty) baseline+design, #28 vault journal
- Comparators: Ozzy (tryozzy.xyz), Meerkat CRM, CRM in Markdown, Monica
- Interop target: `Coolock-Village/meitheal` (private)
