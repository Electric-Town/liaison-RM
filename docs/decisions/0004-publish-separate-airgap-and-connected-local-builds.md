# 0004: Publish separate Airgap and Connected-local builds

- Status: proposed
- Date: 2026-07-17
- Contexts: workspace, connections, automation, packaging

## Context and problem

The product must be completely usable locally, while optional features require CardDAV, WebDAV, object storage, calendars, email, webhooks, API clients, MCP, and remote AI. A runtime `offline` switch does not prove that a binary cannot connect to a network.

## Alternatives considered

1. One binary with networking disabled by default.
2. One binary with an enterprise policy file.
3. Separate Airgap and Connected-local build profiles.

A default or policy can be changed, bypassed, or mispackaged. It cannot satisfy the stronger assurance expected by an air-gapped user.

## Decision

Publish separate artifacts. The Airgap profile removes network clients and listeners from the dependency graph and receives no network sandbox permission. Connected-local includes optional connection and local-service capabilities but starts with no configured provider, account, telemetry, or remote endpoint.

## Consequences

- CI builds and tests both feature graphs.
- Packaging manifests and dependency audits differ by profile.
- Documentation states which artifact is installed.
- Provider, webhook, remote AI, network MCP, CardDAV, CalDAV, and update-check code cannot appear in the Airgap artifact.
- Local and removable-media exchange remains available in Airgap.

## Reversal conditions

The two profiles may share more code only when binary inspection and runtime tests continue to demonstrate Airgap network absence. They may not be collapsed into a runtime toggle without a new protected-principle decision.
