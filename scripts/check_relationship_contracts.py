#!/usr/bin/env python3
"""Validate Topic Pack, readiness, and Review and Attention contracts."""

from __future__ import annotations

import json
import sys
from pathlib import Path

import yaml
from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[1]
EXAMPLE = ROOT / "examples/configuration/topic-pack-and-review-policy.yaml"
PROFILE_SCHEMA = ROOT / "schemas/profile-configuration.schema.json"
REVIEW_SCHEMA = ROOT / "schemas/review-policy.schema.json"


def load_json(path: Path):
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def load_yaml(path: Path):
    with path.open(encoding="utf-8") as handle:
        return yaml.safe_load(handle)


def format_error(prefix: str, error) -> str:
    location = ".".join(str(item) for item in error.absolute_path) or "<root>"
    return f"{prefix} {location}: {error.message}"


def main() -> int:
    errors: list[str] = []
    document = load_yaml(EXAMPLE)

    if document.get("schema") != "liaison/profile-and-review-example@1":
        errors.append("example: unexpected top-level schema")

    profile = document.get("profile_configuration", {})
    profile_validator = Draft202012Validator(load_json(PROFILE_SCHEMA))
    errors.extend(
        format_error("profile", error)
        for error in sorted(profile_validator.iter_errors(profile), key=str)
    )

    policies = document.get("review_policies", [])
    review_validator = Draft202012Validator(load_json(REVIEW_SCHEMA))
    for index, policy in enumerate(policies):
        errors.extend(
            format_error(f"policy[{index}]", error)
            for error in sorted(review_validator.iter_errors(policy), key=str)
        )

    pack_ids: set[str] = set()
    field_ids: set[str] = set()
    required_for: set[str] = set()
    for pack in profile.get("packs", []):
        pack_id = pack.get("id")
        if pack_id in pack_ids:
            errors.append(f"duplicate pack id: {pack_id}")
        pack_ids.add(pack_id)
        for field in pack.get("fields", []):
            field_id = field.get("id")
            if field_id in field_ids:
                errors.append(f"duplicate field id: {field_id}")
            field_ids.add(field_id)
            required_for.update(field.get("required_for", []))

    readiness_ids: set[str] = set()
    for purpose in profile.get("readiness_purposes", []):
        purpose_id = purpose.get("id")
        if purpose_id in readiness_ids:
            errors.append(f"duplicate readiness id: {purpose_id}")
        readiness_ids.add(purpose_id)
        missing_fields = sorted(set(purpose.get("required_fields", [])) - field_ids)
        if missing_fields:
            errors.append(
                f"readiness purpose {purpose_id} references unknown fields: {missing_fields}"
            )

    missing_purposes = sorted(required_for - readiness_ids)
    if missing_purposes:
        errors.append(f"fields reference unknown readiness purposes: {missing_purposes}")

    policy_ids: set[str] = set()
    for policy in policies:
        policy_id = policy.get("id")
        if policy_id in policy_ids:
            errors.append(f"duplicate policy id: {policy_id}")
        policy_ids.add(policy_id)
        if policy.get("mode") == "weighted":
            total = sum(float(item.get("weight", 0)) for item in policy.get("components", []))
            if abs(total - 1.0) > 1e-9:
                errors.append(f"weighted policy {policy_id} weights total {total}, not 1.0")
        prohibited = {"message-volume", "email-count", "employee-performance"}
        component_ids = {item.get("id") for item in policy.get("components", [])}
        used = sorted(prohibited & component_ids)
        if used:
            errors.append(f"policy {policy_id} uses prohibited components: {used}")

    if errors:
        print("Relationship contract validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print(
        "Relationship contracts passed: "
        f"{len(pack_ids)} packs, {len(field_ids)} fields, {len(policy_ids)} policies"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
