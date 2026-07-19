#!/usr/bin/env python3
"""Validate the candidate semantic-token registry's contrast obligations.

The registry is candidate evidence for the future T-B0-P03D consultation and
the P04 token work: token names are normative per the accepted review
decisions, while values remain provisional until measured on the exact P04
build. This checker computes WCAG 2.x contrast ratios for every declared pair
in every theme and fails when a required pair misses its minimum.

A built-in self-test first reproduces the review's rejected dark pair
(#17231F on #514819 at about 1.77:1) so the arithmetic is anchored to the
same measurement that produced the rejection.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REGISTRY = ROOT / "docs" / "evidence" / "design" / "semantic-tokens.candidate.json"

REJECTED_CONTENT = "#17231F"
REJECTED_BACKGROUND = "#514819"


def channel(value: int) -> float:
    scaled = value / 255.0
    if scaled <= 0.04045:
        return scaled / 12.92
    return ((scaled + 0.055) / 1.055) ** 2.4


def relative_luminance(hex_colour: str) -> float:
    text = hex_colour.lstrip("#")
    if len(text) != 6 or any(character not in "0123456789abcdefABCDEF" for character in text):
        raise ValueError(f"not a six-digit hex colour: {hex_colour}")
    red, green, blue = (int(text[index : index + 2], 16) for index in (0, 2, 4))
    return 0.2126 * channel(red) + 0.7152 * channel(green) + 0.0722 * channel(blue)


def contrast(first: str, second: str) -> float:
    lighter = max(relative_luminance(first), relative_luminance(second))
    darker = min(relative_luminance(first), relative_luminance(second))
    return (lighter + 0.05) / (darker + 0.05)


def self_test(errors: list[str]) -> float:
    measured = contrast(REJECTED_CONTENT, REJECTED_BACKGROUND)
    if not 1.6 <= measured <= 1.9:
        errors.append(
            "self-test failed: the rejected review pair should measure about 1.77:1, "
            f"got {measured:.2f}:1"
        )
    return measured


def main() -> int:
    errors: list[str] = []
    rejected_pair = self_test(errors)

    try:
        registry = json.loads(REGISTRY.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        print(f"Design-token check failed: cannot read registry: {error}")
        return 1

    themes = registry.get("themes", {})
    pairs = registry.get("pairs", [])
    required_tokens = registry.get("required_tokens", [])

    for theme_name, tokens in themes.items():
        for token in required_tokens:
            if token not in tokens:
                errors.append(f"{theme_name}: missing required token {token!r}")

    token_names: set[str] = set()
    for tokens in themes.values():
        token_names.update(tokens)
    for theme_name, tokens in themes.items():
        for name in sorted(token_names - set(tokens)):
            errors.append(f"{theme_name}: token {name!r} is not defined in this theme")

    print(f"self-test: rejected review pair measures {rejected_pair:.2f}:1 (rejected, as expected)")
    print()
    for theme_name in themes:
        tokens = themes[theme_name]
        print(f"[{theme_name}]")
        for pair in pairs:
            foreground = pair["foreground"]
            background = pair["background"]
            minimum = float(pair["minimum"])
            required = bool(pair.get("required", True))
            if foreground not in tokens or background not in tokens:
                continue
            ratio = contrast(tokens[foreground], tokens[background])
            status = "pass" if ratio >= minimum else ("FAIL" if required else "flag")
            print(
                f"  {foreground} on {background}: {ratio:.2f}:1 "
                f"(minimum {minimum:.1f}:1, {status})"
            )
            if required and ratio < minimum:
                errors.append(
                    f"{theme_name}: {foreground} on {background} measures {ratio:.2f}:1, "
                    f"below the required {minimum:.1f}:1"
                )
        print()

    if errors:
        print("Design-token check failed:")
        for error in errors:
            print(f"- {error}")
        return 1
    print("Design-token check passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
