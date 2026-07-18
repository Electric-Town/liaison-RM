# People bounded context

## Purpose

People owns a human profile and the provenance of facts about that person. It does not own relationship history, interactions, events, organisational structures, provider address books, or workspace membership.

## Language

- **Person** — a human represented by a stable ID.
- **Profile** — current person information and field provenance.
- **Contact point** — a typed email address, phone number, postal address, URL, or messaging handle.
- **Important date** — a birthday, anniversary, or user-defined date; a birthday may omit its year.
- **Archive** — reversible removal from active workflows.
- **Dietary requirement** — a later R3 value object containing kind, coverage, instruction, verification, and disclosure policy.

## Current R1 invariants

- Person identity is independent of filename and contact points.
- Display name is required.
- Email and phone values are typed rather than stored in a single generic string.
- Unknown birth year does not create an inferred age.
- Every behavioural change increments the profile revision.
- Archive is reversible.

## Application services

- `CreatePerson`
- `ListPeople`

Show, edit, search, archive, and restore services follow through the same repository port as the CLI surface expands.

## Outbound ports

`PersonRepository` is owned by People. Adapters translate file, database, import, or test representations into domain values. Address-book provider DTOs remain in provider adapters.

## Cross-context rules

Relationships and interactions reference `PersonId`. They do not mutate `PersonProfile` directly. Events may request an authorised dietary operational view without becoming the owner of the source requirement.

## Data classification

Ordinary contact data can be `shared` or `private`. Dietary detail, sensitive characteristics, private notes, and selected contact points can be `restricted`. Credentials are never person fields.
