# Liaison RM

> A local-authoritative relationship memory and attention system with CRM-grade organisation—without treating people as leads or reducing relationships to message volume.

Liaison RM is an open-source relationship manager for individuals, families, executive assistants, reception and workplace teams, facilities teams, event organisers, professional networkers, and community organisers.

It combines:

- readable Markdown/YAML records that the user owns;
- a shared Rust application core;
- a native Tauri desktop application;
- a first-class `liaison` command-line interface;
- relationship, organisation, event, resource, and interaction graphs;
- structured personal context such as food requirements, preferences, important dates, accessibility needs, roles, memberships, and commitments;
- provider-neutral backup, import, export, and—only where proven safe—synchronisation;
- local APIs, MCP tools, Ollama-compatible local AI, and capability-controlled plugins.

Liaison RM does **not** require an Electric Town account or hosted relationship database. Canonical records remain on storage selected and controlled by the workspace owner. Search indexes, caches, and graph layouts are rebuildable projections.

> **Status: pre-alpha.** The repository contains a reviewed product specification, interaction prototype, Rust domain foundations, Markdown/CLI vertical slices, and provider-contract work. It is not yet ready for daily use or public binary distribution. Do not describe planned features as implemented.

## Start here

### Building with a coding agent

Read these files in order before changing code:

1. [`AGENTS.md`](AGENTS.md) — normative contributor and agent rules.
2. [`PROJECT_CONTEXT.md`](PROJECT_CONTEXT.md) — product, architecture, status, terminology, active work, and handoff context.
3. [`SPEC.md`](SPEC.md) — product and build specification.
4. [`AI_BUILD_INSTRUCTIONS.md`](AI_BUILD_INSTRUCTIONS.md) — task selection and implementation sequence.
5. The owning bounded-context README under [`contexts/`](contexts/).
6. Related decisions, knowledge articles, requirements, UAT cases, feature gates, and implementation tasks.

Do not begin with a new screen or provider integration. Start with a bounded vertical slice through domain rules, application services, ports, adapters, CLI/API surface, tests, knowledge, and changelog.

### Reviewing the product direction

- [Product and build specification](SPEC.md)
- [Complete agent handoff and current context](PROJECT_CONTEXT.md)
- [Product roadmap](docs/product/roadmap.md)
- [Context map](docs/architecture/context-map.md)
- [Ubiquitous language](docs/architecture/ubiquitous-language.md)
- [Open workspace format](docs/architecture/open-workspace.md)
- [Provider-neutral connections](docs/architecture/provider-connections.md)
- [Sharing and synchronisation](docs/architecture/sharing-and-synchronization.md)
- [Threat model](docs/security/threat-model.md)
- [Local-integrity requirements](docs/security/local-integrity.md)
- [Interaction prototype and screens](docs/prototypes/README.md)
- [Machine-readable requirements](spec/requirements.json)
- [Persona UAT catalogue](spec/uat-cases.json)
- [Feature gates](spec/feature-gates.yaml)
- [Implementation plan](spec/implementation-plan.yaml)

## Product thesis

Most personal CRMs either behave like sales tools, hide data in a hosted database, or infer relationship value from communication frequency. Liaison RM takes a different position:

- a person is not a lead;
- frequent email does not prove closeness, trust, or importance;
- sparse information does not mean an unimportant relationship;
- every relationship does not need a recurring cadence;
- private assessments are not objective facts;
- workplace access or communication data must not become productivity or risk scoring.

The domain separates four concepts:

| Concept | Meaning | Source |
|---|---|---|
| **Relationship intent** | How the user wants to maintain the relationship, including tier, importance, cadence, boundaries, and future state | Manually configured |
| **Relationship evidence** | Interactions, notes, activities, commitments, events, resources, and imported history | Recorded or imported facts |
| **Maintenance status** | Whether the relationship is current relative to its own cadence and boundaries | Explainable calculation |
| **Profile readiness** | Whether the information required for a specific purpose is known and current | Purpose-specific calculation |

The product uses **Review Priority** only to order a review queue. It must never be presented as relationship strength or human worth. Reason-only review is the default; any weighted policy must be transparent, configurable, versioned, and overridden by hard suppressions such as do-not-contact, archive, pause, or snooze.

## Core use cases

### Personal and family

- remember preferences, allergies, favourite things, gifts, pets, family context, birthdays, anniversaries, and important dates;
- keep a searchable interaction and commitment history;
- run small, low-pressure daily or monthly reviews;
- maintain a local graph of people, households, groups, organisations, events, and resources;
- exchange selected information with a family or group without exposing private overlays.

### Executive assistants

- prepare source-linked meeting briefings;
- maintain principal-private notes separately from shared operational profiles;
- track commitments, introductions, important dates, travel, scheduling, hospitality, and communication preferences;
- delegate follow-up without disclosing unauthorised context.

### Reception, workplace, and events

- record structured dietary coverage and operational instructions;
- select attendees by organisation, location, team, department, cost centre, group, or saved view;
- identify unknown, stale, declined, conflicting, or unresolved information;
- produce a least-disclosure catering brief;
- record attendance and event history;
- import access logs with explicit mapping, retention, unresolved identities, and role controls.

