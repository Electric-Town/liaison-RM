#!/usr/bin/env python3
"""Validate the canonical P03D design contract and P04 plan bindings."""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DESIGN = ROOT / "DESIGN.md"
TOKENS = ROOT / "design" / "semantic-tokens.v1.json"
PLAN = ROOT / "docs" / "product" / "p04-amended-plan.md"
CONSULTATION = ROOT / "docs" / "evidence" / "design" / "p03d-design-consultation-2026-07-19.md"
REVIEW = ROOT / "docs" / "evidence" / "design" / "p03d-plan-design-review-2026-07-19.md"

REQUIRED_DESIGN_TERMS = [
    "Editorial Ledger",
    "Overview",
    "Directory",
    "Events",
    "Health",
    "Settings",
    "Details → Cohort → Attendees → Readiness → Brief",
    "aria-current=\"step\"",
    "content-on-highlight",
    "current-step-content",
    "400% zoom",
    "en-XA",
    "OperationStatus",
    "Unknown — needs attention",
    "React / TypeScript presentation",
]

REQUIRED_TOKENS = {
    "canvas",
    "surface",
    "surface-subtle",
    "content",
    "content-muted",
    "border",
    "border-strong",
    "action",
    "content-on-action",
    "focus",
    "highlight",
    "content-on-highlight",
    "current-step",
    "current-step-content",
    "surface-information",
    "success",
    "surface-success",
    "warning",
    "surface-warning",
    "danger",
    "surface-danger",
}


def main() -> int:
    errors: list[str] = []
    for path in [DESIGN, TOKENS, PLAN, CONSULTATION, REVIEW]:
        if not path.is_file():
            errors.append(f"missing design artifact: {path.relative_to(ROOT)}")

    design = DESIGN.read_text(encoding="utf-8") if DESIGN.is_file() else ""
    for term in REQUIRED_DESIGN_TERMS:
        if term not in design:
            errors.append(f"DESIGN.md is missing required contract term: {term}")

    if "http://" in design or "https://" in design:
        errors.append("DESIGN.md must not introduce a runtime remote-asset URL")

    if TOKENS.is_file():
        try:
            registry = json.loads(TOKENS.read_text(encoding="utf-8"))
        except json.JSONDecodeError as error:
            errors.append(f"semantic token registry is invalid JSON: {error}")
        else:
            if registry.get("registry") != "liaison-semantic-tokens/1":
                errors.append("semantic token registry has the wrong version identifier")
            themes = registry.get("themes")
            if not isinstance(themes, dict):
                errors.append("semantic token registry has no theme map")
                themes = {}
            if set(themes) != {"light", "dark", "high-contrast"}:
                errors.append("semantic token registry must contain light, dark, and high-contrast palettes")
            for name, values in themes.items():
                if not isinstance(values, dict):
                    errors.append(f"theme {name} is not an object")
                    continue
                missing = sorted(REQUIRED_TOKENS - values.keys())
                if missing:
                    errors.append(f"theme {name} is missing tokens: {missing}")
            system_rule = str(registry.get("system_theme", ""))
            if "resolution rule" not in system_rule:
                errors.append("semantic token registry must define system as a resolution rule")

    plan = PLAN.read_text(encoding="utf-8") if PLAN.is_file() else ""
    for required in [
        "generate or compile-check",
        "ui-legacy",
        "npm run test:axe",
        "universal macOS",
        "zero external requests",
    ]:
        if required not in plan:
            errors.append(f"P04 amended plan is missing: {required}")

    if errors:
        print("Design-contract check failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print("Design-contract check passed: canonical direction, token registry, consultation, review, and P04 plan agree")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
