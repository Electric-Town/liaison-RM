---
id: KCS-0013
title: How do I validate a Rust change so CI agrees with my local run?
state: Draft
owner: repository-governance
created: 2026-07-19
reviewed: 2026-07-19
applies_to:
  - contributors
  - coding agents
  - maintainers
search_terms:
  - cargo locked
  - cargo offline
  - Cargo.lock
  - lock file
  - cannot update the lock file
  - local passes CI fails
  - dev-dependency
  - workspace member
  - green locally
related_requirements: []
related_gates: []
---

# KCS-0013: How do I validate a Rust change so CI agrees with my local run?

## Problem

Every Rust job in CI runs with `--locked`. A contributor validates locally, sees everything pass, pushes, and CI fails every platform job with:

```text
error: cannot update the lock file …/Cargo.lock because --locked was passed to prevent this
```

The code is fine. The committed state is not: `Cargo.lock` does not match `Cargo.toml`. This has already cost one change set ten failed jobs across three platforms.

## Environment and preconditions

- The workspace pins its toolchain in `rust-toolchain.toml`.
- CI invokes `cargo check`, `cargo clippy`, and `cargo test` with `--locked`, so it refuses to modify `Cargo.lock` and fails instead.
- `Cargo.lock` is committed and is part of the reviewable change.

## Resolution

Anything that changes the dependency graph changes `Cargo.lock`. That includes cases that feel too small to matter:

- adding a workspace member (a new crate under `contexts/`, `adapters/`, or `crates/`);
- adding a dependency **or a dev-dependency** to an existing crate;
- changing a version or a feature list.

The safe sequence:

1. Make the change.
2. Regenerate the lock without network access: run any cargo command with `--offline` (for example `cargo check --workspace --offline`). This updates `Cargo.lock` in the working tree.
3. **Stage `Cargo.lock` with the rest of the change.** It is not an artifact; it is part of the commit.
4. Verify the way CI does, with `--locked`:

```bash
cargo fmt --all --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

5. Confirm `git status` is clean. A modified `Cargo.lock` sitting unstaged means the commit is still wrong even though every command above passed.

## Why it works

`--offline` and `--locked` answer different questions.

- `--offline` means "do not reach the network." It will still happily rewrite `Cargo.lock` from the crates already in the local registry cache — silently.
- `--locked` means "do not modify `Cargo.lock` at all." It is the only mode that tests the state actually committed.

Validating with `--offline` therefore tests the *working tree*, which includes a lock file cargo just repaired for you. CI has no such repair step: it checks out the commit and fails on the mismatch. The local run and the CI run were never asking the same question.

## Verification

- The four `--locked` commands above pass.
- `git status --porcelain` prints nothing.
- `git show --stat HEAD` lists `Cargo.lock` whenever the change added a crate, a dependency, or a dev-dependency.

## Recovery or rollback

If CI has already failed this way:

1. Run `cargo check --workspace --all-targets --all-features --offline` to regenerate the lock.
2. Inspect `git diff Cargo.lock` — the diff should be small and should mention only the packages you expect. A large or surprising diff means something else changed, such as a toolchain or registry difference; investigate before committing it.
3. Commit the lock as a focused fix and re-run the `--locked` commands before pushing.

No revert is needed; the source was never wrong.

## Known limitations

- `--locked` legitimately fails *before* step 2 when a new workspace member has not been recorded yet. That is the expected order: regenerate with `--offline` first, then verify with `--locked`. Do not treat the initial failure as a reason to keep using `--offline` for verification.
- A clean `--locked` run proves the committed dependency graph resolves. It does not prove platform-specific behaviour; native Linux, macOS, and Windows evidence remains separate.
- Branches parked for later harvest may carry a stale lock if they were only ever validated with `--offline`. Regenerate and re-verify when rebasing them, rather than trusting an older green report.

## Related decisions, tests, and articles

- `AI_BUILD_INSTRUCTIONS.md` §10 — the required checks for a change.
- `PROJECT_CONTEXT.md` §26 — the baseline validation commands.
- KCS-0010 — keeping inbound adapters on one application workflow, whose evidence relies on the same workspace-wide commands.
