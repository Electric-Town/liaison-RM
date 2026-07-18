# macOS desktop alpha evidence

Status: browser evidence generated; native exact-head CI pending.

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

## Native evidence required on the same head

- Rust formatting, compilation, Clippy and tests;
- Workspace and People vertical-slice tests through the Tauri command adapter;
- universal Apple Silicon and Intel application build;
- ad-hoc signature verification;
- DMG verification and checksums.

## Distribution boundary

Pull-request artifacts are reviewer builds. They are not notarized public releases. Publication requires the manually gated `macos-release.yml` workflow, Apple Developer credentials, notarization, stapling, Gatekeeper assessment and clean-machine UAT.
