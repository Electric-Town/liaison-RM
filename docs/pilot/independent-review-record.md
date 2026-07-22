# Independent legal and privacy review record

- Record: independent review
- Gate evidence for: `FG-B0-PILOT-001`
- Record state: **open — no review has taken place**
- Last updated: 2026-07-22

`FG-B0-PILOT-001` requires an independent legal and privacy review that is dated and scoped. After B0, any unresolved review condition keeps the gate blocked and the `real-workplace-data` capability denied. This file is that review's record. It is empty by design: nothing in it may be written by the people who prepared the other records, by the accountable operators, or by a repository coding agent. Preparation work — including every document in this directory — is input to the review, not a substitute for it.

This review may be prepared in draft, but it must not record a final conclusion before exact-artifact B0 acceptance and the post-B0 pilot technical-denial evidence exist. Those artifacts are part of the final review scope.

## Independence requirement

The reviewer must be able to say no. That means the reviewer:

- is not an accountable operator of the pilot and did not author the governance records;
- has recognisable competence in data-protection law or privacy practice for the recorded jurisdiction;
- states any relationship to the controller that a reader would want to know when weighing the review.

| Field | Value | State |
|---|---|---|
| Reviewer name | — | Open |
| Role and qualification | — | Open |
| Relationship to the controller | — | Open |
| Independence statement | — | Open |

## Scope

The review covers, at minimum, every record in this directory at an exact commit, plus the DPIA if one was required.

| Field | Value | State |
|---|---|---|
| Repository commit reviewed | — | Open |
| Exact-artifact B0 acceptance receipt | — | Open |
| Post-B0 pilot technical-denial artifact | — | Open |
| Records reviewed (list, with any exclusions and why) | — | Open |
| Reviewed participant-notice content SHA-256 | — | Open |
| Applicable law confirmed as the record set assumes? | — | Open |
| Date(s) of review | — | Open |

## Findings

Free-form findings, written by the reviewer:

—

## Conditions

Every condition the reviewer sets is tracked here. Before B0, the PILOT task, requirement, and gate remain deferred. Once pilot work begins after B0, any condition whose state is not `resolved` keeps `FG-B0-PILOT-001` blocked and the `real-workplace-data` capability denied — this is the gate's rule, not a discretionary preference.

| # | Condition | Set on | State | Resolution evidence |
|---|---|---|---|---|
| — | — | — | — | — |

## Conclusion

| Field | Value | State |
|---|---|---|
| Conclusion (authorise / authorise with conditions / do not authorise / defer) | — | Open |
| Signed (name, date) | — | Open |

## After the review

Only a signed final conclusion of exactly `authorise` with every condition `resolved` completes this governance record. `Authorise with conditions`, `do not authorise`, defer, an unresolved condition, or an unsigned conclusion keeps real data denied. Even an affirmative completed governance record does not yet authorise the pilot:

1. update the checklist in [README.md](README.md);
2. issue the exact participant-notice content artifact reviewed here, record `issued_notice_content_sha256`, and prove it equals the recorded `reviewed_notice_content_sha256`; any mismatch or content change reopens this review;
3. update the feature-gate evidence for `FG-B0-PILOT-001` with the exact governance commits, notice-issuance evidence, and technical-control artifact identities;
4. complete `T-B0-PILOT` and `FG-B0-PILOT-001` through the machine-owned lifecycle;
5. only after both the task and gate are `complete` may real workplace data enter the pilot workspace. Drafted, current, or merely activated states remain denied.

Any later material change — new data class, new operator, purpose change, incident, or scope growth — reopens this record, returns `T-B0-PILOT` and `LRM-EV-012` to current, returns `FG-B0-PILOT-001` to blocked, and returns the `real-workplace-data` capability to denied until the changed scope is affirmatively reviewed and requalified.
