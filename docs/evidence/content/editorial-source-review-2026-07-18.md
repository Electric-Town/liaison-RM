# Editorial source review: machine-signal and localization guidance

- Reviewed: 2026-07-18
- Scope: repository prose, product copy, errors, KCS articles, PR descriptions, release notes, and localized UI text
- Source class: maintainer-supplied editorial documents

## Adopted review lenses

The supplied material identified observable writing defects that can reduce trust regardless of who drafted the text. Liaison reviews for:

- generic openings that delay the actual point;
- repeated binary “not X, but Y” scaffolding;
- uniform sentence and paragraph cadence;
- repeated groups of three used as decoration;
- lists compressed into prose;
- conclusions with no new decision, evidence, or next action;
- excessive em dashes and parenthetical detours;
- abstract promotional verbs, adjectives, and nouns where a named action or test is available;
- unsupported statistics, regulations, standards, users, quotations, or compliance claims;
- logical drift between the problem, decision, implementation, and evidence;
- noun-heavy error text that hides who can do what next.

The localization material also supports expansion testing, Unicode normalization, locale-aware formatting, script-specific line breaking, stable semantic keys, and accountable fluent review. Irish, Japanese, and Brazilian Portuguese fixtures are not production translations without that review.

## Explicitly rejected uses

The project does not:

- classify authorship from perplexity, letter distribution, phonology, or similar statistical signals;
- alter consonant or stress patterns to evade a detector;
- add typos, anecdotes, dialect, or irregular syntax to impersonate a person;
- treat polished grammar or a particular word as proof of automated authorship;
- replace a fluent language reviewer with a stylometric score;
- claim the source documents' percentages or scientific conclusions without checking their primary sources.

These techniques do not establish correctness, usability, or accountable authorship. They can also damage accessibility and localization.

## Enforcement

`docs/standards/content-quality.md` is the normative rule. PR review checks actor, action, evidence, consequence, recovery, terminology, cadence, repetition, and claim support. Repository automation catches a deliberately narrow set of high-signal defects; human review remains responsible for meaning and context.

For user-facing work, copy review occurs in the compiled interface with long content and pseudolocale fixtures. A sentence that passes repository lint can still fail because it is vague, clipped, poorly announced, culturally wrong, or hard to recover from.
