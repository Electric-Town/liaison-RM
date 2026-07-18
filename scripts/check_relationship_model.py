#!/usr/bin/env python3
"""Validate Liaison's Topic Pack and Review and Attention specification."""

from __future__ import annotations

import json
import math
import sys
import xml.etree.ElementTree as ET
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
SPEC = ROOT / "docs/product/relationship-memory-and-attention.md"
EXAMPLE = ROOT / "examples/profile-configuration/topic-pack-review-policy.yaml"
SCREENS = ROOT / "docs/prototypes/screens"
EXPECTED_STATES = {
    "known",
    "verified",
    "unverified",
    "unknown",
    "not_applicable",
    "declined",
    "stale",
    "conflicting",
    "needs_clarification",
    "derived",
}
REQUIRED_SUPPRESSIONS = {"archived", "do_not_contact", "paused", "snoozed"}


def main() -> int:
    errors: list[str] = []
    for path in [
        SPEC,
        EXAMPLE,
        ROOT / "schemas/topic-pack.schema.json",
        ROOT / "schemas/review-policy.schema.json",
    ]:
        if not path.is_file():
            errors.append(f"missing file: {path.relative_to(ROOT)}")

    if errors:
        return finish(errors)

    specification = SPEC.read_text(encoding="utf-8")
    required_phrases = [
        "Relationship intent",
        "Relationship evidence",
        "Maintenance status",
        "Profile readiness",
        "Reason-only is the default for personal workspaces",
        "No aggregate or interface may collapse these concepts",
        "The Review and Attention context must never",
        "Weighted review must not precede reason-only review",
    ]
    for phrase in required_phrases:
        if phrase not in specification:
            errors.append(f"specification is missing normative phrase: {phrase}")

    for schema_path in [
        ROOT / "schemas/topic-pack.schema.json",
        ROOT / "schemas/review-policy.schema.json",
    ]:
        try:
            schema = json.loads(schema_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as error:
            errors.append(f"{schema_path.relative_to(ROOT)}: invalid JSON: {error}")
            continue
        if schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
            errors.append(f"{schema_path.relative_to(ROOT)}: wrong JSON Schema version")
        if not schema.get("$id"):
            errors.append(f"{schema_path.relative_to(ROOT)}: missing stable $id")

    try:
        configuration = yaml.safe_load(EXAMPLE.read_text(encoding="utf-8"))
    except yaml.YAMLError as error:
        errors.append(f"example YAML is invalid: {error}")
        return finish(errors)

    if configuration.get("schema") != "liaison/profile-configuration@1":
        errors.append("example uses the wrong profile-configuration schema")

    packs = configuration.get("topic_packs", [])
    pack_ids = [pack.get("id") for pack in packs]
    if len(set(pack_ids)) != len(pack_ids):
        errors.append("Topic Pack IDs are not unique")
    field_ids: list[str] = []
    for pack in packs:
        fields = pack.get("fields", [])
        if not fields:
            errors.append(f"Topic Pack {pack.get('id')} has no fields")
        for field in fields:
            field_id = field.get("id")
            if not isinstance(field_id, str) or "." not in field_id:
                errors.append(f"field ID is not stable and namespaced: {field_id!r}")
            else:
                field_ids.append(field_id)
            if field.get("classification") == "sensitive" and not field.get("sealed_by_default"):
                errors.append(f"sensitive field is not sealed by default: {field_id}")
    if len(set(field_ids)) != len(field_ids):
        errors.append("field IDs are not unique")

    states = set(configuration.get("field_states", {}).get("allowed", []))
    if states != EXPECTED_STATES:
        errors.append(f"field states differ from the normative set: {sorted(states)}")

    policies = configuration.get("review_policies", [])
    if not policies:
        errors.append("no review policies are defined")
    reason_only = [policy for policy in policies if policy.get("mode") == "reason_only"]
    weighted = [policy for policy in policies if policy.get("mode") == "weighted"]
    if not reason_only:
        errors.append("a reason-only policy is required")
    for policy in reason_only:
        if not policy.get("reasons"):
            errors.append(f"reason-only policy {policy.get('id')} has no reasons")
        if policy.get("daily_capacity", 0) < 1:
            errors.append(f"reason-only policy {policy.get('id')} has no capacity bound")
        check_suppressions(policy, errors)
    if not weighted:
        errors.append("the transparent weighted-policy example is missing")
    for policy in weighted:
        components = policy.get("components", {})
        total = sum(float(value) for value in components.values())
        if not math.isclose(total, 1.0, rel_tol=0.0, abs_tol=1e-9):
            errors.append(f"weighted policy {policy.get('id')} totals {total}, not 1.0")
        if policy.get("explanations_required") is not True:
            errors.append(f"weighted policy {policy.get('id')} does not require explanations")
        if policy.get("hidden_components_allowed") is not False:
            errors.append(f"weighted policy {policy.get('id')} permits hidden components")
        check_suppressions(policy, errors)

    expected_screens = [
        "review-reasons.svg",
        "profile-readiness.svg",
        "review-mobile.svg",
    ]
    for filename in expected_screens:
        path = SCREENS / filename
        if not path.is_file():
            errors.append(f"missing review screen: {filename}")
            continue
        try:
            root = ET.parse(path).getroot()
        except ET.ParseError as error:
            errors.append(f"{filename}: invalid SVG: {error}")
            continue
        namespace = {"svg": "http://www.w3.org/2000/svg"}
        title = root.find("svg:title", namespace)
        description = root.find("svg:desc", namespace)
        if title is None or not (title.text or "").strip():
            errors.append(f"{filename}: missing accessible title")
        if description is None or not (description.text or "").strip():
            errors.append(f"{filename}: missing accessible description")
        if root.get("role") != "img" or not root.get("aria-labelledby"):
            errors.append(f"{filename}: missing image role or aria-labelledby")
        if not root.get("viewBox"):
            errors.append(f"{filename}: missing viewBox")

    return finish(errors)


def check_suppressions(policy: dict, errors: list[str]) -> None:
    present = set(policy.get("suppress_when", []))
    missing = REQUIRED_SUPPRESSIONS - present
    if missing:
        errors.append(f"review policy {policy.get('id')} lacks suppressions: {sorted(missing)}")


def finish(errors: list[str]) -> int:
    if errors:
        print("Relationship-model check failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1
    print("Relationship-model check passed: topics, field states, review policies, guardrails, and screens are consistent")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
