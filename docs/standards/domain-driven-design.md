# Domain-driven design standard

## Purpose

Liaison RM spans personal relationships, workplace operations, events, communications, access history, sharing, providers, automation, and AI tools. A single undifferentiated contact model would make those concerns inseparable. This standard keeps business rules understandable and prevents storage, UI, or vendor choices from defining the product language.

## Bounded contexts

The initial context map contains:

- **Workspace** — workspace identity, format version, members, profiles, and lifecycle.
- **People** — person profiles, characteristics, contact points, important dates, and structured dietary requirements.
- **Relationships** — typed links, circles, priority, cadence, status, and relationship history.
- **Interactions** — notes, communications, meetings, provenance, counts, and last-interaction summaries.
- **Events** — activities, attendance, invitations, catering readiness, and event participation history.
- **Organisations** — organisations, teams, cost centres, departments, locations, and memberships.
- **Facilities** — access-log imports, identity resolution, retention, and bounded occupancy summaries.
- **Reminders** — follow-up commitments, recurrence, completion, and due-state policy.
- **Connections** — provider descriptors, capabilities, grants, jobs, and conformance evidence.
- **Sharing** — workspace membership, private overlays, encrypted operations, self-service requests, and disclosure policy.
- **Automation** — local API, webhooks, n8n, MCP, plugin execution, and approvals.

The map may change through an architecture decision record. Folder names alone do not establish a bounded context; independent language, invariants, ownership, and interfaces do.

## Internal layers

A context may contain:

```text
domain        Entities, value objects, aggregates, policies, domain events
application   Use cases, commands, queries, orchestration, unit-of-work boundaries
ports         Interfaces the context requires from external mechanisms
```

Adapters live outside the context and implement its ports. Application entry points compose contexts and adapters.

## Dependency direction

Allowed:

```text
application/UI/CLI -> application service -> domain
adapter             -> context-owned port + domain types
context             -> shared kernel when deliberately accepted
```

Disallowed:

```text
domain -> filesystem, SQL, HTTP, Tauri, React, provider SDK, CLI
domain -> another context's internal model
UI     -> canonical files or database tables directly
adapter -> private domain state through reflection or serialization tricks
```

## Aggregates and invariants

Use an aggregate only when a consistency boundary is required. Large object graphs are not aggregates merely because the UI shows them together.

Examples:

- A `PersonProfile` controls profile revision and field provenance; it does not own all interactions, events, organisations, or access records.
- A `DietaryRequirement` requires a kind, coverage state, disclosure policy, and optional verification metadata. An absent string cannot mean “no restriction.”
- A `ConnectionGrant` binds a purpose, capabilities, fields or data classes, endpoint, schedule, expiry, and revocation state.
- An `Event` owns attendance and dietary-readiness evaluation for its selected cohort, but does not rewrite source person profiles.

Invariants belong in constructors, value objects, aggregate methods, and domain policies. Controllers and React components do not reproduce them.

## Cross-context integration

Choose one of:

- an application interface with explicit input and output types;
- a published domain event;
- a read model built from public events;
- an anti-corruption layer translating external or legacy language.

Do not import another context’s persistence structs. Stable identifiers may cross boundaries; internal mutable objects may not.

## Ubiquitous language

Each context README records:

- accepted terms and definitions;
- rejected synonyms that create ambiguity;
- invariants;
- commands, queries, and events;
- upstream and downstream dependencies;
- data classifications;
- compatibility promises.

A terminology change that affects requirements, file formats, API contracts, or user copy updates the glossary and migration notes in the same pull request.

## Shared kernel

The shared kernel is intentionally small. Candidates include stable identifiers, revision tokens, clocks, hashes, and common error envelopes. Business terms such as person, event, note, organisation, or grant remain in their owning contexts.

Adding shared-kernel code requires an architecture note explaining why duplication would be more dangerous than coupling.

## Plugins and providers

Plugins and providers integrate through versioned contracts owned by a bounded context. They cannot:

- obtain a database handle;
- access arbitrary workspace paths;
- discover secrets not granted to them;
- call the network without an approved destination grant;
- modify canonical files outside application services;
- claim capabilities without conformance evidence.

Provider-specific DTOs stay inside the provider adapter and are translated through an anti-corruption layer.

## Review checklist

- Is one context clearly responsible for the behaviour?
- Does code use that context’s language consistently?
- Are invariants enforced once in the domain?
- Are external mechanisms behind ports?
- Are cross-context dependencies explicit?
- Is the shared kernel still narrow?
- Can another contributor understand the context from its README and tests?
- Does a provider, storage, framework, or UI term leak into the business model?
