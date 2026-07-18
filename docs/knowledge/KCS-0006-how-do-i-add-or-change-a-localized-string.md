---
id: KCS-0006
title: How do I add or change a localized string?
state: Draft
owner: localization
created: 2026-07-18
reviewed: 2026-07-18
applies_to:
  - locale catalogues
  - desktop interface
  - public documentation
search_terms:
  - localization
  - translation
  - pseudolocale
  - placeholder
  - Irish
  - Japanese
  - Portuguese
related_requirements: []
related_uat: []
related_adrs: []
---

# How do I add or change a localized string?

## Context

A contributor needs to add visible text, change wording, introduce an interpolation placeholder, or provide a translation.

## Answer

1. Add or change the key in the `en-IE` source catalogue.
2. Keep the key stable and language-neutral. Do not encode English wording in the key.
3. Add translator context when the label is ambiguous, describes an icon-only action, or contains a placeholder.
4. Update the expansion pseudolocale and every checked-in locale so key and placeholder parity remain exact.
5. Run `python scripts/check_localization.py`.
6. Exercise the affected screen with at least 45% text expansion and at 390 CSS pixels.
7. Obtain the required human language review before changing a locale from `human-review-required` to an approved state.
8. Record screenshots, unresolved terminology, source revision, and reviewer scope in release evidence.

## Rules

- Natural-language UI text does not belong in Rust, HTML, JavaScript, provider descriptors, or schemas when it is user-facing and localizable.
- Placeholder names are compatibility surfaces and must carry the same meaning in every locale.
- Dates, numbers, currencies, units, lists, and relative time use locale-aware formatters rather than string concatenation.
- Draft translations remain labelled as drafts.
- Machine translation can propose text but cannot approve production language.
- Do not introduce errors, forced irregularity, phonological tricks, or stylometric patterns to make text appear human.

## Public metadata

JSON-LD, Open Graph, `hreflang`, `inLanguage`, h-card, and h-entry apply only to explicitly public website surfaces. Private vault content and relationship records must never be copied into public structured metadata.

## Recovery

When a key was hardcoded or removed without deprecation:

- restore or map the stable key;
- search interfaces, plugins, tests, and documentation for downstream use;
- add catalogue parity and layout tests;
- document any irreversible wording or placeholder migration.