### Networking and community

- organise contacts by relationship intent, group, organisation, event, and follow-up state;
- record last interaction, last meaningful note, open commitments, and introductions;
- prioritise or deprioritise attention without claiming an objective closeness score;
- preserve readable local records rather than depend on a SaaS CRM.

## Topic Packs and structured context

Profiles are composed from reusable **Topic Packs**, not one fixed contact form. Packs may be enabled for a workspace, template, organisation, group, person, plugin, event, trip, or other purpose.

Planned built-in packs include:

- identity and communication;
- food and hospitality;
- travel;
- favourites and gifts;
- family and household;
- pets;
- professional context;
- interests and life context;
- events and hosting;
- executive-assistant briefing;
- accessibility and sensory preferences;
- resources.

Fields have stable IDs independent of labels and layouts. Values can explicitly be known, verified, unverified, unknown, not applicable, declined, stale, conflicting, in need of clarification, or derived. Liaison RM does not use one universal profile-completeness score; readiness is calculated for a named purpose such as event catering, a meeting brief, travel, CardDAV export, or birthday preparation.

## Architecture

### Technology direction

- **Core:** Rust workspace with context-owned domain and application services.
- **Desktop:** Tauri 2 with React/TypeScript as the intended production interface.
- **CLI:** first-class `liaison` binary using the same application services.
- **Canonical records:** Markdown with versioned YAML front matter.
- **High-volume streams:** documented, partitioned JSONL.
- **Projections:** disposable SQLite, full-text search, graph layouts, thumbnails, and caches.
- **Plugins:** WASI Component Model/WIT with explicit capabilities.
- **Automation:** loopback OpenAPI, webhooks, MCP, and n8n-compatible workflows.
- **AI:** local Ollama-compatible operation plus optional remote providers behind explicit disclosure grants.
- **Browser:** optional local client; it never owns canonical data.

### Domain-driven structure

```text
apps/                 User-facing interfaces: desktop, CLI, local services
contexts/             Bounded contexts and their application services
crates/               Narrow shared technical libraries
adapters/             Filesystem, projection, import/export, and provider adapters
interfaces/           Versioned WIT, OpenAPI, MCP, and external contracts
providers/            Optional provider packages and conformance evidence
docs/                 Product, architecture, decisions, knowledge, UX, security, evidence
spec/                 Requirements, UAT, feature gates, implementation plan
scripts/              Repository, architecture, format, and conformance checks
```

Current and planned bounded contexts:

| Context | Owns |
|---|---|
| Workspace | Workspace identity, profile, members, schema lifecycle, validation, migration, and projection lifecycle |
| Identity and Profiles | People, contact methods, Topic Packs, field states, provenance, profile templates |
| Organisations and Groups | Organisations, locations, households, groups, memberships, departments, cost centres |
| Relationships | Relationship edges, intent, boundaries, tiers, cadence, optional private assessments |
| Interactions and Commitments | Communication and activity history, notes, commitments, tasks, reminders |
| Events and Calendar | Events, invitations, attendance, recurrence, cohorts, important dates |
| Knowledge and Resources | Files, URLs, photos, documents, calendar/email references, backlinks |
| Review and Attention | Maintenance status, readiness, review reasons, queues, policies, sessions |
| Facilities | Access-log imports, identity mapping, retention, bounded summaries |
| Connections | Provider identity, capabilities, grants, jobs, conformance, health |
| Sharing | Members, devices, encrypted operations, acknowledgements, conflicts, overlays, cards |
| Customisation | Field schemas, layouts, dashboards, saved views, plugin contributions |
| Automation | Local API, MCP, AI tools, webhooks, plugin host, approved commands and queries |

No UI, connector, provider, plugin, or AI tool may implement domain rules independently of the owning context.

## Local authority and storage

A workspace is an ordinary directory the user can inspect, back up, transform, and recover without Liaison RM:

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

Human-scale records remain readable. High-volume machine events do not become millions of tiny Markdown files. Unknown fields and user-authored Markdown sections must survive supported round trips. Secrets never enter canonical files.

## Airgap and Connected-local

Liaison RM has two separately testable release profiles:

- **Airgap:** network clients and listeners are compiled out. Local files, removable-media exchange, import/export, validation, backup, and recovery remain available.
- **Connected-local:** the same local source of truth may use explicitly granted providers, CardDAV, calendars, email metadata, webhooks, local APIs, MCP, or peer exchange.

A runtime toggle is not accepted as proof of an Airgap build.

## Connections, backup, and synchronisation

Google Drive, WebDAV, S3-compatible services, AWS S3, MinIO, Google Cloud Storage, Azure Blob Storage, local folders, removable media, CardDAV, CalDAV, email services, and future providers are adapters behind versioned capability contracts.

Provider registration grants no access. A connection remains inert until an explicit grant defines:

