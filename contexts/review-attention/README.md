# Review and Attention bounded context

## Purpose

Own explainable maintenance reasons, hard suppressions, capacity-bounded review policies, queue construction, and review-session outputs.

## Invariants

- queue items contain factual reasons;
- personal review defaults to reason-only behavior;
- archived, do-not-contact, ended, excluded, active pause, and active snooze states suppress selection before ordering;
- a relationship with no cadence is not marked overdue by this context;
- message volume cannot infer trust, affection, closeness, or importance;
- output has no relationship-strength field;
- capacity bounds are enforced before a session is returned.

## Current slice

The crate implements reason-only policy and queue construction. Cadence calculation, interaction queries, commitments, important-date queries, persistence, monthly review, interruption recovery, and weighted policy simulation remain later work.

## Specification dependency

This implementation depends on acceptance of PR #9, “Topic Packs and reason-based review attention.” It must not be merged before that contract is accepted or reconciled.
