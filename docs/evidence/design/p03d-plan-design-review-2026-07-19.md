# P03D plan design review

Date: 2026-07-19
Reviewed task: `T-B0-P04`
Inputs: `DESIGN.md` 1.0.0, P03 operation contract, P04 implementation task, candidate token evidence, Events state contract
Decision: approved with the amendments recorded in `docs/product/p04-amended-plan.md`

## Review questions and outcomes

| Question | Outcome |
|---|---|
| Does the plan use one application contract? | Yes. Generated or compile-checked DTOs are a first slice and a merge gate. |
| Does it preserve P02/P03 authority and recovery? | Yes. React is an inbound adapter and operation presentation binds to P03 states. |
| Is the visual direction selected rather than left open? | Yes. Editorial Ledger is canonical. |
| Does it meet `LRM-UX-012`? | The plan establishes one semantic/component contract for system, light, dark, and high contrast. Final B0 journey evidence remains with P11/ACCEPT. |
| Does it meet `UAT-062` now? | No claim. P04 builds the ratchet; P11/ACCEPT run the installed five-stage journey. |
| Is localization structural? | Yes. Locale keys, en-XA, locale formatting, and script behavior are merge gates. |
| Is migration reversible? | Yes. The legacy shell remains until parity passes and no canonical format changes. |
| Are Directory and Events prematurely implemented? | No. P04 creates route/component seams; P05–P11 own behavior. |
| Are accessibility and installed evidence explicit? | Yes. Keyboard, focus, VoiceOver smoke, 400% reflow, reduced motion, themes, and artifact provenance are named. |
| Is bundle/network authority bounded? | Yes. No remote assets or runtime network dependency is permitted. |

## Required plan amendments accepted

1. Add generated/compile-checked DTOs before feature migration.
2. Add explicit parity phase and reversible legacy-shell switch.
3. Bind operation presentation to every P03 phase and error class.
4. Promote the semantic token registry and component contract to versioned artifacts.
5. Define exact route identities and capability-honest visibility.
6. Require en-XA and localization-key audits on every user-facing change.
7. Require installed universal Mac evidence before P04 completion.
8. Keep final five-stage theme/VoiceOver acceptance with P11 and B0 acceptance.

## Residual risks

- Tauri/WebView behavior can diverge from browser fixtures; installed evidence is mandatory.
- Generated contract tooling can become a second source of truth; drift checks must fail closed.
- React can encourage presentation-local business rules; architecture and contract tests must reject them.
- Virtualized Directory tables can harm accessibility; the first scalable implementation needs semantic row-count, focus, and screen-reader evidence.

## Approval boundary

This review approves implementation of P04 after P03 and P03D merge. It does not approve P05–P11 behavior, B0 qualification, A0, provider work, public signing, or conformance claims.
