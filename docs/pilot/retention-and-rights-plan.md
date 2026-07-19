# Retention and rights plan

- Record: retention and rights
- Gate evidence for: `FG-B0-PILOT-001` (retention schedule)
- Record state: **open — structure prepared; periods and owners not decided**
- Last updated: 2026-07-19

This record fixes how long each class of pilot data exists, how it is deleted, and how participants exercise their rights against a local workspace. Retention is measured against the recorded purpose: when the pilot no longer needs a record, the record goes.

## Retention schedule

Proposed default: all pilot personal data is deleted within a short, fixed period after the pilot end date, keeping only de-identified evaluation findings. The controller sets the actual periods.

| Data class | Retention period | Trigger for deletion | Owner | State |
|---|---|---|---|---|
| Person profiles (names, contact, memberships) | — | Pilot end + period | — | Open |
| Dietary coverage states and operational instructions | — | Pilot end + period, or participant withdrawal | — | Open |
| Accessibility and sensory needs | — | Same as dietary | — | Open |
| Events, cohorts, attendance | — | Pilot end + period | — | Open |
| Generated catering briefs (internal copies) | — | Pilot end + period | — | Open |
| Copies delivered to caterers | — | Confirm caterer deletion or rely on named-free content; decide | — | Open |
| Workspace backups and checkpoints, if any | — | Must not outlive the classes they contain | — | Open |
| Audit and evaluation notes | — | De-identify before any longer retention | — | Open |
| Consent and governance records (this directory, signed notices) | Longer retention is legitimate as accountability evidence; decide the period | Accountability | — | Open |

Rules:

- withdrawal deletes or de-identifies the participant's dietary and accessibility data ahead of schedule; the deletion is recorded;
- backups are enumerated before the pilot starts, because a forgotten backup is a retention failure;
- deletion at pilot end is verified by a second named person and recorded below.

## How deletion works in this product

Canonical records are readable files in the workspace. Deletion during the pilot means removing the person's files and any derived projections, then confirming no copy remains in checkpoints or backups. The current alpha does not automate retention; during the pilot, deletion is a manual, verified procedure executed by the workspace owner. This plan must not be read as a claim of automated retention enforcement.

## Rights procedures

Participants exercise rights through the pilot contact recorded in the notice. Because the data is local files under the controller's control, every request is fulfillable without a vendor.

| Right | Procedure | Target response time | State |
|---|---|---|---|
| Access | Export the person's records from the workspace and provide them in a readable format | — | Open |
| Rectification | Correct the field; the coverage state changes accordingly (for example, back to `provided`) | — | Open |
| Erasure | Manual verified deletion as above | — | Open |
| Restriction / objection | Mark the person excluded from processing; they leave readiness denominators as an explicit excluded state | — | Open |
| Withdrawal from the pilot | Erasure or de-identification plus exclusion from further events | — | Open |
| Complaint route | Named supervisory authority in the notice | — | Open |

Requests and their fulfilment are logged (date, request, action, who acted) without recording more personal data than the log needs.

## Pilot-end deletion evidence

| Field | Value | State |
|---|---|---|
| Deletion executed by / date | — | Open |
| Verified by / date | — | Open |
| Backups and checkpoints checked | — | Open |
| Retained items and their justification | — | Open |

## Completion

| Field | Value |
|---|---|
| Recorded by | — |
| Date | — |
| Controller confirmation (name, role) | — |
