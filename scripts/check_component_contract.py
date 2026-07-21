#!/usr/bin/env python3
"""Validate the P04 accessible component-system contract."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
COMPONENTS = ROOT / "apps/desktop/ui-react/src/components"
CSS = COMPONENTS / "components.css"
INDEX = COMPONENTS / "index.ts"
APP = ROOT / "apps/desktop/ui-react/src/App.tsx"

REQUIRED_COMPONENTS = (
    "Button.tsx",
    "Field.tsx",
    "RouteNavigation.tsx",
    "StatusBanner.tsx",
    "Surface.tsx",
)
REQUIRED_EXPORTS = ("Button", "Field", "RouteNavigation", "StatusBanner", "Surface")
REQUIRED_CSS_CLASSES = (
    ".lrm-button",
    ".lrm-field",
    ".lrm-route-navigation",
    ".lrm-status",
    ".lrm-surface",
)


def fail(message: str) -> None:
    print(f"component-contract: {message}")
    raise SystemExit(1)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing required file: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def main() -> int:
    for filename in REQUIRED_COMPONENTS:
        read(COMPONENTS / filename)

    css = read(CSS)
    exports = read(INDEX)
    app = read(APP)

    for name in REQUIRED_EXPORTS:
        if not re.search(rf"\b{name}\b", exports):
            fail(f"component is not exported from the public boundary: {name}")
        if not re.search(rf"\b{name}\b", app):
            fail(f"foundation shell does not exercise the component: {name}")

    for class_name in REQUIRED_CSS_CLASSES:
        if class_name not in css:
            fail(f"missing component style contract: {class_name}")

    if re.search(r"#[0-9a-fA-F]{3,8}\b|\brgba?\(|\bhsla?\(", css):
        fail("component CSS contains raw colors instead of semantic tokens")
    if "rotate(" in css or "skew(" in css:
        fail("operational components may not rotate or skew")
    if "border-radius" in css:
        fail("operational component geometry must remain stable and unornamented")
    if "min-height: 3rem" not in css:
        fail("primary interactive components do not preserve the 48-pixel target floor")
    if "@media (forced-colors: active)" not in css:
        fail("forced-colours component evidence is missing")
    if "@media (prefers-reduced-motion: reduce)" not in css:
        fail("reduced-motion component behavior is missing")

    button = read(COMPONENTS / "Button.tsx")
    if 'aria-busy={busy || undefined}' not in button or "disabled={unavailable}" not in button:
        fail("Button does not expose and enforce its busy state")

    field = read(COMPONENTS / "Field.tsx")
    for fragment in ("htmlFor={inputId}", "aria-describedby={describedBy}", "aria-invalid"):
        if fragment not in field:
            fail(f"Field accessibility contract is missing: {fragment}")

    navigation = read(COMPONENTS / "RouteNavigation.tsx")
    if 'aria-current={item.id === current ? "page" : undefined}' not in navigation:
        fail("RouteNavigation does not expose the current route")
    if "<button" not in navigation:
        fail("RouteNavigation is not keyboard-operable by native controls")

    status = read(COMPONENTS / "StatusBanner.tsx")
    for fragment in ("useId", 'role={assertive ? "alert" : "status"}', 'aria-atomic="true"'):
        if fragment not in status:
            fail(f"StatusBanner accessibility contract is missing: {fragment}")

    print(
        "component-contract: passed — exported semantic primitives, token-only CSS, "
        "stable geometry, 48-pixel controls, recovery semantics, and forced-colours support"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
