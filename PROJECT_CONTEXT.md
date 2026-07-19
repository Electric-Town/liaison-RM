# Liaison RM project context

This document is the high-context handoff for human maintainers and coding agents. It explains the product thesis, current state, architecture, domain language, constraints, delivery order, and definition of done without relying on private conversation history.

It is descriptive where the product is still being designed and normative where it repeats accepted repository rules. When this file conflicts with `AGENTS.md`, an accepted decision record, a bounded-context contract, a versioned schema, or a passing test, stop and resolve the conflict explicitly.

## 1. Executive summary

Liaison RM is a local-authoritative, open-source relationship memory and attention system.

It is intended for:

- individuals and professional networkers;
- families and households;
- executive assistants and principals;
- reception, workplace, culture, and facilities teams;
- event managers;
- clubs, communities, and volunteer groups;
- developers building local automation, providers, and plugins.

The product combines the application experience of a personal CRM with open Markdown/YAML ownership and a local knowledge graph. It does not use sales-pipeline assumptions and does not measure human worth, closeness, trust, or affection from message volume.

The primary implementation direction is:

```text
Rust domain and application core
    ├── liaison CLI
    ├── Tauri desktop application
    ├── loopback OpenAPI service
    ├── MCP server
    ├── import/export workers
    ├── provider adapters
    └── capability-controlled WASI plugins
```

Canonical records are readable files. SQLite, search, graph layout, thumbnails, browser state, and caches are disposable projections.

## 2. Current status

Status: **pre-alpha**.

The current implementation order is governed by [the working-state delivery contract](docs/product/working-state-delivery.md) and accepted ADR 0012: B0 Workplace Review Alpha is delivered and qualified before A0 Personal Memory Alpha. Roadmap breadth, prototypes, and stale pull requests do not change that order.

The Claudia/PingCRM/OKF strategy overlay approved at SHA-256 `795a6e6751cd29a995478e254323f491e68a53ef7c35fa729d8627b87cd37089` is now integrated into the canonical requirements, UAT, gates, tasks, and ownership. It adopts bounded comparator outcomes rather than blanket parity. ADR 0013 pins the B0 People compatibility profile; it does not claim that the current alpha implements it.

The default branch contains the merged P00 contract reconciliation and P01 application composition root together with governance, product and interaction specifications, machine-readable planning, the Rust Workspace/People/CLI and Markdown slice, provider-neutral Connections, Topic Pack contracts, purpose-specific profile readiness, reason-only Review and Attention runtime foundations, localisation architecture, a native Tauri desktop alpha with macOS review bundles and Windows NSIS build configuration, and a public project site. G0, P00, P01, and P02 are complete; G1 is current and P03 recoverable canonical operations are the active package. The repository does not yet have a supported public release.

The accepted P02 source state adds one Rust application composition root shared by the CLI and Tauri, write-authoritative Workspace Sessions with composite path-local and per-user `WorkspaceId` operating-system exclusion, retained capability roots, lock-free read-only Health, structured results and errors, and direct native adapter parity. Cooperating ordinary unconfined Liaison processes on one OS account and machine reject copied workspaces with the same identity independently of `HOME`/XDG overrides. Cross-container Flatpak, macOS App Sandbox, and Windows AppContainer GUI/host-CLI coordination remains unsupported until a shared authority broker/namespace exists; it is not a second valid registry. Merged PR #44 supplied exact-head Linux, macOS, and Windows runtime evidence. P03 now owns recoverable multi-target operations and final mutation preconditions; B0 and a supported public release remain incomplete.

The installed macOS alpha launches and renders the local workspace interface. Native QA on 2026-07-18 found that desktop person creation fails at the Tauri argument contract because the frontend request uses camel-case `workspacePath` and `displayName` fields while the Rust request deserializer expects `workspace_path` and `display_name`. CLI Person tests do not prove the installed desktop path. The merged P01 source repairs and retests that bridge in an exact source build, but the separately installed application remains the old failing build until a later release gate installs and verifies a replacement.

Do not claim any of the following without exact-head evidence:

