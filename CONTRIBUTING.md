# Contributing to Liaison RM

Liaison RM handles relationship, workplace, family, contact, dietary, calendar, email-metadata, and access-log information. Contributions must be understandable, reversible, and proportionate to the sensitivity of that data.

## Before opening a change

1. Search issues, decisions, knowledge articles, and open pull requests.
2. Identify the bounded context that owns the behaviour.
3. Describe the current user workflow and the consequence of leaving it unchanged.
4. For a new product flow, collect past-behaviour evidence rather than asking whether people like the proposed feature.
5. Open a focused issue or draft pull request when the scope crosses more than one bounded context.

## Repository structure

Each bounded context should expose the following shape where applicable:

```text
contexts/<context>/
├── README.md          Purpose, language, invariants, public interfaces
├── Cargo.toml
└── src/
    ├── domain.rs      Entities, value objects, policies, domain events
    ├── application.rs Use cases and ports
    ├── ports.rs       Outbound interfaces owned by the context
    └── lib.rs
```

Adapters implement ports outside the context. Applications compose contexts and adapters. UI code cannot bypass application services to manipulate storage directly.

## Pull requests

Use the repository pull-request template. A complete description records:

- the problem and evidence;
- the owning bounded context;
- why the chosen design is preferable to alternatives;
- knowledge articles created, updated, or reused;
- user, privacy, security, migration, accessibility, and operator impact;
- tests and manual checks;
- rollback and recovery behaviour;
- changelog classification.

Keep pull requests reviewable. Split governance, architecture, domain implementation, adapters, and provider integrations when they can be reviewed independently.

## KCS-informed knowledge practice

Liaison RM uses a KCS-informed workflow, not a claim of KCS certification.

During the solve loop:

1. Capture the problem in the user’s language.
2. Search before creating a new article.
3. Reuse and improve an existing article when it describes the same issue.
4. Create a new article when the problem or resolution is materially distinct.
5. Link the pull request, decision, tests, and release evidence.

During the evolve loop:

- review duplicated or stale articles;
- promote reusable patterns into standards;
- retire incorrect guidance without erasing its history;
- measure whether articles resolve real contributor or user work.

## Changelog

Update `CHANGELOG.md` under `Unreleased` for:

- new capabilities;
- behaviour changes;
- fixes users or operators can observe;
- schema or migration changes;
- security or privacy changes;
- deprecations and removals;
- contributor workflow changes that affect release reproducibility.

Pure refactors with no observable or contributor-facing effect may record `No changelog entry` in the pull request.

## Testing

Use the narrowest relevant checks and the full workspace checks before requesting review:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
python scripts/check_repository.py
```

User-facing work also requires keyboard, screen-reader, zoom, reduced-motion, error-state, and interruption-recovery evidence as specified in `docs/standards/ux-review.md`.

## Commit style

Use imperative, scoped commits that explain the durable change:

```text
docs: establish KCS-informed contribution workflow
feat(people): add structured dietary requirement value object
fix(vault): preserve unknown front-matter fields during rewrite
```

Do not use generated commit messages that overstate the diff.

## Security reports

Do not open a public issue for an exploitable vulnerability or disclosure of real personal data. Follow `SECURITY.md`.
