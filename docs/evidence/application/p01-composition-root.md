# P01 application composition-root evidence

Date: 2026-07-18
Status: local review complete; exact-head remote matrices pending

## Claim boundary

The reviewed P01 stack has one `LiaisonApplication` composition root used by the CLI and native Tauri adapter. It returns typed command results, structured application errors, and opaque workspace session identifiers after open or initialisation.

The current session binds workspace identity and repository access. This evidence does not prove writer authority, recovery, key availability, projection health, Airgap isolation, an installed replacement application, accessibility conformance, B0, or release readiness.

## Behaviours exercised

- CLI and Tauri construct no People, Workspace, or Markdown services directly.
- Workspace initialise/open returns a session identifier; later commands use it.
- Initial Person creation with an email remains revision 1.
- A malformed or semantically invalid sibling appears as a Health finding while healthy People remain visible.
- Duplicate Person identities are reported and omitted from ambiguous Directory results.
- Workspace creation refuses existing non-empty folders, filesystem roots, and dot/parent aliases without echoing the rejected path.
- Invalid CLI validation emits its report and exits 6.
- Human Health output includes severity, path, message, recovery, and correlation identifier.
- Human and JSON application failures retain a versioned stable code, recovery guidance, safe details, and correlation identifier.
- Rejected email input and private diagnostic details are not rendered.
- Create/open errors are retryable in the current interface, and invalid tolerant opens announce that Health needs review.
- Base and dark semantic tokens have an automated 4.5:1 text-contrast floor for the exercised roles.
- The review build says `connected-local`, `no connection configured`, and `not yet release-proven`; it does not claim Airgap proof.

## Reproducible checks

```bash
cargo fmt --all --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
python3 scripts/check_repository.py
python3 scripts/check_spec.py
python3 scripts/check_architecture.py
python3 scripts/check_desktop_shell.py
node --check apps/desktop/ui/app.js
python3 scripts/test_desktop_ui.py
```

Browser interaction evidence uses a deterministic fake of the typed native bridge. It proves interface contract handling, focus movement, mobile reflow, and zero external requests in that fixture. It does not prove native persistence or installed-artifact network absence.

The CLI and native adapter also consume `spec/fixtures/application-parity.json`. That fixture normalizes the generated correlation identifier to its UUID contract while requiring identical version, error fields, malformed-Health finding, and initial revision semantics.