- that Liaison RM is ready for daily use;
- that the native desktop app is complete;
- that a downloadable Mac, Windows, or Flatpak build works;
- that the product is WCAG-, EN 301 549-, privacy-, or security-certified;
- that Google Drive, WebDAV, S3, CardDAV, email, calendar, AI, MCP, or plugins are production integrations;
- that backup restore, multi-writer synchronisation, Airgap isolation, or migration is release-ready.
- that current People files already conform to the pinned OKF profile or that the required normalizer has shipped.

### Current review stack

Do not maintain a hand-written list of open PR states here. It becomes false as soon as a branch is updated or closed. The [working-state delivery contract](docs/product/working-state-delivery.md) records durable scope decisions and reviewed branch dispositions; GitHub remains the authority for live PR state, exact heads, changed files, conflicts, and checks.

An open PR is not part of `main`. A prototype, workflow file, passing unit test, screenshot, and installed bundle are different evidence classes.

## 3. Canonical read order

Before implementing a task, read:

1. `AGENTS.md`.
2. This file.
3. `docs/product/working-state-delivery.md`.
4. `spec/traceability-ownership.json` and generated `docs/product/traceability.md`.
5. `SPEC.md`.
6. `AI_BUILD_INSTRUCTIONS.md`.
7. The owning bounded-context README and tests.
8. Relevant decision records under `docs/decisions/`.
9. Relevant knowledge articles under `docs/knowledge/`.
10. Relevant architecture, security, UX, and data-format documents.
11. The matching records in:
   - `spec/requirements.json`;
   - `spec/uat-cases.json`;
   - `spec/feature-gates.yaml`;
   - `spec/implementation-plan.yaml`;
   - `spec/traceability-ownership.json`.
12. `CHANGELOG.md` and current pull requests touching the same boundary.

When sources conflict, do not silently pick one. Open a focused decision or clarification change.

## 4. Source-of-truth hierarchy

Use this order when deciding what is authoritative:

1. Released, versioned canonical format and compatibility contracts.
2. Accepted architecture decisions.
3. Bounded-context domain invariants and tests.
4. Security, privacy, and local-integrity invariants.
5. Machine-readable requirements, UAT, feature gates, and task dependencies.
6. Product and interaction specifications.
7. Knowledge articles and operational guidance.
8. Prototypes and screenshots.
9. Issue or pull-request discussion.
10. Ideas not yet committed to the repository.

A prototype shows a proposed task flow; it does not prove the production implementation. A passing unit test does not prove platform packaging or user comprehension. A provider upload test does not prove safe synchronisation.

## 5. Product thesis

The product should be described as:

> A personal relationship memory and attention system with CRM-grade organisation, without treating people as leads or reducing relationships to message volume.

Liaison RM should make it practical to remember and act on context such as:

- contact methods and communication preferences;
- food allergies, intolerances, dietary patterns, preferences, dislikes, and operational catering instructions;
- accessibility and sensory needs;
- family, household, partner, child, parent, sibling, and pet context;
- birthdays, anniversaries, partial dates, recurring dates, and other important events;
- organisations, roles, departments, cost centres, offices, reporting lines, and historical memberships;
- meetings, calls, messages, visits, notes, commitments, introductions, gifts, events, and attendance;
- files, URLs, photos, documents, maps, calendar events, email references, and other resources;
- desired cadence, boundaries, paused relationships, do-not-contact state, and future intent.

It should support the organisation and recall power of a CRM without converting relationships into a sales funnel or guilt-producing backlog.

## 6. Four separate relationship concepts

Never collapse the following into one score.

### Relationship intent

Manually configured information about how the user wants to maintain the relationship:

- relationship type;
- tier;
- importance;
- desired cadence;
- preferred communication channel;
- desired future state;
- reason the relationship matters;
- boundaries;
- paused-until date;
- do-not-contact state;
- review date.

Default tier labels may include:

```text
core
active
warm
loose
paused
archive
```

They are editable labels, not universal truths.

### Relationship evidence

Recorded or imported facts:

- interactions;
- notes;
- activities;
- commitments;
- events and attendance;
- files and resources;
- dates;
- calendar items;
- authorised communication metadata;
- source and provenance.

### Maintenance status

An explainable state relative to the relationship's own configuration:

- on track;
- due soon;
- overdue relative to configured cadence;
- open commitment;
- important date approaching;
- required context is stale;
- no cadence configured;
- paused;
- do not contact;
- archived.

Good explanation:

> Quarterly cadence; last meaningful interaction was 112 days ago; one commitment remains open.

