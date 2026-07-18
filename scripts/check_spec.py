#!/usr/bin/env python3
"""Validate Liaison RM machine-readable product specifications."""

from __future__ import annotations

import json
import math
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ID = re.compile(r"^[A-Z0-9]+(?:-[A-Z0-9]+)+$")


def load_json(relative: str):
    path = ROOT / relative
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def duplicate_ids(items, label, errors):
    seen = set()
    for item in items:
        identifier = item.get("id")
        if not identifier or not ID.match(identifier):
            errors.append(f"{label}: invalid or missing id: {identifier!r}")
        elif identifier in seen:
            errors.append(f"{label}: duplicate id: {identifier}")
        seen.add(identifier)
    return seen


def parse_simple_yaml_ids(path: Path, key: str) -> list[str]:
    # The catalog intentionally uses a restricted, reviewable YAML shape. This
    # check avoids adding a Python dependency solely for repository policy.
    text = path.read_text(encoding="utf-8")
    if f"{key}:" not in text:
        raise ValueError(f"missing top-level {key}")
    return re.findall(r"^\s+- id:\s*([^\s#]+)\s*$", text, flags=re.MULTILINE)


def main() -> int:
    errors: list[str] = []

    requirements_doc = load_json("spec/requirements.json")
    uat_doc = load_json("spec/uat-cases.json")
    localization_doc = load_json("spec/localization-requirements.json")
    localization_requirements = [
        {"context": localization_doc.get("context", "localization"), **item}
        for item in localization_doc.get("requirements", [])
    ]
    requirements = requirements_doc.get("requirements", []) + localization_requirements
    cases = uat_doc.get("cases", []) + localization_doc.get("uat", [])
    personas = uat_doc.get("personas", [])

    requirement_ids = duplicate_ids(requirements, "requirements", errors)
    case_ids = duplicate_ids(cases, "uat cases", errors)
    persona_ids = duplicate_ids(personas, "personas", errors)

    valid_priorities = {"must", "should", "could", "wont"}
    valid_releases = {"R0", "R1", "R2", "R3", "R4", "R5", "R6", "B0", "A0"}

    for req in requirements:
        if req.get("priority") not in valid_priorities:
            errors.append(f"{req.get('id')}: invalid priority")
        if req.get("release") not in valid_releases:
            errors.append(f"{req.get('id')}: invalid release")
        for field in ("context", "statement", "acceptance"):
            if not str(req.get(field, "")).strip():
                errors.append(f"{req.get('id')}: missing {field}")

    for case in cases:
        if case.get("persona") not in persona_ids:
            errors.append(f"{case.get('id')}: unknown persona {case.get('persona')}")
        if case.get("release") not in valid_releases:
            errors.append(f"{case.get('id')}: invalid release")
        for field in ("title", "given", "when", "then"):
            if not str(case.get(field, "")).strip():
                errors.append(f"{case.get('id')}: missing {field}")

    try:
        gate_ids = parse_simple_yaml_ids(ROOT / "spec/feature-gates.yaml", "gates")
        duplicate_ids([{"id": value} for value in gate_ids], "feature gates", errors)
    except (OSError, ValueError) as exc:
        errors.append(f"feature gates: {exc}")

    try:
        task_text = (ROOT / "spec/implementation-plan.yaml").read_text(encoding="utf-8")
        task_ids = parse_simple_yaml_ids(ROOT / "spec/implementation-plan.yaml", "tasks")
        task_id_set = duplicate_ids([{"id": value} for value in task_ids], "tasks", errors)
        for dependency in re.findall(r"depends_on:\s*\[([^]]*)\]", task_text):
            for value in [item.strip() for item in dependency.split(",") if item.strip()]:
                if value not in task_id_set:
                    errors.append(f"task dependency references unknown task: {value}")
        for requirement in re.findall(r"requirements:\s*\[([^]]*)\]", task_text):
            for value in [item.strip() for item in requirement.split(",") if item.strip()]:
                if value not in requirement_ids:
                    errors.append(f"task references unknown requirement: {value}")
        for uat in re.findall(r"uat:\s*\[([^]]*)\]", task_text):
            for value in [item.strip() for item in uat.split(",") if item.strip()]:
                if value not in case_ids:
                    errors.append(f"task references unknown UAT case: {value}")

        scored_task_blocks = re.findall(
            r"(?ms)^  - id:\s*(T-(?:B0|A0)-[^\s]+)\s*$\n(.*?)(?=^  - id:|\Z)",
            task_text,
        )
        expected_scored_tasks = {
            identifier
            for identifier in task_ids
            if identifier.startswith(("T-B0-", "T-A0-"))
        }
        found_scored_tasks = {identifier for identifier, _ in scored_task_blocks}
        for missing in sorted(expected_scored_tasks - found_scored_tasks):
            errors.append(f"{missing}: missing parseable task block for RICE validation")
        rice_pattern = re.compile(
            r"^\s*rice:\s*\{reach:\s*([0-9.]+),\s*impact:\s*([0-9.]+),\s*"
            r"confidence:\s*([0-9.]+),\s*effort_weeks:\s*([0-9.]+),\s*"
            r"score:\s*([0-9.]+)\}\s*$",
            flags=re.MULTILINE,
        )
        for identifier, block in scored_task_blocks:
            match = rice_pattern.search(block)
            if match is None:
                errors.append(f"{identifier}: missing or malformed inline RICE evidence")
                continue
            reach, impact, confidence, effort, score = map(float, match.groups())
            if not 0 < reach <= 12:
                errors.append(f"{identifier}: RICE reach must be greater than 0 and at most 12")
            if impact not in {0.5, 1.0, 2.0, 3.0}:
                errors.append(f"{identifier}: RICE impact must be one of 0.5, 1, 2, or 3")
            if not 0.5 <= confidence <= 0.95:
                errors.append(f"{identifier}: RICE confidence must be between 0.50 and 0.95")
            if effort <= 0:
                errors.append(f"{identifier}: RICE effort_weeks must be greater than 0")
            if effort > 0:
                expected_score = round(reach * impact * confidence / effort + 1e-12, 2)
                if not math.isclose(score, expected_score, abs_tol=0.005):
                    errors.append(
                        f"{identifier}: RICE score {score:.2f} differs from calculated "
                        f"score {expected_score:.2f}"
                    )
    except (OSError, ValueError) as exc:
        errors.append(f"implementation plan: {exc}")

    if errors:
        print("Specification validation failed:")
        for error in sorted(set(errors)):
            print(f"- {error}")
        return 1

    print(
        "Specification validation passed: "
        f"{len(requirements)} requirements, {len(personas)} personas, "
        f"{len(cases)} UAT cases, {len(gate_ids)} feature gates, "
        f"{len(task_ids)} implementation tasks"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
