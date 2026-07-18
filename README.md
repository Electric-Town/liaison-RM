# Liaison RM

Liaison RM is an open-source, local-authoritative relationship manager for individuals, families, executive assistants, reception teams, workplace operations, facilities teams, and community organisers.

The product combines a readable Markdown workspace with a native desktop interface, a first-class command-line interface, a relationship graph, event and interaction history, structured personal context, and provider-neutral connections. Canonical records remain on hardware or storage controlled by the user. Search indexes and caches are rebuildable projections.

## Project status

Liaison RM is at repository-foundation stage. The first review stack establishes governance, product architecture, the Rust domain core, and the provider SDK before broader feature implementation begins.

## Non-negotiable product constraints

- Canonical human-scale records use documented Markdown and YAML.
- High-volume machine streams use documented JSONL rather than opaque hosted storage.
- SQLite and search indexes are disposable projections.
- The domain model does not depend on Tauri, React, filesystems, databases, or providers.
- Airgap and Connected-local are separate release profiles.
- No Electric Town account, telemetry service, remote licence check, or mandatory cloud service is required.
- Remote providers are optional adapters behind versioned capability contracts.
- Desktop, CLI, local API, MCP, importers, and plugins call the same application services.

## Planned application surfaces

- Native desktop application using Rust, Tauri, React, and TypeScript
- `liaison` CLI for creation, validation, import, export, backup, recovery, and automation
- Local OpenAPI and MCP services with explicit grants
- Capability-controlled WASI plugins
- Optional browser client served by the local process; the browser does not own canonical data

## Repository map

```text
apps/                 User-facing applications
contexts/             Domain-driven bounded contexts
crates/               Shared technical libraries with narrow responsibilities
adapters/             Filesystem, database, provider, and UI-facing adapters
interfaces/           Versioned external contracts such as WIT and OpenAPI
providers/            Optional provider packages and descriptors
docs/                 Product, architecture, knowledge, evidence, and release records
spec/                 Machine-readable requirements, gates, UAT, and plans
scripts/              Repository policy and validation tools
```

## Contributing

Read [AGENTS.md](AGENTS.md) and [CONTRIBUTING.md](CONTRIBUTING.md) before changing the repository. Every behavioural change must preserve domain boundaries, update or cite the relevant knowledge article, state the user evidence behind the change, and provide an inspectable validation trail.

## Licence

Liaison RM is licensed under the GNU Affero General Public License v3.0. See [LICENSE](LICENSE).