Rejected explanation:

> Relationship strength: 42%.

### Profile readiness

Purpose-specific information coverage. There is no universal profile-completeness score.

Examples:

```text
Basic contact readiness       Complete
Meeting briefing              92% ready
Travel briefing               Missing seat and hotel preferences
Event catering                Verified
Birthday preparation          Missing gift ideas
CardDAV export                Ready
```

A person can be complete for one purpose and incomplete for another.

## 7. Review and Attention model

The review engine helps users decide where to place attention. It must not claim an objective measure of relationship quality.

### Modes

1. **Reason-only** — no number; group by reasons such as open commitment or important date.
2. **Tiered** — low, normal, high, and urgent attention.
3. **Weighted** — a transparent 0–100 queue-ordering value.

Reason-only is the default for personal use. Weighted policies come after users understand and trust the reasons.

### Example weighted policy

```text
Review Priority =
  30% cadence status
+ 20% manual importance
+ 20% open commitments
+ 10% upcoming dates
+ 10% stale required context
+ 10% manual boost
```

Every component, weight, mapping, and policy version must be visible. Users can preview changes before saving and maintain separate policies for family, friends, professional contacts, an executive, or a workplace.

### Hard suppressions

These override all review calculations:

- archived;
- do not contact;
- relationship ended;
- paused until a date;
- snoozed;
- excluded from the active policy.

### Guardrails

The engine must never:

- measure human worth;
- infer trust, affection, or closeness from message volume;
- rank employees;
- become a social-credit system;
- shame users for overdue contact;
- send messages automatically;
- expose private assessments through sharing, exports, AI, or providers;
- assume every relationship requires recurring contact.

## 8. Topic Packs and custom fields

Profiles use Topic Packs rather than one enormous fixed form.

A pack can be enabled for:

- the workspace;
- a profile template;
- an organisation or group;
- a person;
- a plugin;
- an event, trip, meeting, or other temporary purpose.

Planned built-in packs:

| Pack | Example fields |
|---|---|
| Identity and communication | preferred name, pronunciation, pronouns, language, time zone, channel, acceptable contact times |
| Food and hospitality | allergies, intolerances, dietary pattern, dislikes, favourites, restaurants, drinks, cross-contact requirements |
| Travel | airport, airline, seat, rail, hotel, room, accessibility, pace, buffers |
| Favourites and gifts | books, films, music, games, colours, brands, flowers, ideas, past gifts, avoid list |
| Family and household | partner, children, parents, siblings, household context, important dates |
| Pets | names, species, birthday/adoption date, temperament, treats, care context |
| Professional | organisation, role, expertise, projects, introductions, resources offered/requested |
| Interests and life context | hobbies, causes, goals, challenges, learning, ask/avoid topics |
| Events and hosting | event interests, plus-one, dietary coverage, arrival preferences, attendance |
| EA briefing | recent interactions, commitments, mutual contacts, travel, meeting, food and scheduling |
| Accessibility and sensory | communication, meeting, venue, travel and sensory requirements |
| Resources | files, URLs, events, email references, photos, documents, maps, bookmarks |

Supported field types should include:

- short and long text;
- Markdown;
- dates, partial dates, and recurring dates;
- enums and multiple selections;
- booleans;
- numbers and measurements;
- addresses and locations;
- entity references;
- file and URL references;
- sealed values;
- calculated read-only values;
- plugin-supplied fields.

Field IDs are stable and independent of labels:

```yaml
id: travel.seat_preference
label: Seat preference
type: enum
options:
  - window
  - aisle
  - middle
  - no_preference
classification: private
required_for:
  - executive_travel_brief
stale_after: P2Y
```

## 9. Explicit information states

An empty value is ambiguous and unsafe. Fields must support explicit states:

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

Fields may also carry:

- source;
- author or connector;
- capture date;
- verification date;
- review date;
- visibility;
- confidence;
- purpose;
- change history.

Users need actions such as:

- mark not applicable;
- ask later;
- mark declined;
- review after next interaction;
- exclude from this profile;
- request from the person.

For dietary and accessibility workflows, an empty field never means "none".

## 10. Domain entities

Do not represent everything as a generic Contact.

Core entity types:

- Person;
- Organisation;
- Group;
- Household;
- Location;
- Event;
- Resource.

