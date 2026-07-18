# Topic Packs and field-state contract

## Decision

Liaison RM uses versioned Topic Packs rather than a single fixed contact form. A Topic Pack defines typed fields, display guidance, purpose-readiness participation, classification defaults, staleness policy, and optional plugin provenance.

Stable field IDs are separate from labels. A label, layout, language, or help text may change without changing the canonical field identity used by files, saved views, policy rules, imports, APIs, or plugins.

## Assignment scopes

A pack may be enabled for:

- the workspace;
- a profile template;
- an organization or group;
- a person;
- a plugin contribution;
- a temporary purpose such as an event, trip, or executive briefing.

More specific assignments may add fields or change display policy. They cannot silently weaken classification, disclosure, retention, or provenance requirements.

## Field types

The initial contract supports short text, long text, Markdown, date, partial date, recurring date, enum, multiple selection, boolean, number, measurement, address, location, entity reference, file reference, URL reference, sealed value, and calculated read-only value.

## Information states

An absent scalar is not enough to represent personal information. A field value records one of:

- known;
- verified;
- unverified;
- unknown;
- not applicable;
- declined to disclose;
- stale;
- conflicting;
- needs clarification;
- derived.

A field may additionally record source, author or connector, capture date, verification date, review date, classification, visibility, confidence, purpose, expiry, and change history.

`declined to disclose` and `unknown` are not errors. They are valid states that prevent an interface or import from interpreting an empty value as “none.”

## Purpose-specific readiness

There is no universal profile-completeness percentage. Readiness is evaluated against a named purpose and versioned requirement set.

Examples include basic contact, meeting briefing, executive travel, event catering, birthday preparation, CardDAV export, and emergency contact.

A readiness report lists satisfied requirements, unresolved requirements, stale values, conflicting values, excluded fields, and the source revisions used. Changing a relevant source marks the report stale; it does not rewrite historical evidence.

## Extensibility

Plugin-contributed packs use a namespace owned by the plugin or organization. Removing a plugin preserves canonical values and schema references. Unknown fields and packs remain readable and survive round trips even when their rendering plugin is unavailable.

## Disclosure boundary

Operational views may expose a purpose-limited instruction derived from a more sensitive field. The detailed source value remains protected by its own classification and grant. An event-catering view, for example, may disclose an approved catering instruction without disclosing diagnosis or private medical detail.
