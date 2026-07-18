# Gemini repository entry point

Use this read order:

1. `AGENTS.md`
2. `PROJECT_CONTEXT.md`
3. `docs/product/working-state-delivery.md`
4. `SPEC.md`
5. `AI_BUILD_INSTRUCTIONS.md`
6. relevant decisions, knowledge, requirements, UAT, and gates

Confirm what exists on the checked-out branch before proposing work. Do not turn planned features, screenshots, or draft PRs into completion claims.

Prefer one dependency-complete vertical slice. Domain rules belong to the owning Rust context; interfaces and providers call application services and receive no authority by convenience.

B0 Workplace Review Alpha is the current product gate; A0 Personal Memory Alpha follows it. An accepted architecture decision is not proof that its current binary, recovery, provider, or release gate has passed.
