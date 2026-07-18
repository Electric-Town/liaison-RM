# Localization and language-quality standard

## Scope

This standard applies to desktop, CLI, local browser, documentation, public website, generated exports, error messages, accessibility names, and plugin-contributed interface text.

The initial locale architecture recognizes:

- `en-IE` as the source locale;
- `ga-IE` for Irish;
- `ja-JP` for Japanese;
- `pt-BR` for Brazilian Portuguese;
- `en-XA` as a non-production expansion and layout-test locale.

A locale is not release-ready merely because every key has a value. It needs named human review, product-context review, layout evidence, accessibility review, and explicit status.

## Catalog contract

1. Every user-facing string has a stable semantic key.
2. Natural-language strings do not appear directly in UI source except test fixtures and explicitly allowlisted technical tokens.
3. Catalog values are UTF-8 and normalized to NFC during validation.
4. Placeholders use named braces such as `{person_name}`. Every locale preserves the exact placeholder set.
5. HTML is not stored in ordinary catalog values. Rich messages use documented structured components.
6. Error codes are stable and untranslated; their visible titles, explanations, and recovery steps are localized.
7. Dates, numbers, units, lists, names, relative times, and currencies use locale-aware formatters rather than concatenated strings.
8. The source catalog includes context, character, and accessibility notes where ambiguity would affect translation.
9. Plugin catalogs are namespaced and cannot replace core keys without an explicit compatibility contract.
10. Removing a key requires a deprecation period and usage audit.

## Layout resilience

Interfaces must tolerate at least 45% expansion for ordinary labels and substantially more for error and recovery prose. Fixed heights cannot contain natural-language text. Components use wrapping, flexible grids, logical CSS properties, and testable overflow behavior.

The layout test suite covers:

- 320 and 390 CSS-pixel widths;
- 200% zoom and text-only zoom where supported;
- long names and organization titles;
- right-to-left readiness even before an RTL locale is released;
- Japanese line-breaking rules;
- Irish and Portuguese text expansion;
- keyboard focus and visible errors after reflow;
- table, graph, dialog, toast, menu, form, and print/export states.

## Irish review

Irish strings require a named fluent reviewer. Review covers terminology, noun gender and case, initial mutations, numbers, articles, adjective agreement, register, punctuation, and consistency with the product glossary. A translation-memory or language model can propose text but cannot approve it.

Do not manufacture irregularity, errors, unusual phonology, or forced mutations to make prose appear human. Stylometric patterns are not reliable authorship proof and are not a quality gate. The repository checks clarity, context, grammar, terminology, evidence, and accountable review instead.

## Japanese review

Japanese UI defaults to an appropriate polite product register. Review covers line breaking, punctuation, date and name order, counters, truncation, mixed Latin/Japanese text, and whether the label fits the actual action. CSS uses appropriate language-aware line-breaking behavior rather than inserting spaces.

## Brazilian Portuguese review

Brazilian Portuguese uses an accessible professional register. Review covers gender and number agreement, verb form, terminology, pronoun treatment, date and number formatting, and text expansion. Product terms are chosen for Brazilian usage rather than copied from European Portuguese.

## Public semantic metadata

JSON-LD, Open Graph, `hreflang`, `inLanguage`, and IndieWeb metadata apply to the public documentation or marketing website where relevant. They are not injected into the local desktop vault or relationship records. Visible page language, metadata, canonical URL, and structured data must agree.

## Accessibility

Localized accessibility names communicate the same action and state as visible labels. Translators receive context for abbreviations, icon-only controls, form errors, live regions, table headers, graph alternatives, and status messages. A locale cannot pass while its screen-reader text remains in the source language.

## Evidence required for release

A locale evidence record identifies:

- locale and catalog version;
- source commit;
- reviewer and review scope;
- unresolved terms;
- screenshot and overflow matrix;
- automated key, placeholder, Unicode, and hardcoded-string results;
- accessibility sampling;
- known limitations and expiry condition.

The project must not claim certification or native-quality translation without the corresponding evidence.
