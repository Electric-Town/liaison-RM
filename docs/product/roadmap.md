# Product delivery roadmap

The roadmap is ordered by dependency and user outcome. Dates remain absent until the team has measured delivery throughput. [`working-state-delivery.md`](working-state-delivery.md) is the current status and claim boundary; accepted ADR 0012 fixes the product order as B0 Workplace Review before A0 Personal Memory.

The broad R1–R6 catalog remains useful as long-term scope, but it is not the working merge order. In particular, personal relationship features, broad packaging, providers, sharing, and automation cannot become prerequisites for the first complete event workflow.

## Current boundary

The current P02 source state is pre-alpha. It has governance and product contracts, one Rust application composition root shared by CLI and Tauri, path-bound write-authoritative Workspace Sessions with operating-system writer exclusion and retained capability roots, lock-free read-only Health, structured results/errors, a narrow Workspace/People/Markdown slice, profile/readiness and reason-only Review foundations, provider contracts, and review packaging workflows. It does not yet reject an independently copied workspace with the same workspace identity at another path, so `LRM-WS-009` and `T-B0-P02` remain blocked. It also lacks recoverable multi-target writes, final mutation preconditions, sealed dietary persistence, local-purpose authorization, scalable Directory reads, encrypted clean-install recovery, and the cohort-to-brief workflow. B0 assumes one trusted local workspace owner and must structurally omit relationship allocation, ranking, and scoring.

The current installed Mac app is an internal local-authoritative review build. It is not a proven Airgap artifact, notarized public release, or supported daily-use product.

## P00 — Reconcile contracts and truth

Outcome: code, decisions, formats, planning catalogs, commands, documentation, versions, evidence, dependency policy, and active-branch disposition describe one product and one delivery order.

Exit evidence:

- ADRs 0001–0012 are accepted and explicitly distinguish decision from implementation status;
- requirements, UAT, feature gates, and implementation tasks encode P00–P11, B0, and A0 without personal-first dependencies;
- current command and implemented-feature inventories are separate from target surfaces;
- package/app versions and provider/release evidence do not contradict each other;
- stale pull-request and nonexistent requirement/UAT references are removed or mapped;
- repository, link, content, and specification checks pass.

## P01 — One application composition root

Outcome: CLI and Tauri call one `liaison-application` command/query layer with typed identifiers, DTOs, results, errors, clocks, and job/session correlation.

Exit evidence:

- invalid workspace validation returns a stable non-zero exit and versioned JSON/human result;
- malformed sibling records do not make Health unreachable;
- Tauri no longer reduces typed failures to strings;
- serialization failure, initial revision, and current false build-profile claims are corrected;
- CLI/Tauri parity snapshots pass.

## P02 — Workspace authority

Outcome: an opened `WorkspaceSession` owns canonical root, workspace identity/schema, one advisory writer lock, recovery state, key state, repositories, and projection status.

Exit evidence:

- a second writer receives a typed lock result;
- process death releases authority without unsafe PID-based lock stealing;
- read-only Health/recovery works for locked, malformed, or newer-schema workspaces where safe;
- checkpoint/import/mutation code cannot bypass the session authority.

P02 owns the readable manifest contract and session authority only. Installed no-account/no-egress workspace creation, projection deletion/rebuild, and the complete canonical round trip remain acceptance evidence rather than P02 implementation claims.

Local source-worktree evidence covers the session aggregate, capability-root
binding, same-path typed contention, quiescence, lock-free Health,
process-exit release, and current inbound-adapter mutations. A copied workspace
at another path can still obtain an independent path-local lock, so the
workspace-identity authority requirement remains open. Windows runtime and
exact-head remote matrices also remain open; P03 still owns recoverable writes
and final preconditions.

## P03 — Recoverable canonical operations

Outcome: every mutation uses a Workspace-owned multi-target operation with staged outputs, digest/revision preconditions, a durable commit decision, progress, roll-forward recovery, and projection-stale handling.

Exit evidence:

- existing Workspace and Person mutations use the new protocol;
- fault injection covers every pre-commit and post-commit boundary;
- non-cooperating external edits stop recovery instead of being overwritten;
- minimal activity evidence is recorded once without sensitive payloads;
- Linux, macOS, and Windows durability/race tests pass.

## P04 — Typed desktop inbound adapter

P04 cannot begin directly after P03. Once P03 has stabilised typed commands, state, errors, and recoverable-operation contracts, design consultation creates canonical `DESIGN.md`. Plan design review then checks the proposed P04 direction against the complete B0 journey, localisation, recovery states, the semantic theme contract, and the accessibility matrix. G0 records this future gate; it does not create `DESIGN.md` or preselect a visual direction.

Outcome: React/TypeScript/Vite replaces the disposable vanilla shell as a Tauri inbound adapter while Rust remains the only domain, storage, authorization, readiness, and recovery authority.

