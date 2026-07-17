# Liaison RM interaction prototype

This directory contains the reviewable interaction prototype used to establish information architecture before the production Tauri application is implemented.

The prototype is deliberately static. It demonstrates navigation, profile tabs, event dietary readiness, network graph and semantic table modes, feature gates, keyboard interaction, responsive layout, reduced-motion behaviour, and local-authority messaging. It does not claim production accessibility, privacy, security, or platform conformance.

Open [`liaison-rm-review.html`](liaison-rm-review.html) in a browser to exercise the interaction model.

## Desktop screens

### Today

![Today dashboard](screens/dashboard.svg)

### People and profile

![People directory and profile](screens/people.svg)

### Event dietary readiness

![Event dietary readiness](screens/events.svg)

### Relationship network

![Relationship network](screens/network.svg)

### Settings and feature gates

![Settings and feature gates](screens/settings.svg)

## Mobile screen

![Mobile dashboard](screens/mobile-dashboard.svg)

## Review focus

Reviewers should check:

- whether the next action is clear without relying on colour;
- whether the interface supports interruption recovery and low-capacity review;
- whether sensitive dietary information is separated from operational catering output;
- whether graph information has an equivalent semantic table;
- whether settings explain local, Airgap, and Connected-local behaviour accurately;
- whether the layout remains usable at 390 CSS pixels and with text expansion;
- whether wording avoids guilt, gamification, sales language, and unsupported claims.

## Running the prototype tests

```bash
python -m pip install playwright==1.57.0
python -m playwright install chromium
python scripts/test_prototype.py
```

For a system Chromium installation:

```bash
CHROMIUM_PATH=/usr/bin/chromium python scripts/test_prototype.py
```

The SVG screens are deterministic outputs of `scripts/generate_prototype_screens.py`. The browser test checks routes, focus, labels, tabs, event coverage, graph/table equivalence, feature gates, mobile navigation, and horizontal overflow. The images are reviewer evidence, not production conformance evidence.
