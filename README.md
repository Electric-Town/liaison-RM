# Liaison RM

> A local-authoritative relationship memory and attention system with CRM-grade organisation, readable files, and no requirement to treat people as leads.

Liaison RM is an open-source relationship manager for individuals, families, executive assistants, reception teams, workplace operations, facilities teams, event organisers, and professional networkers. It combines readable Markdown records with a native desktop application, a first-class CLI, structured personal context, purpose-specific profile readiness, explainable review queues, and provider-neutral connections.

Canonical records remain on storage selected by the user. SQLite, search indexes, graph layouts, thumbnails, and caches are disposable projections.

## Status

**Pre-release. Do not use Liaison RM as the sole copy of important personal or workplace data.**

The foundational stack through profile readiness, reason-only review, and provider-neutral object storage is merged into `main`. The native Tauri desktop and macOS review bundles remain under review. Remote providers, encrypted sharing, production installers, and publication-grade release evidence are not complete.

| Area | Current state |
|---|---|
| Governance, DDD, KCS-informed contribution rules | Merged and checked |
| Product specification, requirements, UAT, feature gates | Merged and checked |
| Rust Workspace and People contexts | Merged and cross-platform tested |
| Markdown/YAML workspace adapter and CLI | Initial vertical slice merged |
| Topic Pack field states and purpose-specific readiness | Initial domain runtime merged |
| Reason-only Review and Attention queue | Initial domain runtime merged |
| Provider-neutral Connections contracts | Merged and cross-platform tested |
| Local-folder object-store provider | Passed with explicit backup/single-writer limits |
| Native Tauri desktop application | Alpha under review |
| Apple Silicon and Intel review bundles | Workflow under review; not a public release |
| Localization architecture | Draft review work exists; production translations are not approved |
| Sharing, WebDAV, S3, CardDAV, email, facilities, AI and plugins | Specified; not production-ready |

See [Current status](docs/STATUS.md) before selecting work or making release claims.

## Start here

| Need | Read |
|---|---|
| Full product and engineering context | [Project context](docs/PROJECT_CONTEXT.md) |
| Normative contributor and agent rules | [AGENTS.md](AGENTS.md) |
| Executable build order for coding agents | [AI_BUILD_INSTRUCTIONS.md](AI_BUILD_INSTRUCTIONS.md) |
| Product and architecture specification | [SPEC.md](SPEC.md) |
| Current implementation and release gates | [docs/STATUS.md](docs/STATUS.md) |
| Development setup and validation commands | [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) |
| Delivery sequence | [docs/product/roadmap.md](docs/product/roadmap.md) |
| Documentation index | [docs/README.md](docs/README.md) |

A new contributor or coding agent should read those files in the listed order before changing code.

## Product model

Liaison separates concepts that conventional CRMs often collapse into one misleading score:

| Concept | Meaning |
|---|---|
| **Relationship intent** | What the user wants from a relationship, its importance, boundaries, cadence, and desired future state |
| **Relationship evidence** | Recorded interactions, notes, events, commitments, files, calendar references, and imported facts |
| **Maintenance status** | An explainable state relative to the relationship's own cadence and boundaries |
| **Profile readiness** | Whether required information is known and current for a named purpose |
| **Review Priority** | Optional ordering of an attention queue, never a measurement of a person or relationship |

Liaison does not infer affection, trust, employee value, or relationship strength from message volume. Reason-only review is the personal-use default.

Profiles use configurable **Topic Packs** rather than one universal form. Examples include identity and communication, food and hospitality, travel, family, pets, gifts, professional context, events, accessibility, executive-assistant briefing, and linked resources. Empty values are not interpreted as negative facts. Fields can be explicitly known, verified, unverified, unknown, stale, declined, conflicting, not applicable, derived, or awaiting clarification.

The merged `profiles` and `review-attention` crates provide the first runtime contract for stable field IDs, classifications, sealed sensitive values, purpose-specific readiness, hard review suppressions, factual reasons, deterministic ordering, and capacity-bounded reason-only queues. Persistence, activation inheritance, encryption, profile editing, and weighted policy execution remain open work.

