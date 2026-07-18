# Liaison RM product and build specification

Status: Pre-alpha product contract; B0-before-A0 working order accepted
Repository: `Electric-Town/liaison-RM`  
Licence: AGPL-3.0  
Primary implementation: Rust, Tauri, React, TypeScript  
Canonical storage: Markdown/YAML plus documented JSONL streams

## 1. Product definition

Liaison RM is a local-authoritative relationship manager for individuals, families, executive assistants, reception teams, workplace operations, facilities teams, event organisers, and professional networkers.

It records people, organisations, locations, relationships, interactions, notes, events, attendance, reminders, important dates, contact methods, structured personal characteristics, dietary requirements, communication metadata, and bounded facilities history. It provides a native desktop interface, relationship graph, configurable profile and dashboard views, first-class CLI, local APIs, MCP tools, provider-neutral connections, and permissioned plugins.

The product does not require an Electric Town account or hosted backend. Users can inspect, back up, transform, and recover canonical records without Liaison RM.

### 1.1 Current implementation boundary

The exact working-state boundary and accepted delivery order are maintained in [`docs/product/working-state-delivery.md`](docs/product/working-state-delivery.md). At the current pre-alpha boundary, `main` contains a narrow Workspace/People/Markdown/CLI/Tauri slice, profile and reason-only Review domain foundations, and provider contracts with a limited local-folder reference adapter. It does **not** contain a complete event workflow, `WorkspaceSession`, recoverable multi-target commits, sealed dietary persistence, local grant enforcement, an encrypted recovery package, a proven Airgap artifact, or a supported public release.

An accepted decision defines what an implementation must become; it does not prove that the current binary satisfies the decision. Screenshots and browser fixtures are not native persistence, security, accessibility, Airgap, or release evidence.

## 2. Product principles

### 2.1 Local authority

- Canonical records live on storage selected and controlled by the workspace owner.
- Human-scale records use readable Markdown with versioned YAML front matter.
- High-volume append-oriented streams use documented JSONL partitions.
- SQLite, full-text search, thumbnails, graph layouts, and caches are disposable projections.
- Removing projection files and rebuilding them cannot remove current semantic information.
- No telemetry, crash upload, remote logging, account check, or licence check is enabled by default.

### 2.2 Separate Airgap and Connected-local profiles

The product contract requires two separately testable profiles.

**Airgap** compiles out network clients and listeners. It supports local files, removable-media packages, offline import/export, validation, and encrypted recovery packages.

**Connected-local** keeps the same local source of truth but may enable explicit providers, CardDAV, calendar import, email-metadata import, local API, webhooks, MCP, and peer exchange. Each connection requires a purpose-bound grant.

A runtime setting alone is not accepted as proof of an Airgap build.

The current installed review application is described as a local-authoritative review build, not as Airgap. It may be relabelled only after its dependency graph, package permissions, metadata, and runtime DNS/loopback/remote-socket denial evidence pass on the exact artifact.

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

Owns workspace identity, schema version, build profile, settings, enabled modules, lifecycle, `WorkspaceSession`, advisory writer authority, recovery state, quiescence, recoverable canonical operations, and projection lifecycle.

### Identity and Profiles

Owns person identity, name, pronouns, contact points, addresses, personal characteristics, important dates, Topic Packs, stable custom-field schema, field states, dietary source records, provenance, visibility, and profile revision. B0 consumes the shared schema through fixed workplace views; full user-organised profile layouts belong to A0.

### Workspace Security and Local Policy

Owns the workspace data-encryption key, authenticated sealed envelopes, passphrase recovery envelope, optional platform-keychain cache, local owner/device principal, purpose grants, role presets as grant bundles, policy decisions, and payload-minimal activity evidence. For B0 this policy prevents accidental or purpose-inappropriate use; it does not claim confidentiality from a person controlling the unlocked operating-system account and workspace files.

### Organisations

Owns organisations, teams, departments, cost centres, locations, roles, memberships, and organisational history.

### Relationships

Owns typed person-to-person and person-to-organisation links, circles, priority, status, cadence, relationship notes, and relationship-specific visibility. Workplace/B0 types, projections, filters, exports, and UI structurally omit relationship allocation, relationship-value ranking, and relationship-strength scoring.

