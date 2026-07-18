# Liaison RM project context

Status: living handoff document  
Last substantial review: 2026-07-18  
Repository: `Electric-Town/liaison-RM`

This document gives a new maintainer, contributor, or coding agent enough context to continue the project without private conversation history. It describes the product boundary, architecture, current state, accepted constraints, release gates, and expected working method. It does not replace normative requirements, schemas, decisions, or tests.

## 1. Authority and document precedence

Use this precedence when sources disagree:

1. accepted architecture decision records;
2. versioned canonical schemas and external contracts;
3. `SPEC.md` and machine-readable requirements or UAT;
4. `AGENTS.md` and repository standards;
5. bounded-context READMEs and tests;
6. knowledge articles and release evidence;
7. this context document;
8. implementation code and open pull-request descriptions;
9. prototypes and exploratory notes.

Code does not become the intended design merely because it exists. Record a conflict before changing a protected principle.

## 2. Product statement

Liaison RM is a local-authoritative personal relationship memory and attention system with CRM-grade organisation. It is designed for people who need to remember context, commitments, preferences, dates, organisations, interactions, events, and resources without surrendering their relationship history to a hosted CRM.

The product serves personal users, families, executive assistants, reception and culture teams, workplace and facilities operators, event managers, professional networkers, CRM administrators, developers, and privacy administrators.

The first narrow workplace outcome is:

> A receptionist selects the correct attendee cohort, identifies every unresolved dietary-information state, records or requests missing information, and produces a safe least-disclosure catering brief before an event.

The personal outcome is broader:

> A user can remember and act on relationship context without relying on working memory, being shamed by an overdue backlog, or reducing relationships to communication volume.

## 3. What Liaison is not

Liaison is not:

- a sales funnel, lead-scoring engine, or marketing automation system;
- a hosted social network or mandatory Electric Town service;
- an employee productivity, attendance-compliance, performance, or risk-ranking tool;
- an objective relationship-strength, trust, affection, compatibility, or human-value calculator;
- a plaintext shared-folder multi-writer database;
- a browser application whose origin-private storage is the canonical vault;
- a general surveillance or email-archive product;
- an autonomous AI agent with unrestricted write or disclosure authority;
- a promise that every remote AI vendor will offer free integration.

## 4. Protected product principles

### 4.1 Local authority

- The workspace owner selects and controls canonical storage.
- Human-scale records are readable Markdown with versioned YAML front matter.
- High-volume append-oriented data uses documented JSONL partitions.
- Attachments are content addressed.
- SQLite, full-text search, graph layouts, thumbnails, and caches are projections.
- Deleting and rebuilding projections cannot remove current semantic information.
- The user can inspect, transform, back up, and recover records without Liaison.

### 4.2 Separate release profiles

**Airgap** compiles out network clients and listeners. It supports local files, removable media, offline import/export, validation, backup, and recovery.

**Connected-local** keeps the same local source of truth but may enable explicitly granted providers, local APIs, webhooks, MCP, contact or calendar sync, email metadata, and peer exchange.

A runtime preference does not prove that a binary is Airgap-safe.

### 4.3 One application core

Desktop, CLI, local API, MCP, background jobs, importers, and plugins call the same Rust application services. Business rules are not copied into React, JavaScript, command parsing, provider adapters, workflow YAML, or prompt text.

### 4.4 Provider neutrality

Google Drive, WebDAV, S3-compatible storage, AWS S3, MinIO, Google Cloud Storage, Azure Blob Storage, local folders, removable media, CardDAV, CalDAV, Gmail, and future services are adapters behind versioned capability contracts.

Provider identity may be domain data in the Connections context. Provider SDK types do not enter People, Events, Relationships, Sharing, or other business models.

### 4.5 Explainable attention

The product separates:

- relationship intent;
- relationship evidence;
- maintenance status;
- profile readiness;
- Review Priority.

Reason-only review is the default. A weighted policy is optional, transparent, versioned, configurable, and subordinate to hard suppressions such as do-not-contact, paused, archived, ended, or snoozed.

### 4.6 Least disclosure

