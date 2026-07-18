# Relationships bounded context

## Purpose

Relationships owns the user's explicit maintenance intent for one Person: relationship type, tier, desired cadence, circles, next reason, last meaningful topic, pause state, and do-not-contact boundary.

It does not infer closeness, trust, affection, employee value, or relationship strength from communication volume.

## Core language

- **Relationship intent** — manually configured type, tier, cadence, circles, and boundaries.
- **Maintenance status** — an explainable state relative to the configured cadence or explicit next-contact date.
- **Circle** — a user-defined grouping label such as family, close friends, or book club.
- **Paused** — temporarily excluded until a date.
- **Do not contact** — a hard suppression that overrides cadence.

## Invariants

- One relationship-intent record exists at most once for a Person in a workspace.
- Stable Person identity comes from the shared kernel.
- Edits use optimistic revisions.
- Circle names are non-empty, normalized, and deduplicated.
- Custom cadence is between 1 and 3,650 days.
- Archived, paused, and do-not-contact states override due-date calculations.
- Maintenance status is factual and explainable; it is not a score.

## Persistence

The Markdown vault adapter stores one readable YAML record per Person under `relationships/`. The context owns semantics and revision rules; the adapter owns file translation and unknown-key preservation.

## First vertical slice

- load or create relationship intent;
- edit type, tier, cadence, circles, reason, topic, last and next contact dates;
- compute maintenance status;
- list records for People search and circle views;
- expose the same application services to CLI, desktop, API, MCP, and plugins.
