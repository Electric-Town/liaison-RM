# Relationship runtime evidence

Status: pending exact-head CI.

## Scope

- stable Topic Pack, field, and purpose IDs;
- explicit information-state invariants;
- sealed-storage requirement for sensitive and secret values;
- purpose-specific readiness calculation;
- reason-only policy validation;
- hard suppression before queue ordering;
- capacity-bounded deterministic queue output;
- no relationship-strength output.

## Evidence required

- formatting, compilation, Clippy, and tests on Ubuntu, macOS, and Windows;
- architecture, repository, and product-specification checks;
- exact successful commit SHA;
- confirmation that PR #9 is accepted or reconciled before merge.

## Deliberately excluded

- persistence formats;
- Topic Pack activation inheritance;
- encryption implementation;
- cadence and meaningful-interaction queries;
- commitment and important-date adapters;
- interruption recovery;
- weighted Review Priority;
- desktop UI.