A workflow receives only the fields, classifications, operations, purposes, and time window it needs. Dietary operational instructions remain separable from medical detail. Private assessments and overlays do not enter shared views, exports, AI context, provider jobs, or backups outside their grant.

### 4.7 Accessible and interruption-tolerant interaction

The engineering target is WCAG 2.2 Level AA with relevant EN 301 549 evidence. Graph and drag interactions have semantic and keyboard alternatives. Workflows support interruption recovery, reduced motion, 200% zoom, reflow, long text, explicit status, undo, and recovery.

This is a delivery target, not a legal certification claim.

## 5. Primary personas and required outcomes

### Personal user or networker

- remember structured and freeform personal context;
- record last interaction, last meaningful note, important dates, commitments, and next actions;
- use editable relationship types, tiers, cadence, boundaries, and statuses;
- see why a person appears in review;
- keep canonical history independent of a phone address book or hosted CRM.

### Family or household

- share selected birthdays, anniversaries, activities, preferences, and contact information;
- keep private notes private;
- maintain portable Liaison Cards or request/response packages;
- use local, removable-media, WebDAV, or object-store transport when authorised.

### Executive assistant

- prepare a principal using source-linked context;
- separate principal-private overlays from shared operational profiles;
- track delegated commitments, introductions, scheduling, travel, food, and meeting preferences;
- disclose only authorised fields for the current purpose.

### Reception, culture, and event operations

- filter attendees by organisation, location, department, team, cost centre, group, or saved view;
- distinguish allergy, intolerance, medical restriction, religious restriction, ethical preference, dislike, positive preference, verified none, pending, stale, declined, unreachable, excluded, and unknown;
- never treat an empty field as no restriction;
- export a least-disclosure operational catering brief;
- count recorded event attendance and identify unresolved identities.

### Facilities

- import access events with mapping preview, idempotency, rollback, and unresolved-identity handling;
- use raw data only for authorised facilities or event purposes;
- apply retention and deletion policy;
- prohibit productivity, performance, attendance-compliance, and risk scoring.

### Developer and automation user

- use stable CLI, OpenAPI, webhooks, n8n, MCP, and plugin surfaces;
- add providers through contracts and conformance suites;
- use local models without a remote account;
- prevent clients, plugins, and prompts from bypassing grants or domain rules.

## 6. Domain model

### 6.1 Entity types

Do not model everything as a generic Contact. The intended entities include:

- Person;
- Organisation;
- Group;
- Household;
- Location;
- Event;
- Resource;
- Membership;
- Relationship;
- Interaction;
- Commitment;
- Reminder;
- Review Session;
- Connection;
- Grant;
- Provider Descriptor.

A UI contact view may combine people and organisations, but that view does not redefine the aggregates.

### 6.2 Topic Packs and fields

Profiles use Topic Packs that can be enabled by workspace, template, organisation, group, person, plugin, or temporary purpose. Built-in candidates include:

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

Each field has a stable ID independent of its label and layout. Supported field types include short and long text, Markdown, date, partial date, recurring date, enum, multi-select, boolean, number, measurement, address, location, entity reference, file or URL reference, sealed value, calculated read-only value, and plugin-defined value.

Field information state is explicit. Initial states include known, verified, unverified, unknown, not applicable, declined, stale, conflicting, needs clarification, and derived. A value may carry source, author or connector, capture date, verification date, review date, visibility, confidence, purpose, and history.

### 6.3 Relationship intent and review

Relationship intent is manual and can include type, tier, importance, desired cadence, preferred channel, desired future state, reason, boundaries, paused-until, do-not-contact, and review date.

Maintenance status uses factual labels such as on track, due soon, overdue relative to cadence, open commitment, important date approaching, stale context, no cadence, paused, do not contact, or archived.

Review Priority orders an attention queue. Operating modes are reason-only, tiered, and weighted. The system does not infer trust or affection from message volume and does not rank employees.

### 6.4 Organisations and memberships

Organisation membership is dated and supports simultaneous roles, historical employment, department, team, cost centre, office location, manager/reporting links, current filtering, and as-of-date filtering.

Groups may be static, query-driven, household-based, team-based, or event snapshots. Groups can own notes, files, dates, events, resources, permissions, and Topic Pack defaults.

### 6.5 Resources