```yaml
purpose:
endpoint:
data_classes:
fields:
operations:
retention:
schedule:
expires_at:
approved_by:
revocable: true
```

Uploading and downloading objects does not prove safe multi-writer synchronisation. Providers advertise only modes supported by conformance evidence:

- import;
- export;
- backup;
- single-writer;
- multi-writer.

Shared plaintext Markdown folders are not treated as safe concurrent databases. Team and family sharing uses encrypted immutable operations, signed manifests, acknowledgements, key envelopes, conflict records, and content-addressed attachments. Each device materialises its own readable local view.

## Privacy, safety, and accessibility

The project targets WCAG 2.2 Level AA and applicable EN 301 549 evidence. This is an engineering target, not a certification claim.

Every user-facing workflow must support:

- keyboard completion and visible focus;
- screen-reader names and live status;
- 200% zoom and reflow;
- reduced motion and low-stimulation modes;
- interruption recovery and draft preservation;
- text alternatives to colour, icons, graphs, spatial layout, and drag-only controls;
- empty, loading, partial, stale, conflict, permission, success, undo, and recovery states.

Sensitive dietary, accessibility, email, calendar, access, and private relationship data require purpose, classification, role, audit, retention, expiry, and least-disclosure handling.

The product explicitly prohibits:

- hidden telemetry, remote logging, account checks, and licence checks;
- undeclared network requests;
- secrets in workspace files, logs, fixtures, or screenshots;
- automatic messages without explicit user action;
- trust or affection inference from message volume;
- employee ranking, productivity scoring, risk scoring, or social-credit behaviour;
- private assessments leaking into shared workspaces, exports, AI context, or provider data.

## Delivery sequence

| Release | Outcome |
|---|---|
| **R0** | Repository governance, product contracts, architecture, threat model, requirements, UAT, and agent-ready context |
| **R1** | Open workspace, crash-safe lifecycle, people, Markdown adapter, CLI, import/export, backup and isolated restore |
| **R2** | Accessible native desktop, configurable profiles/dashboard, graph plus semantic alternative, Linux/macOS/Windows packaging |
| **R3** | Workplace event and dietary-readiness workflow |
| **R4** | Encrypted family/team sharing, WebDAV/S3-compatible transport, connection/grant UI |
| **R5** | CardDAV, calendars, email metadata, migrations, facilities imports |
| **R6** | Local OpenAPI, MCP, Ollama, remote AI grants, WASI plugins |

The first operational wedge is event dietary readiness: select an attendee cohort, identify every unresolved coverage state, and produce a least-disclosure catering brief.

## Current work and review stack

Repository status changes quickly. Verify branches, pull requests, and exact-head CI before relying on this list.

As of 2026-07-18:

- PR #2 — governance, KCS-informed practice, DDD, UX, accessibility, and content standards;
- PR #3 — product, architecture, machine-readable requirements, prototype, and screens;
- PR #4 — Rust Workspace/People core, Markdown adapter, and CLI;
- PR #7 — provider-neutral Connections contract and local reference provider;
- PR #8 — draft native macOS/Tauri alpha; not yet a verified Mac application.

Do not assume an open PR has landed in `main`. Read its base, head, changed files, exact-head checks, and stated limitations.

## Validation

Run the checks required by the changed scope. The canonical baseline and completion rules are in [`PROJECT_CONTEXT.md`](PROJECT_CONTEXT.md#26-baseline-validation-commands) and [`AI_BUILD_INSTRUCTIONS.md`](AI_BUILD_INSTRUCTIONS.md).

Documentation changes run repository policy, specification, content, and link checks. Runtime, UI, provider, plugin, migration, and platform claims require their exact additional suites and exact-head evidence.

## Contribution model

Read [`CONTRIBUTING.md`](CONTRIBUTING.md) and the [pull-request template](.github/pull_request_template.md). Every behavioural pull request must state:

- the user problem and evidence;
- owning bounded context and language changes;
- knowledge action;
- accessibility and UX effect;
- privacy and security effect;
- compatibility and migration effect;
- tests and exact validation evidence;
- risks, rollback, changelog, and remaining gates.

The project is KCS-informed. Contributors search, reuse, improve, and link knowledge while solving work. The repository does not claim KCS certification.

## Inspirations and migration sources

Liaison RM learns from, but is not a fork of:

- [Meerkat CRM](https://github.com/fbuchner/meerkat-crm) — application UX, relationships, activities, reminders, CardDAV, and graph concepts;
- [CRM in Markdown](https://github.com/CLSherrod/crm-markdown) — open-file profiles, follow-up cadence, Markdown workflows, and Obsidian interoperability;
- [Monica](https://github.com/monicahq/monica/tree/4.x) — broad personal-relationship domain concepts and migration reference;
- [Logseq](https://logseq.com/) and [Obsidian](https://obsidian.md/) — local knowledge-graph and open-note workflows.

Any code reuse must be reviewed for licence compatibility and attribution. Product inspiration does not imply endorsement or affiliation.

## Licence

Liaison RM is licensed under the [GNU Affero General Public License v3.0](LICENSE).
