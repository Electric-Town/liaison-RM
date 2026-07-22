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


def transitive_dependencies(
    dependency_map: dict[str, set[str]], identifier: str
) -> set[str]:
    """Return every dependency reachable from identifier without assuming a DAG."""

    reachable: set[str] = set()
    pending = list(dependency_map.get(identifier, set()))
    while pending:
        dependency = pending.pop()
        if dependency in reachable:
            continue
        reachable.add(dependency)
        pending.extend(dependency_map.get(dependency, set()))
    return reachable


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


def require_ordered_fragments(
    identifier: str, text: str, fragments: tuple[str, ...], errors: list[str]
) -> None:
    """Require critical delivery steps to appear in dependency order."""

    lower = text.lower()
    cursor = -1
    for fragment in fragments:
        position = lower.find(fragment.lower(), cursor + 1)
        if position == -1:
            errors.append(
                f"{identifier}: missing ordered semantic assertion {fragment!r}"
            )
            return
        cursor = position + len(fragment)


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


def validate_okf_and_comparator_contract(
    requirements: list[dict],
    cases: list[dict],
    task_blocks: dict[str, str],
    gate_blocks: dict[str, str],
    errors: list[str],
) -> None:
    """Keep the amended OKF, sourced-profile, and later-safety decisions exact."""

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
        "LRM-WS-017": (
            "pinned OKF v0.1 Draft",
            "strict writes",
            "tolerant reads",
            "backup-first",
            "journaled",
            "failure-atomic",
            "idempotent",
            "exactly reversible",
            "body bytes",
            "malformed siblings",
            "reserved or index content",
        ),
        "LRM-PE-016": (
            "pinned OKF v0.1 Draft",
            "type: person",
            "domain fields remain the schema authority",
            "sealed sensitive values",
            "never enter plaintext",
        ),
        "LRM-PE-017": (
            "shall not imply Liaison domain validity",
            "inert or quarantined",
            "cannot affect event readiness",
            "unknown types",
            "healthy People",
        ),
        "LRM-PE-018": (
            "purpose-scoped profile",
            "no global person score shall exist",
            "contain no global person score",
        ),
        "LRM-PE-019": ("canonical source", "missing provider coverage"),
        "LRM-PE-020": ("conflicting", "stale", "withheld", "never silently select a winner"),
        "LRM-PE-021": ("stable-ID field diffs", "no external edit", "silently change disclosure", "operational readiness"),
        "LRM-IN-001": (
            "unified timeline",
            "source-linked",
            "requested and covered range",
            "factual counts shall never become a global person score",
            "no file, CLI, UI, projection, export, API, plugin, or automation output derives a person score",
        ),
        "LRM-CO-015": (
            "operations feed separate from relationship reminders",
            "hidden refresh and unreported egress are prohibited",
            "revocation stops all refresh and egress",
        ),
        "LRM-AU-013": (
            "source-backed staged proposal",
            "shall never directly overwrite a confirmed fact",
            "cannot directly write a confirmed fact",
            "assessment",
            "freshness",
            "cadence",
        ),
        "LRM-UX-015": (
            "explicit geocoding egress",
            "structural Workplace denial",
            "deny the capability in Workplace schemas and surfaces",
            "semantic list or table",
            "shall never rank People by a relationship score",
            "contain no hidden geocoding, global person score, ranking",
        ),
    }
    for identifier, fragments in required_requirement_fragments.items():
        require_fragments(identifier, requirement_text.get(identifier, ""), fragments, errors)

    required_uat_fragments = {
        "UAT-065": ("every B0-released UI and CLI", "OKF v0.1 Draft", "body bytes", "sealed values never appear in plaintext", "does not co-own"),
        "UAT-066": ("every write boundary", "failure-atomic", "idempotent", "byte-exact originals", "no partial profile or index state"),
        "UAT-067": ("last note and last interaction remain distinct", "VoiceOver", "400 percent zoom and reflow", "no global person score"),
        "UAT-068": (
            "remembered rejection",
            "reversible",
            "neither exact identifiers nor fuzzy thresholds automatically merge People",
        ),
        "UAT-069": (
            "requested and covered range",
            "correction history",
            "no activity count becomes a global person score",
        ),
        "UAT-070": (
            "no hidden refresh or unreported provider egress occurs",
            "never pollutes relationship reminders",
        ),
        "UAT-071": (
            "No AI, MCP, plugin, provider or import path directly overwrites",
            "changes assessment or freshness",
            "resets cadence",
        ),
        "UAT-072": (
            "Workplace denies the capability structurally",
            "no relationship score or hidden sync ranks People",
            "semantic result",
        ),
    }
    for identifier, fragments in required_uat_fragments.items():
        require_fragments(identifier, case_text.get(identifier, ""), fragments, errors)

    for identifier, fragments in {
        "FG-B0-001": ("pinned OKF v0.1 Draft", "sealed sensitive values never enter plaintext", "OKF-valid but domain-invalid", "UAT-065", "UAT-066"),
        "FG-A0-001": (
            "source-complete purpose-scoped profile",
            "never merge automatically",
            "timeline counts never become a global person score",
            "UAT-067",
            "UAT-068",
            "UAT-069",
        ),
        "FG-R5-005": (
            "UAT-070",
            "Hidden refresh, unreported egress, and relationship-reminder pollution are structurally denied",
        ),
        "FG-R5-006": (
            "Workplace schemas, files, CLI, UI, settings, exports, and adapters deny spatial discovery structurally",
            "semantic list or table",
            "neither view ranks People by a global person or relationship score",
            "UAT-072",
        ),
        "FG-R6-007": (
            "cannot directly write a confirmed fact",
            "cannot silently change assessment, freshness, cadence",
            "UAT-071",
        ),
    }.items():
        require_fragments(identifier, gate_blocks.get(identifier, ""), fragments, errors)

    for identifier, fragments in {
        "T-B0-P05-OKF": ("LRM-PE-016", "UAT-065", "strict-People-writer", "sealed-plaintext-denial"),
        "T-B0-P06": ("LRM-PE-017", "tolerant-OKF-reader", "domain-validity-quarantine"),
        "T-B0-P09-OKF": ("LRM-WS-017", "UAT-066", "journaled-failure-atomic-migration", "exact-rollback"),
        "T-A0-P04": ("LRM-PE-018", "LRM-PE-021", "LRM-IN-001", "UAT-067", "UAT-069", "no-global-person-score"),
        "T-R5-008": ("LRM-CO-015", "UAT-070", "hidden-refresh-and-egress-denial"),
        "T-R5-009": ("LRM-UX-015", "UAT-072", "Workplace-denial", "semantic-list-table-parity"),
        "T-R6-007": ("LRM-AU-013", "UAT-071", "direct-write-denial"),
    }.items():
        require_fragments(identifier, task_blocks.get(identifier, ""), fragments, errors)

    p05 = task_blocks.get("T-B0-P05", "")
    for leaked in ("LRM-SEC-001", "LRM-SEC-002", "UAT-043", "workspace-security", "sealed-envelope-types"):
        if leaked.lower() in p05.lower():
            errors.append(f"T-B0-P05: sensitive FG-B0-002 contract leaked into G3 task: {leaked}")

    for identifier in ("T-B0-P01", "T-B0-P02"):
        text = task_blocks.get(identifier, "").lower()
        for expansion in ("okf", "lrm-pe-016", "lrm-pe-017", "lrm-ws-017", "uat-065", "uat-066"):
            if expansion in text:
                errors.append(f"{identifier}: OKF work must not expand P01 or P02: {expansion}")

    for relative in ("SPEC.md", "docs/product/working-state-delivery.md"):
        try:
            boundary = (ROOT / relative).read_text(encoding="utf-8")
        except OSError as exc:
            errors.append(f"{relative}: cannot verify B0 migration boundary: {exc}")
            continue
        require_fragments(
            relative,
            boundary,
            ("required OKF People normalization", "general and third-party migrations"),
            errors,
        )


