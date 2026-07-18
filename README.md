<p align="center">
  <a href="https://electric-town.github.io/liaison-RM/">
    <img src="site/assets/social-card.svg" alt="Liaison RM: Remember the person, not the pipeline" width="1200">
  </a>
</p>

<p align="center">
  <a href="https://electric-town.github.io/liaison-RM/"><strong>Project site</strong></a>
  · <a href="PROJECT_CONTEXT.md">Current status</a>
  · <a href="SPEC.md">Product contract</a>
  · <a href="CONTRIBUTING.md">Contributing</a>
</p>

# Liaison RM

**Remember the person. Not the pipeline.**

Liaison RM is a local-authoritative relationship memory and attention system. It is for people who want CRM-grade organisation without sales-pipeline assumptions, a hosted relationship vault or a score that pretends message frequency measures closeness.

The product is being built around readable Markdown and YAML records, a shared Rust core, a first-class `liaison` CLI and a native Tauri desktop. Search indexes, caches and graph layouts are projections that can be rebuilt. The workspace stays under the owner’s control.

> [!WARNING]
> **Pre-alpha.** The default branch has a tested workspace, People and CLI slice, a desktop alpha, provider contracts, profile-readiness and reason-only review foundations. It is not ready for daily use or public binary distribution. There are no signed downloads.

## Why this exists

A contact list remembers an address. A sales CRM remembers an opportunity. Neither is built to hold the context that helps someone follow through on a promise, prepare for a meeting, respect a boundary or remember what matters to another person.

Liaison RM takes four positions:

- a person is not a lead;
- frequent contact does not prove trust, affection or importance;
- missing information is a state to resolve, not permission to guess;
- relationship attention needs an honest reason, not a guilt score.

The intended result is simple: capture a useful detail once, find it before it matters and keep the record even if Liaison RM disappears.

## What works today

| Surface | On the default branch | Boundary |
|---|---|---|
| Open workspace | Create, inspect and validate a versioned local workspace | Complete crash recovery, migrations and projection rebuild remain gated |
| People | Create and list basic person records in readable Markdown | The daily relationship directory and full profile editing flow are not complete |
| CLI | Human and JSON output for workspace and person commands | Import, edit, backup, sharing and destructive commands remain gated |
| Desktop alpha | Create or open a workspace, add a person and run validation | Review builds are not signed public releases |
| Relationship model | Separate intent, evidence, maintenance status and purpose-specific readiness | Weighted priority and the full relationship workflow are not released |
| Review and Attention | Reason-only policy, hard suppressions and bounded queue foundations | No claim of a complete personal review experience |
| Connections | Versioned object-store contract, grant model and local reference adapter | Upload evidence does not prove safe multi-writer synchronisation |
| Localisation | `en-IE` source catalogue, `en-XA` stress locale and draft locale fixtures | Irish, Japanese and Brazilian Portuguese still require named human review |
| Packaging | macOS review bundles and Windows NSIS build configuration in CI | Signing, notarisation, clean-machine UAT and supported downloads remain closed |

