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


def parse_simple_yaml_records(path: Path, key: str) -> dict[str, str]:
    text = path.read_text(encoding="utf-8")
    if f"{key}:" not in text:
        raise ValueError(f"missing top-level {key}")
    return {
        identifier: block
        for identifier, block in re.findall(
            r"(?ms)^  - id:\s*([^\s#]+)\s*$\n(.*?)(?=^  - id:|\Z)", text
        )
    }


def inline_edges(blocks: dict[str, str], field: str) -> dict[str, set[str]]:
    result: dict[str, set[str]] = {}
    pattern = re.compile(rf"^\s{{4}}{re.escape(field)}:\s*\[([^]]*)\]", re.MULTILINE)
    for identifier, block in blocks.items():
        match = pattern.search(block)
        result[identifier] = {
            value.strip()
            for value in match.group(1).split(",")
            if value.strip()
        } if match else set()
    return result


def require_fragments(
    identifier: str, text: str, fragments: tuple[str, ...], errors: list[str]
) -> None:
    """Keep critical product semantics machine-enforced, not merely traceable."""

    lower = text.lower()
    for fragment in fragments:
        if fragment.lower() not in lower:
            errors.append(
                f"{identifier}: missing required semantic assertion {fragment!r}"
            )


def validate_b0_workplace_contract(
    requirements: list[dict],
    cases: list[dict],
    task_blocks: dict[str, str],
    gate_blocks: dict[str, str],
    errors: list[str],
) -> None:
    """Reject high-risk B0 wording contradictions even when IDs are connected."""

    requirement_text = {
        item["id"]: f"{item.get('statement', '')} {item.get('acceptance', '')}"
        for item in requirements
    }
    case_text = {
        item["id"]: " ".join(
            str(item.get(field, "")) for field in ("title", "given", "when", "then")
        )
        for item in cases
    }

    required_requirement_fragments = {
        "LRM-PE-010": (
            "immutable least-disclosure brief bytes",
            "never receive workspace access",
            "never receive",
            "catering-role grant",
        ),
        "LRM-EV-004": (
            "immutable purpose-bound least-disclosure snapshot",
            "names shall be absent by default",
            "approved policy explicitly requires",
            "no workspace account, role grant, or query capability",
        ),
        "LRM-EV-009": (
            "eventreadinessfollowup",
            "bounded to an event",
            "generic task engine",
            "relationship allocation",
            "cadence",
            "attention weighting",
        ),
        "LRM-EV-010": (
            "availability",
            "freshness",
            "conflict",
            "disclosure",
            "exactly one outcome",
            "ordered, versioned, fail-closed decision table",
        ),
        "LRM-EV-011": (
            "duplicate",
            "unresolved identity",
            "cancellation",
            "removal",
            "walk-in",
            "no-show",
            "event cancellation",
            "exact denominator reconciliation",
            "superseding corrections",
        ),
        "LRM-EV-012": (
            "data-controller",
            "legal-basis",
            "sensitive-data condition",
            "dpia decision",
            "independent legal-review",
        ),
        "LRM-EV-013": (
            "one trusted workspace owner",
            "recipients shall never access the workspace",
            "immutable, purpose-bound, expiring, audited least-disclosure brief",
            "emitted bytes match preview",
            "structural absence of names",
            "explicitly required by the approved policy",
        ),
    }
    for identifier, fragments in required_requirement_fragments.items():
        require_fragments(
            identifier, requirement_text.get(identifier, ""), fragments, errors
        )

    require_fragments(
        "UAT-010",
        case_text.get("UAT-010", ""),
        (
            "one trusted workspace owner",
            "preview and emitted bytes match",
            "names are absent by default",
            "approved policy explicitly requires",
            "workspace access",
            "catering-role grant",
            "structurally absent",
        ),
        errors,
    )
    uat_010 = case_text.get("UAT-010", "").lower()
    for contradiction in (
        "catering-export permission",
        "brief contains names or approved identifiers",
    ):
        if contradiction in uat_010:
            errors.append(f"UAT-010: prohibited B0 recipient/role claim {contradiction!r}")

    require_fragments(
        "UAT-041",
        case_text.get("UAT-041", ""),
        (
            "one trusted workspace owner",
            "every lifecycle transition and active denominator reconcile exactly",
            "preview and emitted bytes match",
            "relationship allocation",
            "cadence",
            "attention weights",
            "structurally absent",
            "400 percent zoom and reflow",
        ),
        errors,
    )
    require_fragments(
        "FG-R3-001",
        gate_blocks.get("FG-R3-001", ""),
        ("no B0 catering-role grant exists",),
        errors,
    )
    require_fragments(
        "FG-R3-004",
        gate_blocks.get("FG-R3-004", ""),
        (
            "names are absent by default",
            "approved policy explicitly requires",
            "catering-role grants are structurally absent",
            "preview bytes and emitted bytes match",
        ),
        errors,
    )
    require_fragments(
        "FG-B0-003",
        gate_blocks.get("FG-B0-003", ""),
        (
            "names are absent by default",
            "catering-role grants",
            "relationship allocation",
            "cadence",
            "attention weights",
            "structurally absent",
        ),
        errors,
    )
    require_fragments(
        "T-B0-P10",
        task_blocks.get("T-B0-P10", ""),
        (
            "ordered-versioned-decision-table",
            "attendee-lifecycle",
            "exact-denominator",
            "names-absent-by-default-policy",
            "negative-disclosure-and-role-grant-fixture",
            "preview-emitted-byte-equality",
        ),
        errors,
    )


