# Markdown vault adapter

This adapter implements Workspace and People repository ports using the open Liaison workspace.

## Responsibilities

- create the initial directory structure and readable workspace manifest;
- serialise and parse versioned person Markdown records;
- preserve unknown YAML properties and human-authored body sections;
- use stable ID-bearing filenames;
- apply revision preconditions before replacement;
- retain a capability-bound root for all open-session manifest and Person access;
- return validation findings without deleting invalid files;
- keep healthy People readable while Health reports malformed sibling records;
- keep format documents private to the adapter.

## Non-responsibilities

The adapter does not define person, relationship, event, provider, or sharing rules. It does not expose a generic filesystem API to domain code, run network operations, own encryption keys, or treat a projection as canonical.

## Current limitations

The current P03 source candidate implements durable operation journals,
directory flushes, capability-bound replacement, staged cleanup, roll-forward
recovery, and fault injection for canonical Person mutations. `T-B0-P03`
remains current until its exact-head cross-platform evidence is accepted.
The following remain gated before release:

- exact-head Linux, macOS, and Windows durability, cleanup, and recovery evidence;
- complete unknown Markdown-section round-trip tests;
- duplicate-ID validation across all records;
- schema-generated format validation;
- projection rebuild and migration services.

Local P02 tests exercise the bound-root behavior. Exact-head remote Linux,
macOS, and Windows matrices and the remaining release dependencies are still
required before a release claim.
