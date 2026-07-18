# Liaison RM instructions for GitHub Copilot

The normative instructions are in [`AGENTS.md`](../AGENTS.md). The complete product and engineering handoff is in [`PROJECT_CONTEXT.md`](../PROJECT_CONTEXT.md).

Before editing code:

1. Confirm the branch, pull-request base, exact head, and current checks.
2. Read `AGENTS.md`, `PROJECT_CONTEXT.md`, `SPEC.md`, and `AI_BUILD_INSTRUCTIONS.md`.
3. Identify the owning bounded context and read its README and tests.
4. Search decisions, knowledge articles, requirements, UAT cases, feature gates, implementation tasks, and open pull requests.
5. Distinguish implemented behaviour from planned behaviour.

Do not put business rules in the UI, provider adapters, connectors, or plugins. Preserve local authority, open canonical formats, Airgap boundaries, explicit grants, unknown-field round trips, accessibility requirements, and exact evidence for completion claims.

Leave enough committed context for another contributor to continue without private prompt history.
