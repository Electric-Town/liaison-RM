# macOS desktop alpha evidence

Status: pending exact-head CI.

## Intended evidence

- static shell and local-authority check;
- JavaScript syntax check;
- deterministic icon comparison;
- browser workflow for workspace creation, person capture and validation;
- focus movement and semantic landmarks;
- 390-pixel reflow without horizontal overflow;
- zero external browser requests;
- Rust formatting, compilation, Clippy and tests;
- universal Apple Silicon and Intel application build;
- ad-hoc signature verification;
- DMG verification and checksums.

## Distribution boundary

Pull-request artifacts are reviewer builds. They are not notarized public releases. Publication requires the manually gated `macos-release.yml` workflow, Apple Developer credentials, notarization, stapling, Gatekeeper assessment and clean-machine UAT.
