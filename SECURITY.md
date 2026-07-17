# Security policy

Liaison RM stores information that can be personal, confidential, operationally sensitive, or regulated. Report vulnerabilities responsibly and do not include real relationship, dietary, access, email, calendar, credential, or workspace data in a public report.

## Reporting

For an exploitable vulnerability, contact the maintainers through GitHub Security Advisories when that facility is enabled for the repository. Until then, contact an Electric Town maintainer privately and request a private advisory thread. Do not open a public issue containing exploitation steps or personal data.

A useful report includes:

- affected version or commit;
- build profile and operating system;
- preconditions and required permissions;
- minimal reproduction using synthetic data;
- impact on confidentiality, integrity, availability, consent, local authority, or recovery;
- whether Airgap, Connected-local, provider, plugin, local API, or MCP behaviour is involved;
- suggested mitigation when known.

## Project response

Maintainers will acknowledge a report, establish a private coordination channel, reproduce the issue, assess affected releases, and publish a fix and advisory when users can act safely. Timelines depend on severity and reproducibility; maintainers will not claim a deadline before confirming the issue.

## Security invariants

Contributions must preserve these invariants:

- canonical records remain under user-controlled storage;
- Airgap builds contain no network clients or listeners;
- remote connections require explicit, inspectable grants;
- provider and plugin capabilities are denied by default;
- secrets are stored outside canonical records and logs;
- destructive operations are previewable and auditable;
- encrypted transport does not imply unrestricted field access;
- indexes and caches do not become an untracked second source of truth;
- real personal data is never used in tests, examples, screenshots, or support bundles.

## Supported versions

Before the first tagged release, only the current default branch receives fixes. Version support policy will be added before the first stable release.
