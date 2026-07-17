#!/usr/bin/env python3
"""Validate the committed provider WIT contract without mutating the workspace."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
WIT = ROOT / "interfaces/wit/liaison-provider.wit"

REQUIRED = {
    "package": "package electric-town:liaison-provider@0.1.0;",
    "object-store interface": "interface object-store",
    "provider world": "world provider",
    "immutable write": "put-immutable",
    "read": "get:",
    "metadata": "head:",
    "list": "list:",
    "guarded delete": "delete-if-permitted",
    "guarded manifest": "replace-manifest-if-revision",
    "world export": "export object-store;",
}


def main() -> int:
    errors: list[str] = []
    if not WIT.is_file():
        print(f"WIT contract is missing: {WIT.relative_to(ROOT)}")
        return 1
    text = WIT.read_text(encoding="utf-8")
    for label, fragment in REQUIRED.items():
        if fragment not in text:
            errors.append(f"missing {label}: {fragment}")
    if text.count("{") != text.count("}"):
        errors.append("unbalanced braces")
    package_count = len(re.findall(r"(?m)^package\s+", text))
    if package_count != 1:
        errors.append(f"expected one package declaration, found {package_count}")
    if "http" in text.lower() or "google" in text.lower() or "amazon" in text.lower():
        errors.append("provider-specific transport language leaked into the WIT contract")
    if errors:
        print("Provider WIT validation failed:")
        for error in errors:
            print(f"- {error}")
        return 1
    print("Provider WIT validation passed: object-store@1 surface is present")
    return 0


if __name__ == "__main__":
    sys.exit(main())
