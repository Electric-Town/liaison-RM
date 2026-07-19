# Data controller and accountable operators record

- Record: data controller
- Gate evidence for: `FG-B0-PILOT-001`
- Record state: **open — required decisions are not made**
- Last updated: 2026-07-19

This record names who is legally accountable for the pilot's personal data and who operates the pilot workspace day to day. Both must be recorded before any other record can be meaningfully completed, because purpose, legal basis, notice, and incident duties all attach to the controller.

## Architectural starting position

Liaison RM is local-authoritative software. In a pilot there is no Electric Town hosted service, account, or telemetry receiving pilot data; canonical records live on storage selected and controlled by the pilot workspace owner. The expected arrangement is therefore:

- the organisation hosting the pilot is the **data controller**;
- Liaison RM maintainers supply software only and are **not** a processor, unless the pilot separately engages them to handle data;
- any storage location, backup target, or device used for the workspace belongs to the controller's estate and choices.

The controller must confirm or correct this analysis for their actual arrangement; a deviation (for example, a maintainer operating the workspace on the organisation's behalf) changes the records that follow.

## Decisions to record

### Controller identity

| Field | Value | State |
|---|---|---|
| Pilot identifier | — | Open |
| Controller organisation (legal name) | — | Open |
| Controller contact for this pilot | — | Open |
| Data protection officer or privacy contact, if one exists | — | Open |
| Jurisdiction and applicable data-protection law | — | Open |
| Competent supervisory authority | — | Open |

This record set uses GDPR concepts (Articles 6, 9, 13, 33, and 35) as its structure. If the controller's applicable law differs, the independent review must confirm the record set still covers the local obligations.

### Joint control and processors

| Field | Value | State |
|---|---|---|
| Is any other party a joint controller? | — | Open |
| Are any processors engaged for pilot data (for example, managed device backup)? | — | Open |
| If yes: processor agreements in place? | — | Open |

### Accountable operators

B0 assumes one trusted local workspace owner. The pilot must name that person and any further operators, because access to the workspace is access to the data.

| Role | Name | Responsibilities | State |
|---|---|---|---|
| Workspace owner | — | Creates and holds the workspace; controls storage location and any backups | Open |
| Additional operator(s), if any | — | Must be listed individually with their access purpose | Open |
| Incident coordinator | — | First contact in the incident plan | Open |

Rules for operators:

- every operator is named here before receiving workspace access;
- operator access is for the recorded pilot purpose only;
- the operator list is part of the participant notice's "who can see this" answer;
- removing an operator is recorded with a date.

## Completion

This record is complete when every table above has a value or an explicit "not applicable" with a reason, and the section below is filled in.

| Field | Value |
|---|---|
| Recorded by | — |
| Date | — |
| Controller confirmation (name, role) | — |
