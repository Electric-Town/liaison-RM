# Developer Experience (DX) Evidence & Scorecard Governance

This directory contains exact-HEAD benchmark receipts, onboarding timing evidence, and governed scorecards for Liaison RM.

## Privacy & Integrity Policy

1. **Zero Contributor Telemetry**: All DX scorecards, benchmark timings, and validation receipts are generated locally at explicit execution time. Nothing is sent to remote endpoints.
2. **No Person Ranking**: DX metrics measure build times, command discovery, error catalog completeness, and documentation accuracy. They never measure developer activity, productivity, or human worth.
3. **Exact HEAD Binding**: Every receipt records the exact git commit SHA, OS environment, and exact validation command invoked.

## Canonical Command Entry Points

- `cargo xtask p04 doctor` — Contributor environment and toolchain health check.
- `cargo xtask p04 quick` — Prepared/offline fast verification (target: <= 2 minutes).
- `cargo xtask p04 verify` — Full contract, architecture, and workspace test suite.
- `cargo xtask p04 qualify` — Exact-HEAD qualification suite.
- `cargo xtask p04 scorecard` — Render current DX scorecard against targets.
- `cargo xtask p04 rehearse-upgrade` — Schema and API upgrade rehearsal.

## DX Dimensions & Benchmarks

| Dimension | Target | Evidence Artifact |
|---|---:|---|
| Getting Started | 9/10 | Prepared/offline Quick <= 2 min receipt |
| API/CLI/SDK | 9/10 | Command help snapshots & JSON contracts |
| Error Messages | 9/10 | Exhaustive error catalog & test traces |
| Documentation | 9/10 | Executable doc tests & link validation |
| Upgrade Path | 9/10 | Compatibility fixtures & rehearsal receipt |
| Dev Environment | 9/10 | Cross-platform doctor receipts |
| Community | 9/10 | Privacy-scrubbed GitHub issue templates |
| DX Measurement | 9/10 | Governed `dx-scorecard.v1.json` |
