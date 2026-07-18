---
id: KCS-0008
title: How do I record relationship intent without scoring a person?
state: Draft
owner: relationships
created: 2026-07-18
reviewed: 2026-07-18
applies_to:
  - relationship intent schema 1
search_terms:
  - relationship type
  - relationship tier
  - contact cadence
  - circles
  - overdue
  - do not contact
---

# How do I record relationship intent without scoring a person?

## Context

A user wants to remember how they intend to maintain a relationship, while avoiding a misleading measure of closeness or human value.

## Answer

Store explicit relationship intent separately from interaction evidence.

Record only values the user chose:

- relationship type;
- tier;
- cadence;
- circles;
- last and next contact dates;
- reason to contact;
- last meaningful topic;
- paused-until and do-not-contact boundaries.

Use the Relationships application service rather than editing YAML directly:

```bash
liaison --workspace "$HOME/People" relationship set \
  --person-id PERSON_UUID \
  --relationship-type friend \
  --tier core \
  --cadence quarterly \
  --circle "Close friends"
```

Use the returned revision for later changes.

## Maintenance status

Liaison may describe the record as on track, due soon, due today, overdue, paused, do not contact, archived, or without cadence. The explanation must come from the configured dates, cadence, and hard boundaries.

It must not be labelled relationship strength and must not infer trust, affection, reciprocity, employee value, or importance from message volume.

## Storage

The local adapter writes `relationships/<person-id>.yaml`. Unknown top-level keys survive supported updates. Relationship facts remain readable without Liaison, but edits through the application service enforce revisions and domain rules.

## Recovery

A stale revision fails without overwriting the file. Reload the current record, compare the changed fields, and submit a new update with the current revision. Preserve an external copy before repairing malformed YAML.

## Limits

This slice does not log communication, send reminders, calculate weighted Review Priority, or synchronize providers. Those are separate bounded-context changes.
