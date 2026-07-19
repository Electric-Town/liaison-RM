# Organisations and Groups bounded context

## Purpose

Organisations owns organisations, groups, households, locations, and effective-dated memberships with provenance. It does not own person profiles, dietary facts, event cohorts, persistence formats, or import mechanics.

This crate adapts the preserved `agent/r4-organizations-groups-domain` branch under its recorded disposition ("selectively transplant domain concepts later"), renames its vocabulary to the context map's language, and extends it with locations, membership provenance, and as-of snapshot queries required by `LRM-OR-001` and `LRM-OR-002`.

## Language

- **Organisation** — a named company, nonprofit, school, government body, community, club, vendor, client, service provider, or informal body with a stable identifier.
- **Location** — a stable office, site, campus, or venue record, optionally belonging to an organisation.
- **Group** — a static list, event snapshot, household, or team.
- **Membership** — a dated link from a person to an organisation or group, carrying role, department, cost centre, location, primary flag, source, and record date.
- **Membership source** — the required provenance label for a membership: an import job, manual entry, or directory sync.
- **Snapshot** — the set of memberships effective for a person on a given date.

## Current invariants

- Organisations, locations, and groups require non-empty names; identifiers are stable and independent of those names.
- Department, cost centre, and location are typed membership fields, so people can be filtered by each dimension without an unvalidated free-text tag.
- A membership cannot end before it starts.
- Every membership records its source and record date.
- Memberships are append-only in practice: a department move is a new membership, so `memberships_as_of` returns what was true on any past date and historical reports keep their applicable snapshot.
- Concurrent memberships (employment plus a team, or two roles) coexist in one snapshot.
- Archiving an organisation or location is reversible and never deletes membership history.

## Application services

None yet. Directory onboarding, staged import, cohort predicates, and their persistence belong to the owning Directory tasks; this crate supplies the domain rules they call.

## Outbound ports

None yet. Markdown or projection storage for organisations, locations, groups, and memberships is an adapter concern.

## Cross-context rules

- People owns `PersonId` and person records; this context references people by identifier and never mutates a profile.
- Events consumes memberships to build cohorts; cohort finalisation, attendance, and readiness stay in Events.
- Filtering dimensions defined here (organisation, location, department, team, cost centre, role, group) are the vocabulary cohort predicates use.

## Data classification

Organisational structure and memberships are sensitive workplace data. Nothing in this context carries dietary, accessibility, or health information, and it holds no productivity, performance, attendance-compliance, or risk signal of any kind.

## Tests

`cargo test -p liaison-organisations` covers name and identifier validation, membership date rejection, provenance retention, department-move history with as-of snapshots, and concurrent memberships in one snapshot.
