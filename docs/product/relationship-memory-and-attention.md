# Relationship memory, profile topics, and review attention

Status: proposed normative product and domain contract  
Version: 0.4  
Owning contexts: Identity and Profiles, Relationships, Review and Attention, Customization

## Product boundary

Liaison RM is a relationship memory and attention system. It may apply CRM-grade organization to personal, family, professional, community, executive-assistant, and workplace relationships. It must not treat a person as a sales lead or infer human value from communication volume.

The canonical model separates four concepts:

| Concept | Owner | Meaning |
|---|---|---|
| Relationship intent | Relationships | User-authored importance, desired cadence, boundaries, and desired future state |
| Relationship evidence | Interactions and Commitments | Recorded interactions, activities, notes, commitments, dates, and resources |
| Maintenance status | Review and Attention | Explainable state relative to the relationship's own cadence and suppressions |
| Profile readiness | Identity and Profiles | Purpose-specific coverage of required, current, and verified information |

No aggregate or interface may collapse these concepts into a universal relationship-strength value.

## Ubiquitous language

- **Person:** a human profile with stable identity.
- **Organization:** a company, school, nonprofit, public body, club, community, vendor, client, or other organized body.
- **Group:** a static, dynamic, snapshot, household, or team collection.
- **Membership:** a dated edge between a person and an organization or group.
- **Relationship:** a first-class directional or reciprocal edge between entities.
- **Topic Pack:** a versioned set of typed profile fields and display guidance.
- **Field Definition:** a stable field ID, type, classification, validation rules, and purpose requirements.
- **Field Value:** a value plus its epistemic state, provenance, visibility, and review metadata.
- **Purpose:** a named task for which profile readiness is calculated.
- **Review Reason:** a factual explanation for surfacing a person.
- **Review Policy:** versioned rules that select, suppress, group, and optionally order review candidates.
- **Review Priority:** a transparent queue-ordering value. It is not relationship strength.

## Entity boundaries

Liaison must model the following entities separately:

- Person;
- Organization;
- Group;
- Household;
- Location;
- Event;
- Resource.

“Contact” is a UI collection over people and organizations. It is not a domain aggregate.

A Person file must not become a conflict-prone container for every note, event, interaction, membership, resource, or relationship. Stable IDs link records across bounded contexts.

## Topic Pack activation

A Topic Pack may be enabled by:

1. workspace default;
2. profile template;
3. organization or group policy;
4. one person;
5. a plugin contribution;
6. a temporary purpose such as an event, trip, or meeting brief.

Activation order is additive. An explicit exclusion on a person suppresses a lower-scope activation. The resolver must return both the effective pack set and the source of each activation.

Built-in packs may include:

- identity and communication;
- important dates;
- food and hospitality;
- travel;
- favorites and gifts;
- family and household;
- pets;
- professional context;
- interests and life context;
- events and hosting;
- executive-assistant briefing;
- accessibility and sensory preferences;
- resources.

A person does not need every pack. Product copy must not imply that an unused pack makes a profile deficient.

## Field Definition contract

Every field has a stable ID independent from its translated label and screen position.

```yaml
id: travel.seat_preference
label: Seat preference
type: enum
options:
  - window
  - aisle
  - middle
  - no_preference
classification: private
required_for:
  - executive_travel_brief
stale_after: P2Y
```

Supported field types include:

- short text;
- long text;
- Markdown;
- date, partial date, and recurring date;
- enum and multi-select;
- boolean;
- number and measurement;
- address and location;
- entity reference and entity-reference list;
- resource reference and resource-reference list;
- sealed value;
- calculated read-only value;
- plugin-supplied value.

Incompatible changes to a field type require a new field ID or an explicit migration. Changing a label, help text, layout, or locale does not change field identity.

## Field Value and information state

An absent value is not equivalent to “none.” Each Field Value has one of these states:

- known;
- verified;
- unverified;
- unknown;
- not applicable;
- declined;
- stale;
- conflicting;
- needs clarification;
- derived.

