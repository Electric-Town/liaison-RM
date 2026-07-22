#!/usr/bin/env python3
"""Generate deterministic Liaison RM traceability evidence.

The ownership source is explicit. Task and gate references remain supporting
edges and never become owners by list position or inference.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Any

from check_spec import (
    TRACEABILITY_FIXED_SOURCE_PATHS,
    gate_evidence_required_map,
    json_catalog_bundle_issues,
    json_catalog_root_issues,
    parse_simple_yaml_records,
    repository_custody_validation,
    repository_regular_file_path,
    restricted_record_scalar,
    traceability_ownership_schema_issues,
    validate_corrected_phase_ownership,
    validate_traceability,
)

ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_PATH = ROOT / "spec/traceability-ownership.json"
REPORT_PATH = ROOT / "spec/traceability-report.json"
APPENDIX_PATH = ROOT / "docs/product/traceability.md"


class DuplicateJsonKeyError(json.JSONDecodeError):
    """Raised before a generated report can hide duplicate source keys."""

    def __init__(self, key: str):
        super().__init__(f"duplicate JSON key {key!r}", "", 0)


def reject_duplicate_json_keys(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise DuplicateJsonKeyError(key)
        result[key] = value
    return result


def load_json(relative: str) -> dict[str, Any]:
    source_path = repository_regular_file_path(relative, root=ROOT)
    if source_path is None:
        raise ValueError(
            f"{relative} is not a visible, non-ignored regular repository-owned "
            "file without symlink components"
        )
    document = json.loads(
        source_path.read_text(encoding="utf-8"),
        object_pairs_hook=reject_duplicate_json_keys,
    )
    if relative in {
        "spec/requirements.json",
        "spec/uat-cases.json",
        "spec/localization-requirements.json",
    }:
        issues = json_catalog_root_issues(relative, document)
        if issues:
            raise ValueError("; ".join(issues))
    elif relative == "spec/traceability-ownership.json":
        issues = traceability_ownership_schema_issues(document)
        if issues:
            raise ValueError("; ".join(issues))
    return document


def load_traceability_ownership() -> dict[str, Any]:
    """Validate ownership even when a caller replaces the generic loader."""

    document = load_json("spec/traceability-ownership.json")
    issues = traceability_ownership_schema_issues(document)
    if issues:
        raise ValueError("; ".join(issues))
    return document


def parse_yaml_records(relative: str, title_key: str) -> list[dict[str, str]]:
    path = repository_regular_file_path(relative, root=ROOT)
    if path is None:
        raise ValueError(
            f"{relative} is not a visible, non-ignored regular repository-owned "
            "file without symlink components"
        )
    record_key = "tasks" if relative.endswith("implementation-plan.yaml") else "gates"
    blocks = parse_simple_yaml_records(path, record_key)
    records: list[dict[str, str]] = []
    for identifier, block in blocks.items():
        records.append(
            {
                "id": identifier,
                "release": restricted_record_scalar(block, "release"),
                "title": restricted_record_scalar(block, title_key),
                "block": block,
            }
        )
    return records


def records_from_blocks(
    blocks: dict[str, str], title_key: str
) -> list[dict[str, str]]:
    """Render rows from the exact blocks that passed semantic validation."""

    return [
        {
            "id": identifier,
            "release": restricted_record_scalar(block, "release"),
            "title": restricted_record_scalar(block, title_key),
            "block": block,
        }
        for identifier, block in blocks.items()
    ]


@repository_custody_validation
def load_validated_authority_inputs() -> tuple[
    dict[str, Any],
    dict[str, Any],
    dict[str, Any],
    dict[str, Any],
    dict[str, str],
    dict[str, str],
]:
    """Load and semantically validate every source before rendering evidence."""

    requirements_doc = load_json("spec/requirements.json")
    uat_doc = load_json("spec/uat-cases.json")
    localization_doc = load_json("spec/localization-requirements.json")
    ownership = load_traceability_ownership()
    structural_errors: list[str] = []
    for relative, document in (
        ("spec/requirements.json", requirements_doc),
        ("spec/uat-cases.json", uat_doc),
        ("spec/localization-requirements.json", localization_doc),
    ):
        structural_errors.extend(json_catalog_root_issues(relative, document))
    structural_errors.extend(traceability_ownership_schema_issues(ownership))
    structural_errors.extend(
        json_catalog_bundle_issues(requirements_doc, uat_doc, localization_doc)
    )
    if structural_errors:
        raise ValueError(
            "authority inputs fail structural validation: "
            + "; ".join(sorted(set(structural_errors)))
        )

    implementation_path = repository_regular_file_path(
        "spec/implementation-plan.yaml", root=ROOT
    )
    gates_path = repository_regular_file_path("spec/feature-gates.yaml", root=ROOT)
    if implementation_path is None or gates_path is None:
        raise ValueError(
            "implementation plan and feature gates must be visible, non-ignored "
            "regular repository-owned files without symlink components"
        )
    task_blocks = parse_simple_yaml_records(implementation_path, "tasks")
    gate_blocks = parse_simple_yaml_records(gates_path, "gates")

    requirements = requirements_doc["requirements"] + [
        {"context": localization_doc.get("context", "localization"), **item}
        for item in localization_doc.get("requirements", [])
    ]
    cases = uat_doc["cases"] + localization_doc.get("uat", [])
    persona_ids = {item["id"] for item in uat_doc["personas"]}
    errors: list[str] = []
    validate_corrected_phase_ownership(
        requirements,
        cases,
        task_blocks,
        gate_blocks,
        errors,
        ownership_document=ownership,
    )
    validate_traceability(
        requirements,
        cases,
        {item["id"] for item in requirements},
        {item["id"] for item in cases},
        persona_ids,
        task_blocks,
        gate_blocks,
        errors,
        ownership_document=ownership,
        validate_projection=False,
        validate_generated_outputs=False,
        validate_repository_truth=True,
    )
    if errors:
        raise ValueError(
            "authority inputs fail semantic validation: " + "; ".join(errors)
        )
    return (
        requirements_doc,
        uat_doc,
        localization_doc,
        ownership,
        task_blocks,
        gate_blocks,
    )


def _source_digest_for_validated_ownership(ownership: dict[str, Any]) -> str:
    paths = list(TRACEABILITY_FIXED_SOURCE_PATHS)
    paths.extend(sorted(ownership["authority_sources"]))
    digest = hashlib.sha256()
    for relative in paths:
        source_path = repository_regular_file_path(relative, root=ROOT)
        if source_path is None:
            raise ValueError(
                f"{relative} is not a visible, non-ignored regular repository-owned "
                "file without symlink components"
            )
        digest.update(relative.encode("utf-8"))
        digest.update(b"\0")
        digest.update(source_path.read_bytes())
        digest.update(b"\0")
    return digest.hexdigest()


@repository_custody_validation
def source_digest() -> str:
    """Digest only the exact source bundle that passed full semantic validation."""

    _, _, _, ownership, _, _ = load_validated_authority_inputs()
    return _source_digest_for_validated_ownership(ownership)


def supporting_task_edges(tasks: list[dict[str, str]], field: str) -> dict[str, list[str]]:
    result: dict[str, list[str]] = {}
    pattern = re.compile(rf"^\s{{4}}{re.escape(field)}:\s*\[([^]]*)\]", re.MULTILINE)
    for task in tasks:
        values: list[str] = []
        match = pattern.search(task["block"])
        if match is not None:
            values.extend(
                item.strip() for item in match.group(1).split(",") if item.strip()
            )
        evidence_match = re.search(
            r"^\s{4}evidence_dependencies:\s*\[([^]]*)\]",
            task["block"],
            re.MULTILINE,
        )
        if evidence_match is not None:
            values.extend(
                item.strip()
                for item in evidence_match.group(1).split(",")
                if item.strip()
            )
        for value in values:
            result.setdefault(value, []).append(task["id"])
    return result


def supporting_gate_edges(
    gate_blocks: dict[str, str],
) -> dict[str, list[str]]:
    result: dict[str, list[str]] = {}
    for gate, values in gate_evidence_required_map(gate_blocks).items():
        for value in sorted(values):
            if re.fullmatch(r"UAT-[A-Z0-9-]+", value):
                result.setdefault(value, []).append(gate)
    return result


@repository_custody_validation
def build_report() -> dict[str, Any]:
    (
        requirements_doc,
        uat_doc,
        localization_doc,
        ownership,
        task_blocks,
        gate_blocks,
    ) = load_validated_authority_inputs()
    tasks = records_from_blocks(task_blocks, "title")
    gates = records_from_blocks(gate_blocks, "name")

    requirements = requirements_doc["requirements"] + [
        {"context": localization_doc.get("context", "localization"), **item}
        for item in localization_doc.get("requirements", [])
    ]
    cases = uat_doc["cases"] + localization_doc.get("uat", [])
    requirement_support = supporting_task_edges(tasks, "requirements")
    uat_support = supporting_task_edges(tasks, "uat")
    gate_support = supporting_gate_edges(gate_blocks)

    requirement_rows = []
    for item in requirements:
        edge = ownership["requirement_ownership"][item["id"]]
        requirement_rows.append(
            {
                **edge,
                "id": item["id"],
                "repository_release": item["release"],
                "context": item["context"],
                "priority": item["priority"],
                "statement": item["statement"],
                "supporting_tasks": sorted(
                    set(requirement_support.get(item["id"], []))
                    - {edge["owning_task"]}
                ),
            }
        )

    uat_rows = []
    for item in cases:
        edge = ownership["uat_ownership"][item["id"]]
        uat_rows.append(
            {
                **edge,
                "id": item["id"],
                "repository_release": item["release"],
                "persona": item["persona"],
                "title": item["title"],
                "supporting_tasks": sorted(
                    set(uat_support.get(item["id"], [])) - {edge["owning_task"]}
                ),
                "supporting_gates": sorted(
                    set(gate_support.get(item["id"], [])) - {edge["owning_gate"]}
                ),
            }
        )

    task_rows = []
    for task in tasks:
        task_rows.append(
            {
                **ownership["task_ownership"][task["id"]],
                "id": task["id"],
                "repository_release": task["release"],
                "title": task["title"],
            }
        )

    gate_rows = []
    for gate in gates:
        gate_rows.append(
            {
                **ownership["gate_ownership"][gate["id"]],
                "id": gate["id"],
                "repository_release": gate["release"],
                "name": gate["title"],
            }
        )

    return {
        "schema_version": ownership["schema_version"],
        "status": ownership["status"],
        "source_sha256": _source_digest_for_validated_ownership(ownership),
        "delivery_boundary": ownership["delivery_boundary"],
        "design_sequence": ownership["design_sequence"],
        "p03_resume_authority": ownership["p03_resume_authority"],
        "approved_strategy": ownership["approved_strategy"],
        "counts": {
            "requirements": len(requirement_rows),
            "personas": len(uat_doc["personas"]),
            "uat_cases": len(uat_rows),
            "feature_gates": len(gate_rows),
            "implementation_tasks": len(task_rows),
            "evidence_owners": len(ownership["evidence_owners"]),
            "proposal_dispositions": len(ownership["proposal_dispositions"]),
            "cross_release_dispositions": len(
                ownership["cross_release_ownership_dispositions"]
            ),
            "forward_evidence_dispositions": len(
                ownership["forward_evidence_dispositions"]
            ),
            "authority_sources": len(ownership["authority_sources"]),
        },
        "milestones": ownership["milestones"],
        "evidence_owners": ownership["evidence_owners"],
        "cross_release_ownership_dispositions": ownership[
            "cross_release_ownership_dispositions"
        ],
        "forward_evidence_dispositions": ownership[
            "forward_evidence_dispositions"
        ],
        "authority_sources": ownership["authority_sources"],
        "requirements": requirement_rows,
        "uat_cases": uat_rows,
        "feature_gates": gate_rows,
        "implementation_tasks": task_rows,
        "proposal_dispositions": ownership["proposal_dispositions"],
        "review_branch_dispositions": ownership["review_branch_dispositions"],
    }


def markdown_cell(value: object) -> str:
    text = str(value).replace("|", "\\|").replace("\n", " ")
    return text


def render_markdown(report: dict[str, Any]) -> str:
    counts = report["counts"]
    resume = report["p03_resume_authority"]
    push_ci = resume["merge_result_ci"]
    notarized = resume["notarized_bundle_preflight"]
    identity = resume["qualified_artifact_identity"]
    observation_receipt = resume["observation_receipt"]
    replacement_task = resume["replacement_task"]
    stopped_disposition = resume["stopped_disposition"]
    lines = [
        "# Normative traceability appendix",
        "",
        "This file is generated by `python3 scripts/generate_traceability.py`. Edit the source catalogs or `spec/traceability-ownership.json`, not this file.",
        "",
        f"Source SHA-256: `{report['source_sha256']}`",
        "",
        f"Delivery boundary: {report['delivery_boundary']}",
        "",
        f"Design sequence: {report['design_sequence']}",
        "",
        "## P03 D1-B resume authority",
        "",
        f"Decision: `{resume['decision']}`; source PR: `#{resume['source_pull_request']}`; baseline: `{resume['baseline_commit']}`.",
        "",
        f"Technical P03 acceptance: `{resume['p03_technical_acceptance']}`; qualified artifact identity: `{identity if identity is not None else 'pending'}`.",
        "",
        f"Observation task: `{resume['observation_task']}`; decision: `{resume['observation_decision']}`; receipt: `{observation_receipt if observation_receipt is not None else 'pending'}`; P03D eligibility: `{resume['p03d_eligibility']}`.",
        "",
        "Observation receipt fields: "
        + ", ".join(f"`{item}`" for item in resume["observation_receipt_contract"])
        + ".",
        "",
        f"Change replacement task: `{replacement_task if replacement_task is not None else 'none'}`; Stop disposition: `{stopped_disposition if stopped_disposition is not None else 'none'}`.",
        "",
        "Required identity fields: "
        + ", ".join(f"`{item}`" for item in resume["qualification_identity_contract"])
        + ".",
        "",
        "Exact ordinary-push CI: "
        + f"`{push_ci['conclusion']}` across runs "
        + ", ".join(f"`{item}`" for item in push_ci["workflow_run_ids"])
        + ".",
        "",
        "Separately dispatched notarized-bundle preflight: "
        + f"run `{notarized['run_id']}` `{notarized['conclusion']}` at "
        + f"`{notarized['failure_boundary']}`; release evidence `{notarized['release_evidence']}`.",
        "",
        "Approved strategy: {title} (`{sha}`; {status})".format(
            title=report["approved_strategy"]["title"],
            sha=report["approved_strategy"]["approved_sha256"],
            status=report["approved_strategy"]["status"],
        ),
        "",
        f"Strategy scope: {report['approved_strategy']['scope']}",
        "",
        "## Coverage summary",
        "",
        "| Contract | Count |",
        "|---|---:|",
        f"| Requirements | {counts['requirements']} |",
        f"| Personas | {counts['personas']} |",
        f"| UAT cases | {counts['uat_cases']} |",
        f"| Feature gates | {counts['feature_gates']} |",
        f"| Implementation tasks | {counts['implementation_tasks']} |",
        f"| Evidence owners | {counts['evidence_owners']} |",
        f"| Exact cross-release dispositions | {counts['cross_release_dispositions']} |",
        f"| Exact forward-evidence dispositions | {counts['forward_evidence_dispositions']} |",
        f"| Bound authority sources | {counts['authority_sources']} |",
        f"| Founder-plan dispositions | {counts['proposal_dispositions']} |",
        "",
        "## Authoritative milestone order",
        "",
        "| Milestone | Status | Depends on | Outcome |",
        "|---|---|---|---|",
    ]
    for item in report["milestones"]:
        lines.append(
            f"| {item['id']} | {item['status']} | {', '.join(item['depends_on']) or '—'} | {markdown_cell(item['name'])} |"
        )

    lines += [
        "",
        "## Exact cross-release ownership dispositions",
        "",
        "Every row is identifier-exact. The repository checker rejects missing, stale, wildcard, or mismatched dispositions.",
        "",
        "| Contract | Kind | Repository release | Owning task (release) | Owning gate (release) | Authority | Rationale |",
        "|---|---|---|---|---|---|---|",
    ]
    for identifier, item in report[
        "cross_release_ownership_dispositions"
    ].items():
        row = {
            **item,
            "identifier": identifier,
            "rationale": markdown_cell(item["rationale"]),
        }
        lines.append(
            "| {identifier} | {artifact_type} | {repository_release} | "
            "{owning_task} ({task_release}) | {owning_gate} ({gate_release}) | "
            "{authority} | {rationale} |".format(**row)
        )

    lines += [
        "",
        "## Exact forward-evidence dispositions",
        "",
        "Later-milestone evidence is non-blocking input only for these exact task and contract pairs. The repository checker rejects missing, stale, wildcard, or mismatched edges.",
        "",
        "| Task (milestone) | Contract | Kind | Owner (milestone) | Relationship | Authority | Rationale |",
        "|---|---|---|---|---|---|---|",
    ]
    for identifier, item in report["forward_evidence_dispositions"].items():
        row = {
            **item,
            "identifier": identifier,
            "rationale": markdown_cell(item["rationale"]),
        }
        lines.append(
            "| {task} ({task_milestone}) | {artifact_id} | {artifact_type} | "
            "{artifact_owner_task} ({artifact_milestone}) | {relationship} | "
            "{authority} | {rationale} |".format(**row)
        )

    lines += [
        "",
        "## Bound authority sources",
        "",
        "Each source is content-addressed and contains every identifier-exact anchor in its group. Accepted decisions must retain accepted status.",
        "",
        "| Source | Kind | SHA-256 | Anchors |",
        "|---|---|---|---:|",
    ]
    for path, item in report["authority_sources"].items():
        lines.append(
            f"| {path} | {item['kind']} | {item['sha256']} | {len(item['anchors'])} |"
        )

    lines += [
        "",
        "## Requirements",
        "",
        "Supporting tasks are non-owning evidence edges.",
        "",
        "| Requirement | Repository release | Milestone | Owning gate | Owning task | Evidence owner | Status | Supporting tasks |",
        "|---|---|---|---|---|---|---|---|",
    ]
    for item in report["requirements"]:
        lines.append(
            "| {id} | {repository_release} | {milestone} | {owning_gate} | {owning_task} | {evidence_owner} | {status} | {support} |".format(
                **item, support=", ".join(item["supporting_tasks"]) or "—"
            )
        )

    lines += [
        "",
        "## UAT cases",
        "",
        "Supporting tasks and gates may share evidence; they do not change atomic ownership.",
        "",
        "| UAT | Repository release | Milestone | Owning gate | Owning task | Evidence owner | Status | Supporting tasks | Supporting gates |",
        "|---|---|---|---|---|---|---|---|---|",
    ]
    for item in report["uat_cases"]:
        lines.append(
            "| {id} | {repository_release} | {milestone} | {owning_gate} | {owning_task} | {evidence_owner} | {status} | {tasks} | {gates} |".format(
                **item,
                tasks=", ".join(item["supporting_tasks"]) or "—",
                gates=", ".join(item["supporting_gates"]) or "—",
            )
        )

    lines += [
        "",
        "## Feature gates",
        "",
        "| Gate | Repository release | Milestone | Acceptance task | Evidence owner | Status |",
        "|---|---|---|---|---|---|",
    ]
    for item in report["feature_gates"]:
        lines.append(
            "| {id} | {repository_release} | {milestone} | {acceptance_task} | {evidence_owner} | {status} |".format(
                **item
            )
        )

    lines += [
        "",
        "## Implementation tasks",
        "",
        "| Task | Repository release | Milestone | Owning gate | Evidence owner | Status | Disposition |",
        "|---|---|---|---|---|---|---|",
    ]
    for item in report["implementation_tasks"]:
        disposition = item.get("disposition", "—")
        if item.get("superseded_by"):
            disposition = f"Superseded by {item['superseded_by']}. {disposition}"
        row = {**item, "disposition": markdown_cell(disposition)}
        lines.append(
            "| {id} | {repository_release} | {milestone} | {owning_gate} | {evidence_owner} | {status} | {disposition} |".format(
                **row
            )
        )

    lines += [
        "",
        "## Founder-plan proposal dispositions",
        "",
        "| Proposal | Disposition | Canonical contract | Rationale |",
        "|---|---|---|---|",
    ]
    for item in report["proposal_dispositions"]:
        lines.append(
            f"| {item['proposal_id']} | {item['disposition']} | {item['canonical_id']} | {markdown_cell(item['rationale'])} |"
        )

    lines += [
        "",
        "## Review-branch dispositions",
        "",
        "| Reference | Branch state | Execution state | Milestone | Condition |",
        "|---|---|---|---|---|",
    ]
    for item in report["review_branch_dispositions"]:
        lines.append(
            f"| {item['reference']} | {item['status']} | {item['execution_status']} | {item['milestone']} | {markdown_cell(item['condition'])} |"
        )
    lines.append("")
    return "\n".join(lines)


@repository_custody_validation
def expected_outputs() -> tuple[str, str]:
    report = build_report()
    report_text = json.dumps(report, indent=2, ensure_ascii=False) + "\n"
    return report_text, render_markdown(report)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--check", action="store_true", help="Fail when committed outputs are stale."
    )
    args = parser.parse_args()
    try:
        report_text, appendix_text = expected_outputs()
    except (KeyError, OSError, ValueError, json.JSONDecodeError) as exc:
        print(f"Traceability generation failed: {exc}")
        return 1
    expected = ((REPORT_PATH, report_text), (APPENDIX_PATH, appendix_text))

    if args.check:
        stale = [
            path.relative_to(ROOT).as_posix()
            for path, text in expected
            if not path.is_file() or path.read_text(encoding="utf-8") != text
        ]
        if stale:
            print("Traceability generation check failed; regenerate:")
            for value in stale:
                print(f"- {value}")
            return 1
        print("Traceability generation check passed")
        return 0

    for path, text in expected:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
    print(
        "Generated traceability evidence: "
        f"{REPORT_PATH.relative_to(ROOT)} and {APPENDIX_PATH.relative_to(ROOT)}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
