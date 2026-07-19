# Open workspace format

## Authority

The open workspace is the source of truth for current semantic data. Rust structs, SQLite tables, search documents, UI state, and provider representations are implementations or projections. A format change is therefore a product compatibility change and requires a versioned migration.

## Root manifest

`.liaison/workspace.yaml` contains:

```yaml
format: liaison-workspace
schema_version: 1
workspace_id: 018f2b29-82c7-7ef0-89c8-e1e3229c9453
name: Example workspace
created_at: 2026-07-17T12:00:00Z
profile: personal
build_profile: connected-local
default_locale: en-IE
modules:
  people: true
  organisations: true
  relationships: true
  interactions: true
  events: true
  facilities: false
  sharing: false
```

The build profile is informational evidence about the application that last wrote the manifest. It cannot convert a Connected-local binary into an Airgap binary.

## Common record envelope

Canonical Markdown records use front matter containing at least:

```yaml
---
format: liaison-person
schema_version: 1
id: 018f2b34-5e45-79ee-b81c-ff8781fb46d1
revision: 7
created_at: 2026-07-17T12:10:00Z
updated_at: 2026-07-17T14:40:00Z
created_by: member:018f2b29...
updated_by: member:018f2b29...
classification: private
provenance:
  source: manual
extensions: {}
---
```

Context schemas add typed properties. The Markdown body holds named human-authored sections. Unknown front-matter keys, extension namespaces, and unrecognised body sections survive read-modify-write cycles.

## Pinned People compatibility profile

ADR 0013 pins B0 People authoring to OKF v0.1 Draft at immutable source commit `ee67a5ca27044ebe7c38385f5b6cffc2305a9c1a` and raw specification SHA-256 `b9655e607346dbbdc6de21190e9a953313eda6a7eba68d4d272a65975940ad6e`.

Every Liaison-authored non-reserved People Markdown file has a non-empty `type: person` and mapped OKF title, description, tags, and timestamp fields. The Liaison envelope above remains a versioned, namespaced domain extension and is authoritative for identity, purpose, revision, provenance, information state, classification, disclosure, and operational meaning.

OKF-valid does not mean Liaison-valid. The reader tolerates missing optional fields, unknown types/keys, ordinary broken links, unknown body sections, and malformed siblings. Unsupported or invalid domain facts remain inert or quarantined and cannot affect event readiness. Sealed sensitive values never enter plaintext frontmatter, body text, generated indexes, projections, logs, errors, or evidence to satisfy OKF.

Read-modify-write preserves controlled-field meaning while retaining unknown safe keys and sections, original body bytes outside controlled regions, stable IDs, ordinary links, reserved names, and curated index content. Generated indexes never overwrite curated bodies.

## File naming

Files use a readable slug plus a stable ID suffix when collision is possible:

```text
people/alex-murphy--018f2b34.md
organisations/electric-town--018f2b41.md
events/2026-07-17-dublin-team-lunch--018f2b52.md
```

Links use stable IDs or standard Markdown links resolved through the index. A rename does not change identity.

## Write protocol

1. Read the current file and revision.
2. Validate the command against domain invariants.
3. Render deterministic front matter and preserved extension content.
4. Write a temporary file in the same filesystem.
5. Flush file content and required directory metadata where supported.
6. Compare the expected revision and current hash immediately before replacement.
7. Replace atomically where supported; otherwise use a documented journalled fallback.
8. Append an audit envelope without copying sensitive payload unnecessarily.
9. Update projections after the canonical replacement succeeds.
10. Leave recoverable journal evidence if interruption occurs.

A projection failure does not roll back a successful canonical write. It marks the projection stale and schedules or requests rebuild.

## External edits

The watcher treats external files as untrusted input.

- New or changed files are parsed and schema-validated before indexing.
- Duplicate IDs, revision regressions, invalid dates, broken signatures, and cross-context invariant failures are reported.
- Invalid files remain on disk and appear in `liaison workspace validate`; they are not discarded.
- Concurrent changes create a conflict record with base, local, and incoming revisions.
- People and identity candidates never merge automatically, including exact shared identifiers or fuzzy matches. Contract-defined independent file fields or append-only sections may be combined only after an inspectable preview and explicit confirmation.
- The user can inspect and resolve conflicts without opening SQLite.

## High-volume streams

Raw access and email-metadata events use JSONL rather than one Markdown file per event.

Event envelope:

```json
{
  "format": "liaison-access-event",
  "schema_version": 1,
  "id": "018f2b61-5a9e-76c5-a7a1-d5bc7ac28468",
  "source_id": "badge-system-event-88311",
  "import_job_id": "018f2b60-8d42-7ce5-b504-a9c1dca0d66e",
  "occurred_at": "2026-07-17T08:12:11Z",
  "person_id": null,
  "source_subject": "badge-0042",
  "location_id": "018f2b48-0492-7222-9953-ccb46d84aa97",
  "event_type": "entry",
  "resolution_state": "unresolved"
}
```

Partition manifests record count, byte size, first/last event time, content hash, schema version, and compaction lineage.

## Attachments

Content is stored under `attachments/sha256/<prefix>/<hash>`. Metadata belongs to a context record or attachment manifest. The application verifies the content hash before use. Duplicate content is stored once unless policy requires isolation.

## Projections

Default local projections may include:

```text
.liaison/projections/index.sqlite
.liaison/projections/search/
.liaison/projections/thumbnails/
.liaison/projections/graph-layouts/
```

`liaison workspace rebuild` deletes or replaces projections only after confirming the path is inside `.liaison/projections`. Rebuild reads canonical records and produces a manifest containing source schema version, record counts, stream partitions, errors, and tool version.

## Migration

A migration:

- declares source and target schema versions;
- supports dry-run and produces a change summary;
- creates or verifies a backup before mutation;
- is deterministic and idempotent;
- preserves unknown extensions unless explicitly rejected;
- records each changed file and resulting hash;
- validates the complete workspace before activation;
- provides rollback or states why reversal is impossible.

The required B0 OKF People normalization is stricter: it requires an exact backup, journaled staging, final preconditions, one durable commit decision, injected failure at every write boundary, restart recovery, idempotent rerun, and exact rollback with no partial profile/index state. It is the only B0 migration exception. General and third-party migrations remain outside B0.

The application refuses a workspace whose schema is newer than it supports unless opened in an explicit read-only recovery mode.

## Obsidian and Logseq interoperability

Records use standard Markdown and YAML, not proprietary block encodings. Relationships use ordinary links and stable properties. Liaison-specific extension data is namespaced and documented. Users may open the workspace in Obsidian, Logseq, a text editor, or version-control tooling without Liaison RM.

Concurrent external editing remains subject to revision and conflict rules; interoperability does not imply safe shared-folder multi-writer synchronisation.
