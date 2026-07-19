# T-B0-P04 amended implementation plan

Status: approved by the P03D plan design review; execution begins only after P03 and P03D are merged with exact-head evidence
Owning contexts: Experience and Application
Primary gate: `FG-R2-001`
Design authority: `DESIGN.md` 1.0.0

## Outcome

Replace the disposable vanilla webview with a typed React/TypeScript client while preserving every working Workspace, People, Health, authority, and recovery behavior. Establish the semantic design system and automated evidence ratchet that P05–P11 can extend without route-specific styling or duplicate domain logic.

## Repository shape

```text
apps/desktop/
├── package.json
├── vite.config.ts
├── tsconfig.json
├── src/
│   ├── app/
│   │   ├── AppShell.tsx
│   │   ├── routes.tsx
│   │   ├── operation-state.ts
│   │   └── generated/
│   ├── design-system/
│   │   ├── tokens.ts
│   │   ├── components/
│   │   └── contracts/
│   ├── features/
│   │   ├── overview/
│   │   ├── directory/
│   │   ├── events/
│   │   ├── health/
│   │   └── settings/
│   ├── i18n/
│   └── test/
├── ui-legacy/                 # retained only until parity evidence passes
└── src-tauri/
```

Feature folders may contain presentation and route composition. Domain rules stay in Rust.

## Delivery slices

### P04.1 — Toolchain and generated contract

- pin Node and package-manager versions;
- add React, TypeScript, Vite, testing-library, axe integration, and no runtime network dependency;
- generate or compile-check TypeScript request/result/error types from the application contract;
- add a CI drift check;
- preserve current Tauri capabilities and CSP.

Acceptance: one typed `app_status` call renders through React with no handwritten duplicate DTO.

### P04.2 — Semantic foundation

- implement `design/semantic-tokens.v1.json` as CSS custom properties and typed token names;
- implement system/light/dark/high-contrast resolution;
- implement Atkinson, Source Serif, and Plex Mono roles with existing local assets;
- create the versioned component contract from `DESIGN.md`;
- add Storybook-free local component fixtures rendered by the test app to avoid another production bundle.

Acceptance: automated contrast, focus, token completeness, forced-colour, reduced-motion, en-XA, and no-external-request checks pass.

### P04.3 — Shell parity

- migrate top bar, Sections navigation, status region, error boundary, Workspace create/open/switch, People create/list/profile reachability, and Health;
- preserve native-operation serialization and stale-result rejection;
- preserve default-workspace resume behavior from the maintained shell;
- run old and new parity tests against the same fake command adapter.

Acceptance: every existing browser and Rust workflow passes against React; no behavior disappears when `ui-legacy` is removed.

### P04.4 — Operation and recovery presentation

- implement the binding table in `DESIGN.md`;
- expose receipts and safe operation identifiers;
- implement persistent committed-recovery and conflict banners;
- prove cancel availability before commit and absence after commit;
- prove focus restoration and restart behavior.

Acceptance: P03 fault fixtures render the correct copy, actions, announcements, and focus behavior.

### P04.5 — Route and component extension seams

- establish route IDs for Overview, Directory, Events, Health, and Settings;
- show only currently available routes;
- add typed extension points for P05–P11 screens without plugin execution;
- add semantic table/summary-detail, Stepper, Drawer, Dialog, ThemePicker, and OperationStatus components.

Acceptance: synthetic fixtures render every required component state in all built-ins and at 320/360 CSS pixels.

### P04.6 — Installed evidence

- build universal macOS and Windows review artifacts;
- verify architecture, checksums, ad-hoc signature for review, zero external requests, theme persistence, rollback, keyboard, VoiceOver smoke test, 400% reflow, and reduced motion;
- record exact head, run IDs, browser version, OS version, assistive technology, and artifact hashes.

Acceptance: P04 evidence is bound to the exact merge candidate. It is not public-release evidence.

## State ownership

| State | Owner |
|---|---|
| active route, drawer/dialog open, unsaved field draft | React presentation |
| canonical records, readiness, operation phase, receipts | Rust application/domain |
| selected built-in appearance | Settings application service |
| resolved system light/dark mode | presentation from OS preference |
| workspace authority, recovery, key, projection state | Workspace Session/application |
| localized visible copy | locale catalogue |

## Testing matrix

Every P04 PR runs:

```text
npm ci --ignore-scripts
npm run typecheck
npm run lint
npm run test
npm run test:axe
npm run test:locales
npm run build
node --check <generated bridge fixtures>
python3 scripts/check_design_tokens.py
python3 scripts/check_design_contract.py
python3 scripts/check_desktop_shell.py
cargo fmt --all --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

The final candidate runs Linux, macOS, and Windows matrices plus the installed universal Mac evidence matrix.

## Dependency and bundle controls

- lock every JavaScript dependency and record licence/provenance evidence;
- no CDN, analytics, telemetry, remote font, remote icon, remote texture, or runtime package download;
- no dependency with install-time network code in CI;
- fail on dependency drift, vulnerable production dependencies above the repository policy threshold, or incompatible licence;
- retain the existing local-only CSP.

## Migration and rollback

- Keep `ui-legacy` until parity evidence passes.
- The React shell reads no new canonical format.
- A build-time switch permits one review release to select the legacy shell if the React bundle fails before P04 acceptance.
- Remove the switch and legacy shell in the final P04 commit only after exact-head parity passes.
- Reverting P04 restores the old shell without a workspace migration.

## Review gates

P04 cannot merge with:

- handwritten duplicate DTOs;
- user-facing strings outside catalogues;
- raw colours outside the token adapter;
- route-local operation-state calculations;
- a hidden or disabled route placeholder;
- remote assets;
- unresolved keyboard, focus, reflow, contrast, reduced-motion, or en-XA failures;
- a missing exact-head installed review artifact.
