# Relationship runtime evidence

Status: implementation is present on `main`; this document does not open B0, A0, persistence, or user-interface gates.

## Scope

- stable Topic Pack, field, and purpose IDs;
- explicit information-state invariants;
- sealed-storage requirement for sensitive and secret values;
- purpose-specific readiness calculation;
- reason-only policy validation;
- hard suppression before queue ordering;
- capacity-bounded deterministic queue output;
- no relationship-strength output.

## Implemented boundary

The checked-in crates contain the pure profile-readiness and reason-only Review foundations listed above. Accepted ADR 0005 defines their separation from relationship intent and evidence. A successful local or CI test of these crates would prove only that bounded domain behavior, not persistence, event readiness, recovery, or a complete personal product.

## Evidence still required for a release claim

- an exact source commit and successful cross-platform formatting, compilation, Clippy, and test runs;
- architecture, repository, and product-specification checks on that same commit;
- versioned persistence and migration evidence where these types become canonical;
- B0 event-specific readiness, local-policy, sealing, recovery, and installed-app evidence;
- after B0, A0 interaction/commitment inputs and installed personal-workflow evidence.

## Deliberately excluded

- persistence formats;
- Topic Pack activation inheritance;
- encryption implementation;
- cadence and meaningful-interaction queries;
- commitment and important-date adapters;
- interruption recovery;
- weighted Review Priority;
- desktop UI.

The previous pull-request dependency has been reconciled and merged; it is no longer an active gate. Current delivery order is B0 Workplace Review before A0 Personal Memory, so reason-only personal Review must not become a B0 prerequisite.