Files, URLs, calendar events, email references, photos, documents, maps, voice-note references, recordings, gifts, and contact cards are first-class Resources with provenance, backlinks, classification, and timeline presence.

External metadata is not fetched without permission. Removing a connector does not remove the local resource record unless deletion policy says so.

## 7. Bounded contexts

### Workspace

Owns workspace identity, schema version, build profile, settings, members, devices, enabled modules, and lifecycle.

### Identity and Profiles / People

Owns person identity, contact points, Topic Packs, field values and states, provenance, important dates, visibility, archive, and profile revision.

### Organisations and Groups

Owns organisations, households, groups, locations, departments, teams, cost centres, roles, memberships, and organisational history.

### Relationships

Owns typed edges, circles, intent, boundaries, cadence, status, relationship-specific notes, and private assessment snapshots.

### Interactions and Commitments

Owns communications, meetings, notes, participants, source references, commitments, tasks, and interaction summaries.

### Events and Calendar

Owns events, recurrence, invitations, cohorts, attendance, participation history, and dietary-readiness snapshots.

### Knowledge and Resources

Owns notes, files, URLs, content-addressed attachments, calendar and email references, resource metadata, and backlinks.

### Review and Attention

Owns cadence evaluation, maintenance status, purpose-specific profile readiness, review reasons, policies, queues, suppressions, sessions, and Markdown review output.

### Facilities

Owns access-import jobs, source mapping, identity resolution, event partitions, retention, deletion, and bounded aggregate summaries.

### Connections

Owns provider descriptors, capability contracts, connection instances, grants, schedules, jobs, health, conformance, expiry, and revocation.

### Sharing

Owns member and device identity, roles, signed operations, acknowledgement, reconciliation, conflict records, key envelopes, private overlays, Liaison Cards, self-service requests, and disclosures.

### Automation

Owns local API tokens, webhooks, MCP tools, plugin manifests, AI proposals, approvals, audit, and local-model configuration.

### Customisation

Owns field schemas, Topic Pack definitions, profile layouts, saved views, dashboard composition, and plugin-contributed definitions.

## 8. Architecture and dependency direction

```text
apps and interfaces
        ↓
application commands and queries
        ↓
domain policies and aggregates
        ↓
context-owned ports
        ↓
adapters and providers
```

Rules:

- inbound applications depend on context APIs;
- domain crates depend only on deliberate shared-kernel types and ordinary libraries required for domain meaning;
- adapters implement context-owned ports;
- provider code depends on Connections contracts, not vice versa;
- read models may combine published data but cannot write another context's aggregates;
- cross-context workflows use explicit orchestration and compensation or recovery;
- persistence, UI, provider, and transport DTOs remain private to their boundary.

## 9. Canonical storage

### Markdown/YAML

- one human-scale record per file unless a context documents another partition;
- stable UUID and record revision in front matter;
- human-readable filename is non-authoritative;
- unknown fields and supported body sections survive rewrites;
- validation precedes replacement;
- revision preconditions and content hashes detect known stale writes;
- invalid files remain visible to validation and repair.

### JSONL

High-volume access and email-metadata streams are UTF-8 JSONL partitioned by source and time. Every event has stable source identity and import-job identity. Re-import is idempotent. Retention evidence does not retain deleted sensitive payloads.

### Attachments

Attachments use SHA-256 content addresses. Metadata records original name, media type, size, classification, provenance, and references. Remote object storage remains optional transport or backup.

### Projections

SQLite, full-text search, graph layouts, thumbnails, and caches are deleted and rebuilt without losing semantic records. Projection lifecycle and rebuild evidence remain release gates.

## 10. Provider and sharing architecture

`object-store@1` supports immutable put, get, head, list, guarded delete, and manifest replacement by expected revision. The local-folder reference provider has evidence only for backup and controlled single-writer use.

A provider descriptor records ID, version, contracts, operations, safe modes, configuration fields, secret references, destinations, consistency, limits, and conformance. Registration grants no access.

A configured connection also needs a grant containing purpose, endpoint, operations, fields or data classes, retention, schedule, expiry, approver, and revocation.

Multi-user sharing does not write concurrently to shared readable Markdown. Devices exchange encrypted immutable operations and locally materialise authorised views. Private overlays remain outside shared indexes and disclosures.

