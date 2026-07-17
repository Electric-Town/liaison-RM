# Liaison RM interaction prototype

This directory contains the reviewable interaction prototype used to establish information architecture before the production Tauri application is implemented.

The prototype is deliberately static. It demonstrates navigation, profile tabs, event dietary readiness, network graph and semantic table modes, feature gates, keyboard interaction, responsive layout, reduced-motion behaviour, and local-authority messaging. It does not claim production accessibility, privacy, security, or platform conformance.

## Desktop screens

### Today

![Today dashboard](screens/home.webp)

### People and profile

![People directory and profile](screens/people.webp)

### Event dietary readiness

![Event dietary readiness](screens/events.webp)

### Relationship network

![Relationship network](screens/network.webp)

### Settings and feature gates

![Settings and feature gates](screens/settings.webp)

## Mobile screen

![Mobile dashboard](screens/mobile-home.webp)

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

The test generates fresh screenshots in a temporary directory and checks the committed screen dimensions. The committed images are reviewer evidence, not golden pixel snapshots; font and rasterization differences make cross-platform pixel hashes unsuitable as a merge gate.
