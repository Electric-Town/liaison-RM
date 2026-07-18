# Liaison RM coding-agent instructions

Read these files before proposing or changing code:

1. `AGENTS.md`
2. `docs/PROJECT_CONTEXT.md`
3. `docs/STATUS.md`
4. `SPEC.md`
5. `AI_BUILD_INSTRUCTIONS.md`
6. the owning bounded-context README, decisions, knowledge articles, requirements, UAT, feature gates, schemas, and tests

Key constraints:

- canonical data is Markdown/YAML plus documented JSONL;
- projections are rebuildable and non-authoritative;
- business rules live in Rust domain/application code, not UI, prompts, workflows, or provider adapters;
- provider registration grants no access;
- Airgap and Connected-local are separate builds;
- no mandatory account, hidden telemetry, remote logging, licence check, or undeclared network request;
- no inferred relationship-strength, trust, affection, employee-value, productivity, or social-credit score;
- Review Priority orders an explainable queue only;
- empty, unknown, stale, declined, conflicting, and not-applicable field states remain distinct;
- user-facing changes require keyboard, screen-reader, reflow, reduced-motion, interruption, error, undo, and recovery review;
- use synthetic data;
- update tests, requirements, UAT, gates, knowledge, status, and changelog;
- report exact-head validation and keep incomplete work in draft.

Do not treat implementation code or an unfinished PR as more authoritative than accepted decisions, schemas, or normative specifications.
