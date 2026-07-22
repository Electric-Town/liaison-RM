# Experience bounded context and Application anti-corruption boundary

Status: P03D candidate; not accepted implementation authority while
`T-B0-P03D` is blocked.

Experience owns how an available Application use case is presented in a local
desktop surface. It does not own Workspace, Person, Event, readiness, recovery,
relationship, or security meaning. Those words cross this boundary only in
versioned Application results produced by their owning contexts.

## Owned language

- **Capability** — a stable Application-published identifier saying that a use
  case is structurally compiled. It is distinct from current availability,
  permission, writer authority, or a temporary blocking state.
- **Route** — an Experience-owned navigation identity mapped exhaustively from
  one or more capabilities. Application and domain crates never publish route
  names.
- **Presentation state** — empty, loading, partial, stale, conflict,
  permission, error, success, undo, and recovery rendering for an applicable
  result.
- **Draft** — uncommitted input retained in the current window until a typed
  Application outcome permits clearing it. A draft is not canonical data.
- **Announcement** — concise accessible status associated with the action and
  focus target that caused it.
- **Disclosure** — the safe public message, recovery action, and correlation
  reference selected from a typed Application error. Private diagnostic data
  is never a presentation input.
- **Transient appearance** — current semantic rendering, preview, and
  operating-system resolution. Persisted preference belongs to the later
  Settings application use case.

“Golden slice” is contributor evidence vocabulary, not a product concept. It
means one request-bearing status/session/mutation/failure scenario that crosses
Application, Tauri, and Experience without substituting fake success data.

## Anti-corruption boundary

1. Domain entities and invariants remain in their owning Rust contexts.
   Experience renders typed results and cannot derive a second readiness,
   maintenance, cohort, relationship, or recovery decision.
2. Application publishes stable use-case and capability identifiers, command
   envelopes, safe public errors, and result DTOs. Experience maps them to
   routes, components, locale keys, focus behavior, and drafts.
3. The complete Tauri command declaration and TypeScript surface must be
   generated or compile-checked from the versioned Application contract before
   P04 can replace the current shell. Handwritten duplicate DTOs are not an
   accepted boundary.
4. Structural capability, current availability, permission, authority, and
   blocking state are separate. An absent capability has no route; a compiled
   but unavailable capability keeps its stable route and draft with an
   accessible reason.
5. A mutation clears its draft only after a matching typed success. Stale,
   superseded, failed, uncertain, and recovery-required results retain the
   draft and identify one safe next action.
6. Experience never reads canonical files, opens repositories, acquires writer
   authority, handles keys, or constructs context services.
7. No browser storage is canonical. No remote request, tracking pixel,
   telemetry listener, remote asset, or provider SDK is allowed in this
   context without a later accepted capability contract.

## Phase route boundary

P04 may map only the accepted foundation capabilities to Overview, Workspace,
People, and Health. Directory, Events, Settings, relationships, interactions,
commitments, Review, and personal-profile customization remain structurally
absent until their owning tasks deliver real contracts. P11 later composes the
five-destination B0 workflow; A0 starts only after B0 acceptance.

## Verification boundary

Architecture checks must fail when:

- a Rust Application or domain module contains an Experience route ID;
- a route lacks an explicit capability mapping;
- a frontend computes a business outcome or accepts private diagnostic data;
- a later-phase destination, placeholder, fake bridge result, browser-backed
  canonical record, or undeclared network request enters the shipped shell; or
- a failed or stale mutation silently clears a draft.

See `DESIGN.md`, ADR 0014, `docs/architecture/context-map.md`, and
`docs/standards/ux-review.md`.
