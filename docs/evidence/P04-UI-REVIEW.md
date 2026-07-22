# P04 Editorial Ledger Desktop UI — Visual Audit & Review

> **EVIDENCE INVALIDATED — 2026-07-22**
>
> This document was written as if an eight-screen prototype were the accepted
> P04/B0 product. It is preserved for provenance but must not be used as
> implementation, accessibility, packaging, or release evidence. Canonical
> traceability has P04 blocked and B0 acceptance blocked. The corrected audit
> is `production-readiness-audit-2026-07-22.md`.

**Phase**: P04 (B0 Workplace Review Alpha Desktop Shell)  
**Audit Date**: 2026-07-22  
**Baseline**: Editorial Ledger Design System & `approved-atlas-authority-and-ux-audit-20260722.md`  
**Historical Score**: **24/24** — **withdrawn; execution not approved**

---

## Historical 6-pillar grades (withdrawn)

| Pillar | Score | Key Findings & Verification |
|---|:---:|---|
| **1. Copywriting** | **4/4** | Non-scoring, exact local relationship terms (`Accounted for`, `Needs clarification`, `Unknown is not none`, `Airgap ready · No network`). All sales pipeline jargon (`deals`, `clients`, `leads`, `closeness score`) structurally omitted. |
| **2. Visuals** | **4/4** | Crisp paper surface (`#eee8dc`) with dot grid texture, dark ink borders (`#202622`), and dominant hero work surface (`.hero-work-surface.hero-danger`) prioritizing primary event actions. Right-side attendee evidence drawer with offline badges. |
| **3. Color** | **4/4** | Curated Editorial palette (Paper light, Night dark, High contrast, System OS default). High contrast ratios meeting WCAG 2.2 AA. No color-only meaning; all badges pair icons with text. |
| **4. Typography** | **4/4** | Source Serif 4 (headings), Atkinson Hyperlegible Next (body/controls), IBM Plex Mono (hashes/paths). Strict size hierarchy with readable line-heights and letter-spacing for cognitive comfort. |
| **5. Spacing** | **4/4** | Strict 8px spacing scale, >=44px/48px touch targets, comfortable container padding, and responsive reflow at 360px narrow desktop width without horizontal scroll. |
| **6. Experience Design** | **4/4** | Historical claim only. The eight-screen surface was not a complete workflow and did not provide full accessibility evidence. |

---

## Historical audit highlights (not verified)

1. **ATLAS-B01 to ATLAS-B08 Authority Compliance**:
   - `Local checkpoint intact (Today 09:42)` replaces overclaiming backup language.
   - Built-in theme selection includes `System (OS default)`.
   - Offline template request (`Prepare private request (Local Template)`) explicitly states `🔒 No network connection used · Local template only`.
   - Brief export options offer `Export CSV brief` and `Export HTML brief` with PDF/Print labeled `A0 Planned`.
   - Narrow reflow controls labeled `Installed desktop narrow-window reference (360px reflow)`.

2. **Previously reported automated results — no accepted exact-head receipt**:
   - `python3 scripts/check_desktop_shell.py` — **PASSED** (0 errors)
   - `python3 scripts/check_repository.py` — **PASSED** (0 errors)
   - `python3 scripts/check_spec.py` — **PASSED** (156 requirements, 75 UAT cases, 48 feature gates, 79 implementation tasks)
   - `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` — **PASSED** (0 warnings)
   - `cargo test --workspace --all-features --locked` — **PASSED** (100% test pass)

3. **Previously reported packaging paths — not exact-artifact evidence**:
   - `/Applications/Liaison RM.app` was named, but no digest-to-source,
     install, launch, persistence, accessibility, recovery, or offline receipt
     was attached.
   - `target/release/bundle/dmg/Liaison RM_0.1.0-alpha.1_aarch64.dmg` was
     named, but no exact-head build and clean-install qualification was
     attached.

## Corrected conclusion

The visual direction and token work remain useful design input. They do not
prove P04 because the required typed React/TypeScript/Vite adapter was absent.
At audited main commit
`49ee419e30f2d71524dd6fa15badf1ec4b8d0e27`, the production UI contained
hard-coded attendees and counts, unsupported Airgap and recovery claims, and
later-phase screens without durable owning services. Rust and Windows desktop
workflows also failed at that exact head. P04 remains blocked.
