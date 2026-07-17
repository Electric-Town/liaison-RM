# Local folder object-store provider

## Purpose

The local-folder provider is the reference `object-store@1` adapter. It stores opaque objects and manifests in a user-selected directory for local backup, removable-media exchange, isolated restore, and single-writer publication.

It is not the canonical Liaison workspace and it is not advertised as multi-writer synchronisation.

## Capabilities

- immutable object publication with caller-supplied SHA-256 digest;
- retrieval with local digest calculation;
- metadata lookup;
- deterministic prefix listing;
- deletion guarded by current digest;
- manifest creation/replacement guarded by current revision;
- configured object-size and list-size limits;
- encountered symbolic-link rejection;
- safe object-key validation.

## Consistency statement

The adapter uses local filesystem operations and a process-local manifest lock. Concurrent writers in separate processes or devices are outside its demonstrated semantics. Atomic replacement and directory durability differ by filesystem and operating system and remain subject to platform fault-injection evidence before release.

Safe labels:

- backup;
- restore;
- single-writer publication.

Unsupported label:

- multi-writer synchronisation.

## Configuration

`root` points to a directory dedicated to provider objects. Do not point it at the readable canonical workspace. `maximum_object_bytes` defaults to 512 MiB.

No secret or network destination is required.

## Recovery

Provider objects are opaque transport or backup material. Restore is performed through the Workspace backup/restore service, which verifies snapshot manifest, object hashes, encryption envelope, schema, and complete canonical records in an isolated directory before activation.

Deleting this provider directory can remove backup copies. It cannot delete the canonical workspace unless the operator separately selected the same path in violation of configuration guidance; future UI work must detect and reject path overlap.