### Interactions

Owns notes, communications, meetings, messages, provenance, direction, participants, source references, interaction counts, and last-interaction summaries.

### Events

Owns events, activities, attendance, invitations, participation counts, immutable cohort revisions, event-local resolution, dietary-readiness derivation, sealed internal brief evidence, staleness, and external delivery evidence.

### Directory read model

Directory is a cross-context application read model backed by a disposable SQLite/FTS projection. It combines non-sensitive Identity/Profile and Organisation/Membership facts for filtering and pagination but owns no canonical business record. Invalid canonical records remain visible as Health findings, and cohort finalisation revalidates canonical source revisions and hashes.

### Facilities

Owns access-import jobs, source mappings, badge identity resolution, raw event partitions, retention, deletions, and bounded aggregate summaries.

### Reminders

Owns bounded personal commitments, follow-up reasons, recurrence, due dates, completion, snooze, and reminder history. Liaison does not provide a generic task-management engine, and this context is not a B0 dependency.

### Connections

Owns provider descriptors, capability contracts, connection instances, provider egress grants, jobs, schedules, health, conformance evidence, and revocation. It does not own B0 local-purpose policy or the workspace key hierarchy.

### Sharing

Owns workspace roles, field and classification policy, encrypted operations, acknowledgements, key envelopes, private overlays, self-service requests, Liaison Cards, disclosures, and conflict resolution.

### Automation

Owns local API tokens, webhook subscriptions, MCP tools, plugin manifests, AI proposals, automation approvals and activity evidence, and local-model configuration. It does not own the repository-wide local-policy or audit contract.

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
- Every canonical mutation is a Workspace-owned recoverable operation. It stages every target with existence, revision, and digest preconditions, flushes a manifest, rechecks all targets, persists a durable commit decision, publishes with progress evidence, and then completes or marks the projection stale.
- An operation without a durable commit decision is discarded during recovery. A committed operation rolls forward and stops rather than overwriting a non-cooperating external edit whose digest matches neither expected nor committed content.
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
- Remote object storage remains optional encrypted-recovery transport, not attachment identity.

## 6. Person and profile model

A person profile supports the following target capabilities:

- structured names, display name, aliases, pronouns, and optional gender data;
- multiple typed email addresses, phone numbers, postal addresses, URLs, and messaging handles;
- birthday with optional unknown year, anniversaries, important dates, and reminders;
- organisation memberships with role, department, team, cost centre, location, and effective dates;
- references to relationship-owned intent, status, cadence, and next-action views without copying those values into the Person aggregate;
- freeform Markdown notes plus typed fields;
- user-defined fields with stable IDs, types, validation, classification, and display policy;
- configurable tabs and field ordering without changing canonical meaning;
- provenance, verification date, visibility, and last-updated metadata per sensitive field;
- archive and restore without destructive deletion.

The stable custom-field schema is shared foundation. B0 uses fixed workplace-oriented layouts and does not ship a general profile-layout designer. A0 owns user-organised profile tabs, stable tab IDs, ordering, visibility, settings export/import, keyboard reordering, and round trips that never lose or reinterpret field data.

### 6.1 Dietary model

A dietary requirement is not a single preference string. It records:

- a constrained operational category such as allergy, intolerance, religious restriction, ethical preference, dislike, positive preference, or another operationally relevant category;
- `coverage_state`: verified none, provided, pending, stale, declined, unreachable, excluded from catering, or unknown;
- operational instruction suitable for authorised catering use;
- verification source and date;
- review due date;
- disclosure policy;
- location or event applicability;
- audit and provenance.

An empty field means unknown. It never means no restriction.

B0 does not collect diagnosis, medical history, treatment detail, or free-form diagnostic narrative. Restricted values and person-to-dietary associations use authenticated sealed envelopes; a plaintext value with a `sealed` marker is invalid. Missing keys or authority have no plaintext fallback.

## 7. Events and dietary readiness

An event can select attendees directly, by import, or through saved cohort filters. Finalisation creates an immutable JSONL cohort stream plus a manifest containing normalized predicates, stable identities, source revisions, schema, and content hash. The readiness calculation reports:

