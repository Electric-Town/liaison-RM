---
id: KCS-0012
title: How do I recover an interrupted canonical operation?
state: Draft
owner: workspace
created: 2026-07-19
reviewed: 2026-07-19
applies_to:
  - liaison-canonical-operation@1
search_terms:
  - COMMIT
  - incomplete operation
  - recovery conflict
  - external edit
  - roll forward
  - operation journal
related_requirements:
  - LRM-WS-004
  - LRM-WS-010
related_uat:
  - UAT-042
related_adrs:
  - ADR-0007
---

# How do I recover an interrupted canonical operation?

## Context

Liaison reports `workspace.recovery-required`, or Health finds an incomplete directory under `.liaison/operations/`.

## Resolution

1. Stop every Liaison process using the workspace.
2. Preserve the entire workspace before changing operation records.
3. Reopen the workspace with the same or a newer compatible Liaison build.
4. Let session open inspect every operation:
   - no `COMMIT`: Liaison discards the staged operation;
   - `COMMIT` but no `COMPLETE`: Liaison rolls the committed targets forward;
   - `COMPLETE`: Liaison verifies evidence and removes leftover staging.
5. Run Health after the workspace opens.

Do not delete `COMMIT`, edit `manifest.yaml`, or copy staged bytes into canonical paths manually.

## Recovery conflict

Liaison stops when a canonical target differs from both:

- the original digest recorded in the precondition; and
- the new digest recorded by the committed operation.

That condition indicates a non-cooperating external edit or file replacement. Liaison will not overwrite it. Preserve both the canonical file and the operation directory, compare them outside the active workspace, and choose a reviewed repair or restore path.

## Verification

A successful recovery leaves:

- every target at the committed digest;
- one progress marker for every target;
- `COMPLETE` evidence;
- no staged payload directory;
- a projection-stale marker so later indexes rebuild from canonical files;
- a valid Health report or bounded findings unrelated to the recovered operation.

## Scope

This procedure recovers Liaison's own cooperative canonical operations. It is not a hostile-process guarantee and does not replace encrypted recovery packages or clean-install restore evidence.
