# Experience Bounded Context & Application Anti-Corruption Boundary

This directory defines the **Experience bounded context** for the Liaison RM frontend and UI applications.

## Bounded Context Ownership

- **Context Name**: Experience
- **Primary Responsibility**: Local user interaction, active route management, transient input draft state, accessible announcements, error disclosure, and visual design rendering.
- **Ubiquitous Language**:
  - **Workspace**: A user-owned directory containing canonical Markdown/YAML records and local sqlite/search projections.
  - **Person**: A canonical record representing an individual in the user's relationship memory.
  - **Readiness**: Purpose-specific completeness state (e.g. dietary or workplace accessibility readiness).
  - **Review Queue**: Reason-only attention queue ordering records needing user review, without subjective human-worth scores.
  - **Golden Slice**: Request-bearing status and session canary traversing Application, Tauri IPC, and Experience.

## Anti-Corruption Boundary Rules

1. **Domain Isolation**: Domain entities, invariants, and business rules belong strictly to Rust crates in `contexts/` and `crates/liaison-application`. Experience does not duplicate business rules.
2. **DTO Compatibility**: All IPC payloads traversing the Tauri bridge deserialize through typed DTO definitions in `apps/desktop/src/app/generated/`. Hand-written ad-hoc DTO types are prohibited.
3. **Local Authority**: No remote network calls, tracking pixels, or telemetry listeners may be imported into the Experience context.
4. **State Seams**:
   - Application state (WorkspaceSession, transaction boundaries) → Owned by Rust Application.
   - Transient state (route selection, draft input, modal visibility) → Owned by Experience UI.
   - System appearance (light/dark/high-contrast) → Resolved by Experience from OS preferences.
