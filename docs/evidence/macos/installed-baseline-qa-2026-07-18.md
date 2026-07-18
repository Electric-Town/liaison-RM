# Installed macOS baseline QA — 2026-07-18

Status: **failed baseline; not release evidence**

This record captures the installed application that existed before the P01 composition-root build. It explains the native defect P01 must eliminate and does not claim that P01 has passed installed-app QA.

The later [P01 native QA record](p01-native-qa-2026-07-18.md) proves the corrected narrow workflow on an exact Apple Silicon build; it does not turn this baseline artifact into a passing build.

## Artifact observed

- bundle: `/Applications/Liaison RM.app`;
- bundle identifier: `io.github.electric-town.liaison-rm`;
- `CFBundleShortVersionString`: `0.1.0-alpha.1`;
- `CFBundleVersion`: `0.1.0-alpha.1`;
- visible runtime status: `Liaison RM 0.1.0 is ready`;
- executable SHA-256: `ca25c9a15f2351e7f3287fd536c14b53eb90d7aaa3de355522521c11306ea13a`;
- executable architectures: Apple Silicon and Intel;
- bundle verification: `codesign --verify --deep --strict` reported that the bundle was not signed;
- executable inspection: linker-signed/ad-hoc code directory with no Team identifier.

The version mismatch and absent bundle signature mean visible status text and the local application path are not source or release provenance.

## Native workflow exercised

Synthetic data only was used in a new temporary workspace.

1. Selected the Workplace profile and created a local workspace.
2. Inspected the generated `.liaison/workspace.yaml` as ordinary text.
3. Ran Workspace Health through the native command bridge.
4. Attempted to create a synthetic Person with an `example.test` address.
5. Quit and relaunched the application.
6. Reopened the same workspace manually.

## Results

| Check | Result | Evidence |
|---|---|---|
| Application launch | Pass | Native window opened at `tauri://localhost`. |
| Workspace creation | Pass | A readable schema-version-1 workspace manifest was created. |
| Workspace profile | Pass | The manifest recorded `profile: workplace`. |
| Build-profile claim | Fail | The manifest recorded `build_profile: airgap`, although this installed artifact has no Airgap runtime proof. |
| Workspace Health | Pass, narrow | Health reported the newly created workspace as valid. |
| Person creation | Fail | The native bridge returned `invalid args request for command create_person: missing field workspace_path`; no Person file was created. |
| Error usability | Fail | The raw Tauri argument error was shown to the user and provided no bounded recovery action. |
| Workspace state after relaunch | Partial | The workspace was not remembered, but manual open succeeded and preserved the manifest. |
| Version consistency | Fail | Bundle metadata reported `0.1.0-alpha.1`; visible runtime copy reported `0.1.0`. |
| Network absence | Not proven | Browser-fixture zero-egress evidence does not establish native process Airgap behavior. |

## Required regression proof

The next installed artifact must be built from an exact commit and pass all of the following on that same checksum:

- the P01 UI and Tauri command adapter exchange the same versioned request/result contract;
- workspace creation, Person creation, list, Health, quit, relaunch, and reopen succeed through the native bridge;
- a Person created in the application is independently readable as Markdown and remains present after relaunch;
- raw framework argument errors never reach product copy;
- runtime and bundle versions match;
- build-profile language remains `connected-local` until a separate Airgap artifact passes compiled and runtime network-denial evidence;
- signature, architecture, source commit, workflow run, checksum, and installation source are bound in one evidence record.

The deterministic browser fixture remains useful for focus, responsive layout, contrast, reduced motion, retry behavior, and request-shape tests. It cannot close these native gates.
