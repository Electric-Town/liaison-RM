# Localization and language-quality standard

## Scope

This standard applies to desktop, CLI, local web, public documentation, release notes, schemas with visible labels, and plugin-contributed interface text.

Localization is an interface architecture. It does not change stable person, field, policy, provider, or plugin identifiers.

## Source and catalogue model

- `en-IE` is the initial source catalogue.
- Every catalogue declares schema version, locale, source locale, source revision, review status, and messages.
- `en-XA` is an expansion pseudolocale used to expose clipping, fixed-height controls, missing keys, and concatenated prose.
- Draft catalogues remain `human-review-required` until a named reviewer records evidence.
- Plugin keys use a namespace controlled by the plugin and cannot silently replace core keys.
- Removing a key requires a deprecation period and a downstream usage audit.

## String rules

1. Stable keys describe meaning, not English wording.
2. User-facing prose is not assembled from fragments that translators cannot reorder.
3. Placeholder names and types remain identical across locales.
4. Error messages include the failed action and a recovery route.
5. Visible labels, accessible names, descriptions, status announcements, and recovery steps are localized together.
6. Dates, partial dates, numbers, units, lists, currencies, and relative times use locale-aware formatters.
7. Translator context identifies the screen, action, character, privacy sensitivity, and placeholder meaning where relevant.
8. Source strings use synthetic examples and never contain real vault data.
9. Translation tooling cannot transmit private workspace text without an explicit connector grant.

## Layout resilience

Interfaces must tolerate at least 45% expansion for ordinary controls and substantially more for error and recovery prose. Fixed heights must not contain natural-language text.

Review includes:

- 320 and 390 CSS-pixel widths;
- 200% zoom and text-only zoom where supported;
- long names and organization titles;
- keyboard focus after reflow;
- dialogs, menus, forms, tables, graphs, toasts, print, and export states;
- Japanese language-aware line breaking;
- Irish and Brazilian Portuguese expansion;
- reduced-motion and high-contrast operation;
- semantic alternatives to visual-only relationships.

## Irish review

Irish strings require a named fluent reviewer. Review covers terminology, noun gender and case, initial mutations, articles, numbers, adjective agreement, register, punctuation, and glossary consistency.

Automated or model-generated suggestions cannot approve production Irish. Do not manufacture irregularity, errors, unusual phonology, forced mutations, or stylometric patterns to make text appear human. Grammar, clarity, purpose, terminology, and accountable review are the quality gates.

## Japanese review

Japanese UI uses an appropriate polite product register. Review covers line breaking, punctuation, date and name order, counters, truncation, mixed Latin and Japanese text, and whether a label accurately describes the action. CSS uses language-aware line-breaking behavior rather than inserting spaces.

## Brazilian Portuguese review

Brazilian Portuguese uses an accessible professional register. Review covers gender and number agreement, verb form, terminology, pronoun treatment, date and number formatting, and expansion. Product terms must reflect Brazilian rather than European Portuguese usage.

## Public semantic metadata

JSON-LD, Open Graph, `hreflang`, `inLanguage`, h-card, and h-entry apply only to explicitly public documentation or marketing surfaces. They do not belong in the local vault or private relationship records.

Visible page language, metadata, canonical URL, and structured data must agree. Private names, notes, events, resources, or interaction text must not enter public metadata.

## Accessibility

Localized accessibility names communicate the same action and state as visible labels. Translators receive context for icon-only controls, form errors, live regions, table headers, graph alternatives, and status messages. A locale cannot pass while screen-reader text remains in the source language.

## Release evidence

A locale evidence record identifies:

- locale and catalogue version;
- source commit and source catalogue revision;
- reviewer and review scope;
- unresolved terminology;
- screenshot and overflow matrix;
- key, placeholder, Unicode, and hardcoded-string results;
- accessibility sampling;
- known limitations and the condition that invalidates the evidence.

The project must not claim native-quality translation, certification, or language approval without the corresponding evidence.