The value may also record:

- source record or connector;
- author;
- captured at;
- verified at;
- review after;
- visibility and classification;
- confidence;
- purpose limitation;
- change-history reference.

The system must preserve `declined`, `not applicable`, and `unknown` as distinct states. An import must not replace an explicit state with an empty string.

Sensitive field definitions, including dietary, accessibility, travel-document, emergency, and private-assessment data, require classification and purpose controls. Sealed values must not be written in plaintext to ordinary Markdown front matter.

## Purpose-specific readiness

Liaison must not expose one universal profile-completeness percentage.

A Purpose Definition names:

- required field IDs;
- acceptable information states;
- maximum staleness;
- optional conditional requirements;
- least-disclosure output fields;
- policy version.

Examples:

```text
Basic contact readiness       Ready
Meeting briefing              Missing current role
Travel briefing               Missing seat and hotel preferences
Event catering                Four attendees need clarification
Birthday preparation          Gift ideas not recorded
CardDAV export                Ready
```

A readiness result contains:

- purpose ID and version;
- entity or cohort ID;
- required field count;
- satisfied field count;
- unresolved field IDs and reasons;
- stale or conflicting field IDs;
- calculation timestamp;
- no inferred value beyond the stated field rules.

## Relationship intent

Relationship intent is manually authored. It may include:

- relationship type;
- editable tier;
- manual importance;
- desired cadence;
- preferred communication channel;
- desired future state;
- reason the relationship matters;
- boundaries;
- paused-until date;
- do-not-contact state;
- review date.

Default tier labels may be `core`, `active`, `warm`, `loose`, `paused`, and `archive`. A workspace may rename or replace them. Importers may propose a tier but must not silently assign importance from interaction volume.

Optional private assessment snapshots may record closeness, trust, familiarity, reciprocity, reliability, shared context, current relevance, emotional effort, or desired change. These are user-authored observations, not objective facts. They are excluded from sharing, AI context, exports, and provider publication unless an explicit field-level grant includes them.

## Maintenance status

Maintenance status uses factual labels:

- on track;
- due soon;
- overdue relative to configured cadence;
- open commitment;
- important date approaching;
- required context stale;
- no cadence configured;
- paused;
- do not contact;
- archived.

A displayed status must include its reason. For example:

> Quarterly cadence; last meaningful interaction was 112 days ago; one commitment remains open.

The product must not display:

> Relationship strength: 42%.

A relationship without a cadence may be healthy and must not be marked overdue.

## Review Policy modes

A Review Policy has one of three modes:

1. **reason-only:** no numeric value; candidates are grouped by reasons;
2. **tiered:** low, normal, high, or urgent queue order;
3. **weighted:** transparent 0–100 queue-ordering value.

Reason-only is the default for personal workspaces.

Weighted policies are versioned. Every component and contribution is visible. Example:

```text
Review priority =
  30% cadence status
+ 20% manual importance
+ 20% open commitments
+ 10% upcoming dates
+ 10% stale required context
+ 10% manual boost
```

The engine must return an explanation for each non-zero component. A UI, connector, plugin, or AI adapter may not recalculate priority independently.

## Hard suppressions

These states override selection and ordering:

- archived;
- do not contact;
- relationship ended;
- paused until a future date;
- snoozed until a future date;
- excluded from the active policy.

A suppression result records its source and expiry. A suppressed person must not appear in normal daily review totals.

## Review guardrails

The Review and Attention context must never:

- claim an objective measure of human worth;
- infer trust, affection, or closeness from messages or meetings;
- rank employees or infer performance;
- create social-credit or popularity scores;
- shame a user for overdue contact;
- send a message automatically;
- expose private assessments through a shared workspace;
- assume every relationship needs regular contact;
- hide the reason a person is surfaced;
- create a permanent red backlog by default.

## Daily review

A daily review is capacity-bounded. Policy settings include:

