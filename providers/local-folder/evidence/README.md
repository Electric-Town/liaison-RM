# Local-folder provider evidence

Status: passed with limits for `object-store@1`.

## Behavioral validation basis

- Source commit: `082abd18d3a1352329b55d93b0f375610d14d07b`
- Provider workflow: `29622707290`
- Rust workspace workflow: `29622707288`
- Repository policy workflow: `29622707287`
- Date: 2026-07-18

The provider and full Rust matrices passed on Ubuntu, macOS, and Windows. The validated checks included descriptor and WIT validation, formatting, compilation, Clippy with warnings denied, provider package tests, the `object-store@1` conformance suite, and the full Rust workspace test suite.

## Limits

The result proves the checked local-folder contract only. It does not prove:

- multi-writer synchronization;
- durability on arbitrary network filesystems;
- protection from non-cooperating writers;
- workspace backup completeness or isolated restore;
- safe use of WebDAV, S3, Google Drive, or another provider implementation.

The descriptor therefore remains limited to `backup` and `single-writer`. The backup release gate remains closed until a Workspace snapshot can be restored and validated in isolation.

Any executable source, descriptor semantics, WIT contract, or test-workflow change invalidates this behavioral result and requires the complete matrix to run again. Evidence-only metadata corrections do not change the validated behavior, but repository policy must still pass.
