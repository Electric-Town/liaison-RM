---
id: KCS-0007
title: How do I use the hand-drawn desktop design system?
state: Draft
owner: desktop
created: 2026-07-18
reviewed: 2026-07-18
applies_to:
  - desktop alpha
search_terms:
  - hand drawn UI
  - sketchbook design
  - wobbly borders
  - hard shadow
  - desktop CSS tokens
---

# How do I use the hand-drawn desktop design system?

## Context

A contributor needs to add or change a desktop component without returning the interface to conventional corporate styling or introducing inaccessible visual noise.

## Answer

Use `apps/desktop/ui/design-system.css` before writing component-specific CSS.

1. Use the paper, ink, marker-red, pen-blue, muted-paper, and post-it tokens.
2. Use a shared wobbly radius rather than a standard rounded rectangle.
3. Use a two- or three-pixel pencil border.
4. Use a hard offset shadow with no blur.
5. Keep decoration optional and non-semantic.
6. Keep controls at least 48 pixels high and preserve visible focus.
7. Verify reduced motion and the 390-pixel layout.
8. Do not fetch fonts, textures, or icons from the network.

## Why

The design should feel like a working relationship notebook while remaining predictable, local, and operable. Central tokens prevent one-off styling from turning intentional irregularity into inconsistency.

## Verification

Run:

```text
python scripts/check_desktop_shell.py
CHROMIUM_PATH=/usr/bin/chromium python scripts/test_desktop_ui.py
node --check apps/desktop/ui/app.js
```

Inspect the generated desktop and mobile screenshots. Check that labels, focus, contrast, status, and structure remain understandable without color, rotation, or decorative elements.

## Limits

The repository does not bundle font binaries in this slice. Preferred handwritten fonts are used when installed; local handwriting-oriented fallbacks preserve the general visual direction without creating a remote dependency.
