# Relationship YAML adapter

This adapter implements the Relationships context's `RelationshipRepository` port with one readable YAML record per Person under `relationships/<person-id>.yaml`.

It owns file translation only. Relationship type, tier, cadence, circles, boundaries, revision rules, and maintenance status remain in the Relationships bounded context.

## Guarantees

- versioned format and schema identifiers;
- optimistic revision checks;
- unknown top-level key preservation;
- create-without-overwrite and same-directory replacement;
- stable listing by Person ID;
- no network, database, provider, or scoring dependency.

## Limits

This first adapter does not yet share the vault write journal. Crash-consistent multi-file units of work remain a separate Workspace gate. It stores only relationship intent; factual communication logs belong to Interactions and Commitments.
