#!/usr/bin/env python3
"""Validate the published Workspace manifest schema and version-one fixture."""

from __future__ import annotations

import copy
import json
import sys
from pathlib import Path

import yaml
from jsonschema import Draft202012Validator, FormatChecker

ROOT = Path(__file__).resolve().parents[1]
SCHEMA = ROOT / "schemas" / "workspace-manifest.schema.json"
FIXTURE = ROOT / "spec" / "fixtures" / "workspace-manifest-v1.yaml"
LEGACY_FIXTURE = (
    ROOT / "spec" / "fixtures" / "workspace-manifest-v1-legacy-without-modules.yaml"
)


def main() -> int:
    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    manifest = yaml.safe_load(FIXTURE.read_text(encoding="utf-8"))
    legacy_manifest = yaml.safe_load(LEGACY_FIXTURE.read_text(encoding="utf-8"))
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema, format_checker=FormatChecker())

    errors = sorted(validator.iter_errors(manifest), key=lambda error: list(error.path))
    if errors:
        for error in errors:
            location = ".".join(str(component) for component in error.path) or "<root>"
            print(f"workspace manifest fixture {location}: {error.message}")
        return 1

    invalid_cases = []
    duplicate = copy.deepcopy(manifest)
    duplicate["enabled_modules"] = ["people", "people"]
    invalid_cases.append(("duplicate modules", duplicate))
    missing = copy.deepcopy(manifest)
    missing.pop("enabled_modules")
    invalid_cases.append(("missing modules", missing))
    invalid_identifier = copy.deepcopy(manifest)
    invalid_identifier["enabled_modules"] = ["people\nprivate"]
    invalid_cases.append(("control character", invalid_identifier))
    padded_identifier = copy.deepcopy(manifest)
    padded_identifier["enabled_modules"] = [" people"]
    invalid_cases.append(("padded module identifier", padded_identifier))
    unicode_identifier = copy.deepcopy(manifest)
    unicode_identifier["enabled_modules"] = ["réseau"]
    invalid_cases.append(("non-ASCII module identifier", unicode_identifier))
    missing_people = copy.deepcopy(manifest)
    missing_people["enabled_modules"] = ["events"]
    invalid_cases.append(("missing required people module", missing_people))
    blank_name = copy.deepcopy(manifest)
    blank_name["name"] = " "
    invalid_cases.append(("blank workspace name", blank_name))
    blank_locale = copy.deepcopy(manifest)
    blank_locale["default_locale"] = "\t"
    invalid_cases.append(("blank default locale", blank_locale))
    invalid_workspace_id = copy.deepcopy(manifest)
    invalid_workspace_id["workspace_id"] = "not-a-uuid"
    invalid_cases.append(("workspace identifier", invalid_workspace_id))

    accepted_invalid = [
        label for label, candidate in invalid_cases if validator.is_valid(candidate)
    ]
    if accepted_invalid:
        for label in accepted_invalid:
            print(f"workspace manifest schema accepted invalid case: {label}")
        return 1

    expected_legacy_keys = set(manifest) - {"enabled_modules"}
    if set(legacy_manifest) != expected_legacy_keys or validator.is_valid(legacy_manifest):
        print(
            "legacy workspace manifest fixture must differ only by the missing "
            "enabled_modules field and remain invalid for new writers"
        )
        return 1

    print(
        "Workspace manifest schema passed: strict v1, legacy-read fixture, and negative cases"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
