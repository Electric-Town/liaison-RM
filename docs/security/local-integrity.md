# Local integrity and privacy architecture

## Security objective

Liaison RM protects the user’s ability to control, inspect, move, recover, and selectively disclose relationship data. “Local” is not treated as a marketing label. It is enforced through build profiles, canonical-format rules, capability grants, egress controls, secret separation, and release evidence.

## Build profiles

### Airgap

The Airgap build:

- excludes HTTP, WebSocket, SMTP, CardDAV, CalDAV, WebDAV, object-store, update-check, webhook, API-listener, MCP-network-listener, and remote-AI implementations from the build graph;
- does not request Flatpak network permission;
- does not bind a network socket during automated tests;
- supports local files, removable-media packages, validation, migration, backup, restore, CLI, and desktop UI;
- includes no dormant provider credentials or endpoints.

A binary inspection and runtime network-denial test form release evidence.

### Connected-local

The Connected-local build includes optional networking but starts with no active connection, no account, no telemetry, no remote logging, and no default remote endpoint. Loopback services are disabled until the user enables them and creates credentials.

## Data classifications

Initial classifications:

- `public` — intentionally publishable.
- `shared` — normal workspace member information.
- `private` — restricted to selected members.
- `restricted` — dietary, access, private overlay, sensitive communication, or comparable information requiring purpose and field scope.
- `secret` — credentials, private keys, recovery material; never canonical profile content.

A context may add a narrower classification policy but cannot weaken the handling of `restricted` or `secret` data.

## Secrets

Secrets are stored through a `secret-store@1` port backed by platform keychains or an encrypted local secret store. Canonical files contain only opaque references.

Prohibited secret locations:

- Markdown/YAML/JSONL canonical records;
- Git history;
- logs, crash reports, screenshots, fixtures, examples, or support bundles;
- provider descriptors;
- CLI history or process arguments when a safer prompt/file-descriptor route is available.

Secret retrieval is scoped to a connection operation and audited without recording the value.

## Egress control

Every outbound request in Connected-local passes an egress decision containing:

- build profile;
- provider and connection;
- active grant revision;
- purpose;
- operation;
- data classifications and approximate size;
- destination host, port, and redirect chain;
- actor and job;
- schedule and expiry.

The controller rejects undeclared destinations, expired grants, scope escalation, unapproved redirects, and provider operations outside the descriptor. DNS rebinding, proxy environment variables, and local-network destinations receive explicit policy treatment.

## Canonical-write integrity

- Revision preconditions prevent blind overwrite.
- Content hashes detect unexpected external change.
- Temporary-file and journal protocols make interrupted writes recoverable.
- Unknown fields are preserved.
- Invalid external edits remain visible.
- Projection failures cannot erase canonical writes.
- Migrations create and verify backup evidence first.

## Encryption

The design separates:

- workspace-at-rest encryption when selected;
- attachment encryption;
- sharing-operation payload encryption;
- provider-backup encryption;
- secret-store encryption;
- transport TLS.

Transport TLS does not replace application-level backup or sharing encryption. Algorithms and formats require a decision record, versioning, test vectors, key-rotation behaviour, and recovery procedure. The project does not invent cryptographic primitives.

## Audit

Audit entries record actor, device, command, aggregate, purpose, grant, result, timestamp, and correlation ID. They avoid duplicating sensitive payloads. Access to audit is itself authorised and logged.

Audit storage is partitioned JSONL with integrity manifests. Local owners can export and verify it. Retention is configurable by classification and legal/organisational need.

## Logging and diagnostics

- Local by default.
- Structured and bounded.
- Sensitive values redacted at source, not after upload.
- No silent crash or analytics upload.
- Diagnostic export provides a field and file preview.
- Support bundles use synthetic replacement where possible and require explicit confirmation.
- Provider HTTP bodies are not logged by default.

## Import and export

Imports are untrusted. They use size limits, streaming parsers, schema validation, mapping preview, duplicate detection, idempotency, and rollback or compensating evidence.

Exports show records, fields, classifications, destination, format, and historical scope. Restricted exports require a purpose. Least-disclosure exports omit unrelated source detail.

## Backup and restore

A backup is created and encrypted locally. A manifest covers files, streams, attachments, schema, hashes, and encryption metadata. Upload occurs only after local validation.

Restore:

1. downloads to an isolated directory;
2. verifies provider object metadata, local hashes, signatures, and manifest completeness;
3. checks supported schema and available migrations;
4. validates canonical records;
5. produces a replacement preview;
6. requires confirmation;
7. preserves the current workspace until the replacement succeeds;
8. rebuilds projections from restored canonical data.

A successful upload without a successful isolated restore test is not accepted as backup evidence.

## AI and automation

AI output is untrusted input. Read context is field- and purpose-scoped. Remote model disclosure requires a provider grant. Write actions are proposals by default and pass normal validation, revision, audit, and approval.

Automation tokens are scoped, expiring, revocable, and stored as hashes or secret references. Webhooks are signed and replay-bounded. MCP and local APIs bind to loopback by default.

## Privacy boundaries

The core product does not:

- infer medical conditions from dietary instructions;
- score employee productivity, attendance compliance, or risk from access events;
- send contact graphs to Electric Town;
- train hosted models on workspace data;
- discover or enrich people through third-party data brokers by default;
- treat consent to one event or provider as consent to unrelated reuse.

Any future change to these boundaries requires a protected-principle decision under `GOVERNANCE.md`.
