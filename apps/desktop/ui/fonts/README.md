# Bundled fonts

Locally bundled subsets implementing the approved Editorial Ledger typography roles. No font is fetched over the network at runtime; the desktop shell loads only these files, preserving the no-external-request boundary.

| File | Family and role | Weights | Source | SHA-256 |
|---|---|---|---|---|
| `atkinson-hyperlegible-next-latin.woff2` | Atkinson Hyperlegible Next — body text, controls, forms, tables, navigation, operational headings | variable 400–700 (latin subset) | Google Fonts `atkinsonhyperlegiblenext/v7` | `1e4cea71d75ec427581d6259fc07148a2e60d60d16cabf4b4f5360487b3f9dc3` |
| `source-serif-4-semibold-latin.woff2` | Source Serif 4 — page titles only | 600 (latin subset) | Google Fonts `sourceserif4/v14` | `d6175042a842343da89c28e9b3105e6f22859c2acdd3d8730f4c45e523b2d42b` |
| `ibm-plex-mono-regular-latin.woff2` | IBM Plex Mono — provenance only: paths, revisions, validation scopes | 400 (latin subset) | Google Fonts `ibmplexmono/v20` | `c36f509c0a8f9f85f29cb44bc8701d8a9e0b14c499e77a884f789ead7093a7ac` |

Licences: all three families use the SIL Open Font License 1.1 — `OFL-atkinson-hyperlegible-next.txt`, `OFL-source-serif-4.txt`, `OFL-ibm-plex-mono.txt` (Source Serif 4's licence file is Adobe's OFL text). OFL is compatible with distributing the fonts inside an AGPL-3.0 application; the fonts remain under their own licence.

Boundaries and open items:

- Latin subsets only; extended-latin, pseudolocale expansion glyph coverage, and any further script coverage are P04 merge evidence items, as are fallback-metric measurements on the exact build.
- System font fallbacks are declared in `styles.css` for every role, so a missing glyph degrades to a legible system face rather than tofu.
- No Japanese bundle is included or claimed for B0.
