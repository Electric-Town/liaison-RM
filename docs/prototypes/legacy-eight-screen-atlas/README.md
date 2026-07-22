# Legacy eight-screen atlas prototype

Status: preserved design exploration; not implementation evidence and not shipped in the desktop bundle.

These modules preserve the July 2026 Today, Events, readiness, Directory, profile, Health, and Settings exploration. They contain presentation-only sample state and must never be imported by `apps/desktop/ui`, Tauri, CLI, API, or another product surface.

The prototype predates the accepted dependency boundary. `T-B0-P11` owns the complete B0 desktop workflow and remains blocked in `spec/traceability-ownership.json`. Until that task is complete, the production desktop exposes only capabilities backed by current Application services: Workspace, People, and read-only Health.

Reuse requires replacing every sample value with owned synthetic fixtures, wiring each action through the normal Application service boundary, satisfying the owning phase dependencies, and producing exact-head browser, native, accessibility, recovery, and packaging evidence.