- maximum people shown;
- intended time budget;
- low, normal, or high social capacity;
- personal, professional, or mixed scope;
- quiet days;
- whether aggregate overdue counts are hidden;
- snooze options;
- wording profile.

Each card contains only the current reasons, last meaningful topic when available, and explicit actions such as open profile, log interaction, complete commitment, snooze, pause, or skip.

Unfinished capture and review state must survive interruption. Returning to the review restores the exact candidate, action state, and draft rather than restarting the queue.

## Monthly review

A monthly review may cover:

- people contacted;
- changed relationship intent;
- unrealistic cadence policies;
- sparse, stale, or conflicting purpose data;
- open commitments;
- upcoming dates;
- possible introductions;
- resources or thank-you notes to send;
- relationships to pause, archive, or reconnect with;
- policy components producing excessive noise.

The completed review is stored as readable Markdown with policy and source versions.

## Filtering and saved views

Combined filters include:

- person, organization, group, or household;
- relationship type and tier;
- maintenance status and review reason;
- interaction type and date;
- next contact due;
- important dates;
- organization, role, department, cost center, and location;
- Topic Pack activation;
- missing, stale, conflicting, or declined fields;
- tags;
- event attendance;
- resource type;
- source connector;
- archived, paused, and do-not-contact state.

Saved views store stable field IDs and policy references, not translated labels.

## Accessibility and executive-function requirements

The design must support:

- interaction capture in approximately 30 seconds for the basic path;
- a visible explanation for every surfaced person;
- preservation of unfinished forms;
- exact-place interruption recovery;
- small review batches;
- reduced-motion and low-stimulation modes;
- optional hiding of irrelevant sections;
- one-action-at-a-time review;
- valid skip, snooze, pause, and archive actions;
- no gamification based on contacts completed;
- searchable history;
- reminders presented as options rather than obligations;
- semantic table equivalents for graphs or radar charts;
- keyboard alternatives to drag-and-drop.

## Context ownership

| Context | Responsibility |
|---|---|
| Identity and Profiles | People, Topic Packs, Field Definitions, Field Values, provenance, templates, readiness inputs |
| Organizations and Groups | Organizations, households, groups, locations, memberships |
| Relationships | Relationship edges, intent, boundaries, private assessment snapshots |
| Interactions and Commitments | Timeline evidence, meaningful-interaction classification, commitments, reminders |
| Events and Calendar | Events, attendance, recurrence, calendar identity |
| Knowledge and Resources | Notes, files, URLs, attachments, backlinks |
| Review and Attention | Cadence, maintenance status, reasons, suppressions, queues, policies, review sessions |
| Customization | Schema contributions, layouts, saved views, plugin contributions |

Cross-context information is consumed through stable IDs, application queries, or published events. No UI or provider owns a duplicate scoring engine.

## Required acceptance cases

1. A phone-imported person with only a name and number appears in a “basic context missing” view without being labelled unimportant.
2. A close friend with no cadence and infrequent contact is not marked overdue.
3. Every review candidate exposes one or more factual reasons.
4. A person may be ready for event catering and not ready for an executive travel brief.
5. An archived or do-not-contact relationship is suppressed regardless of weighted inputs.
6. A low-capacity daily review respects the configured maximum and does not expose a guilt backlog.
7. Private assessment fields do not appear in shared exports or AI context without an explicit grant.
8. A monthly review is saved as Markdown with the policy version used.
9. Stable field IDs survive label and locale changes.
10. Weighted-policy components sum to one and each non-zero contribution is explainable.

## Implementation sequence

1. People, organizations, groups, memberships, and relationship edges.
2. Notes, interactions, commitments, reminders, and important dates.
3. Relationship intent, cadence, suppressions, and reason-only review queues.
4. Topic Packs, typed fields, and explicit information states.
5. Purpose definitions and profile-readiness queues.
6. Resources and organization graph projections.
7. Weighted Review Priority and policy simulation.
8. Plugin-supplied packs and review components.

Weighted review must not precede reason-only review. Users need to inspect and trust the reasons before formulas affect queue order.
