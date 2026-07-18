# Local workspace backup and isolated restore evidence

Status: implementation under review.

## Scope

This evidence record covers the first local directory backup slice:

- source-workspace validation before snapshot creation;
- deterministic manifest ordering;
- exact payload membership;
- per-file byte size and SHA-256 digest;
- projection and transient-state exclusion;
- symbolic-link rejection;
- refusal to overwrite an existing backup or restore target;
- refusal to place a backup inside its source workspace;
- restore into a new isolated directory;
- workspace identity, schema, and layout validation before activation;
- cleanup restricted to a target carrying `.liaison/restore-in-progress`.

## Required commands

```text
cargo fmt --all --check
cargo check -p liaison-workspace -p liaison-backup-local -p liaison-cli --all-targets --all-features --locked
cargo clippy -p liaison-workspace -p liaison-backup-local -p liaison-cli --all-targets --all-features --locked -- -D warnings
cargo test -p liaison-workspace -p liaison-backup-local -p liaison-cli --all-features --locked
```

The dedicated workflow runs these commands on Ubuntu, macOS, and Windows.

## Limits

This slice does not yet provide:

- encrypted backup packaging;
- retention schedules or pruning;
- removable-media policy;
- provider upload or restore download;
- incremental or deduplicated snapshots;
- journalled capture while concurrent writers are active;
- clean-machine restore UAT with a tagged release build.

A successful local snapshot is therefore not yet a complete R1 release claim. Encryption, concurrent-write coordination, and clean-machine restore evidence remain explicit gates.
