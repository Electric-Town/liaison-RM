# Context map

## Relationships

```text
Workspace
  supplies workspace identity, members, profile, and schema version to all contexts

People <-> Organisations
  People consumes effective memberships through an anti-corruption layer
  Organisations never owns personal notes or relationship priority

People <-> Relationships
  Relationships references stable person IDs and publishes relationship-state changes
  People does not embed relationship histories

People/Relationships/Organisations -> Interactions
  Interactions references participants and sources; it owns communication history

People/Organisations/Interactions -> Events
  Events resolves cohorts and records attendance snapshots
  Events reads dietary operational views but does not rewrite People records

People/Organisations -> Facilities
  Facilities resolves badge identities through explicit mappings
  Facilities publishes bounded summaries, not raw domain ownership

People/Relationships/Events -> Reminders
  Reminders references source records and owns due/completion policy

All business contexts -> Connections
  Contexts request capabilities through application interfaces
  Connections owns provider identity, grants, jobs, and conformance

All business contexts -> Sharing
  Sharing transports authorised operations and materialises scoped views
  Sharing does not redefine context invariants

All business contexts -> Automation
  Automation exposes approved application commands and queries
  API, MCP, AI, and plugins cannot bypass context services
```

## Integration patterns

- Stable UUIDs cross context boundaries.
- Published events use versioned envelopes and context-owned payloads.
- Read models may combine public events for dashboards and search.
- Provider, legacy CRM, vCard, calendar, email, and access formats enter through anti-corruption layers.
- Cross-context writes are orchestrated by an application workflow with explicit compensation or recovery; no distributed aggregate is assumed.

## Prohibited coupling

- Importing another context’s private module or persistence model.
- Sharing a mutable aggregate instance across contexts.
- Letting a provider or UI choose business invariants.
- Treating SQLite foreign keys as the context map.
- Using a generic `metadata` map to avoid defining owned concepts.

## Evolution

A new context or material boundary change requires a decision record. The record names upstream/downstream relationships, event and API compatibility, migration, data classification, and stewardship.
