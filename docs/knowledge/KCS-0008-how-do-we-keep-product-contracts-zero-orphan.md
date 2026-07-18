# KCS-0008: How do we change product contracts without creating orphans?

- Status: verified
- Audience: maintainers, product contributors, implementation agents
- Last reviewed: 2026-07-18
- Applies to: requirements, personas, UAT, feature gates, implementation tasks, milestones, evidence, and branch dispositions

## Problem

A valid identifier is not the same as an executable product contract. Requirements and UAT cases can exist without one accountable task, gate, milestone, status, or evidence owner. Tasks can also become executable without owning acceptance, generated reports can drift from their inputs, and old branches can silently reintroduce a superseded delivery order.

## Resolution

Treat `spec/traceability-ownership.json` as the explicit ownership source. Do not infer ownership from array position, release labels, nearby prose, or the first task that happens to mention an identifier.

When changing any governed catalogue:

1. Update the requirement, persona, UAT, gate, or task in its canonical source.
2. Add or update exactly one ownership record with an owning task or acceptance task, owning gate, milestone, evidence owner, and status.
3. Make the owning task name every requirement and UAT it owns. Make the owning gate name each owned UAT in its evidence.
4. Keep task dependencies acyclic and topologically ordered. A dependency must appear before its dependent task.
5. Give superseded tasks a real replacement and a disposition. Preserve reviewed branches only with an explicit blocked or deferred condition.
6. Record the disposition and canonical identifier for every adopted founder-plan proposal.
7. Run `python3 scripts/generate_traceability.py`, then run it again with `--check` to prove byte-deterministic output.
8. Run `python3 scripts/check_spec.py`. Do not weaken a semantic contradiction check to make a catalogue pass.

The generator owns `spec/traceability-report.json` and `docs/product/traceability.md`. Never edit those outputs by hand.

## B0 semantic checks

Zero-orphan structure does not make contradictory product language safe. B0 must continue to assert:

- one trusted workspace owner, with no catering-role grant;
- ordered, versioned, fail-closed readiness derived from orthogonal source facts;
- attendee correction history and exact active-denominator reconciliation;
- names absent from recipient briefs by default, with opaque identifiers only under explicit approved policy;
- byte-identical preview and emitted immutable brief evidence;
- structural absence of diagnoses, private notes, hidden-row metadata, generic tasks, relationship allocation, cadence, scoring, and attention weights;
- a separate independent governance gate before any real workplace-data pilot.

If a changed UAT, gate, task, requirement, README, or public page contradicts those assertions, reconcile the source rather than adding an exception in prose.

## Verification

- `python3 scripts/generate_traceability.py --check` exits successfully.
- `python3 scripts/check_spec.py` reports zero orphan or unknown ownership records.
- Every adopted proposal resolves to an existing canonical identifier.
- Task dependencies are known, acyclic, and topologically ordered.
- `docs/product/traceability.md` and `spec/traceability-report.json` match the generator byte for byte.
- Public README, project context, and roadmap describe B0 before A0 and distinguish current implementation from planned behaviour.

## Common mistakes

- adding a requirement and only mentioning it in a task description;
- using release labels as ownership;
- letting a current task belong to a blocked milestone;
- editing generated traceability by hand;
- treating a passing zero-orphan check as proof that contract wording is consistent;
- calling a role preset a confidentiality boundary for one unlocked local operating-system account;
- allowing a stale personal-first or desktop-design branch to become current authority before its gate.
