# Incident response plan

- Record: incident response
- Gate evidence for: `FG-B0-PILOT-001`
- Record state: **open — structure prepared; roles and thresholds not decided**
- Last updated: 2026-07-19

This plan covers personal-data incidents affecting pilot data. It is written for a local-authoritative deployment: there is no hosted vendor to detect or contain an incident, so the accountable operators are the detection and response capability, and the plan must work at the speed of one small team.

## What counts as an incident

- loss or theft of a device holding the workspace or a backup;
- unauthorised access to the workspace files (person, malware, or misconfigured share);
- a brief or export disclosing more than the least-disclosure content, or reaching the wrong recipient;
- dietary or accessibility information disclosed outside the recorded operator list;
- accidental destruction or corruption of pilot data without a recoverable copy;
- discovery that data was used outside the recorded purpose limits.

A near miss (for example, a brief generated with names present but caught before sending) is recorded and reviewed the same way, without the notification steps.

## Roles

| Role | Name | Duty | State |
|---|---|---|---|
| Incident coordinator | — | Receives reports, runs this plan, keeps the log | Open |
| Controller decision-maker | — | Decides notification obligations | Open |
| Technical responder | — | Contains, preserves evidence, assesses scope | Open |

Anyone in the pilot can report a suspected incident to the coordinator; reporting is a duty, not a fault admission.

## Response steps

1. **Contain.** Stop the exposure: revoke the share, isolate or remote-wipe the device, halt brief delivery, suspend the affected operator access.
2. **Preserve.** Copy the workspace audit information and relevant files before remediation changes them.
3. **Assess.** Establish what data, whose, how many people, and the realistic risk to them. Dietary and accessibility content is special-category data: exposure beyond the operator list is presumptively serious.
4. **Decide notifications.** The controller decision-maker assesses, against the applicable law recorded in the data-controller record, whether the supervisory authority must be notified (under the GDPR, within 72 hours of awareness unless the breach is unlikely to result in risk) and whether affected participants must be told (required where the risk to them is high; for this pilot's data classes, the default expectation is to tell affected participants).
5. **Notify.** Use the supervisory authority recorded in the data-controller record and the participant contact route from the notice.
6. **Recover.** Restore accurate data from a verified copy where destruction occurred; correct the failure cause.
7. **Review.** Within a fixed period, record cause, effect, decisions, and changes. If the incident reveals a product defect, open a repository issue with synthetic reproduction data only — never with pilot data.

## Incident log

Kept at a location the coordinator records here, outside the pilot workspace itself, containing per incident: date and time of detection, reporter, description, data affected, decisions with their times (including the 72-hour clock start), notifications made, and review outcome. The log holds the minimum personal data needed to describe the incident.

| Field | Value | State |
|---|---|---|
| Log location | — | Open |
| Review period after each incident | — | Open |

## Standing safeguards to confirm before the pilot

| Safeguard | State |
|---|---|
| Device encryption on every machine holding the workspace or backups | Open |
| Workspace storage location excluded from generic sync or shared folders | Open |
| Operator list matches the data-controller record | Open |
| A tested, current recovery copy exists so deletion or corruption is survivable | Open |

## Completion

| Field | Value |
|---|---|
| Recorded by | — |
| Date | — |
| Controller confirmation (name, role) | — |
