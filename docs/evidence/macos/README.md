# macOS desktop alpha evidence

Status: browser fixture generated and an installed review app has been observed launching; exact-source native provenance and B0 qualification remain open.

## Generated interface evidence

The deterministic browser fixture completed:

- local workspace creation;
- create/open error recovery and retry;
- rejection of an incompatible application contract version;
- person capture;
- valid and invalid workspace Health rendering with recovery guidance;
- programmatic focus movement;
- semantic landmarks and explicit labels;
- 390-pixel reflow without horizontal overflow;
- reduced-motion mode;
- resolved dark-mode token rendering;
- zero external browser requests across desktop, mobile, and dark-mode pages.

Review artifacts are committed under:

- `docs/evidence/macos/browser-test-report.json`;
- `docs/evidence/macos/screenshots/desktop-workspace-health.png`;
- `docs/evidence/macos/screenshots/mobile-people.png`;
- `docs/evidence/macos/screenshots/dark-workspace.png`.

The separate [installed baseline QA record](installed-baseline-qa-2026-07-18.md) captures the native argument-contract failure and version/build-profile mismatches observed in the pre-P01 installed bundle.

The [P01 native QA record](p01-native-qa-2026-07-18.md) binds the corrected Workspace/Person/Health workflow to an exact Apple Silicon source commit and executable checksum while keeping signing, universal packaging, relaunch restoration, network, accessibility, and release gates open.

The browser fixture exercises interface behavior with a deterministic fake Tauri command bridge. It does not replace the native Rust, filesystem, package, signature, or clean-machine tests.

## Installed review observation

On 2026-07-18 the installed application with bundle identifier `io.github.electric-town.liaison-rm` was observed launching to the local Workspace onboarding screen. Its `Info.plist` reports bundle version `0.1.0-alpha.1`, while the visible runtime status reports `0.1.0`. That mismatch remains open and means the status text is not valid provenance evidence.

This observation proves only that an installed application launches and exposes the narrow pre-alpha interface. Bundle inspection confirms a universal Apple Silicon/Intel executable with an ad-hoc linker signature and no Team identifier; it does not bind the bundle to an exact source commit or CI run. It does not prove workspace persistence, relaunch recovery, Airgap network absence, accessibility conformance, or B0 behavior.

## Native B0 evidence required on the same artifact

- Rust formatting, compilation, Clippy and tests;
- Workspace, Directory, Event, Health, security, and recovery vertical-slice tests through the Tauri command adapter;
- universal Apple Silicon and Intel application build;
- ad-hoc signature verification;
- application checksum and source-commit provenance;
- installed-app workspace create/open/relaunch and readable-file proof;
- clean-install encrypted recovery without the original Keychain state;
- keyboard, VoiceOver, 200% zoom/reflow, contrast, reduced-motion, narrow-window, and error/recovery evidence;
- runtime offline and undeclared-egress proof.

## Distribution boundary

Pull-request artifacts are reviewer builds. They are not notarized public releases. Publication requires the manually gated `macos-release.yml` workflow, Apple Developer credentials, notarization, stapling, Gatekeeper assessment and clean-machine UAT.

Until those gates pass, use “local-authoritative review build” rather than “Airgap build”, “supported Mac release”, or “ready for daily use”.
