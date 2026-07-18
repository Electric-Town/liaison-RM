# Threat model

Status: Initial repository threat model. Each implementation PR updates the relevant assets, boundaries, threats, controls, and evidence.

## Assets

- canonical person, organisation, relationship, interaction, event, reminder, and facilities records;
- dietary and other restricted personal information;
- access and email-metadata streams;
- attachments;
- private overlays;
- workspace, member, device, signing, encryption, and recovery keys;
- provider credentials and tokens;
- connection grants and audit history;
- backups and manifests;
- application binaries, plugins, provider packages, and update metadata;
- local API, MCP, and webhook credentials;
- user trust in the accuracy and disclosure scope of the system.

## Actors

- workspace owner;
- authorised member with limited role;
- administrator;
- external self-service respondent;
- local process or malware running as the user;
- stolen or retired device;
- malicious or compromised provider;
- malicious plugin or provider package;
- remote API, AI, webhook, CardDAV, CalDAV, or email service;
- contributor or dependency supply-chain attacker;
- accidental operator making an incorrect import, export, mapping, or disclosure.

## Trust boundaries

1. Canonical workspace files and application process
2. Disposable projections
3. Platform secret store
4. Desktop webview and Rust command boundary
5. CLI/stdin/stdout and shell environment
6. Local API/MCP listener
7. Plugin WASI component boundary
8. Provider host and egress controller
9. Remote transport/provider
10. Shared operation materialisation
11. Import/export files and removable media
12. Build, dependency, CI, signing, and release pipeline

## Threats and required controls

### Canonical data overwrite or corruption

Threats: stale writer, external editor, partial write, invalid migration, filename collision, malicious archive path, projection treated as source.

Controls: stable IDs, revisions, hashes, atomic/journalled writes, path normalisation, schema validation, dry-run migration, pre-migration backup, complete workspace validation, isolated restore, rebuildable projections, recovery evidence.

### Undeclared disclosure

Threats: hidden telemetry, provider SDK analytics, redirect to undeclared host, broad OAuth scope, log body capture, AI prompt leakage, export containing private overlay.

Controls: separate Airgap build, egress controller, descriptor destinations, grant evaluation, redirect policy, minimal scopes, local/redacted logs, field-level AI and export scopes, overlay exclusion tests, dependency review.

### Privilege escalation by member

Threats: role confusion, forged operation, replay, stale grant, direct file edit, API token overreach, event manager reading diagnostic dietary detail.

Controls: scoped grants, signed operations, replay protection, revision and actor validation, application-service enforcement after file import, hashed/scoped/expiring API tokens, least-disclosure read models, audit.

### Stolen device or key

Threats: offline workspace copy, provider token reuse, signing operations after removal, recovery package theft.

Controls: optional at-rest encryption, platform secret store, short-lived provider tokens where possible, device revocation operation, key rotation, encrypted recovery package, local session lock, explicit limitations for previously exported plaintext.

### Malicious provider or transport

Threats: omission, rollback, reordering, duplicate objects, corrupt bytes, weak consistency, conflict copies, independent lifecycle deletion.

Controls: content hashes, signed operation chains, manifest revisions, idempotency keys, local ordering and conflict detection, provider conformance evidence, backup verification, lifecycle-policy warning, transport replacement, no blind latest-wins.

### Plugin escape

Threats: unrestricted filesystem/network, secret discovery, raw database access, capability confusion, vulnerable host import.

Controls: WASI component model, WIT contracts, denied-by-default manifest, scoped host functions, no raw database handle, destination grants, resource limits, deterministic test host, signature/trust policy, revocation, audit.

### Import poisoning

Threats: path traversal, formula injection, oversized file, parser exhaustion, duplicate identity, malicious HTML/Markdown, incorrect column mapping, source ID collision.

Controls: streaming parsers, limits, safe rendering, escaped spreadsheet export, mapping preview, idempotent source IDs, duplicate and identity-resolution workflow, quarantined invalid rows, synthetic preview, rollback evidence.

### Supply-chain compromise

Threats: malicious crate/npm package/action, typosquatting, compromised release, mutable tag, unreviewed generated code.

Controls: pinned toolchains, lock files, dependency licence/security review, minimal dependencies, exact action commit pinning before stable release, reproducible generation, provenance, signed artifacts/checksums, independent release review, no secrets in untrusted PR workflows.

### Misleading product claims

Threats: calling upload “sync”, claiming Airgap while network code remains, claiming accessibility/compliance without evidence, counts presented as complete history.

Controls: feature gates with evidence, distinct build artifacts, conformance language, source/confidence labels, release checklist, changelog and knowledge review.

## Abuse cases requiring explicit tests

- a provider redirects an approved request to an unapproved host;
- a plugin requests a capability after installation that was not in its approved manifest;
- an event manager exports a catering brief and receives diagnostic details;
- an access import maps one badge to two people;
- two devices update the same dietary coverage state from the same base revision;
- a provider returns an older manifest as current;
- a backup uploads successfully but one remote chunk is truncated;
- a Markdown file introduces `../../` attachment or record paths;
- an AI proposal attempts to modify records outside its grant;
- a revoked member replays a previously signed operation;
- a browser client continues operating after the local token expires;
- an Airgap build attempts DNS, loopback, or remote socket access;
- a support bundle includes a secret, private overlay, or real message body;
- an exported CSV begins a cell with a spreadsheet formula prefix.

## Residual risks

- Malware with the user’s operating-system privileges may read unlocked local data.
- A recipient can retain plaintext legitimately disclosed to them.
- External Markdown editors can create conflicts or invalid records.
- Remote providers can deny service or delete objects outside Liaison control.
- Strong privacy controls can reduce collaborative convenience and require recovery discipline.
- Metadata such as object timing and size may remain visible to a transport even when payloads are encrypted.

The product must state these limits rather than imply absolute security.