- total selected attendees;
- confirmed attendees;
- verified-none count;
- provided requirements count;
- pending, stale, declined, unreachable, excluded, and unknown counts;
- duplicate or unresolved identities;
- requirements requiring manual review;
- least-disclosure operational instructions grouped for catering through a `DietaryOperationalView` that has no diagnostic-detail field;
- the exact profile revisions used for the calculation.

Application services evaluate the current purpose grant before decrypting or deriving this view. Internal generation commits sealed immutable brief evidence. CSV or print-safe HTML delivery is a separate verified operation and never overwrites earlier output. A failed delivery leaves the internal brief valid and retryable. A later profile change marks earlier evidence stale rather than rewriting history.

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

## 10. Local policy, sharing, and disclosure

### 10.1 B0 local role presets

B0 has one honest principal boundary: the trusted local workspace owner on a stable device. Role names are convenience presets that materialize purpose-grant bundles; they do not provide confidentiality from another person controlling the same unlocked operating-system account or files.

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

Multi-member identity, authentication, signed operations, and confidentiality between members belong to the later Sharing product and are not implied by these local presets.

### 10.2 Private overlays

A user may keep a private overlay for a shared person, relationship, event, or organisation. Overlay content:

- is encrypted to its authorised members;
- is excluded from shared projection and export;
- is not sent to AI, provider, plugin, search, or recovery-package destinations outside its grant;
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
- `backup@1` — encrypted recovery-package publication, verification, listing, retention, and restore;
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

Providers advertise safe modes based on evidence. An object store may be approved as encrypted-recovery transport without being approved for multi-writer synchronisation. The UI and CLI must not label a provider “sync” merely because it can upload and download files.

## 12. CLI

The `liaison` binary is a supported product surface.

### 12.1 Implemented pre-alpha commands

The checked-in CLI currently implements only:

```text
liaison --workspace PATH workspace init|inspect|validate
liaison --workspace PATH person create|list
```

This slice does not yet satisfy the accepted typed result/error envelope, explicit mutation-workspace, non-zero invalid-validation, `WorkspaceSession`, recovery, import, event, or provider command contracts. [`apps/cli/README.md`](apps/cli/README.md) is the current executable command inventory.

### 12.2 Target command groups

The following is a compatibility target, not a claim that the commands exist:

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

CLI and Tauri use one versioned application result/error envelope with stable code, safe message, cause, typed details, recovery actions, documentation reference, and correlation ID. Sensitive values and unnecessary absolute paths are redacted before either interface receives them.

## 13. Desktop and browser surfaces

The primary desktop shell uses Tauri 2. The React/TypeScript UI calls typed Tauri commands that map to application services. Tauri capabilities are scoped by window and build profile.

The current pre-alpha shell is directly committed HTML/CSS/JavaScript and is disposable. Before B0 screens land, it migrates once to a React/TypeScript/Vite inbound adapter and must prove Workspace, People, and Health parity through the same Rust application commands. JavaScript does not own canonical persistence, authorization, readiness, recovery, or a second domain model.

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
- Restricted persistable values use authenticated sealed envelopes. No plaintext-plus-marker representation or plaintext fallback is valid.
- A local checkpoint is a quiescent deterministic diagnostic/migration primitive and may be plaintext. It is never labelled a user-portable backup.
- An encrypted recovery package contains canonical data, integrity manifests, minimal audit, and a passphrase-wrapped workspace recovery envelope; it must restore without prior platform-keychain state.
- Encrypted recovery packages are validated before provider upload.
- Restore verifies manifest, hashes, schema compatibility, and target path before replacement.
- Deletion distinguishes archive, local deletion, remote deletion, retention expiry, and cryptographic revocation.
- Audit records actor, operation, scope, purpose, grant, result, and timestamp without copying sensitive payloads unnecessarily.

## 16. Non-functional requirements

### Reliability

- One advisory writer authority per opened workspace and read-only Health/recovery when normal open cannot acquire or validate write authority.
- Recoverable multi-target canonical operations with a durable commit decision, roll-forward recovery, external-edit refusal, and exhaustive fault tests.
- Deterministic migrations with dry-run and an appropriate quiescent checkpoint or encrypted recovery package.
- Idempotent imports and provider jobs.
- Projection rebuild from canonical files.
- Checkpoint verification and encrypted clean-install recovery testing.

