# Liaison RM product and build specification

Status: Draft for review  
Repository: `Electric-Town/liaison-RM`  
Licence: AGPL-3.0  
Primary implementation: Rust, Tauri, React, TypeScript  
Canonical storage: Markdown/YAML plus documented JSONL streams

## 1. Product definition

Liaison RM is a local-authoritative relationship manager for individuals, families, executive assistants, reception teams, workplace operations, facilities teams, event organisers, and professional networkers.

It records people, organisations, locations, relationships, interactions, notes, events, attendance, reminders, important dates, contact methods, structured personal characteristics, dietary requirements, communication metadata, and bounded facilities history. It provides a native desktop interface, relationship graph, configurable profile and dashboard views, first-class CLI, local APIs, MCP tools, provider-neutral connections, and permissioned plugins.

The product does not require an Electric Town account or hosted backend. Users can inspect, back up, transform, and recover canonical records without Liaison RM.

## 2. Product principles

### 2.1 Local authority

- Canonical records live on storage selected and controlled by the workspace owner.
- Human-scale records use readable Markdown with versioned YAML front matter.
- High-volume append-oriented streams use documented JSONL partitions.
- SQLite, full-text search, thumbnails, graph layouts, and caches are disposable projections.
- Removing projection files and rebuilding them cannot remove current semantic information.
- No telemetry, crash upload, remote logging, account check, or licence check is enabled by default.

### 2.2 Separate Airgap and Connected-local profiles

The project publishes two separately testable profiles.

**Airgap** compiles out network clients and listeners. It supports local files, removable-media packages, offline import/export, backup, validation, and recovery.

**Connected-local** keeps the same local source of truth but may enable explicit providers, CardDAV, calendar import, email-metadata import, local API, webhooks, MCP, and peer exchange. Each connection requires a purpose-bound grant.

A runtime setting alone is not accepted as proof of an Airgap build.

### 2.3 One application core

Desktop, CLI, local OpenAPI, MCP, importers, background jobs, and plugins call the same Rust application services. Domain rules are not reimplemented in React, shell scripts, connector adapters, or provider code.

### 2.4 Provider neutrality

Google Drive, S3-compatible stores, AWS S3, MinIO, Google Cloud Storage, Azure Blob Storage, WebDAV, local folders, removable media, Syncthing-compatible exchange, CardDAV, CalDAV, Gmail, and future services are adapters behind versioned capability contracts.

The domain model records provider identity only as data. It does not depend on a provider SDK.

### 2.5 Safe sharing

Liaison RM does not treat a shared plaintext Markdown directory as a safe multi-writer database. Remote transports exchange encrypted immutable operations, manifests, acknowledgements, key envelopes, and content-addressed attachments. Each device validates and materialises its local readable view.

Private overlays remain outside shared indexes and disclosures. A workspace can request selected information from a person without requiring that person to create an account.

### 2.6 Accessible, interruption-tolerant interaction

The product must remain usable with keyboard and screen reader, at 200% zoom, with reduced motion, after interruption, and under time pressure. Graph and drag interactions always have semantic alternatives. The project targets WCAG 2.2 Level AA with corresponding EN 301 549 evidence; this is an engineering target, not an unsupported certification claim.

## 3. Primary personas and outcomes

### 3.1 Individual or networking professional

- Remember personal and professional context without relying on memory.
- Rank relationships as VIP, maintain, watch, deprioritised, ignored, or archived.
- See last interaction, last meaningful note, next action, and relationship cadence.
- Use phone contacts without surrendering the canonical relationship history to a hosted CRM.

### 3.2 Executive assistant

- Prepare a principal for meetings using authorised relationship context.
- Keep principal-private overlays separate from shared operational profiles.
- Track commitments, introductions, important dates, and delegated follow-up.
- Produce a source-linked briefing without disclosing unauthorised notes.

### 3.3 Workplace receptionist and culture team

- Record each location employee’s dietary coverage state and operational instruction.
- Select an event cohort by location, organisation, department, team, cost centre, or saved view.
- Identify unknown, stale, declined, or unresolved dietary information before catering is ordered.
- Export a least-disclosure catering brief that does not expose diagnoses or irrelevant personal detail.

