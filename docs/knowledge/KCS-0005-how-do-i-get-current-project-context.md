---
id: KCS-0005
title: How do I get current Liaison RM context before starting work?
state: Draft
owner: repository-governance
created: 2026-07-18
reviewed: 2026-07-18
applies_to:
  - contributors
  - coding agents
  - maintainers
search_terms:
  - project context
  - agent handoff
  - read order
  - current status
  - open pull requests
  - source of truth
related_documents:
  - AGENTS.md
  - PROJECT_CONTEXT.md
  - SPEC.md
  - AI_BUILD_INSTRUCTIONS.md
---

# How do I get current Liaison RM context before starting work?

## Problem

A contributor or coding agent can see a large product scope, multiple stacked pull requests, prototypes, implementation branches, and machine-readable planning. Without an explicit read order, it is easy to build against stale assumptions, duplicate work, treat planned behaviour as implemented, or put a rule in the wrong bounded context.

## Resolution

Use this sequence:

1. Confirm the repository, branch, pull-request base, exact head, and current checks.
2. Read `AGENTS.md`.
3. Read `PROJECT_CONTEXT.md`.
4. Read `SPEC.md` and `AI_BUILD_INSTRUCTIONS.md`.
5. Identify the owning bounded context.
6. Read its README, domain code, and tests.
7. Search related decisions, knowledge articles, requirements, UAT cases, feature gates, implementation tasks, issues, and open pull requests.
8. State what is implemented, proposed, blocked, and explicitly out of scope.
9. Select a dependency-complete vertical slice.
10. Record the sources read and unresolved conflicts in the pull request.

## Source hierarchy

When sources disagree, prefer:

1. released compatibility and canonical-format contracts;
2. accepted decisions;
3. bounded-context invariants and tests;
4. security, privacy, and local-integrity invariants;
5. machine-readable requirements, UAT, gates, and task dependencies;
6. product specifications;
7. knowledge articles;
8. prototypes and screenshots;
9. issue and pull-request discussion;
10. uncommitted ideas.

Do not silently choose between conflicting sources. Open a focused clarification or decision.

## Current-state checks

Before using an open pull request as a dependency, check:

- its base and head branches;
- whether it is a draft;
- changed files;
- exact-head workflow results;
- stated limitations and unopened gates;
- whether the work already exists on `main` through another commit;
- whether a newer pull request supersedes it.

A prototype, screenshot, test fixture, draft pull request, passing unit test, and public release represent different levels of evidence.

## Handoff rule

A completed change must leave enough committed context for another contributor to continue without private prompt history. Update `PROJECT_CONTEXT.md` when the change materially alters:

- the product thesis or prohibited behaviour;
- a bounded-context responsibility;
- the canonical storage or compatibility model;
- the implementation order;
- platform, provider, sharing, AI, or plugin architecture;
- a release gate or major status assumption;
- the required agent read order.

Do not copy private conversations, personal data, credentials, hidden reasoning, or prompt history into the repository.

## Verification

A contributor should be able to answer these questions before coding:

- What user problem is being solved?
- Which persona and UAT case apply?
- Which bounded context owns the rule?
- What is already implemented?
- Which open work overlaps?
- What contracts and invariants cannot change silently?
- What tests and release gates are required?
- What privacy, security, accessibility, migration, and rollback effects apply?

If those questions cannot be answered from committed repository material, the first change should improve context or open a focused decision rather than guess.
