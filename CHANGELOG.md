# Changelog

All notable changes to Liaison RM are recorded here. The format follows Keep a Changelog principles and the project uses semantic versioning after the first tagged release.

## Unreleased

### Added

- Repository governance, contribution, knowledge, content-quality, architecture, and user-experience review standards.
- Pull-request evidence requirements and repository policy checks.
- Product, domain, security, platform, sharing, provider-neutral connection, and release specifications.
- Machine-readable requirements, persona UAT cases, feature gates, and implementation tasks.
- Interactive desktop and mobile concept with six committed review screens and browser-level interaction and accessibility smoke tests.
- Relationship-memory contract separating intent, evidence, maintenance status, and purpose-specific readiness.
- Topic Pack, field-state, profile-readiness, and Review Policy examples, schemas, validation, and three review screens.

### Changed

- The repository README defines Liaison RM as a local-authoritative, open-file relationship manager.
- Reason-only review is the personal-workspace default; weighted Review Priority is explicitly queue ordering rather than relationship strength.

### Security

- Documented the prohibition on undeclared network requests, hidden telemetry, secret material in canonical files, and provider or plugin access without an explicit grant.
- Defined separate Airgap and Connected-local build profiles and least-disclosure handling for sensitive relationship and workplace data.
- Private assessments and sensitive Topic Pack values require explicit classification, purpose, and sharing grants.