## Primary workflows

### Personal and family

- remember birthdays, anniversaries, household context, gifts, pets, preferences, and important topics;
- review a small, explainable set of relationships without a guilt-producing backlog;
- retain private notes while sharing selected household information;
- keep data readable in Markdown and portable to tools such as Obsidian or Logseq.

### Executive assistants

- prepare source-linked meeting briefs;
- separate principal-private overlays from operational profiles;
- track commitments, introductions, travel, food, scheduling, and meeting preferences;
- expose only fields permitted for the current purpose.

### Reception, culture, and events

- select attendees by organisation, location, department, team, cost centre, group, or saved view;
- identify unknown, stale, declined, or unresolved dietary coverage before ordering catering;
- produce least-disclosure operational briefs;
- record event attendance and participation history.

### Facilities

- import badge and access events through previewed, idempotent jobs;
- resolve unknown badge identities explicitly;
- apply retention and deletion policy;
- prohibit productivity, performance, attendance-compliance, and employee-risk scoring.

### Developers and automation users

- use one Rust application core through CLI, desktop, local OpenAPI, MCP, jobs, and plugins;
- add providers without importing provider SDKs into business contexts;
- use Ollama-compatible local inference without an external account;
- stage AI writes for review instead of allowing unrestricted mutation.

## Non-negotiable constraints

- Human-scale canonical records use documented Markdown with versioned YAML front matter.
- High-volume append-oriented streams use documented JSONL partitions.
- Unknown fields and user-authored body sections survive supported round trips.
- SQLite and other projections are rebuildable and non-authoritative.
- Airgap and Connected-local are separately testable build profiles.
- No Electric Town account, telemetry service, remote licence check, crash upload, or mandatory cloud service is required.
- Remote providers remain inert until the user grants a purpose, endpoint, operation, scope, retention, and expiry.
- Provider registration is not permission.
- A shared plaintext Markdown directory is not treated as a safe multi-writer database.
- Domain crates do not depend on Tauri, React, SQL, filesystems, HTTP clients, or cloud SDKs.
- Desktop, CLI, API, MCP, importers, jobs, and plugins call the same application services.
- Sensitive personal, dietary, accessibility, communication, and facilities data follows least-disclosure policy.
- Graph and drag interactions always have keyboard and semantic alternatives.
- No interface, provider, plugin, or AI client calculates its own relationship score, readiness result, maintenance state, or permission decision.

## Architecture

```text
Desktop / CLI / local API / MCP / jobs / plugins
                         │
                         ▼
                 Application services
                         │
                         ▼
              Bounded-context domain models
                         │
                         ▼
                  Context-owned ports
                         │
                         ▼
       Markdown, projections, providers, OS adapters
```

Initial bounded contexts:

- **Workspace**: identity, schema version, build profile, settings, lifecycle;
- **People**: basic person identity, contact points, important dates, archive;
- **Identity and Profiles**: Topic Packs, stable fields, information states, classification, readiness;
- **Organisations and Groups**: organisations, departments, teams, cost centres, households, locations, memberships;
- **Relationships**: typed edges, intent, boundaries, cadence, private assessments;
- **Interactions and Commitments**: notes, communications, meetings, promises, tasks, reminders;
- **Events and Calendar**: events, attendance, recurrence, cohorts, dietary readiness;
- **Knowledge and Resources**: files, URLs, calendar references, attachments, backlinks;
- **Review and Attention**: maintenance status, purpose-specific readiness inputs, review reasons, policies, queues, suppressions;
- **Facilities**: access import, identity resolution, retention and bounded summaries;
- **Connections**: provider descriptors, contracts, connection instances, grants, jobs and conformance;
- **Sharing**: members, roles, encrypted operations, private overlays, cards and disclosure;
- **Automation**: local API, webhooks, MCP, AI proposals and plugin execution;
- **Customisation**: field schemas, Topic Pack definitions, layouts, saved views and dashboard composition.

A bounded context owns its vocabulary and invariants. Cross-context work uses explicit application interfaces, events, read models, or anti-corruption layers.

## Canonical workspace