The exact status, current branches and open gates live in [`PROJECT_CONTEXT.md`](PROJECT_CONTEXT.md#2-current-status). Do not treat an open pull request, a prototype or a passing unit test as released behaviour.

## Run the current CLI

This path exercises the implemented Rust application services and Markdown adapter. It creates a disposable workspace with synthetic data.

Requirements: Git and the pinned Rust toolchain from [`rust-toolchain.toml`](rust-toolchain.toml).

```bash
git clone https://github.com/Electric-Town/liaison-RM.git
cd liaison-RM

liaison_demo="$(mktemp -d)"

cargo run --locked -p liaison-cli -- \
  --workspace "$liaison_demo" \
  workspace init --name "Liaison demo" --build-profile airgap

cargo run --locked -p liaison-cli -- \
  --workspace "$liaison_demo" \
  person create --name "Alex Example" --email "alex@example.test"

cargo run --locked -p liaison-cli -- \
  --workspace "$liaison_demo" \
  workspace validate
```

Use `--output json` before the command group for structured output. See [`apps/cli/README.md`](apps/cli/README.md) for the full current command set and error contract.

## The relationship model

Liaison RM does not collapse a relationship into one number.

| Concept | Meaning | Source |
|---|---|---|
| **Relationship intent** | How the user wants to maintain the relationship, including cadence, boundaries and future state | Manually configured |
| **Relationship evidence** | Interactions, notes, commitments, events and imported history | Recorded or imported facts |
| **Maintenance status** | Whether something needs attention relative to its own configuration | Explainable calculation |
| **Profile readiness** | Whether the information required for a named purpose is known and current | Purpose-specific calculation |

Good review copy names the reason:

```text
Venue shortlist promised for Friday. Last note: keep the room step-free.
```

Rejected copy invents a truth the evidence cannot support:

```text
Relationship strength: 42%.
```

Reason-only review is the default. Archive, do-not-contact, pause, snooze and no-cadence states override review pressure.

## Topic Packs and explicit information states

Profiles are composed from reusable **Topic Packs** rather than one fixed contact form. A pack can describe identity and communication, food and hospitality, travel, important dates, family context, professional roles, accessibility needs, events or resources.

Fields have stable IDs that do not depend on their visible label. Values can be:

- known or verified;
- unverified, stale or conflicting;
- unknown or in need of clarification;
- not applicable;
- declined;
- derived.

An empty allergy, accessibility or dietary field never means “none”. Readiness is calculated for a named purpose such as event catering or a meeting briefing, not as one universal profile-completeness score.

## Open workspace

A Liaison workspace is an ordinary directory the owner can inspect, back up, transform and recover without a hosted service:

```text
workspace/
├── .liaison/
│   ├── workspace.yaml
│   ├── schema-version
│   ├── grants/
│   ├── operations/
│   └── projections/
├── people/
├── organisations/
├── relationships/
├── interactions/
├── reminders/
├── events/
├── attachments/sha256/
└── audit/
```

Human-scale records use Markdown with versioned YAML front matter. Documented JSONL partitions hold high-volume machine streams. Unknown fields and user-authored Markdown sections must survive supported round trips. Secrets never enter canonical files.

Read the full storage contract in [`docs/architecture/open-workspace.md`](docs/architecture/open-workspace.md).

## Architecture

```text
Desktop / CLI / local API / MCP / importer / plugin adapter
                              ↓
                    application command or query
                              ↓
                      owning domain model
                              ↓
                        outbound port
                              ↓
                           adapter
```

Domain rules live in the owning bounded context. React, Tauri commands, providers, importers, plugins and AI tools call application services; they do not reimplement those rules.

| Area | Direction |
|---|---|
| Core | Rust domain and application services |
| Desktop | Tauri 2; current frontend calls typed Tauri commands |
| CLI | First-class `liaison` binary using the same services |
| Canonical records | Markdown/YAML plus documented JSONL streams |
| Projections | Disposable SQLite, search, graph layout, thumbnails and caches |
| Providers | Versioned capability contracts with explicit grants |
| Plugins | WASI Component Model and WIT with denied-by-default capabilities |
| Automation | Planned loopback OpenAPI, webhooks, MCP and local-model mediation |

Start with the [context map](docs/architecture/context-map.md), [ubiquitous language](docs/architecture/ubiquitous-language.md) and [accepted decisions](docs/decisions/README.md).

## Privacy and safety boundary

Liaison RM may hold sensitive personal, dietary, accessibility, calendar, workplace and private relationship context. The repository requires:

- explicit purpose, scope and expiry before data leaves the workspace;
- least-disclosure exports and previews;
- no hidden telemetry, remote logging, account check or licence check;
- no automatic messages without user action;
- no employee, productivity, attendance-compliance or risk scoring;
- no private assessment in shared search, exports, AI context or provider data;
- no personal data sent to a model without an explicit grant.

Airgap and Connected-local are separate build profiles. A runtime toggle is not proof of an Airgap build.

Read the [threat model](docs/security/threat-model.md), [local-integrity requirements](docs/security/local-integrity.md) and [sharing architecture](docs/architecture/sharing-and-synchronization.md).

## Accessibility and language

The project targets WCAG 2.2 Level AA and applicable EN 301 549 evidence. This is an engineering target, not a certification claim.

User-facing work must support keyboard completion, visible focus, screen-reader names, 200% zoom and reflow, reduced motion, interruption recovery, long content and a semantic alternative to graph-only or drag-only interaction.

`en-IE` is the source locale. Draft `ga-IE`, `ja-JP` and `pt-BR` catalogues are not released translations. A locale needs a named fluent reviewer, product-context review, layout evidence and accessibility sampling before it can appear in a production language selector.

See the [UX review standard](docs/standards/ux-review.md), [language-quality standard](docs/standards/localization-and-language-quality.md) and [current locale evidence](docs/evidence/localization/README.md).

## Build order

| Release | Outcome |
|---|---|
| **R0** | Governance, product contracts, threat model, requirements, UAT and agent-ready context |
| **R1** | Open workspace, people, CLI, validation, import/export foundations, backup and recovery |
| **R2** | Accessible native desktop, search, profiles, dashboard and platform packaging |
| **R3** | Workplace event cohorts, dietary readiness and least-disclosure catering briefs |
| **R4** | Encrypted sharing, grants, provider transport and connection management |
| **R5** | Contacts, calendars, email metadata, migration and bounded facilities imports |
| **R6** | Local OpenAPI, MCP, webhooks, local AI and capability-controlled plugins |

The first operational wedge is event dietary readiness: select an attendee cohort, identify every unresolved coverage state and produce a least-disclosure catering brief.

The dependency order and remaining gates are in the [roadmap](docs/product/roadmap.md), [feature gates](spec/feature-gates.yaml) and [implementation plan](spec/implementation-plan.yaml).

## Build with a coding agent

Read these files before changing code:

1. [`AGENTS.md`](AGENTS.md), the normative contributor and agent contract.
2. [`PROJECT_CONTEXT.md`](PROJECT_CONTEXT.md), the current product and engineering handoff.
3. [`SPEC.md`](SPEC.md), the product and build specification.
4. [`AI_BUILD_INSTRUCTIONS.md`](AI_BUILD_INSTRUCTIONS.md), the implementation sequence.
5. The owning README under [`contexts/`](contexts/).
6. Related decisions, knowledge articles, requirements, UAT cases, gates and open pull requests.

Do not begin with a new screen or connector. Start with one dependency-complete slice through domain rules, application services, ports, adapters, CLI or API surface, tests, knowledge and changelog.

## Validate a change

Run the checks required by the changed scope. The baseline is:

```bash
python3 scripts/check_repository.py
python3 scripts/check_spec.py
python3 scripts/check_architecture.py
python3 scripts/check_providers.py
python3 scripts/check_wit_contract.py
cargo fmt --all --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

Public-site changes also run:

```bash
python3 scripts/check_public_site.py
```

Tests only support claims when they ran against the submitted commit. Platform packaging, browser behaviour, provider conformance and releases need their own exact-head evidence.

## Project map

```text
apps/                 Desktop, CLI and local services
contexts/             Bounded contexts and application services
crates/               Narrow shared technical libraries
adapters/             Filesystem, projection and provider adapters
interfaces/           WIT, OpenAPI, MCP and external contracts
providers/            Optional provider packages and evidence
docs/                 Product, architecture, security, knowledge and evidence
spec/                 Requirements, UAT, feature gates and build order
site/                 Static source for the public GitHub Pages site
scripts/              Repository, contract and site checks
```

The [public-site runbook](docs/public-site.md) explains deployment, metadata, locale and rollback rules.

## Contributing

Read [`CONTRIBUTING.md`](CONTRIBUTING.md) and the [pull-request template](.github/pull_request_template.md). Every behavioural change states the user problem, context owner, evidence, privacy effect, accessibility effect, compatibility path, exact tests and remaining gates.

The project is KCS-informed. Search existing knowledge first, improve it while solving the task and leave enough context for the next contributor. The repository does not claim KCS certification.

## Inspirations and migration sources

Liaison RM learns from [Meerkat CRM](https://github.com/fbuchner/meerkat-crm), [CRM in Markdown](https://github.com/CLSherrod/crm-markdown), [Monica](https://github.com/monicahq/monica/tree/4.x), [Logseq](https://logseq.com/) and [Obsidian](https://obsidian.md/). It is not a fork of those projects.

Any code reuse requires licence review and attribution. Product inspiration does not imply endorsement or affiliation.

## Licence

Liaison RM is licensed under the [GNU Affero General Public License v3.0](LICENSE).
