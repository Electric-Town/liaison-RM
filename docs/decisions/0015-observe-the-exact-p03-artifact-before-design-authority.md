# 0015: Observe the exact P03 artifact before design authority

- Status: accepted
- Date: 2026-07-22
- Deciders: Electric Town maintainer
- Contexts: governance, product, experience, workspace
- Requirement: LRM-PK-010
- Tasks: T-B0-P03, T-B0-P03-OBS, T-B0-P03D
- Feature gate: FG-B0-DESIGN-001

## Context and problem

P03 stabilises the local-authoritative mutation and recovery substrate before the product commits to the P04 interface. A merge, plan, test report, or design atlas can prove useful engineering facts without proving that a representative workplace operator understands and can resume the exact executable workflow.

PR #65 established baseline commit `3499a6e9278fc72d2498a9978df59f30d03722e6`. All seven ordinary `push` workflows for that exact merge result completed successfully in runs `29899084738`, `29899084740`, `29899084741`, `29899084751`, `29899084753`, `29899084769`, and `29899084789`, including the Windows desktop alpha. Separately dispatched notarized-bundle run `29899498005` failed its missing-Apple-credentials preflight; it is not ordinary-push CI or release evidence. Neither the merge nor either CI fact accepts P03. The maintainer selected D1-B so the missing behavioural evidence is resolved before design authority advances.

## Decision

P03 technical acceptance is a separate prerequisite. It requires pairwise-distinct qualified-code, merge-result, and attestation Git SHAs plus distinct executable-artifact and qualification-receipt SHA-256 identities. Accepting that tuple completes P03 and makes `T-B0-P03-OBS` (plan alias D9) current, but it does not close broader `FG-B0-001`, decide product direction, or make P03D eligible. OBS then observes that exact same artifact with synthetic or redacted workplace scenarios.

The observation records:

- exact P03 attestation and artifact identity;
- trigger and deadline, actors and handoffs, current tools, and missing or wrong information;
- interruption, correction, disclosure, and failure consequences;
- whether one trusted local owner can choose or open the workspace, distinguish no-change from committed recovery, avoid blind retry, understand partial versus empty state, resume after interruption, and explain local authority;
- blockers and evidence rather than compliments, hypothetical feature voting, or inferred preference.

Real workplace personal data is not required or accepted. This checkpoint does not open the separately governed real-data pilot.

The observation receipt contains independently hashed record and decision evidence plus the complete observed qualification tuple. Artifact, qualification receipt, observation record, and observation decision SHA-256 identities are pairwise distinct. The observed tuple must equal the technically accepted P03 tuple, and the receipt decision must equal the structured observation decision. Structural validation proves only schema, format, and domain separation; exact-head qualification must separately prove Git ancestry, artifact existence, and receipt validity. The observation ends with one separately recorded outcome:

- **Continue:** mark the observation task complete and make P03D eligible to become current.
- **Change:** mark the observation task complete, keep P03D and P04 blocked, record exactly one known replacement task and make it current, then repeat qualification and observation where the change affects the artifact.
- **Stop:** mark the observation task complete, keep P03D and P04 blocked, and record a structured preservation/support disposition with an honest stopped project or milestone state or a dedicated stopped-support owner.

Design consultation, canonical `DESIGN.md`, plan design review, and P04 implementation cannot start merely because P03 code or PR #65 is merged. Only the recorded Continue decision crosses this checkpoint.

## Scope boundary

The observation is product-discovery evidence, not B0 acceptance, a privacy pilot, accessibility conformance, platform certification, or permission to use real employee information. It does not add mobile, providers, AI, MCP, relationship scoring, or a task engine to B0.

P03 remains limited to recoverable canonical mutations, unknown-content preservation, final preconditions, and `UAT-042`. ADR 0016 separately assigns general and third-party post-A0 migration safety to `LRM-WS-007`, `T-R5-005`, and `FG-R5-005`; the required B0 OKF People normalization remains the narrow `LRM-WS-017` and `UAT-066` exception.

## Consequences

- P03D depends on `T-B0-P03-OBS`, which depends on technically accepted P03.
- The observation task is blocked while P03 is current. Technical acceptance completes P03 and makes OBS current while P03D and P04 remain blocked; Continue completes OBS and makes P03D eligible.
- Change or Stop also completes the observation record, but advances only the exactly recorded replacement or stopped-support disposition, never P03D or P04.
- Artifact identity and the decision receipt are independent evidence, so later changes cannot silently reuse an observation of a different build.
- Synthetic or redacted scenarios keep discovery available without weakening the real-workplace-data gate.

## Rollback or reversal conditions

Changing D1-B to a non-blocking observation requires a new explicit maintainer decision and a zero-orphan update to the requirement, task, feature gate, ownership graph, generated traceability, delivery contract, and status surfaces. A design approval, merged PR, or passing unit suite does not reverse this decision.
