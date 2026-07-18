# Gemini repository entry point

Use this read order:

1. `AGENTS.md`
2. `PROJECT_CONTEXT.md`
3. `SPEC.md`
4. `AI_BUILD_INSTRUCTIONS.md`
5. relevant decisions, knowledge, requirements, UAT, and gates

Confirm what exists on the checked-out branch before proposing work. Do not turn planned features, screenshots, or draft PRs into completion claims.

Prefer one dependency-complete vertical slice. Domain rules belong to the owning Rust context; interfaces and providers call application services and receive no authority by convenience.
