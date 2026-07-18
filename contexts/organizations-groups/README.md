# Organizations and Groups bounded context

## Purpose

Own Organizations, Groups, Households, Locations, and dated Memberships without collapsing them into a generic contact record.

## Invariants

- Person, Organization, Group, Household, Location, Event, and Resource remain distinct entity types;
- a Membership is a dated association and may coexist with other current or historical memberships;
- changing one current role never overwrites role history on the Person profile;
- organization and group labels are not identifiers;
- hierarchy cycles are rejected;
- archive preserves stable IDs and history;
- workplace membership data cannot be reused as an employee-performance or relationship-value score.

## First executable slice

The first slice will add:

- stable Organization, Group, Location, and Membership identifiers;
- Organization and Group aggregates;
- dated Person-to-Organization and Person-to-Group Memberships;
- repository ports and application services;
- readable Markdown/YAML records;
- CLI create/list commands;
- as-of-date membership filtering;
- duplicate and invalid-date rejection;
- workspace validation and cross-platform tests.

Household-specific fields, organization hierarchy, dynamic groups, directories, graph projections, imports, and desktop screens remain later slices.

## Dependency direction

This context may reference `PersonId` from the shared kernel. It does not import People persistence or mutate Person records. Interfaces and adapters call its application services; they do not own membership rules.
