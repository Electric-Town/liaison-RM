# 0001: Use a Rust shared core and Tauri desktop shell

- Status: proposed
- Date: 2026-07-17
- Contexts: all

## Context and problem

Liaison RM needs native Linux, macOS, and Windows distribution, a first-class CLI, local APIs, provider adapters, cryptographic and file-integrity code, and a desktop UI. Separate implementations would cause domain rules and recovery behaviour to diverge.

## Constraints and evidence

- Domain and application rules must be shared by CLI, desktop, API, MCP, importers, and plugins.
- Airgap and Connected-local require explicit compile-time capability differences.
- The UI needs a mature accessible component and testing ecosystem.
- Canonical-file, provider, and plugin code benefits from Rust’s type and ownership model.

## Alternatives considered

1. Electron/TypeScript application with a separate CLI.
2. Go backend with React desktop wrapper.
3. Pure browser/PWA application.
4. Native UI toolkits for each platform.

Electron and a separate CLI would encourage duplicate rules and a larger desktop runtime. Go is viable but offers a weaker path to WASI component hosting and shared low-level libraries for this design. A PWA cannot be the primary owner of ordinary user-visible canonical files across platforms. Separate native UIs multiply implementation and accessibility work.

## Decision

Use a Cargo workspace for domain, application, adapters, CLI, local services, provider SDK, and Tauri commands. Use Tauri 2 as the desktop shell and React/TypeScript for the interface. The browser/PWA surface, if built, is served by the local process and holds only disposable client state.

## Consequences

- Rust boundaries and dependency rules become repository policy.
- Platform webview differences require cross-platform UI testing.
- Tauri capabilities are scoped by window and build profile.
- Rust and frontend toolchains must be pinned and released together.
- The core remains usable without the desktop shell.

## Reversal conditions

Revisit if a supported platform cannot meet accessibility, packaging, security, or maintainability requirements through Tauri without a platform-specific domain fork. A replacement must preserve application-service contracts and canonical formats.
