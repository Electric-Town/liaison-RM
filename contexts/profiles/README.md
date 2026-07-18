# Identity and Profiles bounded context

## Purpose

Own Topic Packs, stable Field Definitions, explicit information states, profile values, Purpose Definitions, and purpose-specific readiness.

## Invariants

- labels and layouts never define field identity;
- an empty value is not treated as “none”;
- known, verified, unverified, unknown, not applicable, declined, stale, conflicting, needs clarification, and derived remain distinct states;
- sensitive and secret fields require sealed storage;
- readiness is calculated for one named purpose and version;
- no universal profile-completeness percentage is produced;
- a missing sensitive value can be reported without disclosing its content.

## Current slice

The crate implements the domain types and readiness calculation. It does not yet implement Markdown persistence, Topic Pack activation inheritance, encryption, profile layouts, imports, or UI.

## Specification dependency

This implementation depends on acceptance of PR #9, “Topic Packs and reason-based review attention.” It must not be merged before that contract is accepted or reconciled.