`Contact` is a UI/search view over people and organisations, not the domain aggregate.

### Membership

A person's relationship to an organisation is a dated Membership:

```yaml
person_id: alex
organization_id: electric-town
role: Executive Assistant
department: Leadership
cost_center: CC-104
location: Dublin
started_on: 2025-01-12
ended_on: null
```

Memberships support simultaneous roles, historical employment, departments, cost centres, offices, reporting lines, organisation graphs, and as-of-date filtering.

### Groups

Groups may be:

- static;
- query-driven;
- event snapshots;
- households;
- teams.

Groups can own notes, files, dates, events, resources, permissions, and Topic Pack defaults.

### Resources

Files and URLs are first-class Resources, not unstructured strings hidden in notes.

Resource types include:

- local file;
- external URL;
- photo;
- PDF or document;
- calendar event;
- `.ics` invitation;
- email-thread reference;
- map or location;
- voice note;
- meeting-recording reference;
- gift or product idea;
- contact card.

Rules:

- local files use portable relative paths and hashes;
- external metadata is not fetched without permission;
- calendar items preserve UID and recurrence identity;
- removing a connector does not delete the local record;
- email bodies and attachments are separately permissioned;
- Resources have backlinks and timeline presence;
- keyboard selection is equivalent to drag-and-drop.

## 11. Bounded contexts

### Workspace

Owns workspace identity, schema version, profile, members, migration, validation, locking, journal, projection lifecycle, and backup/restore orchestration.

### Identity and Profiles

Owns people, contact methods, Topic Packs, fields, values, provenance, templates, and profile readiness inputs.

### Organisations and Groups

Owns organisations, households, locations, groups, memberships, departments, cost centres, and effective-dated organisational structure.

### Relationships

Owns relationship edges, intent, tiers, cadence, boundaries, lifecycle, and optional private manual assessments.

### Interactions and Commitments

Owns communications, meetings, activities, notes, commitments, tasks, reminders, meaningful-interaction semantics, and source references.

### Events and Calendar

Owns events, invitations, attendance, cohorts, recurrence identity, event snapshots, and important-date/calendar views.

### Knowledge and Resources

Owns notes, files, URLs, attachments, document references, backlinks, and resource timeline presence.

### Review and Attention

Owns maintenance status, readiness evaluation, review reasons, queues, policies, simulations, sessions, snooze/pause, and Markdown review records.

### Facilities

Owns access-log imports, source mappings, unresolved identities, retention, deletion, and bounded attendance summaries. It must not feed productivity, compliance, or risk scores.

### Connections

Owns provider identity, capabilities, descriptors, configuration shape, grants, jobs, health, limits, and conformance labels. It does not own business-domain invariants.

### Sharing

Owns members, devices, roles, permissions, signed operations, acknowledgements, conflicts, keys, private overlays, self-service requests, and Liaison Cards.

### Customisation

Owns user-defined field schemas, profile layout, dashboard layout, saved views, Topic Pack composition, and approved plugin contributions.

### Automation

Owns local OpenAPI, MCP, AI tool mediation, webhooks, n8n examples, plugin host, staged writes, source-linked reads, and automation grants.

## 12. Dependency direction

Preferred dependency direction:

```text
Desktop / CLI / API / MCP / importer / plugin adapter
                    ↓
          application command or query
                    ↓
            owning domain model
                    ↓
              outbound port
                    ↓
                 adapter
```

Prohibited shortcuts:

- UI code editing canonical files directly;
- connector code deciding domain invariants;
- provider SDK types leaking into domain entities;
- a shared database schema serving as the context map;
- mutable aggregates shared across contexts;
- a generic `metadata` map used to avoid modelling owned concepts;
- duplicated business rules in React, CLI, API, MCP, importers, or plugins.

## 13. Canonical storage

Human-scale records use Markdown with versioned YAML front matter. High-volume, append-oriented machine data uses documented JSONL partitions.

Representative layout:

```text
workspace/
├── .liaison/
│   ├── workspace.yaml
│   ├── schema-version
│   ├── devices/
│   ├── members/
│   ├── grants/
│   ├── migrations/
│   ├── operations/
│   └── projections/
├── people/
├── organisations/
├── locations/
├── groups/
├── relationships/
├── notes/
├── interactions/
├── reminders/
├── events/
├── views/
├── streams/
│   ├── access/YYYY/MM/*.jsonl
│   └── email-metadata/YYYY/MM/*.jsonl
├── attachments/sha256/
└── audit/YYYY/MM/*.jsonl
```

