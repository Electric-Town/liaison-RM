# Gemini repository entry point

Read [`AGENTS.md`](AGENTS.md) first. Then read [`PROJECT_CONTEXT.md`](PROJECT_CONTEXT.md), [`SPEC.md`](SPEC.md), and [`AI_BUILD_INSTRUCTIONS.md`](AI_BUILD_INSTRUCTIONS.md).

`AGENTS.md` is normative. `PROJECT_CONTEXT.md` contains the complete product, architecture, status, terminology, delivery, and handoff context. Do not infer implementation from prototypes or open pull requests; verify the branch, exact head, changed files, and CI evidence.

Use domain-driven vertical slices. Keep business rules in the owning Rust context and application services. Preserve local authority, readable canonical files, explicit provider and AI grants, Airgap separation, accessibility, privacy, and recovery requirements.