### 3.4 Facilities operator

- Import badge or access events with a mapping preview and idempotent job record.
- Resolve unknown identities without silently attaching events to the wrong person.
- Apply retention and deletion policy.
- View bounded office-use summaries without productivity or performance scoring.

### 3.5 Event manager

- Record activities and attendees.
- Count recorded event participation per person or cohort.
- Check dietary readiness and unresolved invitations.
- Import and export attendee lists without losing source provenance.

### 3.6 Family or household

- Share selected contact, birthday, anniversary, activity, and preference information.
- Keep private notes private.
- See upcoming important dates and recurring family activities.
- Exchange updates through local, WebDAV, object-store, or removable-media transports.

### 3.7 Developer and automation user

- Use a stable CLI, local OpenAPI, webhook, n8n, and MCP surface.
- Add a provider or plugin without modifying domain crates.
- Use Ollama or another local model without an external account.
- Connect remote AI clients only through explicit field and operation grants.

## 4. Bounded contexts

### Workspace

Owns workspace identity, schema version, build profile, members, device registrations, workspace settings, enabled modules, and lifecycle.

### People

Owns person identity, name, pronouns, contact points, addresses, personal characteristics, important dates, dietary requirements, field provenance, visibility, and profile revision.

### Organisations

Owns organisations, teams, departments, cost centres, locations, roles, memberships, and organisational history.

### Relationships

Owns typed person-to-person and person-to-organisation links, circles, priority, status, cadence, relationship notes, and relationship-specific visibility.

### Interactions

Owns notes, communications, meetings, messages, provenance, direction, participants, source references, interaction counts, and last-interaction summaries.

### Events

Owns events, activities, attendance, invitations, participation counts, event groups, catering cohorts, and dietary-readiness evaluation.

### Facilities

Owns access-import jobs, source mappings, badge identity resolution, raw event partitions, retention, deletions, and bounded aggregate summaries.

### Reminders

Owns commitments, follow-up reasons, recurrence, due dates, completion, snooze, and reminder history.

### Connections

Owns provider descriptors, capability contracts, connection instances, grants, jobs, schedules, health, conformance evidence, and revocation.

### Sharing

Owns workspace roles, field and classification policy, encrypted operations, acknowledgements, key envelopes, private overlays, self-service requests, Liaison Cards, disclosures, and conflict resolution.

### Automation

Owns local API tokens, webhook subscriptions, MCP tools, plugin manifests, AI proposals, approvals, audit, and local-model configuration.

## 5. Canonical workspace

