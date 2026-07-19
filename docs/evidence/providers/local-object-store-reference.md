# Local object-store reference evidence

Status: pointer to the current bounded evidence record.

The canonical exact-head record for `org.electric-town.local-folder` is [`providers/local-folder/evidence/README.md`](../../../providers/local-folder/evidence/README.md). Its `passed-with-limits` result covers the checked `object-store@1` adapter behavior only.

## Traceability

- Requirements: `LRM-CO-001`, `LRM-CO-004`, `LRM-CO-005`
- UAT: `UAT-027`, `UAT-034`
- Decision: ADR 0003
- Descriptor: [`providers/local-folder/descriptor.json`](../../../providers/local-folder/descriptor.json)

## Current limits

The evidence does not prove:

- a complete Workspace checkpoint or encrypted recovery package;
- isolated clean-install restore;
- multi-writer synchronization;
- durability on arbitrary network filesystems;
- protection from non-cooperating writers;
- safe behavior for WebDAV, S3, Google Drive, or any other adapter;
- a released provider CLI;
- WIT Component Model parsing or execution by a selected component toolchain.

The checked-in `scripts/check_wit_contract.py` performs a deterministic structural check. A real component-toolchain gate remains future provider/plugin work.

## Reproduction

```bash
python3 scripts/check_providers.py
python3 scripts/check_wit_contract.py
cargo test \
  -p liaison-connections \
  -p liaison-provider-registry-memory \
  -p liaison-object-store-local \
  -p liaison-provider-sdk \
  --all-features --locked
```

These commands exercise source and adapter behavior. They do not open the B0 encrypted-recovery gate or a provider-product release gate.
