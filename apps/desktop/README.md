# Liaison RM desktop

The desktop application is an inbound Tauri adapter over `liaison-application`, the sole Rust composition root. Tauri owns one managed application instance. Opening or creating a workspace returns an opaque session identifier; later commands use that session rather than accepting another filesystem path.

When the user opens or creates a replacement workspace, the interface closes
the previous native session before accepting the replacement. If the previous
session cannot close, it keeps the previous selection and best-effort closes
the replacement so an unused writer lock is not silently retained.

## Current review-build workflow

- show the build's local-authority status;
- suggest a local Documents path;
- create a local-authoritative workspace with no connection configured;
- open an existing Liaison workspace;
- create a basic person profile;
- list current person records;
- validate workspace layout and records without deleting files.

The desktop layer does not construct context services or repositories, own canonical schemas, calculate relationship priority, configure providers, or write files outside application-service ports. Successful native commands return typed `CommandResult` envelopes. Failures return `ApplicationError` values with a stable code, display message, recovery guidance, and private diagnostic details. The interface displays the message and recovery guidance only.

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
