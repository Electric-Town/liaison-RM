# Liaison RM instructions for GitHub Copilot

Read `AGENTS.md`, `PROJECT_CONTEXT.md`, `SPEC.md`, and `AI_BUILD_INSTRUCTIONS.md` before editing.

Then:

- confirm the branch, PR base, exact head, changed files, and checks;
- identify the owning bounded context;
- read its README and tests;
- search decisions, knowledge, requirements, UAT, gates, tasks, and overlapping PRs;
- distinguish implemented, proposed, blocked, and prohibited behaviour.

Keep rules in the owning Rust context and application services. Preserve local authority, open formats, grants, Airgap boundaries, accessibility, privacy, compatibility, and recovery.

Leave committed handoff context. Do not add private prompt history, personal data, credentials, or unsupported completion claims.