def validate_traceability(
    requirement_ids: set[str],
    case_ids: set[str],
    persona_ids: set[str],
    task_blocks: dict[str, str],
    gate_blocks: dict[str, str],
    errors: list[str],
) -> None:
    try:
        ownership = load_json("spec/traceability-ownership.json")
    except (OSError, json.JSONDecodeError) as exc:
        errors.append(f"traceability ownership: {exc}")
        return

    task_ids = set(task_blocks)
    gate_ids = set(gate_blocks)
    task_ownership = ownership.get("task_ownership", {})
    gate_ownership = ownership.get("gate_ownership", {})
    requirement_ownership = ownership.get("requirement_ownership", {})
    uat_ownership = ownership.get("uat_ownership", {})
    milestones = ownership.get("milestones", [])
    evidence_owners = ownership.get("evidence_owners", [])

    expected_maps = (
        ("requirement", requirement_ids, set(requirement_ownership)),
        ("UAT", case_ids, set(uat_ownership)),
        ("task", task_ids, set(task_ownership)),
        ("gate", gate_ids, set(gate_ownership)),
    )
    for label, expected, actual in expected_maps:
        for value in sorted(expected - actual):
            errors.append(f"traceability: orphan {label}: {value}")
        for value in sorted(actual - expected):
            errors.append(f"traceability: unknown {label} ownership: {value}")

    milestone_ids: set[str] = set()
    for item in milestones:
        identifier = item.get("id")
        if not isinstance(identifier, str) or not re.fullmatch(
            r"[A-Z0-9]+(?:-[A-Z0-9]+)*", identifier
        ):
            errors.append(f"traceability milestones: invalid id: {identifier!r}")
        elif identifier in milestone_ids:
            errors.append(f"traceability milestones: duplicate id: {identifier}")
        milestone_ids.add(identifier)
    evidence_owner_ids = duplicate_ids(
        evidence_owners, "traceability evidence owners", errors
    )
    allowed_milestone_status = {"complete", "current", "blocked", "deferred"}
    for item in milestones:
        if item.get("status") not in allowed_milestone_status:
            errors.append(f"{item.get('id')}: invalid milestone status")
        for dependency in item.get("depends_on", []):
            if dependency not in milestone_ids:
                errors.append(
                    f"{item.get('id')}: unknown milestone dependency {dependency}"
                )

    # Milestone dependency cycles make the delivery order non-executable.
    dependency_map = {
        item.get("id"): set(item.get("depends_on", [])) for item in milestones
    }
    visiting: set[str] = set()
    visited: set[str] = set()

    def visit(identifier: str) -> None:
        if identifier in visiting:
            errors.append(f"traceability: milestone dependency cycle at {identifier}")
            return
        if identifier in visited:
            return
        visiting.add(identifier)
        for dependency in dependency_map.get(identifier, set()):
            visit(dependency)
        visiting.remove(identifier)
        visited.add(identifier)

    for identifier in milestone_ids:
        visit(identifier)

    milestone_order = {
        item.get("id"): index for index, item in enumerate(milestones)
    }
    for identifier, dependencies in dependency_map.items():
        for dependency in dependencies:
            if (
                dependency in milestone_order
                and identifier in milestone_order
                and milestone_order[dependency] >= milestone_order[identifier]
            ):
                errors.append(
                    f"{identifier}: milestone dependency {dependency} must appear earlier"
                )

    allowed_contract_status = {
        "complete",
        "current",
        "blocked",
        "deferred",
        "superseded",
    }
    task_requirements = inline_edges(task_blocks, "requirements")
    task_uat = inline_edges(task_blocks, "uat")
    task_dependencies = inline_edges(task_blocks, "depends_on")
    task_order = {identifier: index for index, identifier in enumerate(task_blocks)}
    visiting_tasks: set[str] = set()
    visited_tasks: set[str] = set()

    def visit_task(identifier: str) -> None:
        if identifier in visiting_tasks:
            errors.append(f"traceability: task dependency cycle at {identifier}")
            return
        if identifier in visited_tasks:
            return
        visiting_tasks.add(identifier)
        for dependency in task_dependencies.get(identifier, set()):
            if dependency in task_blocks:
                visit_task(dependency)
                if task_order[dependency] >= task_order[identifier]:
                    errors.append(
                        f"{identifier}: task dependency {dependency} must appear earlier"
                    )
        visiting_tasks.remove(identifier)
        visited_tasks.add(identifier)

    for identifier in task_blocks:
        visit_task(identifier)

    for identifier, edge in task_ownership.items():
        for field, known in (
            ("milestone", milestone_ids),
            ("owning_gate", gate_ids),
            ("evidence_owner", evidence_owner_ids),
        ):
            if edge.get(field) not in known:
                errors.append(
                    f"{identifier}: traceability has unknown {field} {edge.get(field)}"
                )
        status = edge.get("status")
        if status not in allowed_contract_status:
            errors.append(f"{identifier}: invalid traceability status {status}")
        if status == "superseded":
            replacement = edge.get("superseded_by")
            if replacement not in task_ids or replacement == identifier:
                errors.append(f"{identifier}: invalid superseded_by {replacement}")
            if not str(edge.get("disposition", "")).strip():
                errors.append(f"{identifier}: superseded task lacks disposition")
        elif edge.get("superseded_by"):
            errors.append(f"{identifier}: non-superseded task has superseded_by")

    for label, records, known_ids, support in (
        ("requirement", requirement_ownership, requirement_ids, task_requirements),
        ("UAT", uat_ownership, case_ids, task_uat),
    ):
        for identifier, edge in records.items():
            if identifier not in known_ids:
                continue
            task = edge.get("owning_task")
            gate = edge.get("owning_gate")
            milestone = edge.get("milestone")
            evidence_owner = edge.get("evidence_owner")
            if task not in task_ids:
                errors.append(f"{identifier}: unknown owning task {task}")
            if gate not in gate_ids:
                errors.append(f"{identifier}: unknown owning gate {gate}")
            if milestone not in milestone_ids:
                errors.append(f"{identifier}: unknown owning milestone {milestone}")
            if evidence_owner not in evidence_owner_ids:
                errors.append(
                    f"{identifier}: unknown evidence owner {evidence_owner}"
                )
            if edge.get("status") not in allowed_contract_status - {"superseded"}:
                errors.append(f"{identifier}: invalid ownership status {edge.get('status')}")
            if task in support and identifier not in support[task]:
                errors.append(
                    f"{identifier}: owning task {task} does not name the {label}"
                )
            gate_edge = gate_ownership.get(gate, {})
            if gate_edge.get("milestone") != milestone:
                errors.append(
                    f"{identifier}: milestone {milestone} disagrees with owning gate "
                    f"{gate} milestone {gate_edge.get('milestone')}"
                )
            if gate_edge.get("evidence_owner") != evidence_owner:
                errors.append(
                    f"{identifier}: evidence owner {evidence_owner} disagrees with "
                    f"owning gate {gate} owner {gate_edge.get('evidence_owner')}"
                )
            if label == "UAT" and gate in gate_blocks:
                if identifier not in set(re.findall(r"\bUAT-[A-Z0-9-]+\b", gate_blocks[gate])):
                    errors.append(
                        f"{identifier}: owning gate {gate} does not name the UAT as evidence"
                    )

    for identifier, edge in gate_ownership.items():
        task = edge.get("acceptance_task")
        milestone = edge.get("milestone")
        evidence_owner = edge.get("evidence_owner")
        if task not in task_ids:
            errors.append(f"{identifier}: unknown acceptance task {task}")
        if milestone not in milestone_ids:
            errors.append(f"{identifier}: unknown milestone {milestone}")
        if evidence_owner not in evidence_owner_ids:
            errors.append(f"{identifier}: unknown evidence owner {evidence_owner}")
        if edge.get("status") not in allowed_contract_status - {"superseded"}:
            errors.append(f"{identifier}: invalid gate status {edge.get('status')}")
        if task in task_ownership:
            task_edge = task_ownership[task]
            if task_edge.get("milestone") != milestone:
                errors.append(
                    f"{identifier}: milestone {milestone} disagrees with acceptance "
                    f"task {task} milestone {task_edge.get('milestone')}"
                )
            if task_edge.get("evidence_owner") != evidence_owner:
                errors.append(
                    f"{identifier}: evidence owner {evidence_owner} disagrees with "
                    f"acceptance task {task} owner {task_edge.get('evidence_owner')}"
                )

    owned_tasks = {
        edge.get("owning_task") for edge in requirement_ownership.values()
    } | {edge.get("owning_task") for edge in uat_ownership.values()} | {
        edge.get("acceptance_task") for edge in gate_ownership.values()
    }
    for identifier, edge in task_ownership.items():
        if edge.get("status") != "superseded" and identifier not in owned_tasks:
            errors.append(
                f"traceability: orphan executable task {identifier}; it owns no "
                "requirement, UAT, or gate"
            )

    # Only G0 work may be executable while G0 is current. This makes B0 -> A0
    # -> later status a machine rule rather than prose or file ordering.
    current_milestones = {
        item.get("id") for item in milestones if item.get("status") == "current"
    }
    for identifier, edge in task_ownership.items():
        if edge.get("status") == "current" and edge.get("milestone") not in current_milestones:
            errors.append(
                f"{identifier}: current task belongs to non-current milestone "
                f"{edge.get('milestone')}"
            )
        if edge.get("milestone") not in current_milestones and edge.get("status") == "complete":
            errors.append(
                f"{identifier}: non-current milestone task cannot newly claim complete"
            )

    expected_proposals = {
        *(f"LRM-WS-{value:03d}" for value in range(12, 17)),
        *(f"LRM-PE-{value:03d}" for value in range(11, 16)),
        *(f"LRM-RE-{value:03d}" for value in range(6, 9)),
        *(f"LRM-IN-{value:03d}" for value in range(6, 9)),
        *(f"LRM-RM-{value:03d}" for value in range(4, 6)),
        *(f"LRM-UX-{value:03d}" for value in range(10, 15)),
        *(f"LRM-EV-{value:03d}" for value in range(10, 14)),
        *(f"LRM-CO-{value:03d}" for value in range(13, 15)),
        *(f"LRM-AU-{value:03d}" for value in range(10, 13)),
        *(f"LRM-PK-{value:03d}" for value in range(7, 10)),
        *(f"UAT-{value:03d}" for value in range(45, 65)),
        "FG-UX-THEME-001",
        "P-PROFESSIONAL",
    }
    dispositions = ownership.get("proposal_dispositions", [])
    known_canonical_ids = (
        requirement_ids | case_ids | persona_ids | task_ids | gate_ids | milestone_ids
    )
    seen_proposals: set[str] = set()
    for item in dispositions:
        proposal = item.get("proposal_id")
        if proposal in seen_proposals:
            errors.append(f"proposal dispositions: duplicate id: {proposal}")
        seen_proposals.add(proposal)
        if item.get("disposition") not in {"adopted", "merged", "deferred", "rejected"}:
            errors.append(f"{proposal}: invalid proposal disposition")
        if not str(item.get("rationale", "")).strip():
            errors.append(f"{proposal}: missing proposal rationale")
        if item.get("disposition") in {"adopted", "merged"}:
            canonical_id = item.get("canonical_id")
            if canonical_id not in known_canonical_ids:
                errors.append(
                    f"{proposal}: adopted proposal has unknown canonical_id "
                    f"{canonical_id!r}"
                )
    for identifier in sorted(expected_proposals - seen_proposals):
        errors.append(f"traceability: undispositioned founder-plan proposal {identifier}")
    for identifier in sorted(seen_proposals - expected_proposals):
        errors.append(f"traceability: unknown founder-plan proposal {identifier}")

    required_branch_dispositions = {"PR #31", "PR #32", "PR #33"}
    branch_records = ownership.get("review_branch_dispositions", [])
    branch_ids = {item.get("reference") for item in branch_records}
    for identifier in sorted(required_branch_dispositions - branch_ids):
        errors.append(f"traceability: missing review-branch disposition {identifier}")
    for item in branch_records:
        if item.get("execution_status") not in {"blocked", "deferred"}:
            errors.append(
                f"{item.get('reference')}: review branch cannot be current before B0"
            )

    try:
        from generate_traceability import expected_outputs

        report_text, appendix_text = expected_outputs()
        for relative, expected in (
            ("spec/traceability-report.json", report_text),
            ("docs/product/traceability.md", appendix_text),
        ):
            path = ROOT / relative
            if not path.is_file() or path.read_text(encoding="utf-8") != expected:
                errors.append(
                    f"traceability: stale generated output {relative}; run "
                    "python3 scripts/generate_traceability.py"
                )
    except (ImportError, KeyError, OSError, ValueError, json.JSONDecodeError) as exc:
        errors.append(f"traceability generation: {exc}")


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
    valid_releases = {
        "R0",
        "R1",
        "R2",
        "R3",
        "R4",
        "R5",
        "R6",
        "B0",
        "A0",
        "POST-A0",
    }

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

    gate_blocks: dict[str, str] = {}
    try:
        gate_blocks = parse_simple_yaml_records(
            ROOT / "spec/feature-gates.yaml", "gates"
        )
        gate_ids = list(gate_blocks)
        duplicate_ids([{"id": value} for value in gate_ids], "feature gates", errors)
        for identifier, block in gate_blocks.items():
            release = re.search(
                r"^\s{4}release:\s*([^\s#]+)", block, flags=re.MULTILINE
            )
            if release is None or release.group(1) not in valid_releases | {"all"}:
                errors.append(f"{identifier}: invalid or missing gate release")
    except (OSError, ValueError) as exc:
        errors.append(f"feature gates: {exc}")

    task_blocks: dict[str, str] = {}
    try:
        task_text = (ROOT / "spec/implementation-plan.yaml").read_text(encoding="utf-8")
        task_blocks = parse_simple_yaml_records(
            ROOT / "spec/implementation-plan.yaml", "tasks"
        )
        task_ids = list(task_blocks)
        task_id_set = duplicate_ids([{"id": value} for value in task_ids], "tasks", errors)
        for identifier, block in task_blocks.items():
            release = re.search(
                r"^\s{4}release:\s*([^\s#]+)", block, flags=re.MULTILINE
            )
            if release is None or release.group(1) not in valid_releases:
                errors.append(f"{identifier}: invalid or missing task release")
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

    if task_blocks and gate_blocks:
        validate_b0_workplace_contract(
            requirements, cases, task_blocks, gate_blocks, errors
        )
        validate_traceability(
            requirement_ids,
            case_ids,
            persona_ids,
            task_blocks,
            gate_blocks,
            errors,
        )

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