### Performance

- B0 Directory rebuild, filtering, import, cohort finalisation, and readiness meet the committed budgets for 10,000 people and 50,000 memberships without loading the complete workspace into the interface.
- The longer-term architecture supports at least 100,000 people and millions of partitioned stream events; that target is not a B0 completion claim.
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

## 17. Working-state delivery sequence

The accepted order is dependency-driven and supersedes older personal-desktop-before-event readings of R1/R2/R3. The machine-readable plan must use the same order.

1. **P00 — reconcile contracts and truth:** accepted decisions, formats, requirements/UAT/gates/tasks, versions, commands, evidence, dependency licences, installed-binary claims, and stale branches agree.
2. **P01 — application composition root:** one `liaison-application` command/query/error model is shared by CLI and Tauri; validation and degraded-open semantics are corrected.
3. **P02 — Workspace authority:** `WorkspaceSession`, advisory writer lock, read-only recovery, quiescence, session-bound ports, and schema behavior.
4. **P03 — recoverable operations:** staged multi-target writes, durable commit decision, roll-forward recovery, exactly-once minimal evidence, and fault/race tests.
5. **P04 — desktop adapter migration:** React/TypeScript/Vite over typed Rust commands with Workspace/People/Health parity and a semantic design system.
6. **P05 — sensitive/domain contracts:** revisioned People, Organisations, Groups, Locations, Memberships, Events, provenance, field state, sealed value, and event-local resolution types.
7. **P06 — scalable Directory reads:** tolerant scan, disposable SQLite/FTS projection, filters, pagination, canonical revalidation, Health findings, and 10,000/50,000 evidence.
8. **P07 — Workspace Security and local policy:** key lifecycle, recovery envelope, optional Keychain cache, trusted local owner/device, purpose grants, role presets, and payload-minimal activity evidence.
9. **P08 — recovery before real sensitive data:** quiescent local checkpoint plus encrypted clean-install recovery package.
10. **P09 — Directory onboarding:** People maintenance and streaming CSV preview/reconciliation for People, Organisations, Locations, Groups, and Memberships.
11. **P10 — Events core:** immutable cohort, exact readiness, structurally limited `DietaryOperationalView`, sealed internal brief, verified delivery, and staleness.
12. **P11 — B interface:** Overview, Directory, Events, Health, and Settings plus the complete cohort-to-brief state machine.
13. **B0 — Workplace Review Alpha:** for one trusted local workspace owner, the installed universal Mac review artifact passes scale, crash, key, grant, leak, encrypted-restore, accessibility, offline, readable-file, and contributor-journey evidence. Workplace surfaces structurally omit relationship allocation/ranking/scoring. It remains an internal review alpha unless public signing/notarisation gates pass.
14. **A0 — Personal Memory Alpha:** only after B0 acceptance, add profile editing, stable user-organised tabs/layouts with lossless settings round trips, meaningful interactions, bounded commitments, last-interaction/open-loop views, and reason-only Review over the same foundations. A0 does not add a generic task engine.
15. **Post-A0:** sharing, provider transports, contacts/calendars/email, facilities, mobile products, Meitheal integration, OpenAPI/MCP/AI/plugins, Linux/Windows support, and public notarized distribution advance under their own gates.

## 18. Explicit exclusions from B0

B0 Workplace Review Alpha does not require:

- multi-writer remote sync;
- cloud AI;
- Gmail or calendar connectors;
- CardDAV server;
- plugin marketplace;
- access-log analysis;
- full browser PWA;
- native mobile applications or phone synchronisation;
- every platform package;
- provider transports, AI, MCP, or Meitheal integration;
- personal interactions, commitments, or reason-only Review;
- user-organised profile tabs or a general profile-layout designer;
- a generic task-management engine;
- relationship allocation, relationship-value ranking, or relationship-strength scoring;
- Developer ID signing, notarisation, or a supported public download.

The first acceptance workflow is a receptionist selecting an event cohort, finding every dietary-information gap, and producing a purpose-authorised least-disclosure catering brief from local records. B0 is independently reviewable but is not a supported public release.

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
