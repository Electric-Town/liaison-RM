# Markdown vault adapter

This adapter implements Workspace and People repository ports using the open Liaison workspace.

## Responsibilities

- create the initial directory structure and readable workspace manifest;
- serialise and parse versioned person Markdown records;
- preserve unknown YAML properties and human-authored body sections;
- use stable ID-bearing filenames;
- apply revision preconditions before replacement;
- return validation findings without deleting invalid files;
- keep healthy People readable while Health reports malformed sibling records;
- keep format documents private to the adapter.

## Non-responsibilities

The adapter does not define person, relationship, event, provider, or sharing rules. It does not expose a generic filesystem API to domain code, run network operations, own encryption keys, or treat a projection as canonical.

## Current limitations

This R1 draft establishes the vertical slice. The following remain gated before release:

- durable write journal and directory flush behaviour on every supported filesystem;
- cross-platform atomic-replacement fallback and fault injection;
- workspace locking and bound workspace sessions;
- complete unknown Markdown-section round-trip tests;
- duplicate-ID validation across all records;
- schema-generated format validation;
- projection rebuild and migration services.

The pull request remains draft until the Rust CI matrix and these release dependencies are addressed.
