# 0014: Own desktop presentation in an Experience bounded context

- Status: proposed
- Date: 2026-07-22
- Deciders: Electric Town maintainer through `T-B0-P03D`
- Contexts: experience, application, workspace, people, events
- Requirements: LRM-UX-016, LRM-UX-017
- UAT: UAT-073, UAT-074
- Feature gates: FG-B0-DESIGN-001, FG-B0-P04-001

## Context and problem

The desktop currently mixes Application command vocabulary, browser route
identity, transient form state, and future domain destinations. That ambiguity
allowed unavailable Event behavior and fake bridge results to appear as product
capability, while an over-broad P04 gate also claimed later B0 and A0 journeys.

The domain contexts must remain presentation-independent. Application needs to
publish stable use-case and capability identities, but a route name, focus
target, draft, announcement, component, or locale key is not domain language.
P04 also needs one accountable owner for those concepts without becoming the
owner of Workspace, Person, Event, readiness, recovery, or relationship rules.

## Constraints and evidence

- ADR 0006 keeps composition, repositories, writer authority, and operation
  state in one Rust Application and `WorkspaceSession`.
- `docs/product/p04-amended-plan.md` assigns route mapping and presentation
  state to Experience while retaining business meaning in the owning context.
- `DESIGN.md` requires generated request/result contracts, capability-honest
  navigation, retained drafts, semantic components, and safe public errors.
- P04 may preserve only the existing Workspace, People, and Health capabilities;
  P11 owns the complete B0 Directory, Events, Health, and Settings workflow.
- A0 remains blocked until installed B0 acceptance.

This proposal is not accepted while `T-B0-P03` is current and
`T-B0-P03D` is blocked. Source or browser work cannot use it to claim P04.

## Alternatives considered

### Let Application own route identifiers

Rejected. It couples the Rust use-case boundary to one presentation structure
and encourages later clients to inherit desktop navigation vocabulary.

### Let each page interpret domain DTOs directly

Rejected. It duplicates capability, error, recovery, and business-state logic
and cannot guarantee exhaustive behavior as contracts evolve.

### Treat Experience as a generic design-system library

Rejected. Tokens and components alone do not own route mapping, drafts, focus,
announcements, disclosure, or recovery presentation.

## Decision

Create an **Experience** bounded context inside the desktop application.

- Application owns stable use-case and capability identifiers, typed commands,
  result envelopes, safe public error descriptors, correlation, and current
  authority or availability state.
- Experience owns exhaustive capability-to-route mapping, route state,
  transient drafts, components, locale keys, focus restoration, announcements,
  and safe disclosure rendering.
- Domain contexts own all business entities, state transitions, policy,
  readiness, relationship, recovery, and persistence meaning.
- Tauri owns native bridge lifecycle and operating-system integration; it does
  not invent a second Application contract.
- Structural capability, availability, permission, writer authority, and a
  temporary block remain separate states.
- Generated or compile-checked bindings cover the complete public Tauri seam.
  Private diagnostics are not part of Experience input.
- A failed, stale, uncertain, or recovery-required mutation retains its draft.
  Only a matching typed success may clear it.
- Browser storage is never canonical and Experience performs no undeclared
  network request.

P04 maps only Overview, Workspace, People, and Health. P11 later introduces the
complete B0 destinations from accepted downstream capabilities. A0 routes do
not exist before B0 acceptance.

## Consequences

- Architecture tests can reject route IDs in Application/domain code and
  reject unmapped capabilities in Experience.
- Browser and native clients can share one scenario oracle without sharing
  presentation implementation.
- Later contexts add routes with their own vertical slice instead of P04
  freezing speculative interfaces.
- Experience is an anti-corruption boundary, not a place for replicated domain
  calculations or generic metadata.
- The current `apps/desktop/src/experience/README.md` remains candidate context
  documentation until this decision and P03D are accepted.

## Migration, rollback, or reversal conditions

P04 may introduce the generated Experience client only after P03 and P03D are
accepted at exact heads. The current shell remains the rollback surface until
legacy/replacement parity and installed evidence pass. Reverting a presentation
implementation does not migrate or rewrite canonical data.

If another local client needs different navigation, it reuses Application
capability IDs and creates its own route mapping; it does not move route
identity into Application.

## Related knowledge, tests, and evidence

- `DESIGN.md`
- `docs/product/p04-amended-plan.md`
- `docs/architecture/context-map.md`
- `apps/desktop/src/experience/README.md`
- `docs/standards/ux-review.md`
- `scripts/check_desktop_shell.py`
- `scripts/check_spec.py`
