# Semantic-token contrast evidence — candidate

Status: preserved candidate evidence. It cannot enter `T-B0-P03D` or P04 token work until technical P03 acceptance, exact-artifact `T-B0-P03-OBS`, and a recorded Continue decision; no production or conformance claim.

This document and its companion files preserve one candidate-review finding ahead of P04: the candidate material identified the dark highlight pairing at about 1.77:1 and proposed theme-specific `content-on-highlight` and `current-step-content` tokens. The proposed correction is recorded here as measured arithmetic, not as a screenshot judgement or accepted design authority.

What this is not:

- It is not the P04 semantic-token registry; after accepted P03 qualification/attestation and D1-B OBS records Continue, P03D creates canonical `DESIGN.md` and P04 creates the implemented registry. This is one prepared input.
- Token **names** follow roles proposed by the preserved Editorial Ledger candidate; every **value** is provisional — taken verbatim from that candidate palette of 2026-07-19 except the two correction tokens — and may change when measured on the exact P04 build without implying that the visual direction is accepted.
- Computed ratios here are arithmetic on declared values. They are not rendering evidence: the exact-build theme matrix, forced-colours behaviour, OS preference changes, persistence, and rollback remain P04/P11 obligations on the installed application.

## Files

- [semantic-tokens.candidate.json](semantic-tokens.candidate.json) — the machine-readable candidate registry: three value sets (light, dark, high-contrast), the `system` resolution rule, required tokens, and the declared contrast pairs.
- `scripts/check_design_tokens.py` — the validator: WCAG 2.x relative-luminance contrast for every declared pair in every theme; fails on any required pair below its minimum. Its self-test first reproduces the candidate review's rejected pair so the arithmetic is anchored to the measurement that caused the finding.

## The rejected pair and its correction

| Pairing | Measured | Verdict |
|---|---:|---|
| `#17231F` content on `#514819` highlight (the candidate dark preview) | 1.77:1 | Rejected in the candidate review evidence; reproduced by the validator's self-test |
| `content-on-highlight` `#F4F1E8` on `highlight` `#514819` (dark, this registry) | 8.12:1 | Passes 4.5:1 |
| `content-on-highlight` `#202622` on `highlight` `#F2D98D` (light) | 11.08:1 | Passes 4.5:1 |
| `content-on-highlight` `#000000` on `highlight` `#FFD400` (high-contrast) | 14.67:1 | Passes 4.5:1 |

The candidate correction keeps the proposed olive highlight in dark and changes only the content placed on it: light ink, not near-black. `current-step-content` mirrors the same values so the Event stepper's current stage never inherits an unreadable pairing.

## Required-pair results

All required pairs pass in all three themes. Text pairs use a 4.5:1 minimum; focus and action-boundary pairs use 3.0:1. Full validator output is reproducible with `python3 scripts/check_design_tokens.py`; the summary:

| Required pair | Light | Dark | High-contrast |
|---|---:|---:|---:|
| content on canvas | 12.64 | 16.15 | 21.00 |
| content on surface | 15.29 | 14.38 | 21.00 |
| content on surface-subtle | 11.80 | 12.06 | 19.03 |
| content on surface-information | 13.36 | 11.03 | 19.03 |
| content-muted on surface | 6.48 | 9.00 | 16.83 |
| content-on-action on action | 7.32 | 9.61 | 11.86 |
| content-on-highlight on highlight | 11.08 | 8.12 | 14.67 |
| current-step-content on current-step | 11.08 | 8.12 | 14.67 |
| success on surface-success | 5.45 | 6.89 | 15.11 |
| warning on surface-warning | 5.14 | 8.12 | 13.29 |
| danger on surface-danger | 5.40 | 7.16 | 8.48 |
| focus on canvas (3.0) | 4.59 | 12.65 | 14.67 |
| focus on surface (3.0) | 5.56 | 11.27 | 14.67 |
| action on surface (3.0) | 7.26 | 8.61 | 11.86 |

## Findings for the P03D consultation

Informational pairs the validator flags rather than fails — recorded as findings, not silently corrected, because the values come from the candidate palette:

1. **Routine borders are below 3.0:1 against surfaces** — light `border` 1.79:1, dark `border` 2.55:1. Acceptable only while borders are never the sole indicator of a boundary or state; the Editorial Ledger direction uses structure, text, and the strong border for meaning. The consultation should either accept that rule explicitly or adjust the border values.
2. **The highlight surface itself is low-contrast against work surfaces** — light 1.38:1, dark 1.77:1 (highlight-as-area versus surface, distinct from the corrected content-on-highlight pairing). The current-step and selection treatments therefore cannot rely on the highlight fill alone; `aria-current`, text labels, and the strong border carry the state. Same decision point as finding 1.
3. **`system` is a resolution rule, not a palette** — the registry records it as such; the four built-ins claim in P04 evidence must cover the OS-change transition explicitly.
4. **High-contrast values are candidate placeholders more than the others**: the preserved candidate specifies high-contrast sparsely, so several surface tokens reuse `#101010`. The consultation should confirm or replace them, and forced-colours behaviour is untested arithmetic-free territory until the exact build.

## Candidate typography roles and provenance obligations

These roles are non-authoritative inputs from preserved candidate review material; nothing here vendors a font or commits P03D/P04 to these assignments:

| Font | Role |
|---|---|
| Atkinson Hyperlegible Next | Body text, controls, forms, tables, navigation, dialogs, drawers, operational headings, accessibility-critical copy |
| Source Serif 4 | Page titles and non-operational editorial section headings only |
| IBM Plex Mono | Provenance only: file references, operation receipts, hashes, technical identifiers |

Open P04 merge requirements, recorded so they are not rediscovered: exact font versions; licence files; package hashes; fallback metrics; script-coverage evidence; no Japanese bundle claim for B0.

## Reproduction

```text
python3 scripts/check_design_tokens.py
→ self-test: rejected review pair measures 1.77:1 (rejected, as expected)
→ … per-theme pair table …
→ Design-token check passed
```
