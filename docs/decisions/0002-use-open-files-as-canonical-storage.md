# 0002: Use open files as canonical storage

- Status: proposed
- Date: 2026-07-17
- Contexts: workspace, all record-owning contexts

## Context and problem

Users require local control, Markdown interoperability, independent backup, long-term readability, and recovery without a hosted service. A relational database as the only source of truth would make these properties dependent on Liaison RM.

## Alternatives considered

1. SQLite canonical storage with Markdown export.
2. Event database with generated Markdown views.
3. One Markdown file for every record and every machine event.
4. Markdown/YAML for human-scale records plus JSONL for high-volume streams.

Database-first designs make exports secondary and create ambiguity after external edits. One Markdown file per email or access event creates filesystem and indexing problems at scale.

## Decision

Use versioned Markdown/YAML as the canonical representation for human-scale records and documented partitioned JSONL for high-volume append-oriented streams. Use content-addressed attachment storage. SQLite and search indexes are rebuildable projections.

## Consequences

- Canonical schemas and migrations are public compatibility contracts.
- Writes need revision checks, journalling, unknown-field preservation, and external-edit validation.
- Projection rebuild is a release gate.
- Obsidian, Logseq, text editors, backup tools, and version control can inspect the workspace.
- Shared-folder multi-writer editing is not automatically safe and is addressed by the Sharing context.

## Reversal conditions

A future representation may be added only if it preserves independent inspection, documented migration, complete export, and rebuildability. SQLite cannot become the undisclosed source of truth.