Storage invariants:

- deleting projections must not delete current semantic information;
- unknown front-matter fields and Markdown sections survive supported round trips;
- IDs are stable and independent of filenames;
- writes use validation, revision preconditions, content hashes, journals, and tested recovery;
- migrations support dry-run, backup, deterministic execution, validation, and rollback or explicit irreversibility;
- secrets never enter canonical files, logs, fixtures, screenshots, or exported settings;
- fixtures contain synthetic data only.

Planned B0 People authoring pins the OKF v0.1 Draft envelope. Liaison's versioned domain extension remains authoritative for identity, purpose, revision, provenance, state, sensitivity, disclosure, and operational meaning. Writers are strict; readers are tolerant. OKF-valid never means Liaison-valid or event-ready, and sealed sensitive facts never enter plaintext. The required OKF People normalization is previewable, exact-backup-first, journaled, failure-atomic, idempotent, restart-recoverable, and exactly reversible. General and third-party migrations remain excluded from B0.

Pure "everything is one Markdown file" is not a requirement. Millions of access or email-metadata events should not become millions of tiny files. Open, documented JSONL is acceptable for machine streams.

## 14. Local authority

Local-first is not precise enough. Liaison RM is **local-authoritative**:

- the local workspace is the source of truth;
- optional services do not become mandatory authorities;
- the user can inspect, transform, back up, and recover records without Liaison RM;
- account, telemetry, crash upload, remote logging, licence check, and hosted database are not required;
- network access is explicit and purpose-bound.

### Airgap

Airgap builds compile out network clients and listeners. Offline import/export, removable-media exchange, validation, local backup, and recovery remain available.

### Connected-local

Connected-local builds retain the same local authority and may enable explicit providers, local APIs, webhooks, MCP, CardDAV, calendars, email metadata, or peer exchange.

A runtime setting alone does not prove an Airgap build. Release evidence must inspect dependencies and actual socket behaviour.

## 15. Providers and connections

Provider examples include:

- local folders;
- removable media;
- WebDAV;
- S3-compatible services;
- AWS S3;
- MinIO;
- Google Drive;
- Google Cloud Storage;
- Azure Blob Storage;
- CardDAV;
- CalDAV;
- email metadata;
- local and remote AI providers.

They are adapters behind versioned contracts. The domain records provider identity as data and never imports a provider SDK.

A provider descriptor declares:

- provider ID and version;
- contract name and version;
- operations;
- safe modes;
- configuration fields;
- secret references;
- network destinations;
- consistency statement;
- limits;
- conformance status;
- evidence reference.

Provider registration grants no access. A Connection plus an explicit Grant is required.

Safe-mode labels:

- import;
- export;
- backup;
- single-writer;
- multi-writer.

A successful upload does not prove safe multi-writer synchronisation. Provider names and marketing claims do not replace conformance evidence.

## 16. Sharing

A plaintext shared WebDAV, Dropbox, network-drive, or similar folder is not accepted as a safe multi-writer relationship database.

Sharing transports:

- encrypted immutable operations;
- content-addressed attachments;
- signed manifests;
- device acknowledgements;
- key envelopes;
- conflict and reconciliation records;
- revocation and rotation information.

Each device validates operations and materialises its own readable local view.

### Private overlays

An EA, family member, or team member may maintain private context that never enters shared indexes, exports, AI context, or remote operations outside authorised scope.

### Self-service requests and Liaison Cards

A workspace can request selected information from a person without requiring an account. A portable Liaison Card can contain chosen contact, preference, date, organisation, accessibility, or hospitality information. Disclosure is selective, reviewable, signed where applicable, and purpose-bound.

## 17. AI, MCP, and plugins

### AI

- AI output is untrusted input.
- Read tools return source references and the grant used.
- Write tools stage proposals by default.
- Direct AI, MCP, plugin, provider, or import writes to confirmed facts, assessment, freshness, cadence, disclosure, or operational readiness are prohibited.
- Proposal review shows records, fields, old values, new values, provenance, and consequences.
- No personal data is sent to a model without a provider, purpose, scope, and expiry grant.
- Local Ollama-compatible operation must not require a remote account.
- Remote model pricing or free-plan availability is not a product guarantee.