def validate_corrected_phase_ownership(
    requirements: list[dict],
    cases: list[dict],
    task_blocks: dict[str, str],
    gate_blocks: dict[str, str],
    errors: list[str],
) -> None:
    """Lock the reviewed P02/P03, repair, settings, and acceptance boundaries."""

    try:
        ownership = load_json("spec/traceability-ownership.json")
    except (OSError, json.JSONDecodeError) as exc:
        errors.append(f"corrected phase ownership: {exc}")
        return

    requirement_by_id = {item["id"]: item for item in requirements}
    case_by_id = {item["id"]: item for item in cases}
    task_requirements = inline_edges(task_blocks, "requirements")
    task_uat = inline_edges(task_blocks, "uat")
    task_dependencies = inline_edges(task_blocks, "depends_on")
    task_evidence = inline_edges(task_blocks, "evidence_dependencies")

    expected_task_edges = {
        "T-B0-P02": (
            {"LRM-WS-002", "LRM-WS-009"},
            set(),
        ),
        "T-B0-P03": (
            {"LRM-WS-004", "LRM-WS-005", "LRM-WS-007", "LRM-WS-010"},
            {"UAT-042"},
        ),
        "T-B0-P06-REPAIR": (set(), {"UAT-040"}),
    }
    for task, (expected_requirements, expected_uat) in expected_task_edges.items():
        if task_requirements.get(task, set()) != expected_requirements:
            errors.append(
                f"{task}: corrected canonical requirements must be "
                f"{sorted(expected_requirements)}"
            )
        if task_uat.get(task, set()) != expected_uat:
            errors.append(
                f"{task}: corrected canonical UAT must be {sorted(expected_uat)}"
            )

    repair_dependencies = task_dependencies.get("T-B0-P06-REPAIR", set())
    if repair_dependencies != {"T-B0-P03", "T-B0-P06"}:
        errors.append(
            "T-B0-P06-REPAIR: dependencies must be exactly T-B0-P03 and T-B0-P06"
        )
    if "T-B0-P06-REPAIR" not in task_dependencies.get("T-B0-P09-OKF", set()):
        errors.append(
            "T-B0-P09-OKF: must depend on T-B0-P06-REPAIR before normalization"
        )
    require_fragments(
        "T-B0-P06-REPAIR",
        task_blocks.get("T-B0-P06-REPAIR", ""),
        (
            "guided-repair-preview",
            "exact-pre-repair-backup",
            "backup-first-failure-atomic-repair",
            "exact-repair-receipt",
            "exact-rollback",
            "recovery-knowledge",
        ),
        errors,
    )

    b0_accept_requirements = task_requirements.get("T-B0-ACCEPT", set())
    b0_accept_uat = task_uat.get("T-B0-ACCEPT", set())
    if "LRM-WS-001" not in b0_accept_requirements:
        errors.append("T-B0-ACCEPT: must own LRM-WS-001")
    if "UAT-001" not in b0_accept_uat:
        errors.append("T-B0-ACCEPT: must own UAT-001")
    if not {"UAT-001", "UAT-002"} <= task_evidence.get("T-A0-001", set()):
        errors.append(
            "T-A0-001: must evidence UAT-001 and UAT-002 for FG-R1-001"
        )

    a0_settings_requirements = task_requirements.get("T-A0-P01", set())
    a0_settings_uat = task_uat.get("T-A0-P01", set())
    if not {"LRM-WS-013", "LRM-WS-014"} <= a0_settings_requirements:
        errors.append("T-A0-P01: must own LRM-WS-013 and LRM-WS-014")
    if "UAT-050" not in a0_settings_uat:
        errors.append("T-A0-P01: must own UAT-050")
    for task in ("T-B0-P03", "T-B0-P04", "T-B0-P11"):
        if {"LRM-WS-013", "LRM-WS-014"} & task_requirements.get(task, set()):
            errors.append(f"{task}: B0 must not own settings-transfer requirements")
        if "UAT-050" in task_uat.get(task, set()):
            errors.append(f"{task}: B0 must not own settings-transfer UAT-050")
        if "settings-preview-diff-rollback" in task_blocks.get(task, "").lower():
            errors.append(f"{task}: B0 must not expose settings transfer")
    for gate in ("FG-B0-001", "FG-B0-003"):
        if "UAT-050" in gate_blocks.get(gate, ""):
            errors.append(f"{gate}: B0 must not require settings-transfer UAT-050")

    expected_edges = (
        (
            "LRM-WS-001",
            ownership.get("requirement_ownership", {}),
            {
                "owning_task": "T-B0-ACCEPT",
                "owning_gate": "FG-B0-003",
                "milestone": "B0",
                "evidence_owner": "EO-EXPERIENCE",
                "status": "blocked",
            },
        ),
        (
            "LRM-WS-002",
            ownership.get("requirement_ownership", {}),
            {
                "owning_task": "T-B0-P02",
                "owning_gate": "FG-B0-001",
                "milestone": "G1",
                "evidence_owner": "EO-WORKSPACE",
                "status": "complete",
            },
        ),
        (
            "LRM-WS-009",
            ownership.get("requirement_ownership", {}),
            {
                "owning_task": "T-B0-P02",
                "owning_gate": "FG-B0-001",
                "milestone": "G1",
                "evidence_owner": "EO-WORKSPACE",
                "status": "complete",
            },
        ),
        (
            "LRM-WS-013",
            ownership.get("requirement_ownership", {}),
            {
                "owning_task": "T-A0-P01",
                "owning_gate": "FG-A0-G2C",
                "milestone": "G2C",
                "evidence_owner": "EO-EXPERIENCE",
                "status": "blocked",
            },
        ),
        (
            "LRM-WS-014",
            ownership.get("requirement_ownership", {}),
            {
                "owning_task": "T-A0-P01",
                "owning_gate": "FG-A0-G2C",
                "milestone": "G2C",
                "evidence_owner": "EO-EXPERIENCE",
                "status": "blocked",
            },
        ),
        (
            "UAT-001",
            ownership.get("uat_ownership", {}),
            {
                "owning_task": "T-B0-ACCEPT",
                "owning_gate": "FG-B0-003",
                "milestone": "B0",
                "evidence_owner": "EO-EXPERIENCE",
                "status": "blocked",
            },
        ),
        (
            "UAT-040",
            ownership.get("uat_ownership", {}),
            {
                "owning_task": "T-B0-P06-REPAIR",
                "owning_gate": "FG-R1-002",
                "milestone": "G1",
                "evidence_owner": "EO-WORKSPACE",
                "status": "blocked",
            },
        ),
        (
            "UAT-042",
            ownership.get("uat_ownership", {}),
            {
                "owning_task": "T-B0-P03",
                "owning_gate": "FG-B0-001",
                "milestone": "G1",
                "evidence_owner": "EO-WORKSPACE",
                "status": "current",
            },
        ),
        (
            "UAT-050",
            ownership.get("uat_ownership", {}),
            {
                "owning_task": "T-A0-P01",
                "owning_gate": "FG-A0-G2C",
                "milestone": "G2C",
                "evidence_owner": "EO-EXPERIENCE",
                "status": "blocked",
            },
        ),
        (
            "T-B0-P06-REPAIR",
            ownership.get("task_ownership", {}),
            {
                "milestone": "G1",
                "owning_gate": "FG-R1-002",
                "evidence_owner": "EO-WORKSPACE",
                "status": "blocked",
            },
        ),
        (
            "T-B0-P05",
            ownership.get("task_ownership", {}),
            {
                "milestone": "G1",
                "owning_gate": "FG-R3-001",
                "evidence_owner": "EO-EVENTS",
                "status": "blocked",
            },
        ),
        (
            "T-B0-P01",
            ownership.get("task_ownership", {}),
            {
                "milestone": "G1",
                "owning_gate": "FG-B0-001",
                "evidence_owner": "EO-WORKSPACE",
                "status": "complete",
            },
        ),
        (
            "FG-R1-001",
            ownership.get("gate_ownership", {}),
            {
                "acceptance_task": "T-A0-001",
                "milestone": "A0",
                "evidence_owner": "EO-EXPERIENCE",
                "status": "blocked",
            },
        ),
        (
            "FG-R1-002",
            ownership.get("gate_ownership", {}),
            {
                "acceptance_task": "T-B0-P06-REPAIR",
                "milestone": "G1",
                "evidence_owner": "EO-WORKSPACE",
                "status": "blocked",
            },
        ),
        (
            "FG-R1-004",
            ownership.get("gate_ownership", {}),
            {
                "acceptance_task": "T-B0-ACCEPT",
                "milestone": "B0",
                "evidence_owner": "EO-EXPERIENCE",
                "status": "blocked",
            },
        ),
        (
            "FG-R3-001",
            ownership.get("gate_ownership", {}),
            {
                "acceptance_task": "T-B0-P05",
                "milestone": "G1",
                "evidence_owner": "EO-EVENTS",
                "status": "blocked",
            },
        ),
        (
            "FG-B0-001",
            ownership.get("gate_ownership", {}),
            {
                "acceptance_task": "T-B0-P09-OKF",
                "milestone": "G1",
                "evidence_owner": "EO-WORKSPACE",
                "status": "blocked",
            },
        ),
    )
    for identifier, records, expected in expected_edges:
        if records.get(identifier) != expected:
            errors.append(
                f"{identifier}: corrected ownership must be {expected}, "
                f"not {records.get(identifier)}"
            )

    if requirement_by_id.get("LRM-WS-013", {}).get("release") != "A0":
        errors.append("LRM-WS-013: repository release must be A0")
    if requirement_by_id.get("LRM-WS-014", {}).get("release") != "A0":
        errors.append("LRM-WS-014: repository release must be A0")
    if case_by_id.get("UAT-050", {}).get("release") != "A0":
        errors.append("UAT-050: repository release must be A0")

    execution_status = {
        "G0": (ownership.get("milestones", []), "complete"),
        "G1": (ownership.get("milestones", []), "current"),
        "T-R0-003": (ownership.get("task_ownership", {}), "complete"),
        "T-B0-P00": (ownership.get("task_ownership", {}), "complete"),
        "T-B0-P01": (ownership.get("task_ownership", {}), "complete"),
        "T-B0-P02": (ownership.get("task_ownership", {}), "complete"),
        "LRM-PK-007": (ownership.get("requirement_ownership", {}), "complete"),
        "LRM-PK-009": (ownership.get("requirement_ownership", {}), "complete"),
        "LRM-AP-001": (ownership.get("requirement_ownership", {}), "complete"),
        "LRM-WS-011": (ownership.get("requirement_ownership", {}), "complete"),
        "LRM-WS-002": (ownership.get("requirement_ownership", {}), "complete"),
        "LRM-WS-009": (ownership.get("requirement_ownership", {}), "complete"),
        "FG-R0-002": (ownership.get("gate_ownership", {}), "complete"),
    }
    milestone_status = {
        item.get("id"): item.get("status")
        for item in ownership.get("milestones", [])
    }
    for identifier, (records, expected_status) in execution_status.items():
        actual_status = (
            milestone_status.get(identifier)
            if isinstance(records, list)
            else records.get(identifier, {}).get("status")
        )
        if actual_status != expected_status:
            errors.append(
                f"{identifier}: execution status must be {expected_status}, "
                f"not {actual_status}"
            )

    workspace_requirement = requirement_by_id.get("LRM-WS-001", {})
    workspace_case = case_by_id.get("UAT-001", {})
    workspace_requirement_text = " ".join(
        str(workspace_requirement.get(field, ""))
        for field in ("statement", "acceptance")
    )
    workspace_case_text = " ".join(
        str(workspace_case.get(field, ""))
        for field in ("title", "given", "when", "then")
    )
    if "airgap" in workspace_requirement_text.lower():
        errors.append("LRM-WS-001: installed review acceptance must not claim Airgap")
    if "airgap" in workspace_case_text.lower():
        errors.append("UAT-001: installed review acceptance must not claim Airgap")
    require_fragments(
        "LRM-WS-001",
        workspace_requirement_text,
        (
            "installed local-authoritative review artifact",
            "network access is denied",
            "no account is used",
        ),
        errors,
    )
    require_fragments(
        "UAT-001",
        workspace_case_text,
        (
            "installed local-authoritative review artifact",
            "network access denied",
            "no account configured",
            "no account or network operation occurs",
        ),
        errors,
    )
    airgap_case_text = " ".join(
        str(case_by_id.get("UAT-024", {}).get(field, ""))
        for field in ("title", "given", "when", "then")
    )
    require_fragments(
        "UAT-024",
        airgap_case_text,
        ("Airgap artifact", "no enabled network provider or listener capability"),
        errors,
    )
    require_fragments(
        "FG-R2-005",
        gate_blocks.get("FG-R2-005", ""),
        ("Airgap artifact proven", "Network clients and listeners are absent", "UAT-024"),
        errors,
    )
    uat_024_gates = {
        identifier for identifier, block in gate_blocks.items() if "UAT-024" in block
    }
    if uat_024_gates != {"FG-R2-005"}:
        errors.append(
            "UAT-024: compiled-out Airgap proof must belong only to FG-R2-005, "
            f"not {sorted(uat_024_gates)}"
        )

    mandatory_sources = {
        "SPEC.md": ("**P06 —", "`T-B0-P06-REPAIR`", "`T-B0-P09-OKF`"),
        "AI_BUILD_INSTRUCTIONS.md": (
            "**P05/P05-OKF/P06",
            "`T-B0-P06-REPAIR`",
            "**P09-OKF/P09",
        ),
        "PROJECT_CONTEXT.md": (
            "P05-OKF/P06/P06-REPAIR/P09-OKF",
            "`T-B0-P06-REPAIR`",
        ),
        "docs/product/rice-prioritization.md": (
            "| P06 Tolerant Directory projection",
            "| P06-REPAIR Guided canonical repair",
            "| P09-OKF required legacy-People normalization",
        ),
        "docs/product/roadmap.md": (
            "## P06 — Scalable Directory reads",
            "## P06-REPAIR — Guided canonical repair",
            "## P09 — Required OKF normalization",
        ),
        "docs/product/working-state-delivery.md": (
            "P05-OKF",
            "P06-REPAIR",
            "P09-OKF",
        ),
    }
    for relative, ordered in mandatory_sources.items():
        try:
            source = (ROOT / relative).read_text(encoding="utf-8")
        except OSError as exc:
            errors.append(f"{relative}: cannot verify corrected delivery order: {exc}")
            continue
        require_ordered_fragments(relative, source, ordered, errors)

    for relative, fragments in {
        "SPEC.md": (
            "built-in theme choice and persistence only",
            "settings bundle transfer begins in A0",
            "the G1 P05 task",
            "closes `FG-B0-001`",
        ),
        "AI_BUILD_INSTRUCTIONS.md": (
            "G0/P00 and P01 are complete",
            "establish P05's Directory/Event/dietary contracts in G1",
            "close `FG-B0-001`",
        ),
        "PROJECT_CONTEXT.md": (
            "G0, P00, P01, and P02 are complete",
            "G1 is current and P03 recoverable canonical operations are the active package",
            "without a reverse milestone dependency",
        ),
        "docs/product/roadmap.md": (
            "P02 owns the readable manifest contract and session authority only",
            "the G1 `T-B0-P05`",
            "closes `FG-B0-001`",
        ),
        "docs/product/working-state-delivery.md": (
            "P02 owns the readable manifest and write-authoritative session boundary",
            "G0, P00, P01, and P02 are complete",
            "Compiled-out Airgap proof remains exclusively `UAT-024` under `FG-R2-005`",
        ),
    }.items():
        try:
            source = (ROOT / relative).read_text(encoding="utf-8")
        except OSError as exc:
            errors.append(f"{relative}: cannot verify corrected boundary: {exc}")
            continue
        require_fragments(relative, source, fragments, errors)

    try:
        rice = (ROOT / "docs/product/rice-prioritization.md").read_text(
            encoding="utf-8"
        )
    except OSError as exc:
        errors.append(f"RICE: cannot verify P06-REPAIR evidence: {exc}")
    else:
        require_fragments(
            "P06-REPAIR RICE",
            rice,
            (
                "| P06-REPAIR Guided canonical repair | 7 | 3 | 0.85 | 3 | 5.95 |",
                "three-engineer-week integrity slice",
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

    # P03D split the historical R2 umbrella so P04 cannot accidentally claim
    # downstream B0, A0, or public-site acceptance. Keep these exact owner
    # assignments executable rather than relying on prose in the amended plan.
    expected_task_gates = {
        "T-B0-P04": "FG-B0-P04-001",
        "T-B0-P11": "FG-B0-003",
        "T-A0-P03": "FG-A0-G2B",
        "T-REL-001": "FG-REL-001",
    }
    for task, gate in expected_task_gates.items():
        if task_ownership.get(task, {}).get("owning_gate") != gate:
            errors.append(f"{task}: phase ownership must use {gate}")

    expected_requirement_owners = {
        "LRM-UX-009": ("T-B0-P11", "FG-B0-003"),
        "LRM-UX-012": ("T-B0-P11", "FG-B0-003"),
        "LRM-UX-016": ("T-B0-P04", "FG-B0-P04-001"),
        "LRM-UX-017": ("T-B0-P04", "FG-B0-P04-001"),
        "LRM-L10N-008": ("T-REL-001", "FG-REL-001"),
    }
    for identifier, (task, gate) in expected_requirement_owners.items():
        edge = requirement_ownership.get(identifier, {})
        if (edge.get("owning_task"), edge.get("owning_gate")) != (task, gate):
            errors.append(
                f"{identifier}: phase ownership must be {task} under {gate}"
            )

    expected_uat_owners = {
        "UAT-021": ("T-A0-P01", "FG-A0-G2C"),
        "UAT-022": ("T-A0-P03", "FG-A0-G2B"),
        "UAT-062": ("T-B0-P11", "FG-B0-003"),
        "UAT-073": ("T-B0-P04", "FG-B0-P04-001"),
        "UAT-074": ("T-B0-P04", "FG-B0-P04-001"),
    }
    for identifier, (task, gate) in expected_uat_owners.items():
        edge = uat_ownership.get(identifier, {})
        if (edge.get("owning_task"), edge.get("owning_gate")) != (task, gate):
            errors.append(
                f"{identifier}: phase ownership must be {task} under {gate}"
            )

    if gate_ownership.get("FG-R2-001", {}).get("acceptance_task") == "T-B0-P04":
        errors.append("FG-R2-001: historical umbrella cannot be P04 acceptance")

    approved_strategy = ownership.get("approved_strategy", {})
    if approved_strategy.get("approved_sha256") != (
        "795a6e6751cd29a995478e254323f491e68a53ef7c35fa729d8627b87cd37089"
    ):
        errors.append("traceability: approved strategy SHA-256 is missing or stale")
    require_fragments(
        "traceability approved strategy",
        str(approved_strategy.get("scope", "")),
        ("required OKF People normalization", "general or third-party migrations"),
        errors,
    )

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
    try:
        implementation_plan_text = (
            ROOT / "spec/implementation-plan.yaml"
        ).read_text(encoding="utf-8")
    except OSError as exc:
        errors.append(f"traceability: cannot verify delivery sequence: {exc}")
    else:
        sequence_match = re.search(
            r"^delivery_sequence:\s*\[([^]]*)\]",
            implementation_plan_text,
            flags=re.MULTILINE,
        )
        declared_sequence = (
            [
                item.strip()
                for item in sequence_match.group(1).split(",")
                if item.strip()
            ]
            if sequence_match
            else []
        )
        expected_sequence = [item.get("id") for item in milestones]
        if declared_sequence != expected_sequence:
            errors.append(
                "traceability: implementation delivery_sequence must match "
                f"milestone order {expected_sequence}, not {declared_sequence}"
            )
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

    milestone_by_id = {item.get("id"): item for item in milestones}
    current_milestones = {
        item.get("id") for item in milestones if item.get("status") == "current"
    }
    if len(current_milestones) != 1:
        errors.append(
            "traceability: exactly one milestone must be current, "
            f"found {sorted(current_milestones)}"
        )
    for identifier, item in milestone_by_id.items():
        if item.get("status") not in {"complete", "current"}:
            continue
        incomplete_dependencies = {
            dependency
            for dependency in transitive_dependencies(dependency_map, identifier)
            if milestone_by_id.get(dependency, {}).get("status") != "complete"
        }
        if incomplete_dependencies:
            errors.append(
                f"{identifier}: active or complete milestone has incomplete "
                f"dependencies {sorted(incomplete_dependencies)}"
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
    task_evidence_dependencies = inline_edges(task_blocks, "evidence_dependencies")
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

    # An executable task may depend only on work in its own milestone or a
    # transitively required predecessor milestone. This rejects milestone
    # plans that are acyclic on paper but cyclic through task ownership.
    for identifier, dependencies in task_dependencies.items():
        task_edge = task_ownership.get(identifier, {})
        if task_edge.get("status") == "superseded":
            continue
        task_milestone = task_edge.get("milestone")
        reachable_milestones = transitive_dependencies(
            dependency_map, str(task_milestone)
        )
        for dependency in dependencies:
            dependency_milestone = task_ownership.get(dependency, {}).get("milestone")
            if (
                dependency_milestone
                and dependency_milestone != task_milestone
                and dependency_milestone not in reachable_milestones
            ):
                errors.append(
                    f"{identifier}: milestone {task_milestone} cannot depend on "
                    f"{dependency} in unreachable milestone {dependency_milestone}"
                )

    # Canonical task arrays express ownership. Reused regression or prerequisite
    # coverage must be explicit evidence_dependencies, never a duplicate-owner
    # illusion in requirements or uat.
    for task, identifiers in task_requirements.items():
        for identifier in identifiers:
            owner = requirement_ownership.get(identifier, {}).get("owning_task")
            if owner != task:
                errors.append(
                    f"{task}: canonical requirements contains non-owned {identifier}; "
                    "move it to evidence_dependencies"
                )
    for task, identifiers in task_uat.items():
        for identifier in identifiers:
            owner = uat_ownership.get(identifier, {}).get("owning_task")
            if owner != task:
                errors.append(
                    f"{task}: canonical uat contains non-owned {identifier}; "
                    "move it to evidence_dependencies"
                )
    known_evidence_dependencies = requirement_ids | case_ids
    for task, identifiers in task_evidence_dependencies.items():
        for identifier in identifiers:
            if identifier not in known_evidence_dependencies:
                errors.append(f"{task}: unknown evidence dependency {identifier}")
                continue
            owner = (
                requirement_ownership.get(identifier, {}).get("owning_task")
                if identifier in requirement_ids
                else uat_ownership.get(identifier, {}).get("owning_task")
            )
            if owner == task:
                errors.append(
                    f"{task}: owned {identifier} belongs in its canonical array, "
                    "not evidence_dependencies"
                )

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
            owner_status = task_ownership.get(task, {}).get("status")
            edge_status = edge.get("status")
            if owner_status in {"complete", "current"} and edge_status != owner_status:
                errors.append(
                    f"{identifier}: status {edge_status} must match {task} "
                    f"status {owner_status}"
                )
            if edge_status in {"complete", "current"} and owner_status != edge_status:
                errors.append(
                    f"{identifier}: status {edge_status} is ahead of {task} "
                    f"status {owner_status}"
                )
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
            if (
                milestone_by_id.get(milestone, {}).get("status") == "complete"
                and edge_status != "complete"
            ):
                errors.append(
                    f"{identifier}: completed milestone {milestone} requires "
                    "complete ownership status"
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
        assigned_owner_tasks = {
            record.get("owning_task")
            for records in (requirement_ownership, uat_ownership)
            for record in records.values()
            if record.get("owning_gate") == identifier
        }
        reachable_tasks = transitive_dependencies(task_dependencies, str(task)) | {task}
        unreachable_owners = assigned_owner_tasks - reachable_tasks
        if unreachable_owners:
            errors.append(
                f"{identifier}: acceptance task {task} does not transitively depend "
                f"on assigned owner tasks {sorted(unreachable_owners)}"
            )
        if (
            edge.get("status") == "complete"
            and task_ownership.get(task, {}).get("status") != "complete"
        ):
            errors.append(
                f"{identifier}: complete gate requires complete acceptance task {task}"
            )
        if (
            task_ownership.get(task, {}).get("status") == "complete"
            and edge.get("status") != "complete"
        ):
            errors.append(
                f"{identifier}: complete acceptance task {task} requires complete gate"
            )
        if (
            milestone_by_id.get(milestone, {}).get("status") == "complete"
            and edge.get("status") != "complete"
        ):
            errors.append(
                f"{identifier}: completed milestone {milestone} requires complete gate"
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

    # Completed work remains valid after its milestone advances. Current work
    # must belong to the one current milestone; future milestones cannot claim
    # execution or completion early.
    current_tasks: set[str] = set()
    for identifier, edge in task_ownership.items():
        status = edge.get("status")
        milestone = edge.get("milestone")
        milestone_status = milestone_by_id.get(milestone, {}).get("status")
        if status == "current":
            current_tasks.add(identifier)
        if status == "current" and milestone not in current_milestones:
            errors.append(
                f"{identifier}: current task belongs to non-current milestone "
                f"{milestone}"
            )
        if status == "complete" and milestone_status not in {"complete", "current"}:
            errors.append(
                f"{identifier}: complete task belongs to {milestone_status} "
                f"milestone {milestone}"
            )
        if milestone_status == "complete" and status not in {"complete", "superseded"}:
            errors.append(
                f"{identifier}: completed milestone {milestone} contains "
                f"non-complete task status {status}"
            )
        if milestone_status in {"blocked", "deferred"} and status in {
            "complete",
            "current",
        }:
            errors.append(
                f"{identifier}: future milestone {milestone} cannot contain "
                f"task status {status}"
            )
        if status in {"complete", "current"}:
            incomplete_task_dependencies = {
                dependency
                for dependency in task_dependencies.get(identifier, set())
                if task_ownership.get(dependency, {}).get("status")
                not in {"complete", "superseded"}
            }
            if incomplete_task_dependencies:
                errors.append(
                    f"{identifier}: {status} task has incomplete dependencies "
                    f"{sorted(incomplete_task_dependencies)}"
                )
    if not current_tasks:
        errors.append("traceability: current milestone has no current task")

    expected_proposals = {
        *(f"LRM-WS-{value:03d}" for value in range(12, 18)),
        *(f"LRM-PE-{value:03d}" for value in range(11, 22)),
        *(f"LRM-RE-{value:03d}" for value in range(6, 9)),
        *(f"LRM-IN-{value:03d}" for value in range(6, 9)),
        *(f"LRM-RM-{value:03d}" for value in range(4, 6)),
        *(f"LRM-UX-{value:03d}" for value in range(10, 16)),
        *(f"LRM-EV-{value:03d}" for value in range(10, 14)),
        *(f"LRM-CO-{value:03d}" for value in range(13, 16)),
        *(f"LRM-AU-{value:03d}" for value in range(10, 14)),
        *(f"LRM-PK-{value:03d}" for value in range(7, 10)),
        *(f"UAT-{value:03d}" for value in range(45, 73)),
        "LRM-IN-001",
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
        validate_okf_and_comparator_contract(
            requirements, cases, task_blocks, gate_blocks, errors
        )
        validate_corrected_phase_ownership(
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
