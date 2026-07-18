# 0003: Use provider-neutral capability contracts

- Status: proposed
- Date: 2026-07-17
- Contexts: connections, sharing, automation

## Context and problem

The product needs optional integrations with Google Drive, WebDAV, S3-compatible services, contact and calendar servers, email systems, local folders, removable media, and future providers. Direct provider logic in product workflows would produce vendor-specific domain concepts, duplicated grant handling, and inconsistent safety claims.

## Alternatives considered

1. Implement Google Drive first and generalise later.
2. Expose a generic HTTP plugin interface.
3. Let each provider define its own commands and configuration.
4. Define versioned capability contracts with provider descriptors and conformance evidence.

Provider-first designs lock early workflows to one API. Generic HTTP or arbitrary commands provide extensibility without reliable semantics, grants, or testability.

## Decision

The Connections context owns versioned capability contracts. Providers implement contracts through anti-corruption layers and publish descriptors declaring operations, destinations, secret slots, consistency, limits, safe modes, and conformance evidence. Product workflows depend on contracts, not provider SDKs.

## Consequences

- Google Drive, S3, WebDAV, and local folders can share object-store and backup workflows while preserving different guarantees.
- A provider may be approved for backup but not multi-writer synchronisation.
- New contracts receive independent architecture review.
- Provider packages can be modular and eventually distributed as capability-controlled WASI components where practical.
- Grant and egress checks are uniform.

## Reversal conditions

A provider-specific path is permitted only when its user-visible capability cannot be expressed honestly through an existing or new reusable contract. The exception requires a decision record and cannot leak its DTOs into domain contexts.