Entry gate: P03 is stable, gstack design consultation has produced an accepted `DESIGN.md`, and plan design review has passed. The React implementation does not choose its visual language ad hoc.

Exit evidence:

- Workspace, People, and Health parity passes before event screens land;
- generated DTO/result contracts are current;
- native folder selection replaces raw-path primary UX;
- one semantic token/component contract provides accessible system, light, dark, and high-contrast built-ins, preserves operating-system reduced-motion preferences, and handles long content;
- density, text-scale, user palette controls, and theme packages are not smuggled into B0/P04;
- the web fixture uses a deterministic fake bridge and makes no product-authority claim.

## P05 — G1 domain contracts and atomic OKF authoring seam

Outcome: the G1 `T-B0-P05` defines revisioned People, Organisations, Groups, Locations, Memberships, Events, provenance, explicit field states, and event-local resolutions before P06/P07; G3 consumes those accepted contracts. The separate G1 `T-B0-P05-OKF`, after P03 and P04, owns the pinned OKF v0.1 Draft People schema, strict writer port, Liaison extension mapping, reserved paths, and sealed-plaintext denial under `FG-B0-001`. Sensitive types and policy are owned only by P07/P08 under `FG-B0-002`.

Exit evidence:

- every non-reserved People writer emits `type: person` and the supported OKF core mapping through one typed port;
- Liaison's domain extension remains schema authority and plaintext plus `sealed: true` is unrepresentable for restricted persisted values;
- B stores no diagnosis, medical history, treatment detail, or diagnostic narrative;
- every dietary value has source, state, recorded/verified time, purpose, classification, revision, and disclosure policy;
- schemas, round trips, unknown-field handling, and migrations are explicit.
- `UAT-065` proves all released B0 UI/CLI writers, Markdown round trip, unknown-content preservation, and sealed-data non-disclosure.

## P06 — Scalable Directory reads

Outcome: tolerant canonical scanning and a disposable SQLite/FTS projection provide pagination, filtering, membership queries, and Health findings without making the index authoritative.

Exit evidence:

- one malformed record does not hide healthy records;
- cohort finalisation revalidates canonical hashes/revisions;
- sensitive payloads and person-to-dietary associations do not enter plaintext SQLite;
- deletion/rebuild reconciliation passes;
- deterministic 10,000-person/50,000-membership budgets pass.
- unknown OKF types, keys, links, sections and malformed siblings remain tolerable, while OKF-valid but Liaison-invalid facts are quarantined and cannot affect readiness.

## P06-REPAIR — Guided canonical repair

Outcome: after P03 provides recoverable operations and P06 can identify invalid canonical records without hiding healthy Directory entries, an administrator can preview and repair a duplicate Person ID and invalid birthday without silent deletion or partial mutation.

Exit evidence:

- validation identifies every affected source and proposed change before mutation;
- an exact pre-repair backup and bounded operation receipt exist before canonical targets publish;
- injected failures leave the pre-repair state or a recoverable committed operation;
- exact rollback restores the original bytes;
- `UAT-040`, the fault matrix, and the recovery knowledge article close `FG-R1-002`.

## P07 — Workspace Security and honest local policy

Outcome: B has authenticated sealed envelopes, workspace DEK lifecycle, passphrase recovery envelope, optional Keychain cache, a trusted local owner/device principal, purpose grants, role presets, and payload-minimal activity evidence.

Exit evidence:

- key unavailable, wrong passphrase, tamper, rotation, expiry, revocation, wrong purpose, and wrong scope fail closed;
- policy is enforced before decryption, projection, readiness, generation, or delivery;
- no secret or restricted value appears in logs, errors, projections, audit, fixtures, screenshots, or diagnostic bundles;
- product copy states that one unlocked OS account is not multi-user confidentiality.

## P08 — Recovery before real sensitive data

Outcome: the product distinguishes a quiescent local checkpoint from an encrypted user-portable recovery package and proves the latter on a clean installation.

Exit evidence:

- a checkpoint has deterministic file/directory membership, hashes, no overwrite, and explicit activatable/non-activatable state;
- a recovery package contains canonical data, integrity manifests, minimal audit, and the passphrase-wrapped recovery envelope;
- tamper, omission, wrong passphrase, schema, identity, and target-path failures leave the current workspace untouched;
- isolated restore succeeds on a clean Mac without the original Keychain entry.

## P09 — Required OKF normalization, then Directory onboarding

Outcome: `T-B0-P09-OKF` first normalizes legacy Liaison People files through exact preview/backup, journaled failure-atomic commit, restart recovery, idempotent rerun, curated-index preservation, and exact rollback after P03/P05-OKF/P06 and the guided repair gate, then closes `FG-B0-001`. The separate G3 P09 lets a workplace operator maintain People and import Organisations, Groups, Locations, and effective Memberships through a streaming staged workflow.

