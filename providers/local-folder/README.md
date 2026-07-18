# Local folder provider

Reference package for `object-store@1` using a user-selected local directory. The provider advertises backup-transport and controlled single-writer modes only. Configuration contains a path reference and no secret value or network destination.

Conformance status is `passed-with-limits` for the checked `object-store@1` behavior recorded in [`evidence/README.md`](evidence/README.md). This result does not prove workspace backup completeness, encrypted recovery, arbitrary network-filesystem durability, non-cooperating-writer safety, or multi-writer synchronization.

There is no released `liaison provider` CLI command in the current pre-alpha slice. Contributors can rerun the implemented checks with:

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

`check_wit_contract.py` verifies the checked-in contract shape only; it is not a WIT parser or component-toolchain conformance result.
