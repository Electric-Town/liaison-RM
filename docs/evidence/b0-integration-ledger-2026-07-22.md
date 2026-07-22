# B0 integration ledger

**Opened:** 2026-07-22

**Integration branch:** `vscode/b0-integration-reset-20260722`

**Starting source:** `c5a7c13e91b42d0ee50d8de4aa2da10f278a4413`

**Default-branch comparison:** `origin/main` at `49ee419e30f2d71524dd6fa15badf1ec4b8d0e27`

**Machine authority:** `spec/traceability-ownership.json`

This is the operational receipt for converging the preserved B0 work. It is not
a second implementation plan and cannot advance a requirement, task, UAT case,
or gate. Machine-owned traceability remains authoritative.

## Product state at opening

| Evidence class | Exact state |
|---|---|
| Accepted | `T-B0-P00`, `T-B0-P01`, and `T-B0-P02` |
| Current | `T-B0-P03` |
| Blocked | `T-B0-P03D`, `T-B0-P04` through `T-B0-P11`, B0 acceptance, and every A0 task |
| Candidate source | Audited Workspace/People/Health shell and corrected Option C People flow at the starting source |
| Installed | An unqualified `/Applications/Liaison RM.app`; this is not accepted or release evidence |
| Released | No supported application and no GitHub release |

`T-B0-P03O` is not present in current machine authority. The divergent DX
branch that introduced it is an input only. Representative observation remains
useful later usability evidence, but it does not replace P03 recovery evidence
or block P03D/P04.

## Preserved inputs

| Input | Disposition |
|---|---|
| `c5a7c13` audit/People/design source | Integration base |
| `4e3d95f`, `2eb311c` P03 operation truth | Review and transplant through the P03 lane |
| Dirty P03 operation and application-boundary worktrees | Preserve; reproduce only reviewed changes in an isolated lane |
| Event domain `6e12b1a` and adapter `8af5fc6` | P05 candidate inputs; no application or desktop claims |
| DX/P03O `73f6afb` and its dirty worktree | Never merge wholesale; harvest only independently justified changes |
| Historical `3499a6e` universal review artifact | Historical evidence only; a later recovery finding reopened P03 |
| `vB0` tag | Preserve as inaccurate history; never use as a release receipt |

No input branch or worktree is deleted, reset, cleaned, or silently superseded
until its useful changes are either integrated or explicitly rejected here.

## Exclusive execution lanes

| Lane | Exclusive ownership | Integration condition |
|---|---|---|
| Integration authority | Branch topology, `spec/*`, product status, changelog, receipts | Single writer; imports reviewed commits only |
| P03 operations | Workspace operation model, vault adapter, application results, CLI parity, focused tests | Complete fault matrix and exact validation |
| P03D design | `DESIGN.md`, Experience boundary, design ADRs/contracts/evidence | Candidate-only until P03 is accepted |
| P05 domain | People revision/dietary, organisations/memberships, Events/readiness/least disclosure | Pure domain/port work; no UI/application claim |
| Pilot governance | `docs/pilot/*` | Human controller and independent reviewer decisions remain mandatory |
| Artifact qualification | Packaging and exact-source install evidence | Serialized after a final candidate and custody release |

Feature lanes do not edit task status, central release claims, or another
lane's paths. Every handoff must name its base, exact commit, changed paths,
focused validation, remaining gaps, and custody release.

## Evidence tiers

1. **Lane evidence:** formatter plus owning-package tests.
2. **Integration evidence:** workspace check and dependency-complete focused
   suites once per imported commit set.
3. **Phase evidence:** full repository/platform workflows only when a phase
   candidate is ready.
4. **Installed evidence:** one exact-source application, one canonical install,
   native persistence/recovery/accessibility/offline checks, and matching
   executable hashes.

Repeating a broader tier cannot compensate for a missing human outcome or an
unimplemented invariant.

## Installed application custody at opening

The live application observed after the earlier reconciliation is:

- path: `/Applications/Liaison RM.app`;
- version: `0.1.0-alpha.1`;
- architecture: arm64 only;
- executable SHA-256:
  `5f08deb40c9fee62515cb09495738083f7a335877b7f8df6ae3827a53d3dd266`;
- strict signing verification: failed;
- source provenance: unknown;
- relationship to current local bundles: executable hash does not match.

It remains in place for forensic comparison. It must not be launched as release
proof, redistributed, or overwritten until the exact-source artifact lane owns
replacement and rollback evidence.

## Integration log

| Date | Source | Result | Validation | Status effect |
|---|---|---|---|---|
| 2026-07-22 | `c5a7c13` | Opened the single B0 integration lane | Clean tree; `cargo fmt --all --check`; `cargo check --workspace --all-targets --all-features --locked` | None; P03 remains current |
