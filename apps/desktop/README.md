# Liaison RM desktop

The desktop application is an inbound Tauri interface over the Workspace, People, and Markdown-vault application services.

## Current alpha workflow

- show the build's local-authority status;
- suggest a local Documents path;
- create an Airgap workspace;
- open an existing Liaison workspace;
- create a basic person profile;
- list current person records;
- validate workspace layout and records without deleting files.

The desktop layer does not own canonical schemas, calculate relationship priority, configure providers, or write files outside application-service ports.

## Development

```bash
python -m http.server --directory apps/desktop/ui 4173
```

The browser view will report that the native bridge is unavailable. Browser acceptance tests inject a deterministic command bridge before loading the page.

Native development requires the Tauri 2 prerequisites for the operating system:

```bash
cd apps/desktop
cargo tauri dev
```

## Validation

```bash
python scripts/check_desktop_shell.py
node --check apps/desktop/ui/app.js
python scripts/test_desktop_ui.py
python scripts/generate_desktop_assets.py --check
cargo fmt --all --check
cargo check -p liaison-desktop --all-targets --all-features --locked
cargo clippy -p liaison-desktop --all-targets --all-features --locked -- -D warnings
cargo test -p liaison-desktop --all-features --locked
```

## Distribution boundary

Pull-request artifacts are ad-hoc-signed review builds. Public distribution requires Developer ID signing, Apple notarization, stapling, Gatekeeper verification, and clean-machine UAT. The release workflow refuses to run without the required Apple secrets.

## Hand-drawn interface contract

The desktop interface uses `ui/design-system.css` as its only visual token source. The system is deliberately light-only and paper-like because the current desktop alpha is local-authoritative and must not depend on remote fonts or theme assets.

Required characteristics:

- warm paper background and pencil-black foreground;
- correction-marker red and ballpoint-blue accents;
- irregular reusable radii rather than standard rounded rectangles;
- thick borders and hard offset shadows with no blur;
- handwriting-oriented local font stacks;
- visible focus, 48-pixel primary targets, reduced-motion support, and 390-pixel reflow;
- decoration that never carries meaning by itself.

Add a reusable token or component class before adding a one-off visual rule. New interface work must remain readable when the preferred handwritten fonts are unavailable.
