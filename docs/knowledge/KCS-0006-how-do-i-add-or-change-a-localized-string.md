---
id: KCS-0006
title: How do I add or change a localized string?
state: Draft
owner: localization
created: 2026-07-18
reviewed: 2026-07-18
applies_to:
  - desktop UI
  - CLI
  - local browser
  - documentation website
search_terms:
  - i18n
  - locale catalog
  - hardcoded string
  - translation
  - placeholder
related_requirements:
  - LRM-L10N-001
  - LRM-L10N-002
  - LRM-L10N-003
  - LRM-L10N-004
related_uat:
  - UAT-LOC-001
  - UAT-LOC-002
---

# How do I add or change a localized string?

## Procedure

1. Choose a stable semantic key under the owning surface namespace.
2. Add the source `en-IE` value with enough context for a translator to understand the user, action, object, state, and tone.
3. Use named placeholders. Do not concatenate fragments to build a sentence.
4. Regenerate or update `en-XA` and inspect expansion, wrapping, keyboard focus, and accessible names.
5. Add translations only through the locale review workflow. Mark untranslated or unreviewed entries explicitly; do not silently copy English into a released locale.
6. Run `python scripts/check_localization.py` and the surface-specific UI tests.
7. Attach screenshots and named human-review evidence for any locale being promoted to release-ready.

## Placeholder example

Source:

```json
"review.why.open_commitment": "Open commitment with {person_name}"
```

Do not split this into `Open commitment with` plus a name. Word order and grammar differ across languages.

## Recovery

When a key is missing, the product uses the source locale and emits a local diagnostic. It does not send catalog content, user data, or missing-key telemetry to Electric Town.
