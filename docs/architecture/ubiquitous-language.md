# Ubiquitous language

These terms are repository-wide unless a bounded context defines a narrower meaning.

| Term | Meaning |
|---|---|
| Workspace | The local-authoritative collection of canonical records, configuration, operations, attachments, audit, and member/device metadata. |
| Canonical record | The durable, documented representation from which disposable projections can be rebuilt. |
| Projection | Rebuildable derived state such as SQLite rows, search indexes, counts, thumbnails, or graph layouts. |
| Person | A human represented by a stable identifier and profile; not interchangeable with a user account. |
| Member | A person or device identity authorised to act in a workspace. |
| Profile | Current person information and field provenance owned by the People context. |
| Characteristic | A structured or freeform fact worth remembering, with classification, provenance, and visibility. |
| Dietary requirement | A typed requirement or preference with coverage state, operational instruction, verification, and disclosure policy. |
| Coverage state | Whether dietary information is provided, verified none, pending, stale, declined, unreachable, excluded, or unknown. |
| Relationship | A typed, directional or mutual connection with its own status, priority, cadence, and history. |
| Circle | A user-managed relationship grouping; it is not an organisation or security role. |
| Interaction | A recorded communication, meeting, note-worthy encounter, or imported relationship event. |
| Activity | A planned or completed occurrence that may involve multiple people; an event is an activity with event-management semantics. |
| Attendance | A person’s recorded participation state for an event, including source and revision. |
| Cohort | A reproducible selection of people using direct selection, import, or saved filters. |
| Catering brief | A least-disclosure operational export derived from a named event cohort and specific profile revisions. |
| Connection | A configured instance of a provider descriptor; it has no authority until granted. |
| Provider | An adapter package implementing one or more versioned capability contracts. |
| Capability contract | Provider-neutral operations and semantics such as object storage, backup, contacts, calendar, or email metadata. |
| Grant | An explicit authorisation binding purpose, provider, endpoint, operations, data scope, schedule, retention, expiry, and approver. |
| Job | One execution of a granted connection or import, with idempotency, result, and evidence. |
| Backup | An encrypted, verifiable recovery snapshot; it does not imply multi-writer synchronisation. |
| Synchronisation | Reconciliation of authorised operations or records across members/devices under defined conflict semantics. |
| Operation | An immutable, signed statement of a domain change transported by Sharing. |
| Private overlay | Member-scoped content associated with a shared record but excluded from unauthorised materialisation, search, export, and AI context. |
| Liaison Card | A signed, selective, portable statement of information a person chooses to share. |
| Self-service request | A purpose-bound request for selected information that can be answered without an account. |
| Airgap | A separately built profile with network clients and listeners absent from the executable. |
| Connected-local | A local-authoritative profile that can use explicitly granted network capabilities. |
| Plugin | A capability-controlled WASI component extending behaviour through versioned WIT contracts. |
| Proposal | A staged AI or automation write set awaiting validation and, by default, user approval. |
| Archive | Reversible removal from active workflows without semantic deletion. |
| Delete | Removal according to retention, audit, backup, sharing, and remote-provider policy. |

## Rejected ambiguous terms

- **Cloud sync** — name the provider, transport, authority, and conflict model.
- **No allergies** — use `verified none` with source and verification date.
- **Contact** — use person, contact point, connection, or imported address-book entry as appropriate.
- **User** — use workspace member, operator, person, administrator, or external respondent.
- **Metadata** — name the fields and owner; generic metadata does not bypass modelling.
- **AI integration** — name the client, provider, tool protocol, data scope, operation, and grant.
- **Secure** — state the threat, control, evidence, and residual risk.
- **Syncable** — state whether the adapter is safe for backup, single-writer publication, or multi-writer reconciliation.
