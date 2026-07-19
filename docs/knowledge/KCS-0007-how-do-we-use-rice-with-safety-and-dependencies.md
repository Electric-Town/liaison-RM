# KCS-0007: How do we use RICE without bypassing safety or dependencies?

- Status: verified
- Audience: maintainers, product contributors, implementation agents
- Last reviewed: 2026-07-18
- Applies to: roadmap, behavioural PRs, release planning

## Problem

RICE can make priorities inspectable, but pre-alpha user counts and effort estimates are uncertain. A raw score can also encourage a visible feature to jump ahead of key recovery, crash safety, privacy, or an accepted dependency.

## Resolution

Use the normalized model in `docs/product/rice-prioritization.md`. Record reach, impact, confidence, effort, and the evidence behind each value. Do not invent production reach.

Apply these checks in order:

1. Confirm the work belongs to the accepted B0-then-A0 product boundary.
2. Confirm its prerequisite tasks and decision records are complete.
3. Confirm no privacy, security, data-integrity, accessibility, or release gate requires it earlier.
4. Compare RICE only with other eligible work.
5. Re-score when scope, evidence, effort, or dependencies change.
6. Record the score and the causal reason in the PR; do not paste a number without its assumptions.

Safety and integrity gates override ranking. P07 key recovery and P08 encrypted recovery remain B0 blockers even if a visible UI task has a larger score. ADR 0012 keeps A0 behind B0 even if a personal feature appears cheaper.

## Verification

- The behavioural PR contains a RICE section or a justified `not applicable` result.
- The score matches the current product table or explains a reviewed update.
- Dependencies in `spec/implementation-plan.yaml` agree with the PR base.
- Feature gates and UAT evidence are named.
- The PR states what new evidence would invalidate the priority.

## Common mistakes

- using social-media interest as measured reach;
- treating confidence as optimism;
- omitting accessibility, migration, KCS, tests, or review evidence from effort;
- ranking a blocked task as if it were eligible;
- using RICE to legitimize a decision already contradicted by an accepted ADR;
- leaving a stale score after adding major scope.
