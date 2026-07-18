# KCS-informed knowledge practice

## Scope

Liaison RM uses Knowledge-Centered Service principles to keep product, contributor, support, and release knowledge close to the work that creates it. This is a KCS-informed operating standard, not a claim of certification or endorsement.

The repository treats knowledge as a maintained product surface. A fix without an explanation is incomplete because the next person will repeat the investigation.

## Knowledge locations

```text
docs/knowledge/          Task and problem-oriented articles
docs/decisions/          Durable architecture and product decisions
docs/evidence/           Test, provider, accessibility, and release evidence
docs/release/            Release checklists, migration, and recovery notes
docs/standards/          Normative contributor standards
```

Source comments remain appropriate for local implementation constraints. They do not replace task-oriented knowledge.

## Solve Loop

Every behavioural issue or pull request follows this sequence:

1. **Capture** — record the problem in the user’s or contributor’s language, including context, symptoms, impact, and known constraints.
2. **Search** — search articles, issues, decisions, tests, and source before inventing a resolution.
3. **Reuse** — cite the article that guided the work.
4. **Improve** — correct or clarify the article while the new evidence is available.
5. **Create** — add a new article only when the problem, audience, or resolution is materially distinct.
6. **Link** — connect the article to the issue, pull request, decision, tests, requirement, feature gate, and release evidence where applicable.

The pull-request template records the search terms used. This makes missing vocabulary visible.

## Evolve Loop

Maintainers periodically review:

- search terms that return no useful article;
- repeated issues and duplicated articles;
- articles with conflicting or obsolete instructions;
- high-impact workflows without recovery guidance;
- release changes that lack upgrade knowledge;
- provider or platform evidence that is older than the supported version;
- contributor questions that reveal unclear architecture.

Outcomes include consolidating articles, promoting patterns into standards, revising vocabulary, adding automation, or retiring stale guidance.

## Article structure

Each article begins with front matter:

```yaml
---
id: KCS-0001
title: How do I ...?
status: draft | validated | retired
audience: user | operator | contributor | maintainer
contexts: [workspace]
symptoms: []
search_terms: []
last_validated: YYYY-MM-DD
validated_against: []
related_requirements: []
related_gates: []
---
```

Recommended body:

1. Problem or task
2. Environment and preconditions
3. Resolution or procedure
4. Why it works
5. Verification
6. Recovery or rollback
7. Known limitations
8. Related decisions, tests, and articles

Use synthetic examples. Never place real personal data, credentials, internal provider URLs, or private support transcripts in an article.

## Article states

- **Draft** — useful but not yet reproduced or reviewed.
- **Validated** — reproduced against named versions or evidence.
- **Retired** — retained for history but replaced or no longer applicable.

Validation is specific. “Works” is insufficient; record the operating system, release, provider contract, schema version, or command used.

## Pull-request rule

A behavioural pull request must do one of the following:

- create a knowledge article;
- update an article;
- cite a validated article and explain why no change is needed.

A mechanical change may state `No knowledge change` with a reason. Mechanical does not include behaviour, migration, security, accessibility, provider, packaging, or user-facing copy changes.

## Release rule

A release cannot close until installation, upgrade, backup, restore, rollback, known-issue, and recovery knowledge reflects the release. Changed knowledge is part of release evidence.

## Measures

Useful measures include:

- repeated issues resolved by an existing article;
- searches with no useful result;
- time from behaviour change to article validation;
- stale articles discovered before release;
- support or contributor work prevented by automation;
- recovery procedures successfully exercised.

Article count and writing volume are not success measures.