### MCP and local API

MCP and OpenAPI call normal application services. They do not receive raw filesystem or database authority. Tokens and tools are scoped, revocable, auditable, and limited by the same domain and grant rules as the desktop and CLI.

### Plugins

Plugins use WASI Component Model/WIT contracts and receive no ambient authority. They are denied every capability not declared and approved. They do not receive:

- a raw database handle;
- unrestricted filesystem access;
- unrestricted network access;
- private fields outside the grant;
- a bypass around domain validation.

WebAssembly is a plugin and portability mechanism, not the foundation of the entire application.

## 18. Platform architecture

### Rust

Use the pinned toolchain. Rust owns domain and application rules, canonical-format translation, imports, migrations, validation, provider contracts, local API, MCP, plugin mediation, and the CLI.

### Tauri

Tauri 2 is the primary desktop shell for Ubuntu/Flatpak, macOS, and Windows. Tauri commands are inbound adapters and call application services.

### React and TypeScript

React/TypeScript renders interaction and presentation state. It must not become the sole implementation of business rules or persistence.

### CLI

The `liaison` CLI is a first-class product surface and automation contract. Mutating commands should support dry-run where meaningful, structured JSON output, stable exit codes, explicit workspace selection, revision checks, audit attribution, and safe non-interactive operation.

### PWA/browser

An optional browser interface may be served by a local Liaison process. Browser-managed storage is not canonical. The browser is a client of the local authority.

## 19. Primary personas and UAT outcomes

### Individual/networker

- create and maintain readable profiles;
- remember context and commitments;
- use reason-based review;
- search interactions, dates, organisations, groups, and resources;
- keep control of local data.

### Family/household

- see birthdays, anniversaries, activities, pets, gifts, and upcoming dates;
- share selected context while preserving private overlays;
- operate without a hosted account.

### Executive assistant

- prepare authorised source-linked briefings;
- separate principal-private and shared operational context;
- track commitments, introductions, travel, meetings, and hospitality.

### Reception/workplace culture

- record employee dietary coverage safely;
- filter by office, organisation, team, department, cost centre, group, or event;
- resolve every selected attendee's coverage state;
- export least-disclosure operational instructions.

### Facilities

- import access logs with source mapping and idempotency;
- resolve or retain unresolved badge identities;
- apply retention and deletion;
- avoid productivity, compliance, or risk scoring.

### Events

- create events and attendance snapshots;
- count recorded attendance;
- track invitations and readiness;
- inspect last interaction and authorised communication metadata.

### CRM administrator

- configure Topic Packs, fields, layouts, views, dashboards, and policies;
- export/import settings;
- preserve stable IDs across label and layout changes.

### Developer/automation user

- call local APIs, webhooks, n8n, MCP, and CLI;
- build providers and plugins against versioned contracts;
- use Ollama locally;
- preserve domain, grant, and audit controls.

## 20. UX and accessibility

Every user-facing change is reviewed against:

- ADHD and AuDHD interruption recovery;
- cognitive load and optional detail;
- past-behaviour/Mom Test-style discovery;
- AskTog interaction principles;
- Gestalt grouping;
- Nielsen's ten heuristics;
- relevant IxDF research;
- WCAG 2.2 Level AA;
- applicable EN 301 549 requirements.

Required behaviour:

- keyboard completion;
- visible focus;
- screen-reader names and live regions;
- 400% zoom and reflow;
- reduced motion;
- low-stimulation and density options;
- persistent drafts and return to the same place after interruption;
- semantic alternatives to graphs and drag-and-drop;
- text alternatives to colour, icons, hover, animation, and spatial position;
- loading, empty, partial, stale, conflict, permission, success, undo, and recovery states.

Review should help memory and activation without producing a guilt backlog. Skip, snooze, pause, archive, and no-cadence are valid states. No gamification around number of people contacted.

## 21. Privacy and security

Data may include sensitive personal, dietary, accessibility, calendar, email, relationship, family, and workplace information.

Controls include:

- classification;
- purpose limitation;
- role and field-level permissions;
- least disclosure;
- retention and expiry;
- source and provenance;
- audit;
- sealed storage where required;
- explicit egress grants;
- preview and confirmation for destructive or broad disclosure actions.

