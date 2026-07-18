# Public project site

The Liaison RM website explains the product boundary, shows default-branch evidence and routes contributors to the repository. It is a project site, not a release download service.

## Source and deployment

The static source lives under `site/`. [`.github/workflows/pages.yml`](../.github/workflows/pages.yml) validates pull requests and deploys the directory after a qualifying push to `main`.

The production URL is:

```text
https://electric-town.github.io/liaison-RM/
```

The site uses no JavaScript, external font, analytics endpoint, tracking pixel or third-party runtime asset. A deploy copies the committed bytes into the GitHub Pages artifact.

## Local preview

```bash
python3 -m http.server --directory site 4173
```

Open `http://127.0.0.1:4173/`. The production repository path is `/liaison-RM/`; local fragment links and relative assets work from the preview root.

## Validation

Run:

```bash
python3 scripts/check_public_site.py
python3 scripts/check_repository.py
```

The public-site check covers required artifacts, local asset resolution, title and description length, one-page heading structure, skip navigation, image alternatives, JSON-LD, Open Graph, canonical URL, `hreflang`, sitemap, manifest scope, CSS focus treatment, reduced motion and Japanese line-breaking readiness.

Browser review still checks what static validation cannot prove:

- 390, 768 and 1440 CSS-pixel layouts;
- 200% zoom and reflow;
- keyboard focus order and visible focus;
- screen-reader landmark and heading output;
- long content and 45% text-expansion stress;
- no horizontal overflow;
- no unexpected network request;
- the full 404 recovery path.

## Truth boundary

The hero includes a synthetic product-direction card. It is labelled as product direction. The desktop image comes from [`docs/evidence/macos/screenshots/desktop-workspace-health.png`](evidence/macos/screenshots/desktop-workspace-health.png) and is labelled as the current alpha.

Every implemented claim must resolve to the default branch, a committed evidence file or an accepted contract. Planned behaviour uses future language. Signed downloads, daily-use readiness, production integrations, multi-writer sharing and compliance claims remain closed until their release evidence exists.

## Language and semantic metadata

`en-IE` is the only published locale. The page language, visible copy, canonical URL, Open Graph locale, JSON-LD `inLanguage`, manifest language and self-referential `hreflang` agree.

Do not publish `ga-IE`, `ja-JP` or `pt-BR` routes from draft catalogues. Each locale first needs the evidence listed in [`docs/evidence/localization/README.md`](evidence/localization/README.md), including a named reviewer and accessibility sampling.

When a reviewed locale is added:

1. create a stable locale path;
2. add self-referential and bidirectional `hreflang` links;
3. keep visible language, metadata and structured data in sync;
4. add the locale URL to `sitemap.xml`;
5. test expansion, line breaking, names, dates and screen-reader text;
6. record the exact source commit and review evidence.

## Public copy

Write for a reader deciding whether to trust, try or contribute to the project.

- Lead with the outcome and the release status.
- Prefer named behaviour over adjectives.
- Put proof beside the claim.
- Answer account, data ownership, scoring, AI disclosure and download objections directly.
- Vary sentence length because the idea needs it, not to imitate a style score.
- Do not add deliberate errors or use an authorship detector as a quality gate.
- Do not use fake scarcity, testimonials, benchmarks or guarantees.

The copy check rejects stock promotional words and em dashes in the public site, README and repository metadata. The project content standard remains authoritative.

## Repository settings

[`docs/repository-metadata.md`](repository-metadata.md) is the source for the GitHub About description, Website field, topics and social card. After a settings change, verify the live About panel and the resolved Pages URL before closing the related issue.

## Rollback

Revert the site commit through an ordinary pull request and let the Pages workflow publish the prior source. If the workflow fails, keep the last known-good deployment live, inspect the failed Actions run and rerun only after the submitted commit passes `scripts/check_public_site.py`.

Disabling Pages removes the public surface and requires repository-administrator intent. Do not use it as the first response to a copy, CSS or asset regression.
