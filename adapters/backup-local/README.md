# Local workspace backup adapter

This adapter creates immutable directory snapshots of a Liaison workspace, verifies every declared payload by SHA-256, and stages restores into a new isolated directory before the Workspace context activates them.

## Layout

```text
backup-directory/
├── manifest.json
└── payload/
    ├── .liaison/workspace.yaml
    ├── people/...
    └── other canonical workspace files
```

Disposable projections, transient locks, temporary files, and restore markers are excluded. Symbolic links are rejected. A backup destination cannot be inside the source workspace, an existing backup is never overwritten, and a restore target must not already exist.

A staged restore contains `.liaison/restore-in-progress` until the Workspace context loads the manifest, checks identity and schema, and reports no error-level layout findings. Failed validation removes only a target that still carries that marker.

This adapter proves local directory backup and isolated restore. It does not provide encryption, removable-media policy, cloud upload, retention scheduling, deduplication, or multi-writer synchronization.