Dietary models distinguish allergy, intolerance, medical restriction, religious restriction, ethical preference, dislike, positive preference, verified none, pending, stale, declined, unreachable, and excluded from catering. An empty field is not "no restrictions".

Access logs are not used for productivity, attendance-compliance, performance, risk, or employee scoring.

## 22. Delivery roadmap

The current merge order is P00–P11, B0 acceptance, then A0. The complete exit evidence is in `docs/product/roadmap.md`; R0–R6 remains only a long-term capability catalogue.

- **P00–P03:** reconcile truth, establish one typed application and Workspace Session boundary, and route canonical mutations through recoverable multi-target operations.
- **P03 design gate:** design consultation creates canonical `DESIGN.md`, then plan design review approves an amended P04 direction. G0 does not create `DESIGN.md`.
- **P04:** deliver the typed accessible desktop system after design consultation and plan design review.
- **P05-OKF/P06/P06-REPAIR/P09-OKF:** after P03/P04, deliver the pinned strict People writer and tolerant Directory projection, then run `T-B0-P06-REPAIR` with preview, exact backup, failure-atomic application, exact receipt, and rollback before the required recoverable OKF People normalization closes `FG-B0-001`; P01/P02 do not expand.
- **P05/P07/P08:** establish P05's Directory/Event/dietary domain contracts in G1 before P06/P07, keep sensitive security and recovery ownership exclusive to P07/P08, and let G3 consume the accepted P05 contracts without a reverse milestone dependency.
- **P09–P11:** complete Directory onboarding, immutable event cohorts, exact dietary readiness, least-disclosure brief delivery, and the compiled B0 interface.
- **B0:** qualify Workplace Review Alpha, including guided canonical repair, in a freshly installed universal Mac review application for one trusted local workspace owner.
- **A0:** add quick/full capture, source-complete purpose-scoped profiles, explicit fact states, reversible identity review, a source/range unified timeline, custom profile layouts, personal interactions, commitments, distinct last-note/last-interaction context, open loops, and reason-only Review while retaining the complete B0 regression matrix.
- **After A0:** independently gate general/third-party migrations, sharing, visible provider operations, mobile, Meitheal, CardDAV/calendars/email, facilities, optional spatial discovery, OpenAPI, MCP, staged AI proposals, and plugins.

B0 has event-bounded preparation and gap resolution only. It does not contain a generic task engine or workplace relationship allocation, cadence, attention weighting, ranking, or scoring. A0 also has no global person score and exact or fuzzy candidates never merge automatically. Later connections have no hidden sync, refresh, or egress.

## 23. Recommended implementation order

Select work from the first incomplete P00–P11 package whose dependencies are complete. Do not revive personal-first R2 work, provider transports, mobile, automation, AI, or plugins as B0 prerequisites. Full custom profile tabs and dashboard configuration belong to A0; B0 uses only the shared typed fields required for the workplace event outcome.

After B0 acceptance, A0 implements factual personal context and reason-only Review before any optional weighted queue policy. A queue-ordering value is never relationship strength and is structurally unavailable to Workplace/B0.

## 24. Selecting work

Choose one task whose dependencies are complete. A task must identify:

- user problem and persona;
- observed or supplied evidence;
- owning bounded context;
- ubiquitous language;
- inputs and outputs;
- invariants;
- application command/query;
- required ports and adapters;
- acceptance and UAT tests;
- privacy classification;
- accessibility effect;
- migration/rollback effect;
- knowledge article action;
- changelog effect.

Prefer a vertical slice over horizontal scaffolding.

## 25. Definition of done

A change is not complete because code compiles.

Required evidence, as applicable:

- domain and contract tests;
- application service tests;
- adapter integration tests;
- unknown-field and canonical round-trip tests;
- failure, interruption, retry, idempotency, and recovery tests;
- CLI human and JSON output tests;
- browser interaction and accessibility smoke tests;
- platform install/launch tests;
- provider or plugin conformance;
- migration and rollback evidence;
- security/privacy review;
- updated requirements, UAT, gates, task traceability, knowledge, and changelog;
- exact commit SHA and workflow run evidence.

Do not mark a draft ready while its stated required checks are failing or absent.

