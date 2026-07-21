# Accessible component system

Status: stacked P04.2 draft. These primitives are not yet the active Tauri interface.

## Purpose

The component boundary translates the canonical `DESIGN.md` semantics into reusable React controls without moving domain behavior into the browser. Components own structure, keyboard behavior, focus affordance, state announcement, semantic-token styling, and responsive geometry. They do not own Workspace authority, validation, readiness, recovery decisions, or relationship scoring.

## Public primitives

- `Button` — primary, secondary, danger, and quiet actions with an enforced busy state.
- `Field` — visible label, hint, input, invalid state, and error association.
- `RouteNavigation` — the stable B0 route identities with native buttons and `aria-current`.
- `StatusBanner` — polite progress, assertive conflict/failure, recovery text, and optional receipt.
- `Surface` — semantic work areas with default, raised, or quiet emphasis.

## Rules

1. Use semantic tokens from the application layer; do not add raw colours to component CSS.
2. Keep primary interactive targets at least 48 CSS pixels high.
3. Do not rotate, skew, animate, or resize operational controls to express state.
4. Every state needs text; colour, border, and shadow are supplemental.
5. Blocking conflict and failure may use `role="alert"`; ordinary progress uses `role="status"`.
6. A busy action is disabled against repeated activation and exposes `aria-busy`.
7. A field error is associated with the input and never replaces the visible label.
8. Route navigation uses the stable route identifiers from the versioned application contract.
9. Forced-colours and reduced-motion behavior are mandatory.
10. Components may display typed application outcomes but may not derive domain outcomes.

## Review checks

```text
python3 scripts/check_component_contract.py
npm run typecheck
npm run test
npm run build
```

The component contract rejects raw colours, rotated operational geometry, missing current-route semantics, missing field associations, missing busy behavior, and missing forced-colours or reduced-motion handling.
