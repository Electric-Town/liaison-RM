# Liaison RM interaction prototypes

This directory contains reviewable interaction artifacts used to establish information architecture before production interfaces are declared complete.

The prototypes are deliberately static. They demonstrate navigation, profile tabs, event dietary readiness, network graph and semantic table modes, feature gates, reason-only review, purpose-specific profile readiness, keyboard interaction, responsive layout, reduced-motion behaviour, and local-authority messaging. They do not claim production accessibility, privacy, security, or platform conformance.

Open [`liaison-rm-review.html`](liaison-rm-review.html) in a browser to exercise the original application concept.

## Core application screens

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

### Mobile dashboard

![Mobile dashboard](screens/mobile-dashboard.svg)

## Relationship memory and attention screens

### Reason-only daily review

![Reason-only relationship review](screens/review-reasons.svg)

### Purpose-specific profile readiness

![Purpose-specific profile readiness](screens/profile-readiness.svg)

### Low-capacity mobile review

![Low-capacity mobile review](screens/review-mobile.svg)

## Review focus

Reviewers should check:

- whether the next action is clear without relying on colour;
- whether every surfaced person has a factual explanation;
- whether infrequent contact is kept separate from relationship value;
- whether the interface supports interruption recovery and low-capacity review;
- whether unused Topic Packs avoid creating universal completeness pressure;
- whether sensitive dietary information is separated from operational catering output;
- whether graph information has an equivalent semantic table;
- whether settings explain local, Airgap, and Connected-local behaviour accurately;
- whether the layout remains usable at 390 CSS pixels and with text expansion;
- whether wording avoids guilt, gamification, sales language, and unsupported claims.

## Validation

The original interaction concept uses:

```bash
python -m pip install playwright==1.57.0
python -m playwright install chromium
python scripts/test_prototype.py
```

The relationship model and its three additional SVG screens use:

```bash
python -m pip install PyYAML==6.0.2
python scripts/check_relationship_model.py
```

The images are reviewer evidence, not production conformance evidence.
