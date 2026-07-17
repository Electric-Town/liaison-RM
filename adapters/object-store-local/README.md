# Local object-store adapter

Reference implementation of `object-store@1` for a user-selected local directory.

It is suitable for backup and controlled single-writer publication. It does not claim multi-writer synchronization. The adapter rejects traversal, verifies content digests before publication, uses expected revisions for manifests, and refuses encountered symbolic links.