```text
workspace/
├── .liaison/
│   ├── workspace.yaml
│   ├── schema-version
│   ├── devices/
│   ├── members/
│   ├── grants/
│   ├── migrations/
│   └── operations/
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

### 5.1 Markdown record rules

- One record per file unless a context explicitly specifies another partition.
- Stable UUID in front matter; filename is human-readable and non-authoritative.
- Schema version and record revision are mandatory.
- Unknown front-matter fields and unknown body sections are preserved on rewrite.
- Writes use a temporary file, flush, atomic replacement where supported, and a recovery journal.
- Concurrent edit detection uses revision preconditions and content hashes.
- External edits are validated before projection update.
- Invalid records remain visible to validation and recovery tools; they are not silently dropped.

### 5.2 JSONL stream rules

- UTF-8, one independently valid object per line.
- Partitioned by source and time.
- Each event has a stable source event ID and import-job ID.
- Imports are idempotent.
- Retention deletion is recorded in audit evidence without retaining deleted sensitive payloads.
- Stream compaction produces a verifiable manifest and preserves semantic event identity.

### 5.3 Attachments

- Stored by SHA-256 content address.
- Metadata records original name, media type, size, classification, provenance, and references.
- Canonical records reference hashes rather than provider URLs.
- Remote object storage remains optional transport or backup, not attachment identity.

## 6. Person and profile model

A person profile supports:

- structured names, display name, aliases, pronouns, and optional gender data;
- multiple typed email addresses, phone numbers, postal addresses, URLs, and messaging handles;
- birthday with optional unknown year, anniversaries, important dates, and reminders;
- organisation memberships with role, department, team, cost centre, location, and effective dates;
- relationship priority, status, cadence, and next-action metadata;
- freeform Markdown notes plus typed fields;
- user-defined fields with stable IDs, types, validation, classification, and display policy;
- configurable tabs and field ordering without changing canonical meaning;
- provenance, verification date, visibility, and last-updated metadata per sensitive field;
- archive and restore without destructive deletion.

### 6.1 Dietary model

A dietary requirement is not a single preference string. It records:

- `kind`: allergy, intolerance, medical restriction, religious restriction, ethical preference, dislike, positive preference, or other;
- `coverage_state`: verified none, provided, pending, stale, declined, unreachable, excluded from catering, or unknown;
- operational instruction suitable for authorised catering use;
- optional detailed note with a stricter classification;
- verification source and date;
- review due date;
- disclosure policy;
- location or event applicability;
- audit and provenance.

An empty field means unknown. It never means no restriction.

## 7. Events and dietary readiness

An event can select attendees directly, by import, or through saved cohort filters. The readiness calculation reports:

- total selected attendees;
- confirmed attendees;
- verified-none count;
- provided requirements count;
- pending, stale, declined, unreachable, excluded, and unknown counts;
- duplicate or unresolved identities;
- requirements requiring manual review;
- least-disclosure operational instructions grouped for catering;
- the exact profile revisions used for the calculation.

The export omits diagnostic detail unless the recipient has a grant for that detail. A later profile change marks an earlier brief stale rather than silently rewriting historical evidence.

## 8. Interaction, calendar, and email metadata

Liaison RM records interactions from manual entry, calendar events, email metadata, contact sync, and approved connectors.

- Full email bodies are not imported by default.
- Default email scope is headers and relationship metadata: participants, direction, timestamps, thread ID, message ID, labels if granted, and optional source link.
- Counts are clearly described as imported/observed counts, not universal truth.
- Last interaction distinguishes source and confidence.
- Imported calendar events preserve provider event IDs and recurrence identity.
- A user can disconnect a provider without losing already imported records, subject to deletion policy.
- Re-import is idempotent and does not duplicate interactions.

## 9. Facilities and access data

Access import supports CSV and provider adapters with mapping, preview, validation, duplicate detection, identity resolution, and rollback.

Permitted product uses include:

- recorded office-use history;
- event or facilities planning;
- aggregate location occupancy summaries;
- identifying unresolved badge mappings;
- retention compliance.

The core product prohibits productivity scoring, performance scoring, attendance-compliance scoring, employee-risk scoring, or hidden monitoring based on access data.

Access streams use stricter role, retention, export, AI, and audit controls than ordinary profile fields.

## 10. Sharing and disclosure

### 10.1 Workspace roles

Initial roles:

- owner;
- workspace administrator;
- relationship editor;
- executive assistant;
- receptionist;
- event manager;
- dietary coordinator;
- facilities importer;
- read-only member;
- guest contributor.

Permissions can be constrained by context, organisation, location, group, record type, field, classification, purpose, operation, and expiry.

### 10.2 Private overlays

A user may keep a private overlay for a shared person, relationship, event, or organisation. Overlay content:

- is encrypted to its authorised members;
- is excluded from shared projection and export;
- is not sent to AI, provider, plugin, search, or backup destinations outside its grant;
- can be independently backed up and recovered.

### 10.3 Self-service requests

A `.liaison-request` package states requested fields, purpose, expiry, disclosure policy, requester identity, response transport, and signature. A recipient can review and return a `.liaison-response` without an account. The workspace records what was requested, what was disclosed, and why.

### 10.4 Liaison Cards

A person can maintain a signed portable card containing selected contact methods, pronouns, important-date visibility, dietary operational instruction, accessibility preference, organisation/role, and communication preference. Cards are selective and revocable for future updates; they are not exports of the full profile.

## 11. Connections and providers

### 11.1 Capability contracts

Initial contracts:

- `object-store@1` — immutable object publication, retrieval, metadata, listing, guarded deletion, and manifest replacement by expected revision;
- `webdav-transport@1` — object-store semantics over WebDAV with declared consistency limits;
- `contacts@1` — vCard/CardDAV import, export, and selected-view sync;
- `calendar@1` — iCalendar/CalDAV discovery and event import;
- `email-metadata@1` — permissioned message/thread metadata import;
- `backup@1` — encrypted snapshot publication, verification, listing, retention, and restore;
- `webhook@1` — signed outbound event delivery;
- `secret-store@1` — reference-based credential retrieval without canonical-file exposure.

A provider descriptor declares contract versions, operations, safe modes, endpoints, limits, consistency model, configuration schema, secret references, and conformance evidence.

### 11.2 Grants

A connection instance is inert until a user creates a grant containing:

- purpose;
- provider and endpoint;
- operations;
- fields or data classes;
- record or cohort scope;
- schedule;
- retention;
- expiry;
- approving member;
- revocation path.

The application shows a dry-run summary before first disclosure. Every job records the grant revision used.

### 11.3 Backup versus sync

Providers advertise safe modes based on evidence. An object store may be approved for encrypted backup without being approved for multi-writer synchronisation. The UI and CLI must not label a provider “sync” merely because it can upload and download files.

## 12. CLI

The `liaison` binary is a supported product surface.

Required command groups:

```text
liaison workspace init|open|validate|rebuild|repair
liaison person create|show|edit|list|search|archive|restore
liaison organisation create|list|show
liaison relationship add|remove|list|prioritise
liaison note add
liaison interaction log|list|import
liaison reminder add|complete|snooze|list
liaison event create|show|list|attendees|dietary-readiness
liaison import csv|vcard|meerkat|monica|access
liaison export markdown|json|vcard|catering-brief
liaison backup create|verify|list|restore
liaison connection add|list|grant|revoke|test|run
liaison provider list|inspect|conformance
liaison share request|respond|pack|apply
liaison serve
liaison mcp serve
liaison doctor
```

Mutating commands support `--dry-run`, `--workspace`, `--output human|json`, deterministic exit codes, revision preconditions, non-interactive operation where safe, and explicit confirmation for irreversible actions.

## 13. Desktop and browser surfaces

The primary desktop shell uses Tauri 2. The React/TypeScript UI calls typed Tauri commands that map to application services. Tauri capabilities are scoped by window and build profile.

Target packages:

- Ubuntu and other Linux distributions, including Flatpak;
- macOS application bundle and DMG;
- Windows MSI or NSIS installer.

A local process may serve an authenticated browser/PWA client. The browser stores only disposable client state and cannot become the sole holder of canonical data or encryption keys.

## 14. API, MCP, AI, and plugins

### 14.1 Local API

- Binds to loopback by default.
- Uses scoped tokens, expiry, and operation grants.
- Publishes OpenAPI.
- Supports webhooks with signatures, replay protection, retry policy, and delivery evidence.
- Does not expose filesystem paths or database internals as the domain API.

### 14.2 MCP

MCP tools call the same application services and include source references. Read tools are grant-scoped. Write tools produce a staged proposal by default; the user reviews the fields, records, and consequences before application.

### 14.3 AI clients

The project supports local Ollama-compatible inference without a subscription. Remote ChatGPT, Claude, Gemini, or other clients may connect only when their product supports a compatible tool protocol and the user creates a grant. Liaison RM does not promise free access to third-party vendor plans.

### 14.4 Plugins

Plugins use the WASI Component Model with WIT-defined contracts and denied-by-default capabilities. A manifest declares contract version, commands, data scope, network destinations, filesystem scope, secrets, UI contributions, migrations, and compatibility.

Plugins cannot receive a raw database handle or unrestricted workspace path. Installation shows a capability diff and records approval.

## 15. Security and privacy

- Threat modelling is required for storage, sharing, provider, plugin, API, MCP, AI, import, export, and packaging changes.
- Secrets use platform secret storage or an encrypted local vault and appear in canonical files only as opaque references.
- Logs are local, structured, redacted, bounded by retention, and exportable for review.
- Sensitive exports require explicit scope and show a preview.
- Workspace encryption and sharing keys remain under member control.
- Backups are encrypted before provider upload.
- Restore verifies manifest, hashes, schema compatibility, and target path before replacement.
- Deletion distinguishes archive, local deletion, remote deletion, retention expiry, and cryptographic revocation.
- Audit records actor, operation, scope, purpose, grant, result, and timestamp without copying sensitive payloads unnecessarily.

## 16. Non-functional requirements

### Reliability

- Crash-safe canonical writes.
- Deterministic migrations with dry-run and backup.
- Idempotent imports and provider jobs.
- Projection rebuild from canonical files.
- Backup verification and isolated restore testing.

### Performance

- Common profile and search operations remain responsive with at least 100,000 people and millions of partitioned stream events on supported hardware.
- Large imports stream rather than loading the complete source into memory.
- Graph views use bounded neighbourhoods, clustering, filters, and progressive loading.

### Accessibility

- WCAG 2.2 Level AA engineering target.
- EN 301 549 evidence for applicable desktop and web requirements.
- Complete keyboard support and visible focus.
- Screen-reader semantics and status announcements.
- 200% zoom/reflow and localisation-safe layout.
- Reduced motion.
- Graph and drag alternatives.

### Portability

- Canonical formats are documented independently of Rust structs.
- Export does not require the index.
- Obsidian and Logseq can read person, organisation, event, note, and relationship records.
- Unknown extensions survive round trips.

### Maintainability

- DDD context boundaries enforced in CI.
- Provider conformance suites.
- Generated contracts reproducible.
- Knowledge and release evidence updated with behaviour.
- No platform-specific domain fork.

## 17. Release sequence

### R0 — Repository foundation

Governance, KCS-informed workflow, product specification, decision records, requirements, feature gates, and review automation.

### R1 — Open workspace and CLI

Rust workspace, workspace lifecycle, person profiles, Markdown adapter, validation, rebuild, import/export foundations, and complete headless CLI path.

### R2 — Desktop foundations

Tauri shell, accessible navigation, profile tabs, list/search, dashboard framework, graph with semantic alternative, and Linux/macOS/Windows packaging.

### R3 — Workplace event wedge

Organisations, locations, groups, structured dietary model, event cohorts, readiness calculation, least-disclosure catering brief, and receptionist/event-manager UAT.

### R4 — Sharing and provider transport

Roles, private overlays, encrypted operations, self-service requests, Liaison Cards, local/removable-media/WebDAV/object-store transport, backup and restore.

### R5 — Contact, calendar, email metadata, and facilities

CardDAV selected-view sync, CalDAV/iCalendar import, provider-neutral email metadata, access import, retention, interaction counts, and migration adapters.

### R6 — Automation, AI, and plugin ecosystem

Local API, webhooks, n8n, MCP, Ollama, remote AI grants, WASI plugin host, SDK, conformance kit, and provider catalogue.

## 18. Explicit exclusions from the first useful release

The first useful release does not require:

- multi-writer remote sync;
- cloud AI;
- Gmail or calendar connectors;
- CardDAV server;
- plugin marketplace;
- access-log analysis;
- full browser PWA;
- every platform package.

The first observed workflow is a receptionist selecting an event cohort, finding every dietary-information gap, and producing a least-disclosure catering brief from local records.

## 19. Definition of done

A capability is done only when:

- its owning context and language are documented;
- requirements and UAT are linked;
- domain, application, adapter, and UI responsibilities are separated;
- migrations and compatibility are addressed;
- privacy, security, retention, and grant effects are reviewed;
- accessibility states and alternatives are tested where user-facing;
- CLI and automation behaviour are defined when applicable;
- knowledge and changelog are updated;
- feature-gate evidence is recorded;
- tests and release checks demonstrate the claimed behaviour;
- unresolved limitations are visible to users and contributors.
