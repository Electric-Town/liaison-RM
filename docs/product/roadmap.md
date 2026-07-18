# Product delivery roadmap

The roadmap is ordered by dependency and user value. Dates are intentionally absent until the team has measured delivery throughput and completed user observation.

## R0 — Repository foundation

Outcome: contributors can make reviewable decisions and AI agents can execute bounded tasks without private context.

Deliverables:

- governance, security, conduct, KCS-informed knowledge, DDD, UX, and content standards;
- product specification, ubiquitous language, context map, threat model, and decision records;
- machine-readable requirements, UAT, feature gates, and implementation plan;
- repository checks and release evidence structure.

Exit evidence:

- policy checks pass;
- all protected principles have an accepted decision or explicit proposal;
- each R1 task has an owner context and acceptance test.

## R1 — Open workspace and first-class CLI

Outcome: a user can create, inspect, validate, back up, restore, and edit a small relationship workspace without a GUI or network.

Deliverables:

- Rust workspace and bounded-context skeleton;
- workspace initialisation, manifest, locking, validation, journal, and projection lifecycle;
- person profile aggregate, typed contact points, important dates, custom fields, provenance, and archive;
- Markdown/YAML adapter with unknown-field preservation and revision preconditions;
- local content-addressed attachments;
- `liaison` CLI with human/JSON output, dry-run, stable exit codes, and completion scripts;
- vCard and CSV import/export foundations;
- backup create, verify, and isolated restore to local/removable storage.

Exit evidence:

- canonical round-trip and unknown-field tests;
- crash/interruption recovery test;
- projection deletion/rebuild test;
- backup restore exercise;
- CLI UAT on Linux, macOS, and Windows runners;
- Airgap dependency and socket-denial evidence.

## R2 — Native desktop foundations

Outcome: a personal user can complete R1 workflows through an accessible desktop application.

Deliverables:

- Tauri shell and typed command bridge;
- accessible navigation, search, person list, profile tabs, edit forms, and validation;
- configurable field visibility and ordering;
- dashboard framework with user-configurable panels and keyboard controls;
- relationship list and graph with equivalent semantic table/tree;
- theme, density, reduced motion, versioned locale catalogs, pseudolocale expansion testing, and interruption-safe drafts;
- Linux Flatpak, macOS, and Windows packaging pipelines.

Exit evidence:

- keyboard, screen-reader, 200% zoom, reflow, contrast, target-size, and reduced-motion tests;
- locale-key, placeholder, Unicode, 45% expansion, and localized accessibility-name tests;
- named human-review evidence before any non-source locale is marked release-ready;
- graph/table parity test;
- installer smoke tests and local-data uninstall/reinstall behaviour;
- no mandatory account or network request on first run.

## R3 — Workplace event and dietary-readiness wedge

Outcome: a receptionist or event manager can select an employee cohort, resolve dietary-information gaps, and produce a least-disclosure catering brief.

Deliverables:

- organisations, departments, teams, cost centres, locations, and effective memberships;
- structured dietary requirement and coverage model;
- bulk profile import with mapping and duplicate preview;
- event creation, attendance, invitations, cohort filters, and saved views;
- readiness dashboard and stale-information detection;
- least-disclosure brief with revision snapshot and audit;
- role presets for receptionist, dietary coordinator, and event manager.

Exit evidence:

- observed unassisted UAT with representative reception/event users;
- every selected attendee represented in readiness output;
- empty never interpreted as no restriction;
- diagnostic detail excluded from operational brief without grant;
- interruption/resume and event-day recovery tests.

## R4 — Sharing, encrypted transport, and provider foundations

Outcome: a family or team can exchange authorised updates through local, removable-media, WebDAV, or object-store transports without sharing a plaintext multi-writer vault.

Deliverables:

- member/device identity, roles, grants, signed operations, acknowledgements, conflict records, and key envelopes;
- private overlays;
- self-service request/response and Liaison Cards;
- provider SDK and descriptor registry;
- local object-store reference adapter;
- WebDAV and S3-compatible transports;
- encrypted provider-neutral backup and restore;
- connection/grant UI and CLI.

Exit evidence:

- two-device concurrent edit tests;
- revoked device replay test;
- provider rollback/corruption test;
- private-overlay non-disclosure test;
- backup versus sync labels match conformance evidence;
- key recovery exercise.

## R5 — Contacts, calendars, email metadata, migration, and facilities

Outcome: users can bring existing relationship history and selected phone/workplace data into Liaison RM without losing provenance or control.

Deliverables:

- CardDAV selected-view synchronisation;
- CalDAV/iCalendar import with recurrence identity;
- provider-neutral email-metadata contract and initial provider;
- interaction counts and source-confidence summaries;
- Meerkat and Monica migration adapters;
- facilities access import, mapping, unresolved identity, retention, and bounded summaries;
- event participation history and communication recency reports.

Exit evidence:

- idempotent re-import tests;
- contact conflict and unknown-vCard-field preservation;
- recurring calendar tests;
- email body absent under default grant;
- access data cannot enter productivity/risk scoring;
- migration reconciliation reports.

## R6 — Local automation, AI, and plugins

Outcome: developers and advanced users can automate Liaison through stable local interfaces without bypassing grants or domain rules.

Deliverables:

- loopback OpenAPI service, scoped tokens, webhooks, and n8n examples;
- MCP server with source-linked reads and staged writes;
- Ollama-compatible local inference;
- remote AI provider grants;
- WASI Component Model plugin host and WIT SDK;
- plugin manifest review, resource limits, UI contributions, migrations, and namespaced locale catalogs;
- provider and plugin conformance kits.

Exit evidence:

- automation and MCP cannot mutate outside granted scope;
- prompt and tool injection tests;
- plugin filesystem/network denial tests;
- plugin locale keys cannot replace core keys without an explicit compatibility contract;
- local-model workflow functions without remote account;
- remote disclosure preview and revocation;
- compatibility tests across supported plugin contract versions.

## Product discovery cadence

Before each release:

1. Observe at least one complete target workflow without guiding the participant.
2. Record the workaround, interruptions, errors, and recovery behaviour.
3. Compare the observed problem with the planned release outcome.
4. Remove or defer work that does not change the target outcome.
5. Update requirements, UAT, knowledge, and feature gates with the evidence.

Interest, waitlists, and feature requests are inputs. Repeated behaviour and failure consequences determine priority.
