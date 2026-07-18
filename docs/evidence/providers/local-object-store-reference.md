# Local object-store reference evidence

- Date: 2026-07-17
- Provider ID: `org.electric-town.local-folder`
- Contract: `object-store@1`
- Requirements: REQ-161, REQ-162
- UAT: UAT-089
- Safe modes claimed: backup, single-writer

## Implemented checks

- safe object keys;
- SHA-256 calculation and expected-digest verification;
- immutable publication through a temporary file and hard link;
- object read and metadata;
- recursive prefix listing;
- guarded deletion using the expected digest;
- expected-revision manifest replacement;
- stale-revision rejection;
- cooperative local lock;
- previous-manifest recovery;
- symlink rejection;
- duplicate provider registration rejection.

## Local command

```bash
liaison provider conformance \
  --provider org.electric-town.local-folder \
  --path ./provider-test
```

The command uses synthetic bytes and writes only under the caller-selected
test path.

## Limits

- no multi-writer claim;
- no arbitrary network-filesystem claim;
- no protection from non-cooperating writers;
- a crashed process can leave a stale lock requiring operator inspection;
- no secret or grant persistence yet;
- no S3 or WebDAV adapter yet;
- no platform test result until CI runs;
- no Cargo lockfile until dependency resolution runs in CI.

## Merge gate

The PR stays draft until:

- Rust formatting, clippy, tests, and all operating-system jobs pass;
- the generated `Cargo.lock` is reviewed and committed;
- WIT syntax is validated with the selected component toolchain;
- dependency licence review confirms `sha2` and transitive packages.
