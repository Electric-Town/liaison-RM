# macOS desktop alpha evidence

Status: browser fixture generated and an installed review app has been observed launching; exact-source native provenance and B0 qualification remain open.

## Generated interface evidence

The deterministic browser fixture completed:

- local workspace creation;
- person capture;
- workspace validation;
- programmatic focus movement;
- semantic landmarks and explicit labels;
- 390-pixel reflow without horizontal overflow;
- reduced-motion mode;
- zero external browser requests.

Review artifacts are committed under:

- `docs/evidence/macos/browser-test-report.json`;
- `docs/evidence/macos/screenshots/desktop-workspace-health.png`;
- `docs/evidence/macos/screenshots/mobile-people.png`.

The browser fixture exercises interface behavior with a deterministic fake Tauri command bridge. It does not replace the native Rust, filesystem, package, signature, or clean-machine tests.

## Installed review observation

On 2026-07-18 the installed application with bundle identifier `io.github.electric-town.liaison-rm` was observed launching to the local Workspace onboarding screen. The reviewed product version is `0.1.0-alpha.1`.

This observation proves only that an installed application launches and exposes the narrow pre-alpha interface. This evidence file does not yet bind that installed bundle to an exact source commit, checksum, architecture inspection, signature assessment, or CI run. It does not prove workspace persistence, relaunch recovery, Airgap network absence, accessibility conformance, or B0 behavior.

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
