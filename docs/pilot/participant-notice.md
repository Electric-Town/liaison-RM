# Participant notice

- Record: participant notice
- Gate evidence for: `FG-B0-PILOT-001`
- Record state: **open — draft text prepared; not completed, not issued**
- Last updated: 2026-07-22

This is the information given to every person whose data the pilot would touch, before any of their real data is entered. Bracketed fields must be filled from the completed records; the notice must not be issued while any source record is open. Before B0, this remains a draft. After B0 and the pilot technical-denial evidence, the exact completed content artifact is included in the independent review before issuance. Its canonical UTF-8 content artifact receives a `reviewed_notice_content_sha256`; issuance metadata is stored separately. Only an issued content artifact whose SHA-256 equals that reviewed digest qualifies; a content change reopens review. Issuing evidence is recorded at the end.

## Draft notice text

> ## We are piloting a tool for event catering — here is what it means for your data
>
> **What is happening.** [Controller organisation] is running a pilot of Liaison RM, locally installed software for organising workplace events, from [start date] to [end date]. It helps us know, before ordering catering, whose dietary information we have, whose we do not, and what the caterer needs to be told — without giving the caterer a list of names.
>
> **What we would record about you.** Your name, work contact details, team or site, event attendance, and — only if you choose to share them — your dietary needs and any accessibility needs for events, stored as a short practical instruction such as "no nuts; separate preparation". We do not record diagnoses or medical detail, and an empty field is treated as "unknown", never as "no restriction".
>
> **What we will never use it for.** No performance, productivity, or attendance monitoring; no ranking or scoring of people; no management decisions about you; no automated messages. These limits are recorded pilot commitments.
>
> **Where your data lives and who can access it.** Liaison itself requires no vendor account or Electric Town hosted service. Your records are files on [storage location], controlled by [controller organisation]. The actual processors, managed-device backup, cloud backup, or connected services used for this pilot are [completed processor/storage/backup record: name each service and purpose, or state none]. The authorised workspace operators are [completed operator record: names/roles]. B0 role presets do not create confidentiality from a person who controls the same unlocked operating-system account or workspace files; the controller limits that technical access with [completed access-control record: device encryption, OS-account controls, physical controls, backup controls, and any other measures]. Caterers receive grouped instructions without names.
>
> **Why we are allowed to process it.** Our legal basis is [Article 6 basis]. Because dietary and accessibility information can reveal health or belief, we rely on [Article 9 condition] for it.
>
> **Sharing your dietary needs is voluntary.** If you say nothing, we record "unknown" and the default catering provision applies to you. You can also explicitly decline, and that is simply recorded as "declined" with no consequences.
>
> **How long we keep it.** Until [retention end point], after which pilot data is deleted and the deletion is verified. Details are in the pilot retention plan, available on request.
>
> **Your rights.** You can ask to see your record, correct it, have it deleted, restrict or object to its use, or withdraw from the pilot at any time by contacting [pilot contact]. Withdrawal has no consequences for you. You can also complain to [supervisory authority].
>
> **Questions.** Contact [pilot contact / privacy contact].

## Completion checklist

| Item | State |
|---|---|
| All bracketed fields filled from completed records | Open |
| Processor, storage, backup, connected-service, operator, and access-control statements match the completed source records | Open |
| Local OS-account and file-access limitation plus the controller's actual controls are disclosed | Open |
| Text reviewed against the adopted purpose and limits | Open |
| Accessibility of the notice itself (format, language, screen-reader use) confirmed | Open |
| Exact completed draft version included in the independent review scope | Open |
| `reviewed_notice_content_sha256` recorded in the independent review | Open |
| Independent review concluded `authorise` with every condition resolved | Open |

## Issuing evidence

| Field | Value | State |
|---|---|---|
| Issued to | — | Open |
| Date issued | — | Open |
| Method (email, meeting, printed copy) | — | Open |
| Where a copy of the issued version is kept | — | Open |
| Reviewed notice content SHA-256 | — | Open |
| Issued notice content SHA-256 | — | Open |
| Issued digest equals reviewed digest | — | Open |
