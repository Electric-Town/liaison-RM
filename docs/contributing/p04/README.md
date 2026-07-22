# P04 Contributor Knowledge Hub & Onboarding Guide

Welcome to the **Liaison RM P04 Contributor Hub**. This document provides a fast, progressive onboarding path, canonical command catalog, and codebase ownership navigator.

## Fast Onboarding Path

Liaison RM targets a **<= 2 minute prepared/offline Quick green** build time and **<= 10 minute native status launch**.

### Prerequisites
- **Rust**: 1.75+ (edition 2024)
- **Node.js**: v20+
- **Python**: 3.11+

### Quick Start (1 Command)
To run the fast contributor validation suite:

```bash
cargo xtask p04 quick
```

To run environment diagnostics:

```bash
cargo xtask p04 doctor
```

## Canonical Command Reference

| Command | Purpose | Target Execution Time |
|---|---|---|
| `cargo xtask p04 doctor` | Diagnostic check of toolchains and workspace health | <= 5s |
| `cargo xtask p04 quick` | Fast unit & contract validation suite | <= 120s |
| `cargo xtask p04 verify` | Full workspace verification (spec, architecture, repository) | <= 300s |
| `cargo xtask p04 qualify` | Exact-HEAD qualification matrix | <= 600s |
| `cargo xtask p04 rehearse-upgrade` | Schema & API upgrade compatibility rehearsal | <= 30s |
| `cargo xtask p04 where <concept>` | Codebase ownership & ubiquitous language search | <= 2s |
| `cargo xtask p04 scorecard` | Display exact-HEAD DX scorecard metrics | <= 5s |
| `cargo xtask p04 scenario` | Execute synthetic test scenarios and generate receipts | <= 60s |

## Codebase Ownership Navigator

Use `cargo xtask p04 where <concept>` to find responsible contexts and specifications:

```bash
cargo xtask p04 where workspace
cargo xtask p04 where person
cargo xtask p04 where readiness
```

## DDD Bounded Context Map

- **`contexts/workspace`**: Workspace identity, manifest management, writer authority, and session locks.
- **`contexts/people`**: Canonical person records, contact details, partial dates, and dietary profile data.
- **`contexts/profiles`**: Purpose-bound readiness calculation, custom fields, topic packs.
- **`contexts/events`**: Event cohorts, participation reconciliation, catering brief generation.
- **`contexts/review-attention`**: Reason-only review queue ordering (no closeness scores).
- **`crates/liaison-application`**: Shared orchestration application service layer and error envelope.
- **`apps/desktop`**: Tauri desktop interface and Experience bounded context.

## Error Handling & Diagnostics Guidelines

Every error emitted by Liaison RM must follow the shared application error envelope:
1. **Problem**: Human-readable problem summary.
2. **Safe Cause**: Diagnostic explanation safe for display without leaking local filesystem paths or personal data.
3. **Recovery Action**: Actionable guidance telling the user how to recover.
4. **Offline Help**: Offline documentation reference code.
