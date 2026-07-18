# Development guide

This guide covers repository setup, validation, and the expected pull-request workflow. Product and architecture context lives in `docs/PROJECT_CONTEXT.md`.

## Toolchain

Baseline tools:

- Git;
- Rust from `rust-toolchain.toml`;
- Cargo and rustfmt/Clippy components;
- Python 3.12 for repository and browser checks;
- Node.js 22 for JavaScript syntax checks;
- Chromium or Playwright Chromium for interaction tests;
- platform prerequisites for Tauri 2;
- Xcode command-line tools for macOS bundles;
- GitHub CLI is optional and not part of the runtime.

Do not silently update the Rust toolchain, Tauri major version, workflow action major versions, or canonical schema versions in an unrelated change.

## Clone and inspect

```bash
git clone https://github.com/Electric-Town/liaison-RM.git
cd liaison-RM
```

The default branch may lag the active stacked review branch. Inspect open pull requests and `docs/STATUS.md` before choosing a base.

```bash
git branch -a
git log --oneline --decorate -20
```

## Repository validation

```bash
python scripts/check_repository.py
python scripts/check_spec.py
python scripts/check_architecture.py
python scripts/check_providers.py
python scripts/check_wit_contract.py
```

Other scripts are path-specific. Read the corresponding workflow before invoking them.

## Rust workspace validation

```bash
cargo fmt --all --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

Do not remove `--locked` from CI to work around an unreviewed lockfile change.

## CLI development

```bash
cargo run -p liaison-cli -- --help
cargo test -p liaison-cli --all-features --locked
```

Typical local workflow:

```bash
cargo run -p liaison-cli -- workspace init \
  --path ./tmp/example-workspace \
  --name "Example workspace"

cargo run -p liaison-cli -- person create \
  --workspace ./tmp/example-workspace \
  --name "Alex Murphy" \
  --email "alex@example.test"

cargo run -p liaison-cli -- person list \
  --workspace ./tmp/example-workspace

cargo run -p liaison-cli -- workspace validate \
  --path ./tmp/example-workspace
```

Use `.test` domains and synthetic people in documentation and fixtures.

## Desktop development

Static UI review:

```bash
python -m http.server --directory apps/desktop/ui 4173
```

The browser reports that the native bridge is unavailable. Browser acceptance tests inject a deterministic bridge.

Native development:

```bash
cd apps/desktop
cargo tauri dev
```

Desktop checks:

```bash
python scripts/check_desktop_shell.py
node --check apps/desktop/ui/app.js
python scripts/test_desktop_ui.py
python scripts/generate_desktop_assets.py --check
cargo check -p liaison-desktop --all-targets --all-features --locked
cargo clippy -p liaison-desktop --all-targets --all-features --locked -- -D warnings
cargo test -p liaison-desktop --all-features --locked
```

Pull-request Mac artifacts are review builds. Do not represent ad-hoc signing as Developer ID signing or notarization.

## Interaction prototype

```bash
python -m pip install playwright==1.57.0
python -m playwright install chromium
python scripts/test_prototype.py
```

A system Chromium can be selected with `CHROMIUM_PATH` where supported.

## Provider development

A provider must implement a versioned Connections contract and publish a descriptor, configuration schema, limitations, synthetic fixtures, conformance evidence, recovery instructions, and knowledge action.

```bash
python scripts/check_providers.py
python scripts/check_wit_contract.py
cargo test -p liaison-provider-sdk --all-features --locked
cargo test -p liaison-object-store-local --all-features --locked
```

Do not infer multi-writer safety from an upload test, provider brand, S3-compatible label, object versioning, or a successful backup.

## Schema and specification changes

A domain or external contract change usually requires:

- human-readable specification update;
- JSON Schema or WIT update;
- example update;
- requirement and UAT update;
- feature-gate update;
- migration or compatibility statement;
- changelog and status decision;
- tests for older and newer supported forms.

Stable field IDs, provider IDs, contract names, record formats, and event envelopes are compatibility surfaces.

## Pull-request workflow

1. Inspect `docs/STATUS.md` and the current stack.
2. Create a focused branch from the correct dependency branch.
3. State the problem and user evidence before implementation.
4. Name the owning bounded context.
5. Add tests before claiming completion.
6. Keep external mechanisms in adapters.
7. Update knowledge, requirements, gates, changelog, and status.
8. Run relevant local checks.
9. Open a draft if any dependency or evidence is incomplete.
10. Wait for exact-head CI before marking ready.
11. Retarget stacked PRs to `main` only after their parent merges.

Do not merge an implementation around a required architecture or schema review.

## Dependency review

Before adding a dependency, record:

- purpose and owning boundary;
- licence compatibility with AGPL-3.0;
- maintenance and release cadence;
- transitive dependency and build-script surface;
- network, filesystem, subprocess, native-code, and secret access;
- Airgap build effect;
- platform and packaging effect;
- removal or migration plan.

## Failure handling

A failed check is evidence. Do not suppress it without understanding the invariant.

- preserve useful diagnostics in the PR discussion, not permanent repository payloads;
- remove one-shot repair workflows before final review;
- fix or document the failing contract;
- rerun the exact head;
- keep the PR in draft while required checks are red or absent.

## Release preparation

A release branch must identify the declared feature set and its gates. At minimum:

```text
format and lint
unit and integration tests
schema compatibility
migration and rollback
backup and isolated restore
platform packages
signatures and checksums
accessibility evidence
security and dependency review
clean-machine UAT
knowledge and changelog
```

No release is created merely because a platform bundle compiled.