Exit evidence:

- mapping, row validation, source-ID mapping, duplicate/ambiguous reconciliation, cancel/resume, and idempotent re-import pass;
- invalid rows remain inspectable and formula-prefixed content is safe;
- large imports use bounded memory and accessible pagination;
- confirmed batches commit through the recoverable operation engine.
- `UAT-066` proves no partial profile/index state and byte-exact rollback across every write boundary.

The required OKF People normalization is the only B0 migration exception. General and third-party migrations remain excluded from B0 and stay in R5/G5.

## P10 — Events and dietary readiness

Outcome: an event operator can finalize an immutable cohort, reconcile every attendee, resolve event-local gaps, generate sealed internal brief evidence, and deliver verified CSV or print-safe HTML.

Exit evidence:

- every cohort identity appears in exactly one outcome and totals reconcile;
- unknown never becomes verified none;
- `DietaryOperationalView` structurally excludes diagnostic detail;
- grants are checked before decryption and again before generation/delivery;
- source changes make prior evidence historical/stale and regeneration creates a new artifact;
- delivery failure never reports success or invalidates the internal brief.

## P11 — B0 desktop workflow

Outcome: the installed application provides Overview, Directory, Events, Health, and Settings, with Event Details subviews for Cohort, Attendees, Readiness, and Brief. B0 Settings contains built-in theme choice and persistence only and does not export or import settings bundles.

Exit evidence:

- keyboard, VoiceOver, 400% zoom/reflow, reduced motion, contrast, narrow-window, pseudolocale, and long-content tests pass;
- interruption-safe drafts and explicit empty/loading/partial/stale/conflict/permission/error/success/undo/recovery states pass;
- disclosure preview and artifact evidence show scope, grant, revisions, path, checksum, and staleness;
- system, light, dark, and high-contrast built-in choice persists across relaunch without creating a settings-transfer surface;
- relaunch, offline, readable-file, and native installed-app flows pass.

## B0 — Workplace Review Alpha

B0 is independently complete when the installed universal Mac review artifact creates, closes, reopens, and projection-rebuilds a local workspace without an account while network access is denied, then passes Directory import through verified cohort-to-brief delivery, guided canonical repair, crash/key/grant/leak matrices, clean-install encrypted restore, scale budgets, compiled design review, native QA, readable-file proof, and the deterministic contributor hello world. Built-in theme persistence is included; settings bundle transfer begins in A0.

B0 is an internal review alpha. Missing Developer ID signing, notarisation, stapling, Gatekeeper, or clean-Mac distribution evidence must remain visible and blocks a supported public download.

## A0 — Personal Memory Alpha

A0 starts only after B0 acceptance. It adds quick/full capture, a source-complete purpose-scoped profile, explicit fact states, stable custom-field layouts, user-organised profile tabs with stable IDs/order/visibility, lossless settings export/import, reversible identity review, a source- and range-labelled unified timeline, keyboard reordering, meaningful interactions, bounded commitments, distinct last-note and last-interaction views, and reason-only Review over the same session, security, recovery, Directory, OKF, and UI foundations. Final A0 acceptance reuses B0-owned `UAT-001` with A0-owned `UAT-002` to close the complete canonical workspace round-trip gate. It does not add a generic task engine, global person score, or automatic exact/fuzzy merge. The complete B0 matrix, including `UAT-065`, remains a regression gate.

## After A0

Later independently gated outcomes include:

- multi-member Sharing, private overlays, encrypted operations, Liaison Cards, and conflict semantics;
- local/removable-media/WebDAV/S3-compatible provider transport and provider-author tooling;
- CardDAV, CalDAV/iCalendar, email metadata, migrations, and facilities imports;
- local OpenAPI, MCP, webhooks, n8n, Ollama, remote AI grants, and WASI plugins;
- visible provider/import operation receipts and history with no hidden sync, hidden refresh, or unreported egress;
- source-backed enrichment proposals that cannot directly write confirmed facts or mutate assessment, freshness, or cadence;
- optional post-A0 spatial discovery only with explicit egress, coarse/local controls, coordinate deletion, structural Workplace denial, and semantic list/table parity;
- native mobile products, phone synchronisation, and Meitheal integration;
- Linux/Flatpak and supported Windows distribution;
- Developer ID signing, notarisation, upgrades, uninstall/reinstall, provenance, and public release operations.

## Product discovery cadence

Before each acceptance gate:

1. Observe at least one complete target workflow without guiding the participant.
2. Record the current workaround, interruptions, errors, and recovery behaviour.
3. Compare the observed problem with the planned outcome.
4. Remove or defer work that does not change that outcome.
5. Update requirements, UAT, knowledge, feature gates, and evidence.

Interest, waitlists, and feature requests are inputs. Repeated behaviour and failure consequences determine priority.
