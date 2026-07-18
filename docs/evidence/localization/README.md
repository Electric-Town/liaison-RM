# Localization release evidence

No non-source locale is release-ready in this directory yet.

Create one evidence record per locale and catalog version. The record must include:

- locale and catalog version;
- exact source commit;
- named reviewer and review scope;
- terminology decisions and unresolved terms;
- screenshots for desktop, 390 CSS pixels, 200% zoom, error states, dialogs, tables, graph alternatives, and print/export where relevant;
- key, placeholder, Unicode, hardcoded-string, and pseudolocale results;
- localized accessibility-name and live-region sampling;
- date, number, name, list, and currency formatter checks;
- known limitations, expiry condition, and rollback route.

Draft catalogs may contain source-language fallback text. They must remain `human-review-required` and must not appear in a production language selector. Automated translation, a language model, or structural key completeness cannot approve a locale.