```text
workspace/
├── .liaison/
│   ├── workspace.yaml
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

Record IDs are stable and independent of filenames. File formats are versioned separately from Rust persistence structs.

## Release profiles

### Airgap

Network clients and listeners are compiled out. The build supports local files, removable-media packages, offline import/export, backup, validation, and recovery. A runtime toggle is not accepted as proof of an Airgap build.

### Connected-local

The same local source of truth may use explicitly granted providers, CardDAV, calendar and email-metadata import, local APIs, webhooks, MCP, or peer exchange. No remote endpoint is enabled merely because the build can support one.

## Providers and sharing

Providers implement versioned capability contracts. The first contract, `object-store@1`, covers immutable object publication, retrieval, listing, guarded deletion, and manifest replacement by expected revision. A provider can be labelled backup, single-writer, or multi-writer only when its evidence supports that mode.

Team and family sharing is designed around encrypted immutable operations, acknowledgements, key envelopes, conflict records, and locally materialised Markdown views. WebDAV, S3-compatible storage, Google Drive, local folders, and removable media are transports or backup destinations, not alternative domain models.

## Local AI and plugins

The planned automation layer includes a loopback OpenAPI service, MCP tools, n8n examples, Ollama-compatible local inference, and capability-controlled WASI plugins. Read tools identify source records and grants. AI writes are staged proposals by default. Plugins receive no ambient network, filesystem, database, or secret access.

## Repository map

```text
apps/                 Desktop and CLI applications
contexts/             Domain-driven bounded contexts
crates/               Narrow shared technical libraries
adapters/             Filesystem, projection, provider, and OS adapters
interfaces/           Versioned WIT, OpenAPI, MCP, and external contracts
providers/            Optional provider packages and evidence
spec/                 Requirements, UAT, feature gates, schemas, plans
schemas/              Canonical and integration schemas
examples/             Synthetic workspace and integration examples
docs/                 Architecture, product, standards, knowledge, evidence
scripts/              Policy, architecture, schema, UI, and conformance checks
packaging/             Platform packaging and capability profiles
```

## Development quick start

Use the pinned Rust toolchain in `rust-toolchain.toml`.

```bash
python scripts/check_repository.py
python scripts/check_spec.py
python scripts/check_architecture.py
python scripts/check_providers.py
python scripts/check_wit_contract.py
python scripts/check_relationship_model.py

cargo fmt --all --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

CLI example:

```bash
cargo run -p liaison-cli -- workspace init --path ./my-liaison --name "My relationships"
cargo run -p liaison-cli -- person create --workspace ./my-liaison --name "Alex Murphy"
cargo run -p liaison-cli -- workspace validate --path ./my-liaison
```

Desktop development:

```bash
cd apps/desktop
cargo tauri dev
```

See [Development guide](docs/DEVELOPMENT.md) for platform prerequisites, UI tests, packaging checks, and the pull-request workflow.

## Roadmap

The dependency order is:

1. repository, architecture, open workspace, CLI, and recovery;
2. native desktop foundations and platform packaging;
3. profile persistence, organisations, interactions, events, and reason-only review adapters;
4. workplace event and dietary-readiness workflow;
5. encrypted sharing and provider-backed backup or exchange;
6. contacts, calendars, email metadata, migration, and facilities;
7. local API, MCP, AI, and capability-controlled plugins.

Weighted Review Priority remains after reason-only review and purpose-specific readiness are understood in real workflows.

## Contributing

Read [AGENTS.md](AGENTS.md), [Project context](docs/PROJECT_CONTEXT.md), and [CONTRIBUTING.md](CONTRIBUTING.md) before changing the repository. Every behavioural pull request identifies the user problem, owning bounded context, domain and privacy effects, knowledge action, tests, risks, rollback, and release impact.

The project uses a KCS-informed solve loop. Durable operational knowledge belongs in `docs/knowledge/`; architecture decisions belong in `docs/decisions/`; exact validation and release claims require inspectable evidence.

## Licence

Liaison RM is licensed under the GNU Affero General Public License v3.0. See [LICENSE](LICENSE).
