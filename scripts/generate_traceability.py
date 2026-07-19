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

ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_PATH = ROOT / "spec/traceability-ownership.json"
REPORT_PATH = ROOT / "spec/traceability-report.json"
APPENDIX_PATH = ROOT / "docs/product/traceability.md"


def load_json(relative: str) -> dict[str, Any]:
    return json.loads((ROOT / relative).read_text(encoding="utf-8"))


def parse_yaml_records(relative: str, title_key: str) -> list[dict[str, str]]:
    text = (ROOT / relative).read_text(encoding="utf-8")
    blocks = re.findall(
        r"(?ms)^  - id:\s*([^\s#]+)\s*$\n(.*?)(?=^  - id:|\Z)", text
    )
    records: list[dict[str, str]] = []
    for identifier, block in blocks:
        release = re.search(r"^\s{4}release:\s*([^\s#]+)", block, re.MULTILINE)
        title = re.search(
            rf"^\s{{4}}{re.escape(title_key)}:\s*(.+)$", block, re.MULTILINE
        )
        records.append(
            {
                "id": identifier,
                "release": release.group(1) if release else "",
                "title": title.group(1).strip() if title else "",
                "block": block,
            }
        )
    return records


def source_digest() -> str:
    paths = [
        "spec/requirements.json",
        "spec/uat-cases.json",
        "spec/localization-requirements.json",
        "spec/feature-gates.yaml",
        "spec/implementation-plan.yaml",
        "spec/traceability-ownership.json",
    ]
    digest = hashlib.sha256()
    for relative in paths:
        digest.update(relative.encode("utf-8"))
        digest.update(b"\0")
        digest.update((ROOT / relative).read_bytes())
        digest.update(b"\0")
    return digest.hexdigest()


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


def supporting_gate_edges(gates: list[dict[str, str]]) -> dict[str, list[str]]:
    result: dict[str, list[str]] = {}
    for gate in gates:
        for value in sorted(set(re.findall(r"\bUAT-[A-Z0-9-]+\b", gate["block"]))):
            result.setdefault(value, []).append(gate["id"])
    return result


def build_report() -> dict[str, Any]:
    requirements_doc = load_json("spec/requirements.json")
    uat_doc = load_json("spec/uat-cases.json")
    localization_doc = load_json("spec/localization-requirements.json")
    ownership = load_json("spec/traceability-ownership.json")
    tasks = parse_yaml_records("spec/implementation-plan.yaml", "title")
    gates = parse_yaml_records("spec/feature-gates.yaml", "name")

    requirements = requirements_doc["requirements"] + [
        {"context": localization_doc.get("context", "localization"), **item}
        for item in localization_doc.get("requirements", [])
    ]
    cases = uat_doc["cases"] + localization_doc.get("uat", [])
    requirement_support = supporting_task_edges(tasks, "requirements")
    uat_support = supporting_task_edges(tasks, "uat")
    gate_support = supporting_gate_edges(gates)

    requirement_rows = []
    for item in requirements:
        edge = ownership["requirement_ownership"][item["id"]]
        requirement_rows.append(
            {
                "id": item["id"],
                "repository_release": item["release"],
                "context": item["context"],
                "priority": item["priority"],
                "statement": item["statement"],
                **edge,
                "supporting_tasks": sorted(requirement_support.get(item["id"], [])),
            }
        )

    uat_rows = []
    for item in cases:
        edge = ownership["uat_ownership"][item["id"]]
        uat_rows.append(
            {
                "id": item["id"],
                "repository_release": item["release"],
                "persona": item["persona"],
                "title": item["title"],
                **edge,
                "supporting_tasks": sorted(uat_support.get(item["id"], [])),
                "supporting_gates": sorted(gate_support.get(item["id"], [])),
            }
        )

    task_rows = []
    for task in tasks:
        task_rows.append(
            {
                "id": task["id"],
                "repository_release": task["release"],
                "title": task["title"],
                **ownership["task_ownership"][task["id"]],
            }
        )

    gate_rows = []
    for gate in gates:
        gate_rows.append(
            {
                "id": gate["id"],
                "repository_release": gate["release"],
                "name": gate["title"],
                **ownership["gate_ownership"][gate["id"]],
            }
        )

    return {
        "schema_version": 1,
        "status": "active",
        "source_sha256": source_digest(),
        "delivery_boundary": ownership["delivery_boundary"],
        "design_sequence": ownership["design_sequence"],
        "approved_strategy": ownership["approved_strategy"],
        "counts": {
            "requirements": len(requirement_rows),
            "personas": len(uat_doc["personas"]),
            "uat_cases": len(uat_rows),
            "feature_gates": len(gate_rows),
            "implementation_tasks": len(task_rows),
            "evidence_owners": len(ownership["evidence_owners"]),
            "proposal_dispositions": len(ownership["proposal_dispositions"]),
        },
        "milestones": ownership["milestones"],
        "evidence_owners": ownership["evidence_owners"],
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
    report_text, appendix_text = expected_outputs()
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
