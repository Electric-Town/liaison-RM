# Customisation bounded context

## Purpose

Customisation owns user-controlled configuration as classified settings entries: the settings-only bundle review, diff preview with explicit conflict choices, and revisioned apply and rollback semantics. It does not own person records, workspace authority, canonical storage formats, provider transport, or any user interface.

This crate is parallel preparation under decision 0014 for the A0 G2c settings work owned by `T-A0-P01` (`LRM-WS-013`, `LRM-WS-014`, `UAT-050` under `FG-A0-G2C`). It ships on a draft pull request that stays draft until that milestone opens.

## Provisional format

`PROVISIONAL_BUNDLE_FORMAT` labels every bundle. The canonical serialized bundle layout, its schema validation, and its migration story are owned by A0 G2c and may replace this shape entirely. The durable content of this crate is the invariants and tests, not the format.

## Language

- **Settings class** — the seven bundle-carriable classes from `LRM-WS-013`: layouts, fields, packs, templates, views, appearance, policies.
- **Setting key** — a namespaced dot-separated identifier; an unknown first segment is an unknown class, preserved when its content is safe.
- **Bundle review** — the adversarial pass over a bundle: rejected entries are listed with reasons, never silently dropped; only accepted entries reach preview and apply.
- **Unsafe content** — record identifiers, secret-like keys or values, and absolute paths. Detection is a deterministic deny floor: rejection is final, passing is not approval.
- **Diff preview** — the dry run; built structurally from a review's accepted entries only.
- **Conflict choice** — an explicit keep-current or take-bundle decision required for every changed entry before a plan exists.
- **Rollback point** — the exact pre-apply state, restorable verbatim.

## Current invariants

- Rejected content cannot be previewed or applied: previews are constructed from accepted entries only.
- A changed entry without an explicit conflict choice cannot enter an apply plan.
- Apply is pure: it returns the next state and an exact rollback point and never mutates its input, so partial mutation is impossible by construction.
- Unknown safe keys survive review, preview, apply, and export round trips.
- Exported bundles come only from reviewed state, so they contain no unsafe literals; a test walks every exported value through the detector.
- Secret-like key names are rejected regardless of their values.

## Application services

None yet. Command wiring, persistence, and the clean-device import journey belong to A0 G2c; this crate supplies the domain rules they will call.

## Cross-context rules

- Workspace owns where settings live and how they are stored; this crate never touches files.
- Appearance entries are classified settings values here; the semantic token registry and theme contract belong to P04 and are not referenced.
- Nothing in this context can carry a person record, secret, token, cache, or device path — that is the point of the review.

## Data classification

Settings entries are configuration, not personal data. The adversarial review exists precisely to keep personal identifiers, secrets, and machine paths out of bundles that users may move between devices.

## Tests

`cargo test -p liaison-customisation` covers the adversarial rejection listing (record identifier, absolute path, secret-like key and value), unknown-safe-key round trips, mandatory conflict choices, pure apply with exact rollback, export safety, and key/value validation — mirroring the `UAT-050` acceptance language.