## 11. AI, MCP, and plugin architecture

The planned local automation surface includes:

- loopback OpenAPI with scoped tokens;
- webhooks and n8n examples;
- MCP tools with source-linked reads and staged writes;
- Ollama-compatible local inference;
- remote model adapters behind disclosure grants;
- WASI Component Model plugins with WIT contracts and resource limits.

AI and plugin output is untrusted input. It cannot expand its own authority. A model, client, or plugin does not receive a raw database handle or unrestricted workspace path.

## 12. User experience standard

Every workflow should support:

- stable navigation and explicit headings;
- visible focus and keyboard completion;
- screen-reader labels and polite live status;
- interruption-safe drafts and return to the prior task state;
- loading, empty, partial, stale, conflict, permission, success, undo, and recovery states;
- 200% zoom and responsive reflow;
- long labels and 30–45% localization expansion;
- reduced motion and low-stimulation modes;
- text alternatives for colour, icons, hover, animation, graph position, and drag;
- small capacity-bounded review sessions;
- skip, snooze, pause, archive, decline, and ask-later as valid outcomes;
- no gamification of relationship volume or contact frequency.

The project uses behavioural observation and task evidence. Feature enthusiasm is not treated as proof of need.

## 13. Security and privacy model

- no mandatory account or hosted relationship database;
- no hidden telemetry, crash upload, remote log, licence check, or undeclared update request;
- secrets remain in an OS or approved secret store and are referenced, not exported;
- purpose-bound grants control providers, APIs, AI, plugins, sharing, and exports;
- sensitive dietary, accessibility, communication, and facilities fields use stricter classification and disclosure;
- destructive changes require preview and recovery;
- imports are parsed as untrusted input;
- path traversal, symlink, checksum, stale revision, replay, and malicious-file threats require focused tests;
- access history cannot enter employee scoring;
- fixtures and screenshots use synthetic identities.

See `docs/security/threat-model.md` and `docs/security/local-integrity.md`.

## 14. Current implementation state

The repository uses stacked pull requests. The current review sequence is documented in `docs/STATUS.md` and must be checked before work begins.

Implemented or present under review:

- repository governance and KCS-informed workflow;
- DDD, UX, accessibility, and content standards;
- product specification, context map, decisions, threat model, requirements, UAT, feature gates, and implementation plan;
- static interaction prototype and committed review screens;
- Rust shared identifiers and revisions;
- Workspace and People domain/application slices;
- Markdown workspace adapter;
- local CLI workspace and person workflows;
- provider-neutral Connections contract and provider SDK;
- local-folder object-store adapter and conformance evidence;
- native Tauri desktop alpha for local workspace and basic People workflows;
- cross-platform Rust checks and macOS review-bundle workflows.

Not production-ready:

- complete crash-safe journalling and repair;
- projection rebuild lifecycle;
- isolated backup and restore;
- Topic Pack runtime and Review and Attention engine;
- organisations, relationships, interactions, events, reminders, resources, facilities, and sharing runtime contexts;
- WebDAV, S3, Google Drive, CardDAV, calendar, email, and migration providers;
- local OpenAPI, MCP, Ollama, and plugin host;
- Flatpak and Windows production installers;
- Developer ID signing, notarization, stapling, Gatekeeper, and clean-machine Mac UAT;
- formal accessibility conformance evidence;
- production release.

## 15. Delivery sequence

### R0: repository foundation

Governance, product specification, decisions, requirements, UAT, gates, standards, and evidence structure.

### R1: open workspace and CLI

Workspace lifecycle, person records, journalling, projections, import/export, attachments, backup/restore, stable CLI, and Airgap evidence.

### R2: native desktop foundations

Tauri shell, accessible navigation and forms, configurable profiles, dashboard, graph/table parity, themes, localization structure, and Linux/macOS/Windows packaging.

### R3: workplace event and dietary-readiness wedge

Organisations, memberships, dietary model, event cohorts, readiness, least-disclosure brief, and operational roles.

### R4: sharing and providers

Member/device identity, grants, signed operations, private overlays, self-service exchange, Liaison Cards, WebDAV/S3 transports, and encrypted backup.

