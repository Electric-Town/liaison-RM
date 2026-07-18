# Local-folder provider evidence

Status: pending exact-commit verification.

The provider remains `not-tested` in `descriptor.json` until the same source commit completes all of the following:

- descriptor and WIT validation;
- formatting and compilation;
- Clippy with warnings denied;
- the `object-store@1` conformance suite;
- the full Rust workspace test suite;
- Ubuntu, macOS, and Windows jobs.

Any source, descriptor, WIT, workflow, or evidence change invalidates the prior run and requires the matrix to run again.

A green matrix proves only the checked local-folder contract. It does not prove multi-writer synchronization, durability on arbitrary network filesystems, or successful workspace restore.
