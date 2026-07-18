---
id: KCS-0005
title: Why does Liaison RM not use a relationship-strength score?
state: Draft
owner: review-and-attention
created: 2026-07-18
reviewed: 2026-07-18
applies_to:
  - relationship intent
  - maintenance status
  - review priority
search_terms:
  - relationship strength
  - review priority
  - message volume
  - social credit
  - overdue contact
related_requirements:
  - LRM-RA-001
  - LRM-RA-002
  - LRM-RA-003
related_uat:
  - UAT-RA-001
  - UAT-RA-002
related_adrs: []
---

# Why does Liaison RM not use a relationship-strength score?

## Context

A contributor wants to order a review queue or show the state of a relationship and proposes a single numeric “relationship strength” value.

## Answer

Do not implement a relationship-strength score.

Separate the problem into:

- **relationship intent**, entered by the user;
- **relationship evidence**, recorded or imported as facts;
- **maintenance status**, calculated relative to the user’s cadence and boundaries;
- **profile readiness**, calculated for a named purpose;
- **Review Priority**, an optional, transparent queue-ordering value.

Personal workspaces default to reason-only review. Weighted Review Priority is opt-in and must expose each component, source fact, suppression, policy version, and explanation.

## Why

Communication frequency is not a defensible proxy for closeness, trust, affection, reciprocity, or importance. Some healthy relationships are infrequent. Some high-volume communication is purely operational. A sparse profile may reflect privacy, not low importance.

A single score encourages false precision, guilt, employee ranking, opaque automation, and social-credit behaviour.

## Acceptable wording

> Quarterly cadence is 18 days overdue. One commitment remains open.

## Unacceptable wording

> Relationship strength: 42%.

## Guardrails

Review and Attention must not:

- infer human worth from message counts;
- rank employees;
- expose private assessments through shared workspaces;
- send messages automatically;
- shame the user for overdue contact;
- assume that every relationship needs a cadence.

## Recovery

When an interface or plugin already calculates “strength,” remove the calculation, preserve only source facts that have a legitimate purpose, migrate configured intent into explicit fields, and replace the output with reason-based explanations.
