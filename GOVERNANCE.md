# Governance

Liaison RM is maintained by Electric Town as an open-source public-interest project. Governance exists to protect user control, review quality, contributor access, and the integrity of sensitive relationship data.

## Roles

### Contributors

Anyone who submits issues, documentation, tests, designs, translations, research, code, or review feedback.

### Maintainers

Contributors trusted to triage work, review pull requests, manage releases, and uphold project standards. Maintainer decisions must be traceable to repository evidence rather than private convention.

### Domain stewards

Maintainers or nominated contributors accountable for the language, invariants, public interfaces, and knowledge health of a bounded context. Stewardship does not confer unilateral product authority.

### Release managers

Maintainers responsible for assembling release evidence, checking feature gates, producing signed artifacts, and verifying that the Airgap and Connected-local profiles match their declared capabilities.

## Decision model

Routine changes are decided through pull-request review. Decisions with durable architectural, privacy, security, compatibility, licensing, or product-scope consequences require an architecture decision record.

A decision record must state:

- context and problem;
- constraints and evidence;
- alternatives considered;
- decision and consequences;
- reversal or migration conditions;
- affected bounded contexts, requirements, knowledge articles, and feature gates.

Maintainers should seek rough consensus. When consensus is not possible, the responsible maintainer records the decision and the unresolved objection in the decision record. Sensitive-data protection, local authority, accessibility, and licence obligations are hard constraints rather than voting preferences.

## Protected principles

The following changes require an explicit public decision record and approval from at least two maintainers:

- introducing a mandatory hosted service or account;
- weakening open-file canonical storage;
- adding telemetry or remote diagnostics;
- changing the licence;
- broadening access-log use into productivity or performance scoring;
- weakening Airgap network exclusion;
- allowing providers or plugins to bypass grants;
- changing encryption, key ownership, or recovery semantics;
- removing a supported export or migration route;
- making a graph or drag interaction the only way to complete a task.

## Maintainer appointment and removal

A maintainer candidate should demonstrate sustained, accurate contributions and respectful review. Appointment is recorded in a pull request updating `MAINTAINERS.md` with the scope of responsibility.

Maintainers may step down at any time. Removal for inactivity, repeated policy violations, undisclosed conflicts, or unsafe handling of personal data requires a documented decision by two other maintainers. Private security or conduct details may be redacted, but the governance action and accountable decision makers remain public.

## Conflicts of interest

Contributors disclose financial, employment, vendor, or personal interests that could reasonably affect a decision. A conflicted maintainer may provide evidence and technical review but should not be the sole approver.

## Releases

A release requires:

- completed release checklist and feature-gate evidence;
- changelog and migration notes;
- reproducible build instructions;
- platform and profile matrix;
- signed checksums and provenance where supported;
- unresolved risk disclosure;
- knowledge articles for installation, upgrade, backup, and recovery changes.

No release may claim compliance, security assurance, platform support, or test coverage beyond the recorded evidence.
