#!/usr/bin/env python3
"""Check the relationship intent domain, adapter, and CLI boundary."""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REQUIRED = [
    "contexts/relationships/Cargo.toml",
    "contexts/relationships/README.md",
    "contexts/relationships/src/lib.rs",
    "adapters/relationship-yaml/Cargo.toml",
    "adapters/relationship-yaml/README.md",
    "adapters/relationship-yaml/src/lib.rs",
    "apps/cli/tests/relationship_workflow.rs",
]


def main() -> int:
    errors: list[str] = []
    for relative in REQUIRED:
        if not (ROOT / relative).is_file():
            errors.append(f"missing relationship baseline file: {relative}")

    cargo = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    for member in [
        '"contexts/relationships"',
        '"adapters/relationship-yaml"',
    ]:
        if member not in cargo:
            errors.append(f"workspace is missing member {member}")

    domain = (ROOT / "contexts/relationships/src/lib.rs").read_text(encoding="utf-8")
    for term in [
        "RelationshipProfile",
        "RelationshipTier",
        "ContactCadence",
        "MaintenanceStatus",
        "RelationshipRepository",
        "SaveRelationship",
    ]:
        if term not in domain:
            errors.append(f"Relationships context is missing {term}")
    forbidden = ["relationship_strength", "employee_score", "message_volume"]
    for term in forbidden:
        if term in domain.casefold():
            errors.append(f"Relationships context contains forbidden scoring term {term}")

    adapter = (ROOT / "adapters/relationship-yaml/src/lib.rs").read_text(encoding="utf-8")
    for term in [
        "liaison-relationship",
        "schema_version",
        "expected_revision",
        "persist_noclobber",
        "serde(flatten)",
    ]:
        if term not in adapter:
            errors.append(f"Relationship adapter is missing {term}")

    cli = (ROOT / "apps/cli/src/main.rs").read_text(encoding="utf-8")
    for term in [
        "RelationshipCommand",
        "Set(RelationshipSetArgs)",
        "expected_revision",
        "RelationshipYamlStore",
    ]:
        if term not in cli:
            errors.append(f"CLI is missing relationship boundary {term}")

    if errors:
        print("Relationship persistence check failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print("Relationship persistence check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