## 26. Baseline validation commands

```bash
python scripts/check_repository.py
python scripts/check_spec.py
python scripts/check_architecture.py
python scripts/check_providers.py
python scripts/check_wit_contract.py

cargo fmt --all --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

Run additional format, browser, platform, import, migration, provider, plugin, accessibility, and release checks required by the changed scope.

## 27. Common failure modes

Reject or correct:

- starting with a screen before domain/application rules;
- describing planned work as complete;
- treating `main`, an open PR, a prototype, and a release as the same state;
- adding Google-, S3-, WebDAV-, Gmail-, or other provider-specific concepts to business domains;
- making SQLite the source of truth;
- using a shared plaintext folder as a multi-writer database;
- conflating backup and synchronisation;
- using a generic `metadata` bag to avoid domain modelling;
- scoring closeness or employees from communication/access volume;
- treating empty dietary data as verified none;
- exposing private notes or assessments through AI, exports, sharing, or connectors;
- adding ambient network or filesystem authority to plugins;
- hiding essential state in colour, graph position, drag-only interaction, or hover;
- adding a dependency without licence, maintenance, Airgap, and transitive review;
- using real personal data in fixtures or screenshots;
- claiming accessibility or security certification without evidence;
- generated prose that is vague, repetitive, promotional, or disconnected from decisions and tests.

## 28. Repository map

```text
AGENTS.md                    Normative agent/contributor contract
PROJECT_CONTEXT.md           Complete product and engineering handoff
SPEC.md                      Product/build specification
AI_BUILD_INSTRUCTIONS.md     Executable implementation sequence
CHANGELOG.md                 Contributor- and user-visible changes
CONTRIBUTING.md              Contribution process

docs/architecture/           Context map, storage, providers, sharing
docs/decisions/              Architecture decision records
docs/knowledge/              KCS-informed operational knowledge
docs/product/                Roadmap and product material
docs/prototypes/             Interaction concept and review screens
docs/security/               Threat model and local-integrity controls
docs/standards/              DDD, UX, content, knowledge practices
docs/evidence/               Exact implementation/release evidence

spec/requirements.json       Product and engineering requirements
spec/uat-cases.json          Persona acceptance cases
spec/feature-gates.yaml      Runtime/policy/release gates
spec/implementation-plan.yaml Dependency-ordered tasks

apps/                        Desktop, CLI, local services
contexts/                    Bounded contexts
crates/                      Shared technical crates
adapters/                    Persistence/import/provider adapters
interfaces/                  WIT, OpenAPI, MCP and other contracts
providers/                   Provider packages and conformance evidence
site/                        Static public project site and discovery metadata
scripts/                     Repository and quality checks
```

## 29. Handoff checklist for another agent

Before making a change:

- [ ] Confirm the repository, branch, base, and exact head.
- [ ] Read `AGENTS.md`, this file, `SPEC.md`, and `AI_BUILD_INSTRUCTIONS.md`.
- [ ] Identify the owning bounded context.
- [ ] Search existing code, decisions, knowledge, requirements, UAT, gates, tasks, and open PRs.
- [ ] State what is implemented versus planned.
- [ ] Choose a dependency-complete vertical slice.
- [ ] Add failing tests before claiming implementation.
- [ ] Keep mechanisms in adapters and rules in the domain/application layer.
- [ ] Preserve local authority, open formats, unknown fields, and Airgap boundaries.
- [ ] Address UX, accessibility, privacy, security, compatibility, migration, and rollback.
- [ ] Update knowledge and changelog.
- [ ] Run exact-scope checks and record the exact commit/workflow evidence.
- [ ] Leave the PR in draft if required evidence is incomplete.

## 30. Maintainer questions that require an explicit decision

Open a focused decision rather than guessing when work changes:

- a bounded-context boundary;
- canonical format or compatibility contract;
- encryption, key, signing, or identity model;
- Airgap/Connected-local separation;
- provider safe-mode semantics;
- sharing conflict or reconciliation policy;
- private overlay disclosure rules;
- use of sensitive workplace data;
- review-priority components or guardrails;
- plugin authority model;
- platform support or release-signing policy;
- licence compatibility or source-code reuse.

The repository should contain enough context for a new agent to continue without private prompt history. Where information is missing, the correct next step is a documented question or decision—not a confident invention.
