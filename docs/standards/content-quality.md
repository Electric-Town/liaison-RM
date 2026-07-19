# Content quality standard

## Objective

Repository text must help a specific reader make a decision, complete a task, understand a constraint, or verify a result. Origin is not a quality criterion. Human and automated drafts are reviewed against the same evidence and editorial standards.

The project does not use AI detectors as an authorship gate. Statistical classifiers are too unreliable for individual attribution and encourage cosmetic evasion rather than better work.

The maintainer-supplied forensic and localization guidance was reviewed and bounded in [`docs/evidence/content/editorial-source-review-2026-07-18.md`](../evidence/content/editorial-source-review-2026-07-18.md). Observable editorial defects are useful review inputs. Statistical authorship claims, phonological manipulation, deliberate errors, and detector evasion are not.

## Required qualities

Good repository content is:

- specific about the actor, task, condition, and consequence;
- consistent with the project’s ubiquitous language;
- traceable to source, test, observation, decision, or stated assumption;
- honest about uncertainty and incomplete validation;
- proportionate in length to the decision or task;
- structured for scanning without reducing every thought to fragments;
- free of fabricated quotes, benchmarks, users, tests, citations, or compliance claims;
- edited by an accountable contributor.
- written in direct language that names the responsible actor and safe next action;
- reviewed in the rendered product when it is user-facing, including pseudolocale and long-content states.

## Common failure patterns

Review and rewrite:

- generic openings that restate the heading;
- conclusions that repeat the introduction without a decision or next action;
- promotional adjectives in place of measurable behaviour;
- repeated “not X, but Y” constructions;
- excessive em dashes, parenthetical asides, or slogan-like fragments;
- uniform sentence length and repeated paragraph cadence;
- long lists whose entries are not prioritised or connected;
- abstract nouns that hide the responsible actor;
- false balance where the evidence supports a clear decision;
- “seamless”, “robust”, “powerful”, “intuitive”, “future-proof”, or “enterprise-ready” without a defined test;
- comments that paraphrase the next line of code;
- citations that do not support the claim made;
- warnings so vague that a user cannot recover.
- unsupported percentages or research claims copied from a secondary editorial source;
- language changed to satisfy an authorship classifier instead of the reader's task.

## Technical writing test

A reviewer should be able to answer:

1. Who needs this information?
2. What task or decision does it support?
3. What evidence or constraint supports the statement?
4. What exact behaviour follows?
5. How is the result verified?
6. What fails, and how does the reader recover?
7. What remains uncertain?

If those answers are absent, add them or remove the prose.

## Automated drafting

Automated assistance may be used for outlining, transformation, examples, tests, and review. The submitting contributor remains responsible for:

- source verification;
- licence and attribution review;
- domain-language accuracy;
- privacy and secret inspection;
- running claimed checks;
- removing repetition and generic filler;
- disclosing uncertainty rather than inventing confidence.

Do not insert spelling mistakes, awkward phrasing, false personal anecdotes, or inconsistent formatting to make generated text appear human. Detector evasion is not editing.

## Examples and fixtures

Use clearly synthetic names and organisations. Avoid examples that resemble real colleagues, customers, family members, addresses, dietary conditions, access events, or communications. Secret-like strings use recognised placeholders and cannot pass as working credentials.

## Error and safety copy

Error messages state:

- what operation failed;
- what data was and was not changed;
- the likely cause when known;
- the next safe action;
- where diagnostic evidence is stored;
- whether retry can duplicate work.

Permission and disclosure prompts name the provider, endpoint, purpose, fields or data classes, operation, schedule, expiry, and revocation path.

## Review evidence

The pull request records whether content was:

- newly written;
- adapted from an existing project source;
- mechanically generated from schemas or code;
- substantially drafted with automated assistance.

This record is for provenance and review routing, not punishment. The quality gate remains the resulting work and its evidence.