### R5: contacts, calendars, email, migration, and facilities

CardDAV, CalDAV/iCalendar, email metadata, Meerkat/Monica migration, access imports, participation history, and source-confidence summaries.

### R6: automation, AI, and plugins

OpenAPI, webhooks, n8n, MCP, Ollama, remote model grants, WASI plugin host, and conformance kits.

## 16. Immediate priorities

Unless an accepted decision changes the order, the next dependency-sensitive priorities are:

1. finish review and merge the existing stack in order;
2. close R1 integrity gaps: journalled writes, interruption recovery, projection rebuild, backup verification, isolated restore, and Airgap evidence;
3. complete native desktop foundations without duplicating application rules;
4. establish runtime Topic Pack field-state contracts and reason-only Review and Attention;
5. implement Organisations and Groups before event cohort and workplace workflows;
6. add Interactions, Commitments, Events, Resources, and Reminders through context-owned vertical slices;
7. delay weighted Review Priority until reason-only review is trusted;
8. delay remote providers and AI until grants, secrets, audit, backup, and recovery are real.

Do not expand breadth while integrity or recovery gates remain untested.

## 17. Development and review method

A complete behavioural change includes:

- a problem statement grounded in supplied or observed behaviour;
- the owning context and ubiquitous-language effect;
- domain tests and application-service tests;
- adapter, integration, interruption, and recovery tests where relevant;
- CLI and other surfaces calling the same service;
- requirements, UAT, feature-gate, schema, and traceability updates;
- knowledge article action;
- changelog and status action;
- privacy, security, accessibility, localization, migration, compatibility, and rollback review;
- exact-head CI evidence.

The PR template is not optional ceremony. It records why the change exists and what evidence supports it.

## 18. Validation baseline

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

Additional checks exist for the interaction prototype, desktop UI, generated assets, provider contracts, macOS bundles, signatures, and release packaging. Run checks implied by the changed paths.

## 19. Repository navigation

- `README.md`: public overview and entry point;
- `AGENTS.md`: normative contributor contract;
- `AI_BUILD_INSTRUCTIONS.md`: executable task order;
- `SPEC.md`: normative product/build specification;
- `docs/STATUS.md`: current implementation, PR stack, and release gates;
- `docs/DEVELOPMENT.md`: setup and commands;
- `docs/architecture/`: context map, storage, sharing, providers, language;
- `docs/decisions/`: accepted and proposed architecture decisions;
- `docs/product/`: roadmap and discovery material;
- `docs/standards/`: DDD, UX, accessibility, knowledge, content, release rules;
- `docs/knowledge/`: reusable support and operational knowledge;
- `docs/security/`: threat model and local-integrity requirements;
- `docs/evidence/`: exact validation and release evidence;
- `spec/`: machine-readable requirements, UAT, gates, releases, personas, tasks;
- `schemas/`: canonical and integration validation schemas;
- `examples/`: synthetic examples and interoperability contracts.

## 20. Glossary

**Canonical record**: authoritative open-file representation of current semantic information.  
**Projection**: rebuildable derived data used for search, display, or performance.  
**Topic Pack**: versioned group of profile field definitions enabled by context or purpose.  
**Field state**: explicit information status such as verified, unknown, stale, or declined.  
**Relationship intent**: manually configured purpose, importance, cadence, boundaries, and desired future state.  
**Relationship evidence**: factual recorded or imported history.  
**Maintenance status**: explainable state relative to configured cadence and boundaries.  
**Profile readiness**: purpose-specific evaluation of required field states.  
**Review Priority**: ordering mechanism for a review queue, never a measure of human worth.  
**Provider**: adapter package implementing one or more versioned capability contracts.  
**Connection**: configured provider instance that is still inert without a grant.  
**Grant**: purpose-bound permission for endpoint, operations, scope, retention, schedule, and expiry.  
**Private overlay**: member-scoped information excluded from shared views and unauthorised disclosure.  
**Liaison Card**: signed, selective portable profile intended for controlled exchange.  
**Airgap**: separately built profile with network clients and listeners compiled out.  
**Connected-local**: local-authoritative profile that can use explicitly granted connections.
