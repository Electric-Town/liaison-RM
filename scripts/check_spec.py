#!/usr/bin/env python3
"""Validate Liaison RM machine-readable product specifications."""

from __future__ import annotations

import json
import hashlib
import math
import os
import re
import stat
import subprocess
import sys
import unicodedata
from functools import wraps
from pathlib import Path, PurePosixPath

ROOT = Path(__file__).resolve().parents[1]
ID = re.compile(r"^[A-Z0-9]+(?:-[A-Z0-9]+)+$")
MILESTONE_ID = re.compile(r"^[A-Z0-9]+(?:-[A-Z0-9]+)*$")
VALID_PRIORITIES = {"must", "should", "could", "wont"}
VALID_RELEASES = {
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
VALID_CONTRACT_STATUSES = {"complete", "current", "blocked", "deferred"}
VALID_MILESTONE_STATUSES = {"complete", "current", "blocked", "deferred"}
APPROVED_STRATEGY_STATUS = "integrated-by-g0-contract-reconciliation"
PORTABLE_REPOSITORY_COMPONENT = re.compile(r"^[A-Za-z0-9._-]+$")
WINDOWS_RESERVED_STEMS = {
    "CON",
    "PRN",
    "AUX",
    "NUL",
    "CLOCK$",
    "CONIN$",
    "CONOUT$",
    *(f"COM{index}" for index in range(1, 10)),
    *(f"LPT{index}" for index in range(1, 10)),
}
TRACEABILITY_FIXED_SOURCE_PATHS = (
    "spec/requirements.json",
    "spec/uat-cases.json",
    "spec/localization-requirements.json",
    "spec/feature-gates.yaml",
    "spec/implementation-plan.yaml",
    "spec/traceability-ownership.json",
)
_CUSTODY_SESSION_DEPTH = 0
_CUSTODY_SESSION_PATHS: dict[str, frozenset[bytes] | None] = {}


def repository_custody_validation(function):
    """Give one top-level validation a fresh, quiescent Git path snapshot."""

    @wraps(function)
    def wrapper(*args, **kwargs):
        global _CUSTODY_SESSION_DEPTH
        outermost = _CUSTODY_SESSION_DEPTH == 0
        if outermost:
            _CUSTODY_SESSION_PATHS.clear()
        _CUSTODY_SESSION_DEPTH += 1
        try:
            return function(*args, **kwargs)
        finally:
            _CUSTODY_SESSION_DEPTH -= 1
            if outermost:
                _CUSTODY_SESSION_PATHS.clear()

    return wrapper


def is_lower_hex(value: object, length: int) -> bool:
    """Return true only for an exact JSON string containing lowercase hex."""

    return type(value) is str and re.fullmatch(rf"[0-9a-f]{{{length}}}", value) is not None


def is_safe_repository_relative_path(value: object) -> bool:
    """Accept a portable ASCII, canonical POSIX repository-relative path."""

    if type(value) is not str or not value or value != value.strip():
        return False
    if unicodedata.normalize("NFC", value) != value:
        return False
    if "\\" in value or any(
        ord(character) < 32 or ord(character) == 127 for character in value
    ):
        return False
    if re.match(r"^[A-Za-z]:", value):
        return False
    path = PurePosixPath(value)
    if not (
        not path.is_absolute()
        and bool(path.parts)
        and all(part not in {"", ".", ".."} for part in path.parts)
        and path.as_posix() == value
    ):
        return False
    for part in path.parts:
        if not PORTABLE_REPOSITORY_COMPONENT.fullmatch(part):
            return False
        if part.endswith((".", " ")):
            return False
        if part.split(".", 1)[0].upper() in WINDOWS_RESERVED_STEMS:
            return False
    return True


def git_visible_repository_paths(root: Path) -> frozenset[bytes] | None:
    """Return exact tracked and visible-untracked Git path bytes.

    No process-global cache is used: index and ignore changes in a long-running
    validation process must take effect on the next check.
    """

    try:
        root = root.resolve(strict=True)
    except OSError:
        return None
    cache_key = str(root)
    if _CUSTODY_SESSION_DEPTH and cache_key in _CUSTODY_SESSION_PATHS:
        return _CUSTODY_SESSION_PATHS[cache_key]

    def finish(result: frozenset[bytes] | None) -> frozenset[bytes] | None:
        if _CUSTODY_SESSION_DEPTH:
            _CUSTODY_SESSION_PATHS[cache_key] = result
        return result

    command = ["git", "-C", str(root)]
    try:
        worktree = subprocess.run(
            command + ["rev-parse", "--show-toplevel"],
            check=False,
            capture_output=True,
            text=False,
        )
    except OSError:
        return finish(None)
    if worktree.returncode != 0:
        return finish(None)
    try:
        git_root = Path(os.fsdecode(worktree.stdout.rstrip(b"\r\n"))).resolve(
            strict=True
        )
    except (OSError, ValueError):
        return finish(None)
    if git_root != root:
        return finish(None)
    try:
        visible = subprocess.run(
            command
            + [
                "--literal-pathspecs",
                "ls-files",
                "--cached",
                "--others",
                "--exclude-standard",
                "-z",
            ],
            check=False,
            capture_output=True,
            text=False,
        )
    except OSError:
        return finish(None)
    if visible.returncode != 0 or (
        visible.stdout and not visible.stdout.endswith(b"\0")
    ):
        return finish(None)
    return finish(
        frozenset(entry for entry in visible.stdout.split(b"\0") if entry)
    )


def repository_path_collision_key(value: str) -> str:
    """Return the portable checkout-equivalence key for a canonical path."""

    return "/".join(
        unicodedata.normalize("NFC", part).casefold()
        for part in PurePosixPath(value).parts
    )


def repository_path_collision_issues(paths: object) -> list[str]:
    """Reject distinct authority spellings that alias on portable checkouts."""

    if not isinstance(paths, (list, tuple, set, frozenset)):
        return ["authority source paths must be a collection"]
    buckets: dict[str, set[str]] = {}
    for value in paths:
        if not is_safe_repository_relative_path(value):
            continue
        buckets.setdefault(repository_path_collision_key(value), set()).add(value)
    return [
        "authority source paths collide under NFC casefold: "
        + ", ".join(sorted(spellings))
        for spellings in buckets.values()
        if len(spellings) > 1
    ]


def repository_regular_file_path(
    value: object, *, root: Path | None = None, source_archive_mode: bool = False
) -> Path | None:
    """Resolve an exact Git-visible repository file without following symlinks.

    ``source_archive_mode`` is an explicit no-Git fallback for inspecting a
    source archive. Normal validation and generation never enable it and thus
    never claim Git custody without a worktree inventory.
    """

    if not is_safe_repository_relative_path(value):
        return None
    repository_root = ROOT if root is None else root
    try:
        resolved_root = repository_root.resolve(strict=True)
    except OSError:
        return None

    parts = PurePosixPath(value).parts
    candidate = resolved_root
    for index, part in enumerate(parts):
        candidate = candidate / part
        try:
            metadata = candidate.lstat()
        except OSError:
            return None
        if stat.S_ISLNK(metadata.st_mode):
            return None
        is_junction = getattr(candidate, "is_junction", None)
        if is_junction is not None:
            try:
                if is_junction():
                    return None
            except OSError:
                return None
        if index < len(parts) - 1 and not stat.S_ISDIR(metadata.st_mode):
            return None

    try:
        resolved_candidate = candidate.resolve(strict=True)
        resolved_candidate.relative_to(resolved_root)
        final_metadata = candidate.lstat()
    except (OSError, ValueError):
        return None
    if not stat.S_ISREG(final_metadata.st_mode):
        return None
    visible_paths = git_visible_repository_paths(resolved_root)
    if visible_paths is None:
        if not source_archive_mode:
            return None
    else:
        requested_bytes = os.fsencode(value)
        if requested_bytes not in visible_paths:
            return None
        requested_key = repository_path_collision_key(value)
        visible_aliases: set[bytes] = set()
        for visible_path in visible_paths:
            try:
                visible_text = visible_path.decode("utf-8", errors="strict")
            except UnicodeDecodeError:
                continue
            if repository_path_collision_key(visible_text) == requested_key:
                visible_aliases.add(visible_path)
        if visible_aliases != {requested_bytes}:
            return None
    # This is a static repository-integrity check. A hostile concurrent path
    # swap would require descriptor-relative no-follow reads and is outside its
    # declared scope; every digest sink therefore re-runs this resolver.
    return resolved_candidate


class DuplicateJsonKeyError(json.JSONDecodeError):
    """Raised before JSON objects can collapse duplicate ownership keys."""

    def __init__(self, key: str):
        super().__init__(f"duplicate JSON key {key!r}", "", 0)


def reject_duplicate_json_keys(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise DuplicateJsonKeyError(key)
        result[key] = value
    return result


def load_json(relative: str):
    path = repository_regular_file_path(relative)
    if path is None:
        raise ValueError(
            f"{relative} is not a visible, non-ignored regular repository-owned "
            "file without symlink components"
        )
    with path.open(encoding="utf-8") as handle:
        return json.load(handle, object_pairs_hook=reject_duplicate_json_keys)


def json_catalog_root_issues(relative: str, document: object) -> list[str]:
    """Validate exact root and item contracts for a canonical JSON catalog."""

    configurations = {
        "spec/requirements.json": {
            "keys": {"schema_version", "status", "requirements"},
            "exact": {"schema_version": 1, "status": "active"},
            "lists": {"requirements"},
        },
        "spec/uat-cases.json": {
            "keys": {"schema_version", "status", "personas", "cases"},
            "exact": {"schema_version": 1, "status": "active"},
            "lists": {"personas", "cases"},
        },
        "spec/localization-requirements.json": {
            "keys": {"schema_version", "status", "context", "requirements", "uat"},
            "exact": {
                "schema_version": 1,
                "status": "draft",
                "context": "localization",
            },
            "lists": {"requirements", "uat"},
        },
    }
    configuration = configurations.get(relative)
    if configuration is None:
        return [f"{relative}: no canonical JSON root contract is registered"]
    if not isinstance(document, dict):
        return [f"{relative}: root must be a JSON object"]

    issues: list[str] = []
    if set(document) != configuration["keys"]:
        issues.append(
            f"{relative}: top-level fields must be exactly "
            f"{sorted(configuration['keys'])}"
        )
    for field, expected in configuration["exact"].items():
        actual = document.get(field)
        if type(actual) is not type(expected) or actual != expected:
            issues.append(
                f"{relative}: {field} must be exactly {expected!r}, "
                f"not {actual!r}"
            )
    for field in configuration["lists"]:
        if not isinstance(document.get(field), list):
            issues.append(f"{relative}: {field} must be a JSON array")

    item_contracts = {
        "spec/requirements.json": {
            "requirements": {
                "fields": {
                    "id",
                    "context",
                    "release",
                    "priority",
                    "statement",
                    "acceptance",
                },
                "strings": {
                    "id",
                    "context",
                    "release",
                    "priority",
                    "statement",
                    "acceptance",
                },
            }
        },
        "spec/uat-cases.json": {
            "personas": {
                "fields": {"id", "name"},
                "strings": {"id", "name"},
            },
            "cases": {
                "fields": {
                    "id",
                    "persona",
                    "release",
                    "title",
                    "given",
                    "when",
                    "then",
                },
                "strings": {
                    "id",
                    "persona",
                    "release",
                    "title",
                    "given",
                    "when",
                    "then",
                },
            },
        },
        "spec/localization-requirements.json": {
            "requirements": {
                "fields": {
                    "id",
                    "release",
                    "priority",
                    "statement",
                    "acceptance",
                },
                "strings": {
                    "id",
                    "release",
                    "priority",
                    "statement",
                    "acceptance",
                },
            },
            "uat": {
                "fields": {
                    "id",
                    "persona",
                    "release",
                    "title",
                    "given",
                    "when",
                    "then",
                },
                "strings": {
                    "id",
                    "persona",
                    "release",
                    "title",
                    "given",
                    "when",
                    "then",
                },
            },
        },
    }
    for collection, contract in item_contracts[relative].items():
        records = document.get(collection)
        if not isinstance(records, list):
            continue
        seen_ids: set[str] = set()
        for index, record in enumerate(records):
            label = f"{relative}:{collection}[{index}]"
            if not isinstance(record, dict):
                issues.append(f"{label}: item must be a JSON object")
                continue
            if set(record) != contract["fields"]:
                issues.append(
                    f"{label}: fields must be exactly {sorted(contract['fields'])}"
                )
            for field in contract["strings"]:
                value = record.get(field)
                if type(value) is not str or not value.strip():
                    issues.append(f"{label}: {field} must be a non-empty JSON string")
            identifier = record.get("id")
            if type(identifier) is str:
                if not ID.fullmatch(identifier):
                    issues.append(f"{label}: id must use the canonical identifier form")
                if identifier in seen_ids:
                    issues.append(f"{label}: duplicate id {identifier}")
                seen_ids.add(identifier)
            if "release" in record and record.get("release") not in VALID_RELEASES:
                issues.append(f"{label}: release must be a canonical release identifier")
            if "priority" in record and record.get("priority") not in VALID_PRIORITIES:
                issues.append(f"{label}: priority must be must, should, could, or wont")
    return issues


def json_catalog_bundle_issues(
    requirements_doc: object, uat_doc: object, localization_doc: object
) -> list[str]:
    """Validate cross-catalog identities and persona references for all consumers."""

    if not all(isinstance(item, dict) for item in (requirements_doc, uat_doc, localization_doc)):
        return ["spec catalogs: every root must be a JSON object"]
    issues: list[str] = []

    def duplicate_bundle_ids(label: str, records: list[object]) -> set[str]:
        seen: set[str] = set()
        for record in records:
            if not isinstance(record, dict) or type(record.get("id")) is not str:
                continue
            identifier = record["id"]
            if identifier in seen:
                issues.append(f"spec catalogs: duplicate {label} id {identifier}")
            seen.add(identifier)
        return seen

    requirement_records = list(requirements_doc.get("requirements", [])) + list(
        localization_doc.get("requirements", [])
    )
    case_records = list(uat_doc.get("cases", [])) + list(
        localization_doc.get("uat", [])
    )
    persona_records = list(uat_doc.get("personas", []))
    duplicate_bundle_ids("requirement", requirement_records)
    case_ids = duplicate_bundle_ids("UAT", case_records)
    persona_ids = duplicate_bundle_ids("persona", persona_records)
    for record in case_records:
        if isinstance(record, dict) and record.get("persona") not in persona_ids:
            issues.append(
                f"{record.get('id')}: unknown persona {record.get('persona')} in UAT catalog"
            )
    if len(case_ids) != len(case_records):
        # The identifier-specific errors above are the actionable evidence; this
        # guard keeps a malformed non-object record from making counts look valid.
        non_object_count = sum(not isinstance(item, dict) for item in case_records)
        if non_object_count:
            issues.append("spec catalogs: every UAT case must be a JSON object")
    return issues


TRACEABILITY_OWNERSHIP_ROOT_FIELDS = {
    "schema_version",
    "status",
    "delivery_boundary",
    "design_sequence",
    "p03_resume_authority",
    "approved_strategy",
    "authority_sources",
    "cross_release_ownership_dispositions",
    "forward_evidence_dispositions",
    "milestones",
    "evidence_owners",
    "task_ownership",
    "requirement_ownership",
    "uat_ownership",
    "gate_ownership",
    "proposal_dispositions",
    "review_branch_dispositions",
}

CANONICAL_DELIVERY_BOUNDARY = (
    "B0 Workplace Review Alpha before A0 Personal Memory Alpha; provider, mobile, "
    "AI, MCP, and theme packages remain later."
)

CANONICAL_DESIGN_SEQUENCE = (
    "After T-B0-P03 is technically accepted with the qualified-code SHA, "
    "merge-result SHA, attestation SHA, and exact executable receipt, "
    "T-B0-P03-OBS becomes current and observes that same artifact under D1-B. "
    "Every Continue, Change, or Stop outcome completes OBS; only Continue makes "
    "T-B0-P03D eligible to run design consultation, create DESIGN.md, and complete "
    "plan design review. Change advances one bound replacement task, Stop records "
    "a structured preservation/support disposition, and T-B0-P04 cannot start "
    "before completed P03D. G0 does not create DESIGN.md."
)

CANONICAL_SUPERSEDED_TASKS = {
    "T-R0-002": "T-B0-P00",
    "T-R1-001": "T-B0-P01",
    "T-R1-002": "T-B0-P02",
    "T-R1-003": "T-B0-P03",
    "T-R1-004": "T-B0-P05",
    "T-R1-005": "T-B0-P03",
    "T-R1-006": "T-B0-P06",
    "T-R1-007": "T-B0-P08",
    "T-R1-008": "T-B0-P01",
    "T-R1-009": "T-B0-P09",
    "T-R1-010": "T-B0-P09",
    "T-R1-011": "T-B0-P08",
    "T-R1-012": "T-B0-ACCEPT",
    "T-R2-001": "T-B0-P04",
    "T-R2-002": "T-B0-P04",
    "T-R2-003": "T-A0-P01",
    "T-R2-004": "T-A0-P03",
    "T-R2-005": "T-A0-P03",
    "T-R2-006": "T-A0-P01",
    "T-R3-001": "T-B0-P09",
    "T-R3-002": "T-B0-P05",
    "T-R3-003": "T-B0-P09",
    "T-R3-004": "T-B0-P09",
    "T-R3-005": "T-B0-P10",
    "T-R3-006": "T-B0-P10",
    "T-R3-007": "T-B0-ACCEPT",
}

CANONICAL_PROPOSAL_IDS = {
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

CANONICAL_REVIEW_BRANCH_DISPOSITIONS = {
    "PR #31": {
        "reference": "PR #31",
        "status": "closed-branch-preserved",
        "execution_status": "deferred",
        "milestone": "G1",
        "condition": (
            "May be reviewed only as design source after accepted P03 technical "
            "qualification, exact-artifact T-B0-P03-OBS, and a Continue decision "
            "make P03D eligible; design consultation, canonical DESIGN.md, and plan "
            "design review must still complete, and this branch is not current P04 "
            "authority."
        ),
    },
    "PR #32": {
        "reference": "PR #32",
        "status": "closed-branch-preserved",
        "execution_status": "blocked",
        "milestone": "G2B",
        "condition": (
            "A0 relationship intent and cadence work cannot execute or merge before "
            "B0 acceptance."
        ),
    },
    "PR #33": {
        "reference": "PR #33",
        "status": "closed-branch-preserved",
        "execution_status": "blocked",
        "milestone": "G2B",
        "condition": (
            "A0 YAML and CLI relationship intent work cannot execute or merge before "
            "B0 acceptance."
        ),
    },
}


@repository_custody_validation
def traceability_ownership_schema_issues(document: object) -> list[str]:
    """Validate the ownership document shape shared by checker and generator."""

    if not isinstance(document, dict):
        return ["traceability: ownership root must be a JSON object"]

    issues: list[str] = []
    if set(document) != TRACEABILITY_OWNERSHIP_ROOT_FIELDS:
        issues.append(
            "traceability: ownership top-level fields must be exactly "
            f"{sorted(TRACEABILITY_OWNERSHIP_ROOT_FIELDS)}"
        )
    if (
        type(document.get("schema_version")) is not int
        or document.get("schema_version") != 1
    ):
        issues.append(
            "traceability: ownership schema_version must be the JSON integer 1"
        )
    if document.get("status") != "active":
        issues.append("traceability: ownership status must be active")
    if document.get("delivery_boundary") != CANONICAL_DELIVERY_BOUNDARY or not isinstance(
        document.get("delivery_boundary"), str
    ):
        issues.append(
            "traceability: delivery_boundary must equal the canonical B0-before-A0 string"
        )
    if document.get("design_sequence") != CANONICAL_DESIGN_SEQUENCE or not isinstance(
        document.get("design_sequence"), str
    ):
        issues.append(
            "traceability: design_sequence must equal the canonical D1-B sequence string"
        )

    resume = document.get("p03_resume_authority")
    resume_fields = {
        "decision",
        "source_pull_request",
        "baseline_commit",
        "observation_task",
        "p03_technical_acceptance",
        "qualification_identity_contract",
        "qualified_artifact_identity",
        "observation_decision",
        "observation_receipt_contract",
        "observation_receipt",
        "replacement_task",
        "stopped_disposition",
        "p03d_eligibility",
        "merge_result_ci",
        "notarized_bundle_preflight",
    }
    if not isinstance(resume, dict) or set(resume) != resume_fields:
        actual_fields = sorted(resume) if isinstance(resume, dict) else []
        issues.append(
            "traceability: p03_resume_authority fields must be exactly "
            f"{sorted(resume_fields)}, not {actual_fields}"
        )
    else:
        if resume.get("decision") != "D1-B":
            issues.append("traceability: p03 resume decision must be D1-B")
        if type(resume.get("source_pull_request")) is not int:
            issues.append(
                "traceability: p03 source_pull_request must be a JSON integer"
            )
        if not is_lower_hex(resume.get("baseline_commit"), 40):
            issues.append(
                "traceability: p03 baseline_commit must be an exact lowercase Git SHA string"
            )
        if (
            type(resume.get("observation_task")) is not str
            or not ID.fullmatch(resume["observation_task"])
        ):
            issues.append(
                "traceability: p03 observation_task must be a canonical task identifier"
            )
        if resume.get("p03_technical_acceptance") not in {"not-accepted", "accepted"}:
            issues.append(
                "traceability: p03_technical_acceptance must be not-accepted or accepted"
            )
        if resume.get("observation_decision") not in {
            "pending",
            "Continue",
            "Change",
            "Stop",
        }:
            issues.append(
                "traceability: observation_decision must be pending, Continue, Change, or Stop"
            )
        if resume.get("p03d_eligibility") not in {"blocked", "eligible", "satisfied"}:
            issues.append(
                "traceability: p03d_eligibility must be blocked, eligible, or satisfied"
            )
        expected_identity_contract = [
            "qualified-code-sha",
            "merge-result-sha",
            "attestation-sha",
            "exact-executable-artifact-receipt",
        ]
        if resume.get("qualification_identity_contract") != expected_identity_contract:
            issues.append(
                "traceability: qualification_identity_contract must equal the canonical field list"
            )
        expected_receipt_contract = [
            "record_sha256",
            "decision_sha256",
            "observed_qualification_identity",
            "decision",
        ]
        if resume.get("observation_receipt_contract") != expected_receipt_contract:
            issues.append(
                "traceability: observation_receipt_contract must equal the canonical field list"
            )

        def identity_schema_issues(label: str, identity: object) -> None:
            if identity is None:
                return
            identity_fields = set(expected_identity_contract)
            if not isinstance(identity, dict) or set(identity) != identity_fields:
                actual = sorted(identity) if isinstance(identity, dict) else []
                issues.append(
                    f"traceability: {label} fields must be exactly "
                    f"{sorted(identity_fields)}, not {actual}"
                )
                return
            artifact_receipt = identity.get("exact-executable-artifact-receipt")
            artifact_fields = {"artifact_sha256", "receipt_sha256"}
            if not isinstance(artifact_receipt, dict) or set(artifact_receipt) != artifact_fields:
                actual = (
                    sorted(artifact_receipt)
                    if isinstance(artifact_receipt, dict)
                    else []
                )
                issues.append(
                    f"traceability: {label} executable receipt fields must be exactly "
                    f"{sorted(artifact_fields)}, not {actual}"
                )
                return
            for field in (
                "qualified-code-sha",
                "merge-result-sha",
                "attestation-sha",
            ):
                if not is_lower_hex(identity.get(field), 40):
                    issues.append(
                        f"traceability: {label} {field} must be an exact lowercase Git SHA string"
                    )
            for field in ("artifact_sha256", "receipt_sha256"):
                if not is_lower_hex(artifact_receipt.get(field), 64):
                    issues.append(
                        f"traceability: {label} {field} must be an exact lowercase SHA-256 string"
                    )

        identity_schema_issues(
            "qualified_artifact_identity", resume.get("qualified_artifact_identity")
        )
        observation_receipt = resume.get("observation_receipt")
        if observation_receipt is not None:
            receipt_fields = set(expected_receipt_contract)
            if not isinstance(observation_receipt, dict) or set(observation_receipt) != receipt_fields:
                actual = (
                    sorted(observation_receipt)
                    if isinstance(observation_receipt, dict)
                    else []
                )
                issues.append(
                    "traceability: observation_receipt fields must be exactly "
                    f"{sorted(receipt_fields)}, not {actual}"
                )
            else:
                identity_schema_issues(
                    "observation_receipt observed_qualification_identity",
                    observation_receipt.get("observed_qualification_identity"),
                )
                for field in ("record_sha256", "decision_sha256"):
                    if not is_lower_hex(observation_receipt.get(field), 64):
                        issues.append(
                            f"traceability: observation_receipt {field} must be an exact lowercase SHA-256 string"
                        )
                if observation_receipt.get("decision") not in {
                    "Continue",
                    "Change",
                    "Stop",
                }:
                    issues.append(
                        "traceability: observation_receipt decision must be Continue, Change, or Stop"
                    )
        replacement_task = resume.get("replacement_task")
        if replacement_task is not None and type(replacement_task) is not str:
            issues.append(
                "traceability: replacement_task must be a JSON string or null"
            )
        stopped_disposition = resume.get("stopped_disposition")
        stopped_fields = {
            "project_state",
            "preservation_receipt_sha256",
            "support_owner",
        }
        if stopped_disposition is not None:
            if not isinstance(stopped_disposition, dict) or set(
                stopped_disposition
            ) != stopped_fields:
                actual = (
                    sorted(stopped_disposition)
                    if isinstance(stopped_disposition, dict)
                    else []
                )
                issues.append(
                    "traceability: stopped_disposition fields must be exactly "
                    f"{sorted(stopped_fields)}, not {actual}"
                )
            else:
                if stopped_disposition.get("project_state") != "stopped":
                    issues.append(
                        "traceability: stopped_disposition project_state must be stopped"
                    )
                if not is_lower_hex(
                    stopped_disposition.get("preservation_receipt_sha256"), 64
                ):
                    issues.append(
                        "traceability: stopped_disposition preservation receipt must be a lowercase SHA-256 string"
                    )
                support_owner = stopped_disposition.get("support_owner")
                if support_owner is not None and (
                    type(support_owner) is not str or not ID.fullmatch(support_owner)
                ):
                    issues.append(
                        "traceability: stopped_disposition support_owner must be a canonical task identifier or null"
                    )
        merge_result_ci = resume.get("merge_result_ci")
        merge_fields = {"event", "conclusion", "workflow_run_ids"}
        if not isinstance(merge_result_ci, dict) or set(merge_result_ci) != merge_fields:
            actual = sorted(merge_result_ci) if isinstance(merge_result_ci, dict) else []
            issues.append(
                "traceability: merge_result_ci fields must be exactly "
                f"{sorted(merge_fields)}, not {actual}"
            )
        else:
            if any(
                type(merge_result_ci.get(field)) is not str
                or not merge_result_ci.get(field)
                for field in ("event", "conclusion")
            ):
                issues.append(
                    "traceability: merge_result_ci event and conclusion must be non-empty JSON strings"
                )
            workflow_run_ids = merge_result_ci.get("workflow_run_ids")
            if not isinstance(workflow_run_ids, list) or any(
                type(value) is not int for value in workflow_run_ids
            ):
                issues.append(
                    "traceability: merge_result_ci workflow_run_ids must be an integer array"
                )
            elif len(workflow_run_ids) != len(set(workflow_run_ids)):
                issues.append(
                    "traceability: merge_result_ci workflow_run_ids must be unique"
                )
        notarized = resume.get("notarized_bundle_preflight")
        notarized_fields = {
            "event",
            "run_id",
            "conclusion",
            "failure_boundary",
            "release_evidence",
        }
        if not isinstance(notarized, dict) or set(notarized) != notarized_fields:
            actual = sorted(notarized) if isinstance(notarized, dict) else []
            issues.append(
                "traceability: notarized_bundle_preflight fields must be exactly "
                f"{sorted(notarized_fields)}, not {actual}"
            )
        else:
            for field in (
                "event",
                "conclusion",
                "failure_boundary",
                "release_evidence",
            ):
                if type(notarized.get(field)) is not str or not notarized.get(field):
                    issues.append(
                        f"traceability: notarized_bundle_preflight {field} must be a non-empty JSON string"
                    )
            if type(notarized.get("run_id")) is not int:
                issues.append(
                    "traceability: notarized_bundle_preflight run_id must be a JSON integer"
                )

    object_record_contracts = (
        (
            "cross_release_ownership_dispositions",
            document.get("cross_release_ownership_dispositions", {}),
            {
                "artifact_type",
                "repository_release",
                "owning_task",
                "task_release",
                "owning_gate",
                "gate_release",
                "authority",
                "rationale",
            },
        ),
        (
            "forward_evidence_dispositions",
            document.get("forward_evidence_dispositions", {}),
            {
                "task",
                "task_milestone",
                "artifact_type",
                "artifact_id",
                "artifact_owner_task",
                "artifact_milestone",
                "relationship",
                "authority",
                "rationale",
            },
        ),
        (
            "authority_sources",
            document.get("authority_sources", {}),
            {"kind", "sha256", "anchors"},
        ),
    )
    for label, records, expected_fields in object_record_contracts:
        if not isinstance(records, dict):
            issues.append(f"traceability: {label} must be an object")
            continue
        for identifier, record in records.items():
            if not isinstance(record, dict) or set(record) != expected_fields:
                actual_fields = sorted(record) if isinstance(record, dict) else []
                issues.append(
                    f"{identifier}: {label} fields must be exactly "
                    f"{sorted(expected_fields)}, not {actual_fields}"
                )
            if label == "authority_sources" and isinstance(record, dict):
                if not isinstance(record.get("anchors"), list):
                    issues.append(f"{identifier}: authority source anchors must be an array")

    list_record_contracts = (
        (
            "milestones",
            document.get("milestones", []),
            {"id", "name", "depends_on", "status"},
        ),
        (
            "evidence_owners",
            document.get("evidence_owners", []),
            {"id", "role", "accountable"},
        ),
        (
            "proposal_dispositions",
            document.get("proposal_dispositions", []),
            {"proposal_id", "disposition", "canonical_id", "rationale"},
        ),
        (
            "review_branch_dispositions",
            document.get("review_branch_dispositions", []),
            {"reference", "status", "execution_status", "milestone", "condition"},
        ),
    )
    for label, records, expected_fields in list_record_contracts:
        if not isinstance(records, list):
            issues.append(f"traceability: {label} must be an array")
            continue
        for index, record in enumerate(records):
            if not isinstance(record, dict) or set(record) != expected_fields:
                actual_fields = sorted(record) if isinstance(record, dict) else []
                issues.append(
                    f"{label}[{index}]: fields must be exactly "
                    f"{sorted(expected_fields)}, not {actual_fields}"
                )

    approved_strategy = document.get("approved_strategy")
    approved_strategy_fields = {"title", "status", "approved_sha256", "scope"}
    if (
        not isinstance(approved_strategy, dict)
        or set(approved_strategy) != approved_strategy_fields
    ):
        actual_fields = (
            sorted(approved_strategy) if isinstance(approved_strategy, dict) else []
        )
        issues.append(
            "traceability: approved_strategy fields must be exactly "
            f"{sorted(approved_strategy_fields)}, not {actual_fields}"
        )

    ownership_record_contracts = (
        (
            "requirement_ownership",
            document.get("requirement_ownership", {}),
            {"owning_task", "owning_gate", "milestone", "evidence_owner", "status"},
        ),
        (
            "uat_ownership",
            document.get("uat_ownership", {}),
            {"owning_task", "owning_gate", "milestone", "evidence_owner", "status"},
        ),
        (
            "gate_ownership",
            document.get("gate_ownership", {}),
            {"acceptance_task", "milestone", "evidence_owner", "status"},
        ),
    )
    for label, records, expected_fields in ownership_record_contracts:
        if not isinstance(records, dict):
            issues.append(f"traceability: {label} must be an object")
            continue
        for identifier, edge in records.items():
            if not isinstance(edge, dict) or set(edge) != expected_fields:
                actual_fields = sorted(edge) if isinstance(edge, dict) else []
                issues.append(
                    f"{identifier}: {label} fields must be exactly "
                    f"{sorted(expected_fields)}, not {actual_fields}"
                )

    task_ownership = document.get("task_ownership", {})
    if not isinstance(task_ownership, dict):
        issues.append("traceability: task_ownership must be an object")
        return issues

    normal_task_fields = {"milestone", "owning_gate", "evidence_owner", "status"}
    superseded_task_fields = normal_task_fields | {"superseded_by", "disposition"}
    canonical_superseded_ids = set(CANONICAL_SUPERSEDED_TASKS)
    actual_superseded_ids = {
        identifier
        for identifier, edge in task_ownership.items()
        if isinstance(edge, dict) and edge.get("status") == "superseded"
    }
    if actual_superseded_ids != canonical_superseded_ids:
        issues.append(
            "traceability: superseded task ids must equal the identifier-exact "
            "historical set; "
            f"missing={sorted(canonical_superseded_ids - actual_superseded_ids)}, "
            f"unexpected={sorted(actual_superseded_ids - canonical_superseded_ids)}"
        )
    for identifier, edge in task_ownership.items():
        expected_fields = (
            superseded_task_fields
            if identifier in CANONICAL_SUPERSEDED_TASKS
            else normal_task_fields
        )
        if not isinstance(edge, dict) or set(edge) != expected_fields:
            actual_fields = sorted(edge) if isinstance(edge, dict) else []
            issues.append(
                f"{identifier}: task_ownership fields must be exactly "
                f"{sorted(expected_fields)}, not {actual_fields}"
            )
        if (
            identifier in CANONICAL_SUPERSEDED_TASKS
            and isinstance(edge, dict)
            and edge.get("superseded_by") != CANONICAL_SUPERSEDED_TASKS[identifier]
        ):
            issues.append(
                f"{identifier}: superseded_by must remain "
                f"{CANONICAL_SUPERSEDED_TASKS[identifier]}, not "
                f"{edge.get('superseded_by')}"
            )

    def require_nonempty_strings(
        label: str, record: object, fields: set[str]
    ) -> None:
        if not isinstance(record, dict):
            return
        for field in fields:
            value = record.get(field)
            if type(value) is not str or not value.strip():
                issues.append(f"{label}: {field} must be a non-empty JSON string")

    cross_release_records = document.get("cross_release_ownership_dispositions", {})
    for key, record in (
        cross_release_records.items()
        if isinstance(cross_release_records, dict)
        else ()
    ):
        label = f"{key}: cross-release disposition"
        require_nonempty_strings(
            label,
            record,
            {
                "artifact_type",
                "repository_release",
                "owning_task",
                "task_release",
                "owning_gate",
                "gate_release",
                "authority",
                "rationale",
            },
        )
        if not isinstance(record, dict):
            continue
        if record.get("artifact_type") not in {"requirement", "uat"}:
            issues.append(f"{label}: artifact_type must be requirement or uat")
        for field in ("repository_release", "task_release", "gate_release"):
            if record.get(field) not in VALID_RELEASES | {"all"}:
                issues.append(f"{label}: {field} must be a canonical release")
        for field in ("owning_task", "owning_gate"):
            value = record.get(field)
            if type(value) is not str or not ID.fullmatch(value):
                issues.append(f"{label}: {field} must be a canonical identifier")
        if not is_safe_repository_relative_path(record.get("authority")):
            issues.append(f"{label}: authority must be a safe repository-relative path")
        elif repository_regular_file_path(record.get("authority")) is None:
            issues.append(
                f"{label}: authority must be a visible, non-ignored regular "
                "repository-owned file without symlink components"
            )

    forward_evidence_records = document.get("forward_evidence_dispositions", {})
    for key, record in (
        forward_evidence_records.items()
        if isinstance(forward_evidence_records, dict)
        else ()
    ):
        label = f"{key}: forward-evidence disposition"
        require_nonempty_strings(
            label,
            record,
            {
                "task",
                "task_milestone",
                "artifact_type",
                "artifact_id",
                "artifact_owner_task",
                "artifact_milestone",
                "relationship",
                "authority",
                "rationale",
            },
        )
        if not isinstance(record, dict):
            continue
        if record.get("artifact_type") not in {"requirement", "uat"}:
            issues.append(f"{label}: artifact_type must be requirement or uat")
        if record.get("relationship") != "non-blocking-future-contract-input":
            issues.append(
                f"{label}: relationship must be non-blocking-future-contract-input"
            )
        for field in ("task", "artifact_id", "artifact_owner_task"):
            value = record.get(field)
            if type(value) is not str or not ID.fullmatch(value):
                issues.append(f"{label}: {field} must be a canonical identifier")
        for field in ("task_milestone", "artifact_milestone"):
            value = record.get(field)
            if type(value) is not str or not MILESTONE_ID.fullmatch(value):
                issues.append(f"{label}: {field} must be a canonical milestone identifier")
        if not is_safe_repository_relative_path(record.get("authority")):
            issues.append(f"{label}: authority must be a safe repository-relative path")
        elif repository_regular_file_path(record.get("authority")) is None:
            issues.append(
                f"{label}: authority must be a visible, non-ignored regular "
                "repository-owned file without symlink components"
            )

    authority_source_records = document.get("authority_sources", {})
    if isinstance(authority_source_records, dict):
        issues.extend(
            "traceability: " + issue
            for issue in repository_path_collision_issues(
                (*TRACEABILITY_FIXED_SOURCE_PATHS, *authority_source_records)
            )
        )
    for path, record in (
        authority_source_records.items()
        if isinstance(authority_source_records, dict)
        else ()
    ):
        label = f"{path}: authority source"
        if not is_safe_repository_relative_path(path):
            issues.append(f"{label}: key must be a safe repository-relative path")
        elif repository_regular_file_path(path) is None:
            issues.append(
                f"{label}: key must name a visible, non-ignored regular "
                "repository-owned file without symlink components"
            )
        if not isinstance(record, dict):
            continue
        if record.get("kind") not in {"accepted-decision", "normative-contract"}:
            issues.append(
                f"{label}: kind must be accepted-decision or normative-contract"
            )
        if not is_lower_hex(record.get("sha256"), 64):
            issues.append(f"{label}: sha256 must be a lowercase SHA-256 string")
        anchors = record.get("anchors")
        if not isinstance(anchors, list) or not anchors or any(
            type(anchor) is not str or not anchor.strip() for anchor in anchors
        ):
            issues.append(f"{label}: anchors must be a non-empty string array")
        elif len(anchors) != len(set(anchors)):
            issues.append(f"{label}: anchors must be unique")

    milestone_ids_seen: set[str] = set()
    for index, record in enumerate(document.get("milestones", [])):
        label = f"milestones[{index}]"
        require_nonempty_strings(label, record, {"id", "name", "status"})
        if not isinstance(record, dict):
            continue
        identifier = record.get("id")
        if type(identifier) is not str or not MILESTONE_ID.fullmatch(identifier):
            issues.append(f"{label}: id must be a canonical identifier")
        elif identifier in milestone_ids_seen:
            issues.append(f"{label}: duplicate id {identifier}")
        else:
            milestone_ids_seen.add(identifier)
        depends_on = record.get("depends_on")
        if not isinstance(depends_on, list) or any(
            type(value) is not str or not MILESTONE_ID.fullmatch(value)
            for value in depends_on
        ):
            issues.append(f"{label}: depends_on must be a canonical identifier array")
        elif len(depends_on) != len(set(depends_on)):
            issues.append(f"{label}: depends_on must contain unique identifiers")
        if record.get("status") not in VALID_MILESTONE_STATUSES:
            issues.append(f"{label}: status must be a canonical milestone status")

    evidence_owner_ids_seen: set[str] = set()
    for index, record in enumerate(document.get("evidence_owners", [])):
        label = f"evidence_owners[{index}]"
        require_nonempty_strings(label, record, {"id", "role", "accountable"})
        if not isinstance(record, dict):
            continue
        identifier = record.get("id")
        if type(identifier) is not str or not ID.fullmatch(identifier):
            issues.append(f"{label}: id must be a canonical identifier")
        elif identifier in evidence_owner_ids_seen:
            issues.append(f"{label}: duplicate id {identifier}")
        else:
            evidence_owner_ids_seen.add(identifier)

    proposal_ids_seen: set[str] = set()
    for index, record in enumerate(document.get("proposal_dispositions", [])):
        label = f"proposal_dispositions[{index}]"
        require_nonempty_strings(
            label, record, {"proposal_id", "disposition", "canonical_id", "rationale"}
        )
        if not isinstance(record, dict):
            continue
        identifier = record.get("proposal_id")
        if type(identifier) is str:
            if identifier in proposal_ids_seen:
                issues.append(f"{label}: duplicate proposal_id {identifier}")
            proposal_ids_seen.add(identifier)
        if record.get("disposition") not in {
            "adopted",
            "merged",
            "deferred",
            "rejected",
        }:
            issues.append(f"{label}: disposition is not canonical")
        canonical_id = record.get("canonical_id")
        if type(canonical_id) is not str or not ID.fullmatch(canonical_id):
            issues.append(f"{label}: canonical_id must be a canonical identifier")
        if identifier in CANONICAL_PROPOSAL_IDS and (
            record.get("disposition") != "adopted" or canonical_id != identifier
        ):
            issues.append(
                f"{label}: {identifier} must map exactly to itself with adopted status"
            )

    branch_ids_seen: set[str] = set()
    for index, record in enumerate(document.get("review_branch_dispositions", [])):
        label = f"review_branch_dispositions[{index}]"
        require_nonempty_strings(
            label,
            record,
            {"reference", "status", "execution_status", "milestone", "condition"},
        )
        if not isinstance(record, dict):
            continue
        reference = record.get("reference")
        if type(reference) is str:
            if reference in branch_ids_seen:
                issues.append(f"{label}: duplicate reference {reference}")
            branch_ids_seen.add(reference)
        if record.get("status") != "closed-branch-preserved":
            issues.append(f"{label}: status must be closed-branch-preserved")
        if record.get("execution_status") not in {"blocked", "deferred"}:
            issues.append(f"{label}: execution_status must be blocked or deferred")
        if (
            type(record.get("milestone")) is not str
            or not MILESTONE_ID.fullmatch(record.get("milestone", ""))
        ):
            issues.append(f"{label}: milestone must be a canonical milestone identifier")
        expected_record = CANONICAL_REVIEW_BRANCH_DISPOSITIONS.get(reference)
        if expected_record is not None and record != expected_record:
            issues.append(
                f"{label}: {reference} must match its exact preserved branch disposition"
            )
    expected_branch_ids = set(CANONICAL_REVIEW_BRANCH_DISPOSITIONS)
    if branch_ids_seen != expected_branch_ids:
        issues.append(
            "traceability: review_branch_dispositions references must be exactly "
            f"{sorted(expected_branch_ids)}, not {sorted(branch_ids_seen)}"
        )

    if isinstance(approved_strategy, dict):
        require_nonempty_strings(
            "approved_strategy", approved_strategy, approved_strategy_fields
        )
        if approved_strategy.get("status") != APPROVED_STRATEGY_STATUS:
            issues.append(
                f"traceability: approved_strategy status must be {APPROVED_STRATEGY_STATUS}"
            )
        if not is_lower_hex(approved_strategy.get("approved_sha256"), 64):
            issues.append(
                "traceability: approved_strategy approved_sha256 must be a lowercase SHA-256 string"
            )

    for label, records, task_field in (
        ("requirement_ownership", document.get("requirement_ownership", {}), "owning_task"),
        ("uat_ownership", document.get("uat_ownership", {}), "owning_task"),
        ("gate_ownership", document.get("gate_ownership", {}), "acceptance_task"),
    ):
        if not isinstance(records, dict):
            continue
        for identifier, record in records.items():
            record_label = f"{identifier}: {label}"
            if type(identifier) is not str or not ID.fullmatch(identifier):
                issues.append(f"{record_label}: map key must be a canonical identifier")
            if not isinstance(record, dict):
                continue
            string_fields = {task_field, "milestone", "evidence_owner", "status"}
            if label != "gate_ownership":
                string_fields.add("owning_gate")
            require_nonempty_strings(record_label, record, string_fields)
            for field in string_fields - {"status"}:
                value = record.get(field)
                pattern = MILESTONE_ID if field == "milestone" else ID
                if type(value) is not str or not pattern.fullmatch(value):
                    issues.append(f"{record_label}: {field} must be a canonical identifier")
            if record.get("status") not in VALID_CONTRACT_STATUSES:
                issues.append(f"{record_label}: status must be a canonical contract status")

    for identifier, record in task_ownership.items():
        label = f"{identifier}: task_ownership"
        if type(identifier) is not str or not ID.fullmatch(identifier):
            issues.append(f"{label}: map key must be a canonical identifier")
        if not isinstance(record, dict):
            continue
        require_nonempty_strings(
            label, record, {"milestone", "owning_gate", "evidence_owner", "status"}
        )
        for field in ("milestone", "owning_gate", "evidence_owner"):
            value = record.get(field)
            pattern = MILESTONE_ID if field == "milestone" else ID
            if type(value) is not str or not pattern.fullmatch(value):
                issues.append(f"{label}: {field} must be a canonical identifier")
        status = record.get("status")
        if status not in VALID_CONTRACT_STATUSES | {"superseded"}:
            issues.append(f"{label}: status must be a canonical task status")
        if identifier in CANONICAL_SUPERSEDED_TASKS:
            if type(record.get("superseded_by")) is not str or not ID.fullmatch(
                record.get("superseded_by", "")
            ):
                issues.append(f"{label}: superseded_by must be a canonical identifier")
            if type(record.get("disposition")) is not str or not record.get(
                "disposition", ""
            ).strip():
                issues.append(f"{label}: disposition must be a non-empty JSON string")
    return issues


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


def validate_restricted_yaml_root(path: Path, text: str, key: str) -> dict[str, str]:
    """Reject root YAML shapes whose meaning could vary by parser."""

    configurations = {
        ("implementation-plan.yaml", "tasks"): {
            "order": (
                "schema_version",
                "status",
                "execution_authority",
                "delivery_sequence",
                "delivery_rule",
                "tasks",
            ),
            "exact": {
                "schema_version": "2",
                "status": "reconciled",
                "execution_authority": "spec/traceability-ownership.json",
                "delivery_sequence": "[G0, G1, G3, B0, PILOT, G2C, G2A, G2B, A0, POST-A0, G4, G5, G6, G7]",
                "delivery_rule": "B0 acceptance precedes every A0 task; the optional real-data PILOT is outside the synthetic B0 acceptance path; A0 acceptance precedes provider, mobile, AI, MCP, and theme-package delivery. Dependencies may establish seams but never transfer acceptance ownership.",
            },
            "sequence": {"delivery_sequence"},
            "container": "tasks",
        },
        ("feature-gates.yaml", "gates"): {
            "order": ("schema_version", "status", "gates"),
            "exact": {"schema_version": "1", "status": "active"},
            "sequence": set(),
            "container": "gates",
        },
    }
    configuration = configurations.get((path.name, key))
    if configuration is None:
        return {}

    root_entries: list[tuple[str, str]] = []
    unexpected: list[str] = []
    for line_number, line in enumerate(text.splitlines(), start=1):
        if not line or line[0].isspace() or line.lstrip().startswith("#"):
            continue
        if line != line.rstrip():
            unexpected.append(f"line {line_number}: trailing whitespace")
            continue
        match = re.fullmatch(
            r"([A-Za-z_][A-Za-z0-9_-]*):(?:[ \t]*(.*))?", line
        )
        if match is None:
            unexpected.append(f"line {line_number}: {line!r}")
            continue
        root_entries.append((match.group(1), (match.group(2) or "").strip()))

    names = [name for name, _ in root_entries]
    expected_order = list(configuration["order"])
    allowed = set(expected_order)
    missing = sorted(allowed - set(names))
    unknown = sorted(set(names) - allowed)
    duplicates = sorted(name for name in set(names) if names.count(name) != 1)
    issues: list[str] = []
    if missing:
        issues.append(f"missing root keys {missing}")
    if unknown:
        issues.append(f"unknown root keys {unknown}")
    if duplicates:
        issues.append(f"duplicate root keys {duplicates}")
    if unexpected:
        issues.append(f"unexpected root syntax {unexpected}")
    if names != expected_order:
        issues.append(
            f"root keys must occur exactly in order {expected_order}, not {names}"
        )

    values = {name: value for name, value in root_entries}
    for name, expected in configuration["exact"].items():
        if names.count(name) == 1 and values.get(name) != expected:
            issues.append(
                f"{name} must be exactly {expected!r}, not {values.get(name)!r}"
            )
    for name in configuration["sequence"]:
        if names.count(name) == 1 and re.fullmatch(
            r"\[[^\]]*\](?:\s*#.*)?", values.get(name, "")
        ) is None:
            issues.append(f"{name} must use the canonical inline [a, b] form")
    container = configuration["container"]
    if names.count(container) == 1 and values.get(container):
        issues.append(f"{container} must be a block collection, not an inline value")

    container_line = next(
        (
            index
            for index, line in enumerate(text.splitlines())
            if re.fullmatch(rf"{re.escape(container)}:\s*", line)
        ),
        None,
    )
    if container_line is not None:
        lines = text.splitlines()
        indented_before = [
            f"line {index + 1}: {line!r}"
            for index, line in enumerate(lines[:container_line])
            if line and line[0].isspace() and not line.lstrip().startswith("#")
        ]
        if indented_before:
            issues.append(
                f"indented content before {container} root {indented_before}"
            )
        body = lines[container_line + 1 :]
        two_space_entries = [
            (container_line + offset + 2, line)
            for offset, line in enumerate(body)
            if line.startswith("  ")
            and not line.startswith("    ")
            and not line.lstrip().startswith("#")
        ]
        invalid_entries = [
            f"line {line_number}: {line!r}"
            for line_number, line in two_space_entries
            if re.fullmatch(r"  - id:\s*[^\s#]+\s*", line) is None
        ]
        if invalid_entries:
            issues.append(f"invalid {container} collection entries {invalid_entries}")
        if not two_space_entries:
            issues.append(f"{container} must contain at least one record")

    if issues:
        raise ValueError(f"{path.name}: " + "; ".join(issues))
    return values


def parse_simple_yaml_ids(path: Path, key: str) -> list[str]:
    # The catalog intentionally uses a restricted, reviewable YAML shape. This
    # check avoids adding a Python dependency solely for repository policy.
    text = path.read_text(encoding="utf-8")
    if f"{key}:" not in text:
        raise ValueError(f"missing top-level {key}")
    return re.findall(r"^\s+- id:\s*([^\s#]+)\s*$", text, flags=re.MULTILINE)


def parse_simple_yaml_records(path: Path, key: str) -> dict[str, str]:
    text = path.read_text(encoding="utf-8")
    validate_restricted_yaml_root(path, text, key)
    lines = text.splitlines()
    container_line = next(
        (
            index
            for index, line in enumerate(lines)
            if re.fullmatch(rf"{re.escape(key)}:", line)
        ),
        None,
    )
    if container_line is None:
        raise ValueError(f"missing top-level {key}")

    records: list[tuple[str, str]] = []
    current_identifier: str | None = None
    current_lines: list[str] = []
    for line_number, line in enumerate(lines[container_line + 1 :], container_line + 2):
        header = re.fullmatch(r"  - id: ([A-Z0-9]+(?:-[A-Z0-9]+)+)", line)
        if header is not None:
            if current_identifier is not None:
                records.append((current_identifier, "\n".join(current_lines) + "\n"))
            current_identifier = header.group(1)
            current_lines = []
            continue
        if not line or line.lstrip().startswith("#"):
            if current_identifier is not None:
                current_lines.append(line)
            continue
        if current_identifier is None:
            raise ValueError(
                f"{path.name}: line {line_number} is unsupported content before "
                f"the first {key} record: {line!r}"
            )
        current_lines.append(line)
    if current_identifier is not None:
        records.append((current_identifier, "\n".join(current_lines) + "\n"))
    if not records:
        raise ValueError(f"{path.name}: {key} must contain at least one record")

    identifiers = [identifier for identifier, _ in records]
    duplicates = sorted(
        identifier for identifier in set(identifiers) if identifiers.count(identifier) > 1
    )
    if duplicates:
        raise ValueError(f"duplicate {key} ids: {duplicates}")
    blocks = dict(records)
    issues = restricted_yaml_block_issues(blocks, key)
    if issues:
        raise ValueError("; ".join(issues))
    return blocks


def restricted_record_scalar(block: str, field: str) -> str:
    """Read a scalar only after restricted record validation has succeeded."""

    match = re.search(rf"^    {re.escape(field)}: (.+)$", block, re.MULTILINE)
    return match.group(1) if match else ""


def restricted_record_sequence(block: str, field: str) -> tuple[str, ...]:
    """Read one canonical inline sequence without scanning explanatory prose."""

    value = restricted_record_scalar(block, field)
    if not value:
        return ()
    if not value.startswith("[") or not value.endswith("]"):
        raise ValueError(f"{field} is not a canonical inline sequence")
    inner = value[1:-1]
    if not inner:
        return ()
    items = tuple(item.strip() for item in inner.split(", "))
    if any(not item for item in items) or len(items) != len(set(items)):
        raise ValueError(f"{field} contains empty or duplicate sequence items")
    return items


def gate_evidence_required_map(
    gate_blocks: dict[str, str],
) -> dict[str, frozenset[str]]:
    """Return the sole gate-evidence authority consumed by checker and generator."""

    return {
        identifier: frozenset(
            restricted_record_sequence(block, "evidence_required")
        )
        for identifier, block in gate_blocks.items()
    }


def restricted_yaml_block_issues(
    blocks: dict[str, str], key: str
) -> list[str]:
    """Validate every line of the canonical task/gate record subset."""

    configurations = {
        "tasks": {
            "order": (
                "release",
                "title",
                "plan_alias",
                "rice",
                "contexts",
                "depends_on",
                "requirements",
                "uat",
                "evidence_dependencies",
                "required_artifacts",
                "outputs",
            ),
            "required": {
                "release",
                "title",
                "contexts",
                "depends_on",
                "requirements",
                "uat",
                "outputs",
            },
            "sequence": {
                "contexts",
                "depends_on",
                "requirements",
                "uat",
                "evidence_dependencies",
                "required_artifacts",
                "outputs",
            },
            "nonempty_sequence": {"contexts", "outputs", "required_artifacts"},
            "block": set(),
        },
        "gates": {
            "order": (
                "release",
                "name",
                "blocks",
                "depends_on",
                "criteria",
                "evidence",
                "evidence_required",
            ),
            "required": {"release", "name", "blocks", "criteria"},
            "sequence": {"blocks", "depends_on", "evidence_required"},
            "nonempty_sequence": {"blocks", "depends_on", "evidence_required"},
            "block": {"criteria", "evidence"},
        },
    }
    configuration = configurations.get(key)
    if configuration is None:
        return [f"unknown restricted YAML record collection {key!r}"]

    implicit_scalar = re.compile(
        r"(?i)(?:null|~|true|false|yes|no|on|off|[-+]?(?:\d+(?:\.\d+)?|\.\d+|\.inf|\.nan)|\d{4}-\d{1,2}-\d{1,2})"
    )

    def canonical_sequence(value: str) -> bool:
        if re.fullmatch(
            r"\[(?:[A-Za-z][A-Za-z0-9 _./%-]*(?:, [A-Za-z][A-Za-z0-9 _./%-]*)*)?\]",
            value,
        ) is None:
            return False
        inner = value[1:-1]
        return not inner or all(
            item == item.strip()
            and implicit_scalar.fullmatch(item) is None
            for item in inner.split(", ")
        )

    def canonical_plain_text(value: str) -> bool:
        return (
            re.fullmatch(r"[A-Za-z][A-Za-z0-9 ,./-]*", value) is not None
            and value == value.strip()
            and implicit_scalar.fullmatch(value) is None
        )

    def canonical_list_text(value: str) -> bool:
        return (
            re.fullmatch(r"[A-Za-z.][A-Za-z0-9 _@,;./-]*", value) is not None
            and value == value.strip()
            and implicit_scalar.fullmatch(value) is None
        )

    order_index = {
        field: index for index, field in enumerate(configuration["order"])
    }
    allowed = set(configuration["order"])
    issues: list[str] = []
    for identifier, block in blocks.items():
        field_names: list[str] = []
        values: dict[str, str] = {}
        block_item_counts = {field: 0 for field in configuration["block"]}
        active_block: str | None = None
        for line_number, line in enumerate(block.splitlines(), start=1):
            if not line or line.lstrip().startswith("#"):
                continue
            if line != line.rstrip():
                issues.append(
                    f"{identifier}: line {line_number} has trailing whitespace"
                )
                continue
            field_match = re.fullmatch(
                r"    ([A-Za-z_][A-Za-z0-9_-]*):(?: (.*))?", line
            )
            if field_match is not None:
                field = field_match.group(1)
                value = field_match.group(2) or ""
                field_names.append(field)
                values[field] = value
                active_block = field if field in configuration["block"] else None
                if field not in allowed:
                    issues.append(f"{identifier}: unknown record field {field!r}")
                    continue
                if field in configuration["block"]:
                    if value:
                        issues.append(
                            f"{identifier}: {field} must be a canonical block list"
                        )
                elif field in configuration["sequence"]:
                    if not canonical_sequence(value):
                        issues.append(
                            f"{identifier}: {field} must use the canonical inline [a, b] form"
                        )
                    elif (
                        field in configuration["nonempty_sequence"]
                        and value == "[]"
                    ):
                        issues.append(
                            f"{identifier}: {field} must contain at least one item"
                        )
                elif field == "release":
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
                    } | ({"all"} if key == "gates" else set())
                    if value not in valid_releases:
                        issues.append(f"{identifier}: release must be a canonical token")
                elif field == "plan_alias":
                    if re.fullmatch(r"[A-Z][A-Z0-9-]*", value) is None:
                        issues.append(
                            f"{identifier}: plan_alias must be a canonical token"
                        )
                elif field == "rice":
                    if re.fullmatch(
                        r"\{reach: (?:0|[1-9]\d*)(?:\.\d+)?, impact: (?:0|[1-9]\d*)(?:\.\d+)?, confidence: (?:0|[1-9]\d*)(?:\.\d+)?, effort_weeks: (?:0|[1-9]\d*)(?:\.\d+)?, score: (?:0|[1-9]\d*)(?:\.\d+)?\}",
                        value,
                    ) is None:
                        issues.append(
                            f"{identifier}: rice must use the canonical inline field order"
                        )
                elif not canonical_plain_text(value):
                    issues.append(
                        f"{identifier}: {field} must be a canonical plain scalar"
                    )
                continue

            list_match = re.fullmatch(r"      - (.+)", line)
            if list_match is not None and active_block is not None:
                item = list_match.group(1)
                if not canonical_list_text(item):
                    issues.append(
                        f"{identifier}: {active_block} contains a non-canonical list item"
                    )
                block_item_counts[active_block] += 1
                continue

            issues.append(
                f"{identifier}: line {line_number} is not canonical record YAML: {line!r}"
            )

        duplicate_fields = sorted(
            field for field in set(field_names) if field_names.count(field) > 1
        )
        if duplicate_fields:
            issues.append(
                f"{identifier}: duplicate record fields {duplicate_fields}"
            )
        missing = sorted(configuration["required"] - set(field_names))
        if missing:
            issues.append(f"{identifier}: missing required record fields {missing}")
        recognized_order = [field for field in field_names if field in order_index]
        if recognized_order != sorted(recognized_order, key=order_index.get):
            issues.append(
                f"{identifier}: record fields are not in canonical order {recognized_order}"
            )
        for field, count in block_item_counts.items():
            if field in field_names and count == 0:
                issues.append(
                    f"{identifier}: {field} must contain at least one canonical list item"
                )
        if key == "gates" and (
            ("evidence" in field_names) == ("evidence_required" in field_names)
        ):
            issues.append(
                f"{identifier}: exactly one of evidence or evidence_required is required"
            )
    return issues


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


def validate_traceability_report_projection(
    requirements: list[dict],
    cases: list[dict],
    persona_ids: set[str],
    task_blocks: dict[str, str],
    gate_blocks: dict[str, str],
    ownership: dict,
    errors: list[str],
) -> None:
    """Independently prove the generated JSON report equals canonical inputs."""

    try:
        report = load_json("spec/traceability-report.json")
    except (OSError, ValueError, json.JSONDecodeError) as exc:
        errors.append(f"traceability report projection: {exc}")
        return
    if not isinstance(report, dict):
        errors.append("traceability report projection: root must be a JSON object")
        return

    expected_top_keys = {
        "schema_version",
        "status",
        "source_sha256",
        "delivery_boundary",
        "design_sequence",
        "p03_resume_authority",
        "approved_strategy",
        "counts",
        "milestones",
        "evidence_owners",
        "cross_release_ownership_dispositions",
        "forward_evidence_dispositions",
        "authority_sources",
        "requirements",
        "uat_cases",
        "feature_gates",
        "implementation_tasks",
        "proposal_dispositions",
        "review_branch_dispositions",
    }
    if set(report) != expected_top_keys:
        errors.append(
            "traceability report projection: top-level fields must be exactly "
            f"{sorted(expected_top_keys)}"
        )

    evidence_by_task = inline_edges(task_blocks, "evidence_dependencies")

    def supporting_tasks(field: str) -> dict[str, set[str]]:
        direct_by_task = inline_edges(task_blocks, field)
        result: dict[str, set[str]] = {}
        for task in task_blocks:
            for identifier in direct_by_task.get(task, set()) | evidence_by_task.get(
                task, set()
            ):
                result.setdefault(identifier, set()).add(task)
        return result

    requirement_support = supporting_tasks("requirements")
    uat_support = supporting_tasks("uat")
    gate_support: dict[str, set[str]] = {}
    for gate, evidence_required in gate_evidence_required_map(gate_blocks).items():
        for identifier in evidence_required:
            if re.fullmatch(r"UAT-[A-Z0-9-]+", identifier):
                gate_support.setdefault(identifier, set()).add(gate)

    requirement_ownership = ownership.get("requirement_ownership", {})
    uat_ownership = ownership.get("uat_ownership", {})
    task_ownership = ownership.get("task_ownership", {})
    gate_ownership = ownership.get("gate_ownership", {})

    expected_requirements: list[dict] = []
    for item in requirements:
        edge = requirement_ownership.get(item.get("id"), {})
        expected_requirements.append(
            {
                **edge,
                "id": item.get("id"),
                "repository_release": item.get("release"),
                "context": item.get("context"),
                "priority": item.get("priority"),
                "statement": item.get("statement"),
                "supporting_tasks": sorted(
                    requirement_support.get(item.get("id"), set())
                    - {edge.get("owning_task")}
                ),
            }
        )

    expected_uat: list[dict] = []
    for item in cases:
        edge = uat_ownership.get(item.get("id"), {})
        expected_uat.append(
            {
                **edge,
                "id": item.get("id"),
                "repository_release": item.get("release"),
                "persona": item.get("persona"),
                "title": item.get("title"),
                "supporting_tasks": sorted(
                    uat_support.get(item.get("id"), set())
                    - {edge.get("owning_task")}
                ),
                "supporting_gates": sorted(
                    gate_support.get(item.get("id"), set())
                    - {edge.get("owning_gate")}
                ),
            }
        )

    def scalar(block: str, field: str) -> str:
        match = re.search(
            rf"^    {re.escape(field)}: (.+)$", block, flags=re.MULTILINE
        )
        return match.group(1) if match else ""

    expected_tasks = [
        {
            **task_ownership.get(identifier, {}),
            "id": identifier,
            "repository_release": scalar(block, "release"),
            "title": scalar(block, "title"),
        }
        for identifier, block in task_blocks.items()
    ]
    expected_gates = [
        {
            **gate_ownership.get(identifier, {}),
            "id": identifier,
            "repository_release": scalar(block, "release"),
            "name": scalar(block, "name"),
        }
        for identifier, block in gate_blocks.items()
    ]

    source_paths = [
        "spec/requirements.json",
        "spec/uat-cases.json",
        "spec/localization-requirements.json",
        "spec/feature-gates.yaml",
        "spec/implementation-plan.yaml",
        "spec/traceability-ownership.json",
    ]
    source_paths.extend(sorted(ownership.get("authority_sources", {})))
    source_digest = hashlib.sha256()
    try:
        for relative in source_paths:
            source_path = repository_regular_file_path(relative)
            if source_path is None:
                raise OSError(
                    f"{relative} is not a visible, non-ignored regular "
                    "repository-owned file without symlink components"
                )
            source_digest.update(relative.encode("utf-8"))
            source_digest.update(b"\0")
            source_digest.update(source_path.read_bytes())
            source_digest.update(b"\0")
    except OSError as exc:
        errors.append(f"traceability report projection: source digest failed: {exc}")
        expected_source_sha256 = ""
    else:
        expected_source_sha256 = source_digest.hexdigest()

    expected = {
        "schema_version": ownership.get("schema_version"),
        "status": ownership.get("status"),
        "source_sha256": expected_source_sha256,
        "delivery_boundary": ownership.get("delivery_boundary"),
        "design_sequence": ownership.get("design_sequence"),
        "p03_resume_authority": ownership.get("p03_resume_authority"),
        "approved_strategy": ownership.get("approved_strategy"),
        "counts": {
            "requirements": len(expected_requirements),
            "personas": len(persona_ids),
            "uat_cases": len(expected_uat),
            "feature_gates": len(expected_gates),
            "implementation_tasks": len(expected_tasks),
            "evidence_owners": len(ownership.get("evidence_owners", [])),
            "proposal_dispositions": len(ownership.get("proposal_dispositions", [])),
            "cross_release_dispositions": len(
                ownership.get("cross_release_ownership_dispositions", {})
            ),
            "forward_evidence_dispositions": len(
                ownership.get("forward_evidence_dispositions", {})
            ),
            "authority_sources": len(ownership.get("authority_sources", {})),
        },
        "milestones": ownership.get("milestones"),
        "evidence_owners": ownership.get("evidence_owners"),
        "cross_release_ownership_dispositions": ownership.get(
            "cross_release_ownership_dispositions"
        ),
        "forward_evidence_dispositions": ownership.get(
            "forward_evidence_dispositions"
        ),
        "authority_sources": ownership.get("authority_sources"),
        "requirements": expected_requirements,
        "uat_cases": expected_uat,
        "feature_gates": expected_gates,
        "implementation_tasks": expected_tasks,
        "proposal_dispositions": ownership.get("proposal_dispositions"),
        "review_branch_dispositions": ownership.get("review_branch_dispositions"),
    }

    for section, expected_value in expected.items():
        actual_value = report.get(section)
        actual_json = json.dumps(
            actual_value,
            ensure_ascii=False,
            sort_keys=True,
            separators=(",", ":"),
        )
        expected_json = json.dumps(
            expected_value,
            ensure_ascii=False,
            sort_keys=True,
            separators=(",", ":"),
        )
        if actual_json != expected_json:
            errors.append(
                f"traceability report projection: {section} does not exactly match canonical sources"
            )

    for section in ("requirements", "uat_cases"):
        rows = report.get(section, [])
        if not isinstance(rows, list):
            continue
        for row in rows:
            if not isinstance(row, dict):
                continue
            if row.get("owning_task") in row.get("supporting_tasks", []):
                errors.append(
                    f"traceability report projection: {row.get('id')} owning task "
                    "must not appear in supporting_tasks"
                )
            if (
                section == "uat_cases"
                and row.get("owning_gate") in row.get("supporting_gates", [])
            ):
                errors.append(
                    f"traceability report projection: {row.get('id')} owning gate "
                    "must not appear in supporting_gates"
                )


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


INVALIDATED_COMPLETION_SNAPSHOTS = {
    ".planning/PROJECT.md": {
        "blob": "888e674e9a921a920e41cb99396889b156a02aab",
        "sha256": "148078eabba934113cb2cad3aa04841c854dfe6fbf9b390ac4012ee76fd868cd",
        "lines": 21,
        "forbidden": ("after vB0 milestone completion",),
    },
    ".planning/ROADMAP.md": {
        "blob": "2228414a330f2fdc1083da471a8f9de8dd9004d2",
        "sha256": "f7a10f15bd4682e46d2b367ed47b218872a2be2cbceef8527bb33a8f1c5a411a",
        "lines": 33,
        "forbidden": ("SHIPPED 2026-07-22",),
    },
    ".planning/STATE.md": {
        "blob": "b815885e06510d26cf37b5a68a9ceba8d096b37e",
        "sha256": "59a9f1c21e382f48de3a747582210a34047e595a27254d7f8b6cde9ce8d57393",
        "lines": 17,
        "forbidden": (
            "All 156 requirements, 75 UAT cases, 48 feature gates, and 79 implementation tasks verified",
            "None. All B0 workplace review alpha requirements satisfied",
        ),
    },
    ".planning/RETROSPECTIVE.md": {
        "blob": "b0435e91524ff89456269c70a5899ce78b75454d",
        "sha256": "eb7d4d6d30429e7ec8fb4893ba3b4f8bd17c476aa918b6fa5569a227b7f0b4ac",
        "lines": 23,
        "forbidden": ("8 fully functional screens", "WCAG 2.2 AA compliant"),
    },
    ".planning/milestones/vB0-REQUIREMENTS.md": {
        "blob": "b2f204e30ae2ae2fc82b01867292de14098992cd",
        "sha256": "8702809d8311d6304ce4b33b170dda5a85c11955b56023114d3c054b0ac93d06",
        "lines": 17,
        "forbidden": ("Status:** All 156 requirements validated",),
    },
    ".planning/milestones/vB0-ROADMAP.md": {
        "blob": "8b1b053ce6d23db52289559c98ebe88c7cb3caea",
        "sha256": "d640f7ceea64026f76d90a4a6e7948aa4563e9960950f4231a46a85055bb3398",
        "lines": 23,
        "forbidden": ("**Status:** ✅ Complete",),
    },
    ".planning/phases/P04-desktop-shell/P04-SUMMARY.md": {
        "blob": "4d1e48e121f2ca4c29a27c1ec1f8a9397205eff6",
        "sha256": "e7e970622b8112675ec4866f7c96b6f7f9b55c42408aee28d0d597e6c6e4383d",
        "lines": 15,
        "forbidden": ("**Status**: Complete",),
    },
    ".planning/phases/P04-desktop-shell/P04-UAT.md": {
        "blob": "077c96d27433dcbf02b4585dac390190586a2f25",
        "sha256": "c59e9e708431e9ce453af1c4ea498605c24251a81d581c7db2fcf6f1b1f865f8",
        "lines": 16,
        "forbidden": ("**Status**: All 75 UAT Cases Complete",),
    },
    ".planning/phases/P04-desktop-shell/P04-VERIFICATION.md": {
        "blob": "dd7d2a19486c552e2a8ab9290fce18ea61f20eaa",
        "sha256": "c384238a8115cdadf11184264c3c5b56ac98f58714ca30e283bfb689025bfb38",
        "lines": 13,
        "forbidden": ("**Result**: 100% Verified",),
    },
    "docs/evidence/P04-UI-REVIEW.md": {
        "blob": "638c8564fbd0e0ffb69c47e8242c271a73a15a44",
        "sha256": "d477b7c417bd636c6841d06483bc4dbb42b2f81c019544fb3df7b980c4c1de58",
        "lines": 41,
        "forbidden": ("**Overall Score**: **24/24**", "Execution Approved"),
    },
    "docs/evidence/design/P04-UI-REVIEW.md": {
        "blob": "bc233e8bc088569231993a126991509c36ec1045",
        "sha256": "50653c5b2721d4071d9fb259bc5e358d019176e471dadfccebc2efb0a36bff6e",
        "lines": 48,
        "forbidden": (
            "The Liaison RM desktop shell fully satisfies all 6 pillars",
            "Overall Audit Score: 24 / 24",
        ),
    },
}


def validate_current_main_truth_sources(errors: list[str]) -> None:
    """Keep 49ee/vB0 provenance from masquerading as accepted delivery."""

    common_fragments = (
        "Status: **invalidated; not an active planning or acceptance source**",
        "Current authority: `docs/product/working-state-delivery.md` and `spec/traceability-ownership.json`",
        "Original commit: `49ee419e30f2d71524dd6fa15badf1ec4b8d0e27`",
        "P00-P02: complete",
        "G1 and `T-B0-P03`: current",
        "`T-B0-P03-OBS`, `T-B0-P03D`, `T-B0-P04`, P05-P11, and B0 acceptance: blocked",
        "PILOT: deferred after B0; real workplace data remains denied",
        "No P03 completion is inferred from PR #65/`3499a6e`, `49ee419`, `vB0`",
        "premature `c2f852c` P03O material",
    )
    for relative, provenance in INVALIDATED_COMPLETION_SNAPSHOTS.items():
        try:
            source = (ROOT / relative).read_text(encoding="utf-8")
        except OSError as exc:
            errors.append(f"{relative}: cannot verify invalidated snapshot: {exc}")
            continue
        require_fragments(
            relative,
            source,
            common_fragments
            + (
                f"Original Git blob: `{provenance['blob']}`",
                f"Original file SHA-256: `{provenance['sha256']}`",
                f"Original line count: {provenance['lines']}",
            ),
            errors,
        )
        for fragment in provenance["forbidden"]:
            if fragment.lower() in source.lower():
                errors.append(
                    f"{relative}: invalidated snapshot revives false completion "
                    f"claim {fragment!r}"
                )
        if re.search(r"\bUAT-P04-\d{3}\b", source):
            errors.append(
                f"{relative}: invalidated snapshot revives invented UAT-P04 identifiers"
            )

    active_truth = {
        "AGENTS.md": (
            "every gate that remains blocked, deferred, current, or otherwise not complete",
        ),
        "docs/product/working-state-delivery.md": (
            "49ee419e30f2d71524dd6fa15badf1ec4b8d0e27",
            "reviewed pre-reconciliation main at 2026-07-22",
            "The C3 commit and any merge result are distinct identities requiring their own exact-head receipt; absent that receipt they do not advance acceptance.",
            "Machine authority therefore keeps P03 `current`",
            "P03D, P04, P05-P11, and B0 blocked, and PILOT deferred",
            "unsupported historical claim, not a release",
            "remain non-accepted candidate material",
            "exposes an Events-labelled destination through the `readiness` route alias",
            "current static-shell policy check does not",
            "reject that alias",
        ),
        "PROJECT_CONTEXT.md": (
            "G1 and P03 are current",
            "P05-P11, and B0 remain blocked; PILOT remains deferred",
            "49ee419e30f2d71524dd6fa15badf1ec4b8d0e27",
            "reviewed pre-reconciliation main at 2026-07-22",
            "The C3 commit and any merge result are distinct identities requiring their own exact-head receipt; absent that receipt they do not advance acceptance.",
            "unsupported historical claim",
            "non-accepted candidate material",
        ),
        "README.md": (
            "unsupported historical claim, not a release",
            "Current machine authority keeps P03 `current`",
            "P03D, P04, P05-P11, and B0 blocked, and PILOT deferred",
            "blocked, deferred, current, or complete gates",
        ),
        "SPEC.md": (
            "reviewed pre-reconciliation main at 2026-07-22",
            "The C3 commit and any merge result are distinct identities requiring their own exact-head receipt; absent that receipt they do not advance acceptance.",
            "P03 remains technically unaccepted",
            "unsupported `vB0` tag",
            "P03D, P04, P05-P11, or B0",
        ),
        "AI_BUILD_INSTRUCTIONS.md": (
            "reviewed pre-reconciliation main at 2026-07-22",
            "The C3 commit and any merge result are distinct identities requiring their own exact-head receipt; absent that receipt they do not advance acceptance.",
            "no accepted P03 identity tuple or OBS receipt",
            "unsupported historical `vB0` tag",
            "P03D/P04 work",
        ),
        "CHANGELOG.md": (
            "Preserved candidate-source history",
            "Historical Tauri alpha source and prototype provenance",
            "A preserved Editorial Ledger candidate proposes",
            "non-accepted design provenance",
            "reviewed pre-reconciliation main at 2026-07-22",
            "The C3 commit and any merge result are distinct identities requiring their own exact-head receipt; absent that receipt they do not advance acceptance.",
            "unsupported historical shipment claim",
            "P03 remains current and P03D/P04 remain blocked",
            "superseded as active status sources",
            "keeps OBS/P03D/P04/P05-P11/B0 blocked and PILOT deferred",
        ),
        "crates/liaison-application/README.md": (
            "P03 remains current",
            "not application-contract or release evidence",
            "P03D, P04, P05-P11, and B0 acceptance remain blocked; PILOT remains deferred",
        ),
        "docs/product/roadmap.md": (
            "reviewed pre-reconciliation main at 2026-07-22",
            "The C3 commit and any merge result are distinct identities requiring their own exact-head receipt; absent that receipt they do not advance acceptance.",
            "unsupported historical claim",
            "does not advance P03, OBS, P03D, P04, P05-P11, or B0",
            "`FG-B0-001` remains blocked for its other owners",
        ),
        "docs/evidence/application/p03-recoverable-operations.md": (
            "invalidated as acceptance evidence; source candidate only; P03 remains current",
            "must not state that every crash or external-edit boundary is safe",
            "Until then, `T-B0-P03` is `current`, not complete",
        ),
        "docs/knowledge/KCS-0014-when-may-the-events-destination-be-enabled.md": (
            "reviewed pre-reconciliation main at 2026-07-22",
            "The C3 commit and any merge result are distinct identities requiring their own exact-head receipt; absent that receipt they do not advance acceptance.",
            "list each affected gate as blocked, deferred, current, complete, or otherwise not complete",
            "the exact blocked, deferred, current, or complete state of every affected gate",
        ),
        "docs/product/rice-prioritization.md": (
            "reviewed pre-reconciliation main on 2026-07-22",
            "The C3 commit and any merge result are distinct identities requiring their own exact-head receipt; absent that receipt they do not advance acceptance.",
            "only P00-P02 are accepted and P03 is current",
        ),
        "docs/evidence/design/semantic-token-contrast-evidence.md": (
            "candidate",
            "not as a screenshot judgement or accepted design authority",
            "roles proposed by the preserved Editorial Ledger candidate",
            "non-authoritative inputs from preserved candidate review material",
        ),
        "docs/decisions/0016-scope-general-migration-safety-to-r5.md": (
            "At exact base `49ee419e30f2d71524dd6fa15badf1ec4b8d0e27`",
            "`LRM-WS-007` was still a release-R1 requirement owned by `T-B0-P03` and `FG-B0-001`",
            "existing general-migration task `T-R5-005` and gate `FG-R5-005` were already scoped to R5",
            "least-invented correction",
        ),
    }
    for relative, fragments in active_truth.items():
        try:
            source = (ROOT / relative).read_text(encoding="utf-8")
        except OSError as exc:
            errors.append(f"{relative}: cannot verify current-main truth: {exc}")
            continue
        require_fragments(relative, source, fragments, errors)

    forbidden_surface_claims = {
        "CHANGELOG.md": (
            "retryable desktop create/open actions",
            "Local Tauri desktop alpha for workspace creation",
            "The Editorial Ledger direction applied to the desktop review shell",
            "the approved paper-canvas palette",
            "Exact current head `49ee419`",
        ),
        "PROJECT_CONTEXT.md": ("Current main is `49ee419",),
        "AI_BUILD_INSTRUCTIONS.md": ("Current main `49ee419",),
        "SPEC.md": ("Current main `49ee419",),
        "docs/product/working-state-delivery.md": (
            "Exact current main",
            "Current main later advanced to `49ee419",
        ),
        "docs/product/roadmap.md": ("Later current main `49ee419",),
        "docs/product/rice-prioritization.md": ("At current main `49ee419",),
        "docs/knowledge/KCS-0014-when-may-the-events-destination-be-enabled.md": (
            "Exact current main `49ee419",
        ),
    }
    for relative, fragments in forbidden_surface_claims.items():
        try:
            source = (ROOT / relative).read_text(encoding="utf-8")
        except OSError as exc:
            errors.append(f"{relative}: cannot verify false-current claims: {exc}")
            continue
        for fragment in fragments:
            if fragment.lower() in source.lower():
                errors.append(
                    f"{relative}: false current-surface or temporal claim {fragment!r}"
                )

    try:
        design_candidate = (
            ROOT / "docs/evidence/design/semantic-token-contrast-evidence.md"
        ).read_text(encoding="utf-8").lower()
    except OSError as exc:
        errors.append(f"semantic-token candidate evidence: {exc}")
    else:
        for forbidden in ("roles are normative", "per the accepted review"):
            if forbidden in design_candidate:
                errors.append(
                    "semantic-token candidate evidence must not claim accepted design "
                    f"authority through {forbidden!r}"
                )

    for forbidden_path in (
        "docs/decisions/0014-observe-the-exact-p03-artifact-before-design-authority.md",
        "docs/knowledge/KCS-0015-how-do-we-complete-p03-exact-artifact-observation.md",
    ):
        if (ROOT / forbidden_path).exists():
            errors.append(
                f"{forbidden_path}: premature competing P03O authority must remain absent"
            )

    for relative in (
        "spec/requirements.json",
        "spec/uat-cases.json",
        "spec/feature-gates.yaml",
        "spec/implementation-plan.yaml",
        "spec/traceability-ownership.json",
    ):
        try:
            source = (ROOT / relative).read_text(encoding="utf-8")
        except OSError as exc:
            errors.append(f"{relative}: cannot verify canonical OBS identity: {exc}")
            continue
        for alias in ("T-B0-P03O", "FG-B0-P03-OBS-001"):
            if alias in source:
                errors.append(
                    f"{relative}: competing premature OBS identifier {alias} is forbidden"
                )


def validate_pilot_boundary_sources(
    sources: dict[str, str], errors: list[str]
) -> None:
    """Lock the post-B0, affirmative, fully completed pilot boundary."""

    expected = {
        "PROJECT_CONTEXT.md": (
            "remain deferred until after B0 acceptance",
            "PILOT is not a prerequisite for synthetic B0, A0, or provider delivery",
            "the independent conclusion must be exactly `authorise`",
            "both the pilot task and gate must be `complete`",
            "drafted, current, or merely activated states remain denied",
        ),
        "docs/knowledge/KCS-0011-how-do-we-authorise-a-real-workplace-data-pilot.md": (
            "remain deferred until B0 is accepted",
            "Before B0, all of these materials remain drafts",
            "no final independent conclusion or pilot execution evidence may be recorded",
            "The final conclusion must be exactly `authorise`",
            "`authorise with conditions`, `do not authorise`, defer, or any unresolved condition keeps real data denied",
            "`T-B0-PILOT` and `FG-B0-PILOT-001` are both `complete`",
            "A drafted, current, or merely activated task or gate is still denied",
            "a mismatch or content change reopens review",
            "returns the `real-workplace-data` capability to denied",
            "returns `T-B0-PILOT` and `LRM-EV-012` to current",
            "returns `FG-B0-PILOT-001` to blocked",
        ),
        "docs/pilot/README.md": (
            "Synthetic-fixture development and UAT continue unaffected",
            "remain deferred and the `real-workplace-data` capability remains denied",
            "starting pilot work makes `T-B0-PILOT` and `LRM-EV-012` current while `FG-B0-PILOT-001` is blocked",
            "Before B0, every governance artifact remains a draft",
            "no final independent conclusion or pilot execution evidence may be recorded",
            "The final independent-review conclusion must be exactly `authorise`",
            "both `T-B0-PILOT` and `FG-B0-PILOT-001` must be `complete`",
            "Drafted, current, or merely activated states remain denied",
            "a mismatch or material content change reopens review",
            "the `real-workplace-data` capability remains denied unless and until",
            "Human decisions are necessary but do not complete the gate",
        ),
        "docs/pilot/independent-review-record.md": (
            "must not record a final conclusion before exact-artifact B0 acceptance and the post-B0 pilot technical-denial evidence exist",
            "Only a signed final conclusion of exactly `authorise` with every condition `resolved` completes this governance record",
            "`Authorise with conditions`, `do not authorise`, defer, an unresolved condition, or an unsigned conclusion keeps real data denied",
            "complete `T-B0-PILOT` and `FG-B0-PILOT-001`",
            "only after both the task and gate are `complete` may real workplace data enter",
            "any mismatch or content change reopens this review",
            "returns the `real-workplace-data` capability to denied",
            "returns `T-B0-PILOT` and `LRM-EV-012` to current",
            "returns `FG-B0-PILOT-001` to blocked",
        ),
        "docs/pilot/participant-notice.md": (
            "the notice must not be issued while any source record is open",
            "Before B0, this remains a draft",
            "the exact completed content artifact is included in the independent review before issuance",
            "Only an issued content artifact whose SHA-256 equals that reviewed digest qualifies",
            "a content change reopens review",
            "Liaison itself requires no vendor account or Electric Town hosted service",
            "actual processors, managed-device backup, cloud backup, or connected services used for this pilot",
            "B0 role presets do not create confidentiality from a person who controls the same unlocked operating-system account or workspace files",
            "Processor, storage, backup, connected-service, operator, and access-control statements match the completed source records",
        ),
    }
    for relative, fragments in expected.items():
        require_fragments(relative, sources.get(relative, ""), fragments, errors)

    ordered = {
        "docs/knowledge/KCS-0011-how-do-we-authorise-a-real-workplace-data-pilot.md": (
            "Complete the participant-notice draft without issuing it",
            "After B0 is accepted",
            "Obtain the independent legal and privacy review",
            "The final conclusion must be exactly `authorise`",
            "Issue the exact participant-notice content artifact that received the affirmative review",
            "complete `T-B0-PILOT` and `FG-B0-PILOT-001`",
            "Only after the task and gate are both `complete` may any person's real data enter",
        ),
        "docs/pilot/README.md": (
            "Complete the participant-notice draft without issuing it",
            "After B0 is accepted",
            "Obtain the dated, scoped independent review of that exact notice content artifact",
            "the final conclusion must be `authorise`",
            "Issue the exact reviewed participant-notice content artifact",
            "Complete `T-B0-PILOT` and `FG-B0-PILOT-001`",
            "only then may real data enter",
        ),
        "docs/pilot/independent-review-record.md": (
            "issue the exact participant-notice content artifact reviewed here",
            "update the feature-gate evidence",
            "complete `T-B0-PILOT` and `FG-B0-PILOT-001`",
            "only after both the task and gate are `complete` may real workplace data enter",
        ),
    }
    for relative, fragments in ordered.items():
        require_ordered_fragments(
            f"{relative} pilot sequence",
            sources.get(relative, ""),
            fragments,
            errors,
        )

    joined = "\n".join(sources.values()).lower()
    forbidden = (
        "pilot is a prerequisite for synthetic b0",
        "synthetic-fixture development and uat are blocked",
        "real data may proceed before b0",
        "do not authorise conclusion completes",
        "merely activated gate authorises real data",
        "an incomplete review authorises real data",
        "a current pilot task and gate authorise real data",
        "an `authorise with conditions` conclusion completes the pilot and authorises real data",
        "issue the participant notice before independent review",
        "issue the notice before independent review",
        "issue the notice before the independent review",
        "complete the pilot task and gate before issuing the notice",
        "gate closes only on human decisions",
        "returns the gate to closed",
        "then closed until governance",
        "holds the `real-workplace-data` capability closed",
        "machine-owned blocked or deferred lifecycle",
        "synthetic-fixture development and uat continue unaffected. only real data is blocked",
        "there is no vendor account and no cloud service behind this pilot",
        "only the named pilot operators can open the workspace",
        "does not reopen review",
    )
    for phrase in forbidden:
        if phrase in joined:
            errors.append(f"pilot boundary: forbidden inversion {phrase!r}")


def validate_google_drive_provider_plan(text: str, errors: list[str]) -> None:
    """Keep the single R5 Google Drive row exact and non-duplicated."""

    expected = "| Google Drive | object-store@1 where conformance permits | R5 |"
    rows = [
        line.strip()
        for line in text.splitlines()
        if re.match(r"^\|\s*Google Drive\s*\|", line, flags=re.IGNORECASE)
    ]
    if rows != [expected]:
        errors.append(
            "providers/README.md: Google Drive must have exactly one plan "
            f"row equal to {expected!r}, not {rows!r}"
        )


def validate_build_guidance_gate_ownership(text: str, errors: list[str]) -> None:
    """Prevent P06 and the OKF slices from being assigned to one gate."""

    require_fragments(
        "AI_BUILD_INSTRUCTIONS.md",
        text,
        (
            "P05-OKF's pinned OKF v0.1 Draft strict writer/schema port as an `FG-B0-001` slice",
            "P06's tolerant Directory reader and domain-validity quarantine, which is accepted by `FG-R1-003`",
            "P09-OKF is the later acceptance task that closes `FG-B0-001`",
        ),
        errors,
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


@repository_custody_validation
def validate_corrected_phase_ownership(
    requirements: list[dict],
    cases: list[dict],
    task_blocks: dict[str, str],
    gate_blocks: dict[str, str],
    errors: list[str],
    ownership_document: object | None = None,
) -> None:
    """Lock the reviewed P02/P03, observation, migration, repair, and acceptance boundaries."""

    if ownership_document is None:
        try:
            ownership = load_json("spec/traceability-ownership.json")
        except (OSError, ValueError, json.JSONDecodeError) as exc:
            errors.append(f"corrected phase ownership: {exc}")
            return
    else:
        ownership = ownership_document
    if not isinstance(ownership, dict):
        errors.extend(traceability_ownership_schema_issues(ownership))
        return
    ownership_schema_errors = traceability_ownership_schema_issues(ownership)
    if ownership_schema_errors:
        errors.extend(ownership_schema_errors)
        return

    requirement_by_id = {item["id"]: item for item in requirements}
    case_by_id = {item["id"]: item for item in cases}
    task_requirements = inline_edges(task_blocks, "requirements")
    task_uat = inline_edges(task_blocks, "uat")
    task_dependencies = inline_edges(task_blocks, "depends_on")
    task_evidence = inline_edges(task_blocks, "evidence_dependencies")
    task_outputs = inline_edges(task_blocks, "outputs")
    task_required_artifacts = inline_edges(task_blocks, "required_artifacts")
    task_contexts = inline_edges(task_blocks, "contexts")

    expected_task_edges = {
        "T-R4-008": (set(), set()),
        "T-R5-005": (
            {"LRM-CO-013", "LRM-WS-007"},
            {"UAT-038"},
        ),
        "T-R5-010": ({"LRM-CO-012"}, {"UAT-074"}),
        "T-B0-P02": (
            {"LRM-WS-002", "LRM-WS-009"},
            set(),
        ),
        "T-B0-P03": (
            {"LRM-WS-004", "LRM-WS-005", "LRM-WS-010"},
            {"UAT-042"},
        ),
        "T-B0-P03-OBS": ({"LRM-PK-010"}, set()),
        "T-B0-P03D": (set(), set()),
        "T-B0-P04": (
            {
                "LRM-UX-001",
                "LRM-UX-002",
                "LRM-UX-003",
                "LRM-UX-004",
                "LRM-UX-006",
                "LRM-UX-008",
                "LRM-UX-012",
                "LRM-L10N-001",
                "LRM-L10N-002",
                "LRM-L10N-003",
                "LRM-L10N-004",
                "LRM-L10N-005",
                "LRM-L10N-006",
                "LRM-L10N-008",
            },
            {"UAT-073", "UAT-LOC-001", "UAT-LOC-002"},
        ),
        "T-B0-P06-REPAIR": (set(), {"UAT-040"}),
        "T-B0-P09-OKF": ({"LRM-WS-017"}, {"UAT-066"}),
        "T-B0-P11": (
            {
                "LRM-WS-008",
                "LRM-UX-005",
                "LRM-UX-009",
                "LRM-UX-011",
                "LRM-UX-016",
            },
            {"UAT-062"},
        ),
        "T-A0-P03": (
            {
                "LRM-RE-001",
                "LRM-RE-002",
                "LRM-RE-003",
                "LRM-RE-004",
                "LRM-RE-005",
                "LRM-RE-006",
                "LRM-RE-007",
                "LRM-RE-008",
                "LRM-RM-001",
                "LRM-RM-002",
                "LRM-RM-003",
                "LRM-RM-004",
                "LRM-RM-005",
            },
            {"UAT-004", "UAT-022", "UAT-023", "UAT-051", "UAT-052"},
        ),
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
    if task_evidence.get("T-B0-P06-REPAIR", set()) != {
        "LRM-WS-006",
        "LRM-WS-010",
        "UAT-042",
    }:
        errors.append(
            "T-B0-P06-REPAIR: evidence dependencies must exclude generic "
            "migration safety LRM-WS-007"
        )
    if task_evidence.get("T-R4-008", set()) != {
        "LRM-SH-001",
        "UAT-019",
        "UAT-026",
        "UAT-027",
    }:
        errors.append(
            "T-R4-008: R4 transports must support but not own the R5 "
            "Google Drive/object-store contract"
        )
    if task_dependencies.get("T-B0-P05", set()) != {"T-B0-P04"}:
        errors.append(
            "T-B0-P05: must depend exactly on completed P04 so accepted P03 "
            "cannot bypass OBS, P03D, or the typed desktop boundary"
        )
    if task_dependencies.get("T-R5-010", set()) != {"T-R4-004", "T-R4-008"}:
        errors.append(
            "T-R5-010: Google Drive must depend exactly on the R4 backup "
            "service and transport foundation"
        )
    if task_evidence.get("T-R5-010", set()) != {
        "UAT-026",
        "UAT-027",
        "UAT-028",
        "UAT-034",
    }:
        errors.append(
            "T-R5-010: Google Drive must reuse the grant, mode-label, "
            "restore, and provider-isolation evidence"
        )
    if task_dependencies.get("T-B0-P03-OBS", set()) != {"T-B0-P03"}:
        errors.append("T-B0-P03-OBS: must depend exactly on T-B0-P03")
    if task_dependencies.get("T-B0-P03D", set()) != {"T-B0-P03-OBS"}:
        errors.append("T-B0-P03D: must depend exactly on T-B0-P03-OBS under D1-B")
    if task_dependencies.get("T-B0-P04", set()) != {"T-B0-P03D"}:
        errors.append(
            "T-B0-P04: must depend exactly on T-B0-P03D so accepted P03 "
            "cannot bypass OBS, Continue, or the design gate"
        )
    if task_evidence.get("T-B0-P03-OBS", set()) != {"UAT-042"}:
        errors.append("T-B0-P03-OBS: must retain UAT-042 as P03 evidence")
    exact_artifact_contract = {
        "qualified-code-sha",
        "merge-result-sha",
        "attestation-sha",
        "exact-executable-artifact-receipt",
    }
    if not exact_artifact_contract <= task_outputs.get("T-B0-P03", set()):
        errors.append(
            "T-B0-P03: must output the complete exact qualification and "
            "executable-artifact identity contract before observation"
        )
    if task_required_artifacts.get("T-B0-P03-OBS", set()) != exact_artifact_contract:
        errors.append(
            "T-B0-P03-OBS: must consume exactly the accepted P03 identity "
            "artifacts rather than inventing identity during observation"
        )
    if "governance" not in task_contexts.get("T-B0-P03-OBS", set()):
        errors.append("T-B0-P03-OBS: must name the governance context owning LRM-PK-010")
    if "connections" not in task_contexts.get("T-R5-005", set()):
        errors.append("T-R5-005: must name the Connections anti-corruption context")
    if task_evidence.get("T-B0-P03D", set()) != {
        "LRM-UX-009",
        "LRM-UX-012",
        "LRM-UX-016",
        "UAT-062",
    }:
        errors.append(
            "T-B0-P03D: future P11 full-journey contracts must remain "
            "evidence-only design inputs"
        )
    if task_evidence.get("T-B0-P04", set()) != {"UAT-041"}:
        errors.append("T-B0-P04: evidence dependencies must be exactly UAT-041")
    if task_evidence.get("T-B0-P11", set()) != {
        "LRM-PK-006",
        "LRM-UX-012",
        "UAT-041",
        "UAT-042",
        "UAT-043",
    }:
        errors.append(
            "T-B0-P11: must consume the P04 theme foundation without "
            "treating owned full-journey contracts as evidence dependencies"
        )
    if "installed-current-surface-theme-recovery-matrix" not in task_outputs.get(
        "T-B0-P04", set()
    ):
        errors.append("T-B0-P04: must output the installed current-surface theme matrix")
    if "installed-every-built-in-details-to-brief-matrix" not in task_outputs.get(
        "T-B0-P11", set()
    ):
        errors.append("T-B0-P11: must output the installed full-journey theme matrix")
    for task in ("T-R2-001", "T-R2-002", "T-R2-003", "T-B0-P04"):
        if "UAT-022" in task_evidence.get(task, set()) or "UAT-022" in task_uat.get(
            task, set()
        ):
            errors.append(f"{task}: must not claim A0 relationship-reminder UAT-022")
    if "migration-dry-run" in task_blocks.get("T-B0-P03", "").lower():
        errors.append("T-B0-P03: generic migration dry-run belongs to T-R5-005")
    require_fragments(
        "T-R5-005",
        task_blocks.get("T-R5-005", ""),
        (
            "migration-dry-run",
            "pre-migration-backup",
            "deterministic-migration",
            "post-migration-validation",
            "rollback-or-explicit-irreversibility",
        ),
        errors,
    )
    require_fragments(
        "T-R5-010",
        task_blocks.get("T-R5-010", ""),
        (
            "Google Drive object-store backup adapter",
            "object-store-contract-reuse-evidence",
            "encrypted-backup-restore-parity",
            "provider-SDK-domain-isolation",
        ),
        errors,
    )
    require_fragments(
        "FG-R5-007",
        gate_blocks.get("FG-R5-007", ""),
        (
            "UAT-074",
            "object-store@1",
            "isolated restore verifies",
            "without a synchronisation overclaim",
        ),
        errors,
    )
    try:
        provider_plan = (ROOT / "providers/README.md").read_text(encoding="utf-8")
    except OSError as exc:
        errors.append(f"providers/README.md: {exc}")
    else:
        validate_google_drive_provider_plan(provider_plan, errors)

    milestone_records = {
        item.get("id"): item for item in ownership.get("milestones", [])
    }
    pilot_milestone = milestone_records.get("PILOT", {})
    if set(pilot_milestone.get("depends_on", [])) != {"B0"}:
        errors.append(
            "PILOT: optional real-data pilot milestone must depend exactly on B0"
        )
    pilot_contracts = (
        ("task_ownership", "T-B0-PILOT"),
        ("requirement_ownership", "LRM-EV-012"),
        ("gate_ownership", "FG-B0-PILOT-001"),
    )
    b0_status = milestone_records.get("B0", {}).get("status")
    for collection, identifier in pilot_contracts:
        edge = ownership.get(collection, {}).get(identifier, {})
        if edge.get("milestone") != "PILOT":
            errors.append(
                f"{identifier}: optional real-data pilot contract must remain in PILOT"
            )
        if b0_status != "complete" and edge.get("status") != "deferred":
            errors.append(
                f"{identifier}: optional real-data pilot contract must remain "
                "deferred until B0 is complete"
            )
    pilot_statuses = {
        identifier: ownership.get(collection, {}).get(identifier, {}).get("status")
        for collection, identifier in pilot_contracts
    }
    if b0_status != "complete" and any(
        status != "deferred" for status in pilot_statuses.values()
    ):
        errors.append(
            "PILOT: task, requirement, and gate must remain deferred until B0 is complete"
        )
    if pilot_statuses.get("FG-B0-PILOT-001") == "complete" and (
        pilot_statuses.get("T-B0-PILOT") != "complete"
        or pilot_statuses.get("LRM-EV-012") != "complete"
    ):
        errors.append(
            "FG-B0-PILOT-001: completed gate requires completed pilot task and requirement"
        )
    milestone_dependencies = {
        identifier: set(item.get("depends_on", []))
        for identifier, item in milestone_records.items()
    }
    for identifier in ("B0", "G2C", "G2A", "G2B", "A0", "G4", "G5", "G6"):
        if "PILOT" in transitive_dependencies(milestone_dependencies, identifier):
            errors.append(
                f"{identifier}: synthetic delivery path must not depend on optional PILOT"
            )
    pilot_sources: dict[str, str] = {}
    for relative in (
        "PROJECT_CONTEXT.md",
        "docs/knowledge/KCS-0011-how-do-we-authorise-a-real-workplace-data-pilot.md",
        "docs/pilot/README.md",
        "docs/pilot/independent-review-record.md",
        "docs/pilot/participant-notice.md",
    ):
        try:
            pilot_sources[relative] = (ROOT / relative).read_text(encoding="utf-8")
        except OSError as exc:
            errors.append(f"{relative}: cannot verify pilot boundary: {exc}")
    validate_pilot_boundary_sources(pilot_sources, errors)
    require_fragments(
        "LRM-EV-012",
        " ".join(
            str(requirement_by_id.get("LRM-EV-012", {}).get(field, ""))
            for field in ("statement", "acceptance")
        ),
        (
            "deferred until after B0 acceptance",
            "never blocks synthetic B0 acceptance",
            "explicit affirmative independent review",
            "issued_notice_content_sha256 equals reviewed_notice_content_sha256",
            "post-B0 technical-denial and scoped-authorisation evidence",
            "both T-B0-PILOT and FG-B0-PILOT-001 are complete",
        ),
        errors,
    )
    require_fragments(
        "T-B0-PILOT",
        task_blocks.get("T-B0-PILOT", ""),
        (
            "depends_on: [T-B0-ACCEPT]",
            "participant-notice-draft",
            "reviewed-notice-content-sha256",
            "affirmative-independent-review",
            "issued-notice-content-sha256-match",
            "participant-notice-issuance-evidence",
            "post-B0-pilot-denial-and-authorisation-evidence",
        ),
        errors,
    )
    if task_dependencies.get("T-B0-PILOT", set()) != {"T-B0-ACCEPT"}:
        errors.append(
            "T-B0-PILOT: pilot authorisation must depend exactly on B0 acceptance"
        )
    require_fragments(
        "FG-B0-PILOT-001",
        gate_blocks.get("FG-B0-PILOT-001", ""),
        (
            "deferred until after exact-artifact B0 acceptance",
            "completing governance records alone cannot activate them",
            "explicit affirmative authorise conclusion",
            "do-not-authorise, defer, or conditional conclusions keep the pilot denied",
            "reviewed_notice_content_sha256",
            "issued participant-notice content SHA-256 equals reviewed_notice_content_sha256",
            "before this gate completes",
            "post-B0 technical-denial and scoped-authorisation evidence",
        ),
        errors,
    )
    require_fragments(
        "T-B0-P03-OBS",
        task_blocks.get("T-B0-P03-OBS", ""),
        (
            "plan_alias: D9",
            "exact-artifact-identity",
            "synthetic-or-redacted-workplace-observation",
            "continue-change-stop-decision-receipt",
        ),
        errors,
    )
    require_fragments(
        "FG-B0-DESIGN-001",
        gate_blocks.get("FG-B0-DESIGN-001", ""),
        (
            "D1-B observation task records Continue",
            "synthetic or redacted workplace scenarios",
            "real workplace personal data is denied",
            "Continue, Change, or Stop decision",
            "Change or Stop leaves P03D and P04 blocked",
        ),
        errors,
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

    r2_accessibility_gate = gate_blocks.get("FG-R2-001", "")
    for identifier in ("UAT-021", "UAT-022", "UAT-062"):
        if identifier in r2_accessibility_gate:
            errors.append(
                f"FG-R2-001: P04 current-surface acceptance must not require {identifier}"
            )
    require_fragments(
        "FG-R2-001",
        r2_accessibility_gate,
        (
            "UAT-073",
            "UAT-LOC-001",
            "UAT-LOC-002",
            "installed P04 macOS Tauri and WKWebView artifact",
            "creates a workspace",
            "closes and reopens or resumes it",
            "opens a separate existing workspace",
            "system, light, dark, and high_contrast",
            "OS-following",
            "pre/post-COMMIT interruption",
            "zero external requests",
            "product fault hooks are absent",
        ),
        errors,
    )
    require_fragments(
        "FG-A0-G2B",
        gate_blocks.get("FG-A0-G2B", ""),
        ("UAT-022",),
        errors,
    )
    require_fragments(
        "FG-B0-003",
        gate_blocks.get("FG-B0-003", ""),
        ("UAT-062",),
        errors,
    )

    expected_edges = (
        (
            "LRM-WS-001",
            ownership.get("requirement_ownership", {}),
            {
                "owning_task": "T-B0-ACCEPT",
                "owning_gate": "FG-B0-003",
                "milestone": "B0",
                "evidence_owner": "EO-EXPERIENCE",
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
            "LRM-WS-007",
            ownership.get("requirement_ownership", {}),
            {
                "owning_task": "T-R5-005",
                "owning_gate": "FG-R5-005",
                "milestone": "G5",
                "evidence_owner": "EO-CONNECTIONS",
            },
        ),
        (
            "LRM-CO-012",
            ownership.get("requirement_ownership", {}),
            {
                "owning_task": "T-R5-010",
                "owning_gate": "FG-R5-007",
                "milestone": "G5",
                "evidence_owner": "EO-CONNECTIONS",
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
            },
        ),
        (
            "LRM-PK-010",
            ownership.get("requirement_ownership", {}),
            {
                "owning_task": "T-B0-P03-OBS",
                "owning_gate": "FG-B0-DESIGN-001",
                "milestone": "G1",
                "evidence_owner": "EO-EXPERIENCE",
            },
        ),
        (
            "LRM-UX-009",
            ownership.get("requirement_ownership", {}),
            {
                "owning_task": "T-B0-P11",
                "owning_gate": "FG-B0-003",
                "milestone": "B0",
                "evidence_owner": "EO-EXPERIENCE",
            },
        ),
        (
            "LRM-UX-012",
            ownership.get("requirement_ownership", {}),
            {
                "owning_task": "T-B0-P04",
                "owning_gate": "FG-R2-001",
                "milestone": "G1",
                "evidence_owner": "EO-EXPERIENCE",
            },
        ),
        (
            "LRM-UX-016",
            ownership.get("requirement_ownership", {}),
            {
                "owning_task": "T-B0-P11",
                "owning_gate": "FG-B0-003",
                "milestone": "B0",
                "evidence_owner": "EO-EXPERIENCE",
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
            },
        ),
        (
            "UAT-022",
            ownership.get("uat_ownership", {}),
            {
                "owning_task": "T-A0-P03",
                "owning_gate": "FG-A0-G2B",
                "milestone": "G2B",
                "evidence_owner": "EO-PROFILES",
            },
        ),
        (
            "UAT-062",
            ownership.get("uat_ownership", {}),
            {
                "owning_task": "T-B0-P11",
                "owning_gate": "FG-B0-003",
                "milestone": "B0",
                "evidence_owner": "EO-EXPERIENCE",
            },
        ),
        (
            "UAT-073",
            ownership.get("uat_ownership", {}),
            {
                "owning_task": "T-B0-P04",
                "owning_gate": "FG-R2-001",
                "milestone": "G1",
                "evidence_owner": "EO-EXPERIENCE",
            },
        ),
        (
            "UAT-074",
            ownership.get("uat_ownership", {}),
            {
                "owning_task": "T-R5-010",
                "owning_gate": "FG-R5-007",
                "milestone": "G5",
                "evidence_owner": "EO-CONNECTIONS",
            },
        ),
        (
            "T-B0-P03",
            ownership.get("task_ownership", {}),
            {
                "milestone": "G1",
                "owning_gate": "FG-B0-001",
                "evidence_owner": "EO-WORKSPACE",
            },
        ),
        (
            "T-B0-P03-OBS",
            ownership.get("task_ownership", {}),
            {
                "milestone": "G1",
                "owning_gate": "FG-B0-DESIGN-001",
                "evidence_owner": "EO-EXPERIENCE",
            },
        ),
        (
            "T-B0-P03D",
            ownership.get("task_ownership", {}),
            {
                "milestone": "G1",
                "owning_gate": "FG-B0-DESIGN-001",
                "evidence_owner": "EO-EXPERIENCE",
            },
        ),
        (
            "T-B0-P04",
            ownership.get("task_ownership", {}),
            {
                "milestone": "G1",
                "owning_gate": "FG-R2-001",
                "evidence_owner": "EO-EXPERIENCE",
            },
        ),
        (
            "T-R5-010",
            ownership.get("task_ownership", {}),
            {
                "milestone": "G5",
                "owning_gate": "FG-R5-007",
                "evidence_owner": "EO-CONNECTIONS",
            },
        ),
        (
            "T-B0-P06-REPAIR",
            ownership.get("task_ownership", {}),
            {
                "milestone": "G1",
                "owning_gate": "FG-R1-002",
                "evidence_owner": "EO-WORKSPACE",
            },
        ),
        (
            "T-B0-P05",
            ownership.get("task_ownership", {}),
            {
                "milestone": "G1",
                "owning_gate": "FG-R3-001",
                "evidence_owner": "EO-EVENTS",
            },
        ),
        (
            "T-B0-P05-OKF",
            ownership.get("task_ownership", {}),
            {
                "milestone": "G1",
                "owning_gate": "FG-B0-001",
                "evidence_owner": "EO-WORKSPACE",
            },
        ),
        (
            "T-B0-P06",
            ownership.get("task_ownership", {}),
            {
                "milestone": "G1",
                "owning_gate": "FG-R1-003",
                "evidence_owner": "EO-WORKSPACE",
            },
        ),
        (
            "T-B0-P09-OKF",
            ownership.get("task_ownership", {}),
            {
                "milestone": "G1",
                "owning_gate": "FG-B0-001",
                "evidence_owner": "EO-WORKSPACE",
            },
        ),
        (
            "T-B0-PILOT",
            ownership.get("task_ownership", {}),
            {
                "milestone": "PILOT",
                "owning_gate": "FG-B0-PILOT-001",
                "evidence_owner": "EO-EVENTS",
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
            "FG-B0-DESIGN-001",
            ownership.get("gate_ownership", {}),
            {
                "acceptance_task": "T-B0-P03D",
                "milestone": "G1",
                "evidence_owner": "EO-EXPERIENCE",
            },
        ),
        (
            "FG-R1-001",
            ownership.get("gate_ownership", {}),
            {
                "acceptance_task": "T-A0-001",
                "milestone": "A0",
                "evidence_owner": "EO-EXPERIENCE",
            },
        ),
        (
            "FG-R1-002",
            ownership.get("gate_ownership", {}),
            {
                "acceptance_task": "T-B0-P06-REPAIR",
                "milestone": "G1",
                "evidence_owner": "EO-WORKSPACE",
            },
        ),
        (
            "FG-R1-003",
            ownership.get("gate_ownership", {}),
            {
                "acceptance_task": "T-B0-P06",
                "milestone": "G1",
                "evidence_owner": "EO-WORKSPACE",
            },
        ),
        (
            "FG-R2-001",
            ownership.get("gate_ownership", {}),
            {
                "acceptance_task": "T-B0-P04",
                "milestone": "G1",
                "evidence_owner": "EO-EXPERIENCE",
            },
        ),
        (
            "FG-R1-004",
            ownership.get("gate_ownership", {}),
            {
                "acceptance_task": "T-B0-ACCEPT",
                "milestone": "B0",
                "evidence_owner": "EO-EXPERIENCE",
            },
        ),
        (
            "FG-R3-001",
            ownership.get("gate_ownership", {}),
            {
                "acceptance_task": "T-B0-P05",
                "milestone": "G1",
                "evidence_owner": "EO-EVENTS",
            },
        ),
        (
            "FG-B0-001",
            ownership.get("gate_ownership", {}),
            {
                "acceptance_task": "T-B0-P09-OKF",
                "milestone": "G1",
                "evidence_owner": "EO-WORKSPACE",
            },
        ),
        (
            "FG-B0-PILOT-001",
            ownership.get("gate_ownership", {}),
            {
                "acceptance_task": "T-B0-PILOT",
                "milestone": "PILOT",
                "evidence_owner": "EO-EVENTS",
            },
        ),
        (
            "FG-R5-007",
            ownership.get("gate_ownership", {}),
            {
                "acceptance_task": "T-R5-010",
                "milestone": "G5",
                "evidence_owner": "EO-CONNECTIONS",
            },
        ),
    )
    for identifier, records, expected in expected_edges:
        actual = records.get(identifier)
        status_is_dynamic = "status" not in expected
        if status_is_dynamic:
            actual_without_status = (
                {key: value for key, value in actual.items() if key != "status"}
                if isinstance(actual, dict)
                else actual
            )
            mismatch = actual_without_status != expected or set(actual or {}) != {
                *expected,
                "status",
            }
        else:
            mismatch = actual != expected
        if mismatch:
            errors.append(
                f"{identifier}: corrected ownership must be {expected}, "
                f"not {actual}"
            )

    # The complete B0 task/gate graph is an authority contract, not a set of
    # loosely related references. Keep every B0 task and every gate accepted by
    # a B0 task bound to its exact milestone, gate/task, and accountable owner.
    # Status remains lifecycle-driven and is validated separately.
    exact_b0_task_bindings = {
        "T-B0-P00": ("G0", "FG-R0-002", "EO-GOVERNANCE"),
        "T-B0-P01": ("G1", "FG-B0-001", "EO-WORKSPACE"),
        "T-B0-P02": ("G1", "FG-B0-001", "EO-WORKSPACE"),
        "T-B0-P03": ("G1", "FG-B0-001", "EO-WORKSPACE"),
        "T-B0-P03-OBS": ("G1", "FG-B0-DESIGN-001", "EO-EXPERIENCE"),
        "T-B0-P03D": ("G1", "FG-B0-DESIGN-001", "EO-EXPERIENCE"),
        "T-B0-P04": ("G1", "FG-R2-001", "EO-EXPERIENCE"),
        "T-B0-P05": ("G1", "FG-R3-001", "EO-EVENTS"),
        "T-B0-P05-OKF": ("G1", "FG-B0-001", "EO-WORKSPACE"),
        "T-B0-P06": ("G1", "FG-R1-003", "EO-WORKSPACE"),
        "T-B0-P06-REPAIR": ("G1", "FG-R1-002", "EO-WORKSPACE"),
        "T-B0-P07": ("G1", "FG-B0-002", "EO-SECURITY"),
        "T-B0-P08": ("G1", "FG-B0-002", "EO-SECURITY"),
        "T-B0-P09-OKF": ("G1", "FG-B0-001", "EO-WORKSPACE"),
        "T-B0-P09": ("G3", "FG-R3-002", "EO-EVENTS"),
        "T-B0-P10": ("G3", "FG-R3-003", "EO-EVENTS"),
        "T-B0-P11": ("B0", "FG-B0-003", "EO-EXPERIENCE"),
        "T-B0-PILOT": ("PILOT", "FG-B0-PILOT-001", "EO-EVENTS"),
        "T-B0-ACCEPT": ("B0", "FG-B0-003", "EO-EXPERIENCE"),
    }
    task_ownership = ownership.get("task_ownership", {})
    current_b0_task_ids = {
        identifier
        for identifier in task_ownership
        if isinstance(identifier, str) and identifier.startswith("T-B0-")
    }
    if set(exact_b0_task_bindings) != current_b0_task_ids:
        errors.append(
            "exact B0 task binding coverage must equal every T-B0 ownership id; "
            f"missing={sorted(current_b0_task_ids - set(exact_b0_task_bindings))}, "
            f"stale={sorted(set(exact_b0_task_bindings) - current_b0_task_ids)}"
        )
    for identifier, expected_values in exact_b0_task_bindings.items():
        actual = task_ownership.get(identifier, {})
        actual_values = tuple(
            actual.get(field)
            for field in ("milestone", "owning_gate", "evidence_owner")
        )
        if actual_values != expected_values:
            errors.append(
                f"{identifier}: exact B0 task binding must be "
                f"milestone={expected_values[0]}, owning_gate={expected_values[1]}, "
                f"evidence_owner={expected_values[2]}, not {actual_values}"
            )

    exact_b0_gate_bindings = {
        "FG-R0-002": ("T-B0-P00", "G0", "EO-GOVERNANCE"),
        "FG-R1-002": ("T-B0-P06-REPAIR", "G1", "EO-WORKSPACE"),
        "FG-R1-003": ("T-B0-P06", "G1", "EO-WORKSPACE"),
        "FG-R1-004": ("T-B0-ACCEPT", "B0", "EO-EXPERIENCE"),
        "FG-R1-005": ("T-B0-P08", "G1", "EO-SECURITY"),
        "FG-R2-001": ("T-B0-P04", "G1", "EO-EXPERIENCE"),
        "FG-R3-001": ("T-B0-P05", "G1", "EO-EVENTS"),
        "FG-R3-002": ("T-B0-P09", "G3", "EO-EVENTS"),
        "FG-R3-003": ("T-B0-P10", "G3", "EO-EVENTS"),
        "FG-R3-004": ("T-B0-P10", "G3", "EO-EVENTS"),
        "FG-R3-005": ("T-B0-ACCEPT", "B0", "EO-EXPERIENCE"),
        "FG-B0-001": ("T-B0-P09-OKF", "G1", "EO-WORKSPACE"),
        "FG-B0-DESIGN-001": ("T-B0-P03D", "G1", "EO-EXPERIENCE"),
        "FG-B0-002": ("T-B0-P08", "G1", "EO-SECURITY"),
        "FG-B0-003": ("T-B0-ACCEPT", "B0", "EO-EXPERIENCE"),
        "FG-B0-PILOT-001": ("T-B0-PILOT", "PILOT", "EO-EVENTS"),
    }
    gate_ownership = ownership.get("gate_ownership", {})
    gates_accepted_by_b0_tasks = {
        identifier
        for identifier, edge in gate_ownership.items()
        if isinstance(edge, dict)
        and edge.get("acceptance_task") in current_b0_task_ids
    }
    if set(exact_b0_gate_bindings) != gates_accepted_by_b0_tasks:
        errors.append(
            "exact B0 gate binding coverage must equal every gate accepted by a "
            "T-B0 task; "
            f"missing={sorted(gates_accepted_by_b0_tasks - set(exact_b0_gate_bindings))}, "
            f"stale={sorted(set(exact_b0_gate_bindings) - gates_accepted_by_b0_tasks)}"
        )
    for identifier, expected_values in exact_b0_gate_bindings.items():
        actual = gate_ownership.get(identifier, {})
        actual_values = tuple(
            actual.get(field)
            for field in ("acceptance_task", "milestone", "evidence_owner")
        )
        if actual_values != expected_values:
            errors.append(
                f"{identifier}: exact B0 gate binding must be "
                f"acceptance_task={expected_values[0]}, milestone={expected_values[1]}, "
                f"evidence_owner={expected_values[2]}, not {actual_values}"
            )

    if requirement_by_id.get("LRM-WS-013", {}).get("release") != "A0":
        errors.append("LRM-WS-013: repository release must be A0")
    if requirement_by_id.get("LRM-WS-014", {}).get("release") != "A0":
        errors.append("LRM-WS-014: repository release must be A0")
    if case_by_id.get("UAT-050", {}).get("release") != "A0":
        errors.append("UAT-050: repository release must be A0")
    if requirement_by_id.get("LRM-WS-007", {}).get("release") != "R5":
        errors.append("LRM-WS-007: generic migration safety release must be R5")
    if requirement_by_id.get("LRM-CO-012", {}).get("release") != "R5":
        errors.append("LRM-CO-012: Google Drive adapter release must remain R5")
    if requirement_by_id.get("LRM-PK-010", {}).get("release") != "B0":
        errors.append("LRM-PK-010: D1-B observation release must be B0")
    if requirement_by_id.get("LRM-UX-016", {}).get("release") != "B0":
        errors.append("LRM-UX-016: full installed theme journey release must be B0")
    if case_by_id.get("UAT-022", {}).get("release") != "A0":
        errors.append("UAT-022: relationship-reminder workflow release must be A0")
    if case_by_id.get("UAT-062", {}).get("release") != "B0":
        errors.append("UAT-062: full installed theme journey release must be B0")
    if case_by_id.get("UAT-073", {}).get("release") != "B0":
        errors.append("UAT-073: P04 installed current-surface release must be B0")
    if case_by_id.get("UAT-074", {}).get("release") != "R5":
        errors.append("UAT-074: Google Drive backup workflow release must be R5")

    expected_resume_provenance = {
        "decision": "D1-B",
        "source_pull_request": 65,
        "baseline_commit": "3499a6e9278fc72d2498a9978df59f30d03722e6",
        "observation_task": "T-B0-P03-OBS",
        "merge_result_ci": {
            "event": "push",
            "conclusion": "success",
            "workflow_run_ids": [
                29899084738,
                29899084740,
                29899084741,
                29899084751,
                29899084753,
                29899084769,
                29899084789,
            ],
        },
        "notarized_bundle_preflight": {
            "event": "workflow_dispatch",
            "run_id": 29899498005,
            "conclusion": "failure",
            "failure_boundary": "missing-Apple-credentials",
            "release_evidence": "not-established",
        },
    }
    resume_authority = ownership.get("p03_resume_authority", {})
    expected_resume_keys = {
        "decision",
        "source_pull_request",
        "baseline_commit",
        "observation_task",
        "p03_technical_acceptance",
        "qualification_identity_contract",
        "qualified_artifact_identity",
        "observation_decision",
        "observation_receipt_contract",
        "observation_receipt",
        "replacement_task",
        "stopped_disposition",
        "p03d_eligibility",
        "merge_result_ci",
        "notarized_bundle_preflight",
    }
    if set(resume_authority) != expected_resume_keys:
        errors.append("traceability: p03_resume_authority top-level schema drifted")
    actual_resume_provenance = {
        key: resume_authority.get(key) for key in expected_resume_provenance
    }
    if actual_resume_provenance != expected_resume_provenance:
        errors.append(
            "traceability: P03 resume authority must record D1-B, PR #65 baseline, "
            "the exact ordinary-push-green receipt, the separate failed notarized "
            "credential preflight, and the immutable resume provenance boundary"
        )
    technical_acceptance = resume_authority.get("p03_technical_acceptance")
    observation_decision = resume_authority.get("observation_decision")
    p03d_eligibility = resume_authority.get("p03d_eligibility")
    replacement_task = resume_authority.get("replacement_task")
    stopped_disposition = resume_authority.get("stopped_disposition")
    if technical_acceptance not in {"not-accepted", "accepted"}:
        errors.append(
            "traceability: p03_technical_acceptance must be not-accepted or accepted"
        )
    if observation_decision not in {"pending", "Continue", "Change", "Stop"}:
        errors.append(
            "traceability: observation_decision must be pending, Continue, Change, or Stop"
        )
    if p03d_eligibility not in {"blocked", "eligible", "satisfied"}:
        errors.append(
            "traceability: p03d_eligibility must be blocked, eligible, or satisfied"
        )
    task_status = {
        identifier: record.get("status")
        for identifier, record in ownership.get("task_ownership", {}).items()
    }
    requirement_status = {
        identifier: record.get("status")
        for identifier, record in ownership.get("requirement_ownership", {}).items()
    }
    uat_status = {
        identifier: record.get("status")
        for identifier, record in ownership.get("uat_ownership", {}).items()
    }
    gate_status = {
        identifier: record.get("status")
        for identifier, record in ownership.get("gate_ownership", {}).items()
    }
    identity = resume_authority.get("qualified_artifact_identity")
    observation_receipt = resume_authority.get("observation_receipt")
    identity_contract = resume_authority.get("qualification_identity_contract", [])
    expected_identity_contract = [
        "qualified-code-sha",
        "merge-result-sha",
        "attestation-sha",
        "exact-executable-artifact-receipt",
    ]
    expected_identity_fields = set(expected_identity_contract)
    receipt_contract = resume_authority.get("observation_receipt_contract", [])
    expected_receipt_contract = [
        "record_sha256",
        "decision_sha256",
        "observed_qualification_identity",
        "decision",
    ]
    expected_receipt_fields = set(expected_receipt_contract)
    if identity_contract != expected_identity_contract:
        errors.append("traceability: D1-B exact artifact identity contract drifted")
    if receipt_contract != expected_receipt_contract:
        errors.append("traceability: D1-B observation receipt contract drifted")
    if technical_acceptance == "not-accepted":
        if identity is not None:
            errors.append(
                "traceability: unaccepted P03 must not claim a qualified artifact identity"
            )
        if task_status.get("T-B0-P03") != "current":
            errors.append("T-B0-P03: unaccepted technical state must remain current")
        if task_status.get("T-B0-P03-OBS") != "blocked":
            errors.append(
                "T-B0-P03-OBS: cannot become current before P03 technical acceptance"
            )
        for identifier in ("LRM-WS-004", "LRM-WS-005", "LRM-WS-010"):
            if requirement_status.get(identifier) != "current":
                errors.append(f"{identifier}: unaccepted P03 requirement must be current")
        if uat_status.get("UAT-042") != "current":
            errors.append("UAT-042: unaccepted P03 UAT must be current")
        if requirement_status.get("LRM-PK-010") != "blocked":
            errors.append("LRM-PK-010: observation requirement remains blocked")
    else:
        if not isinstance(identity, dict) or set(identity) != expected_identity_fields:
            errors.append(
                "traceability: accepted P03 requires the complete exact qualification "
                "and executable-artifact identity tuple"
            )
        else:
            git_identity_fields = (
                "qualified-code-sha",
                "merge-result-sha",
                "attestation-sha",
            )
            for field in git_identity_fields:
                if not is_lower_hex(identity[field], 40):
                    errors.append(
                        f"traceability: accepted P03 {field} must be an exact lowercase Git SHA string"
                    )
            if len({identity[field] for field in git_identity_fields}) != 3:
                errors.append(
                    "traceability: qualified-code, merge-result, and attestation "
                    "Git SHAs must be pairwise distinct evidence identities"
                )
            artifact_receipt = identity["exact-executable-artifact-receipt"]
            if not isinstance(artifact_receipt, dict) or set(artifact_receipt) != {
                "artifact_sha256",
                "receipt_sha256",
            }:
                errors.append(
                    "traceability: exact executable artifact receipt must bind "
                    "artifact_sha256 and receipt_sha256"
                )
            else:
                for field in ("artifact_sha256", "receipt_sha256"):
                    if not is_lower_hex(artifact_receipt[field], 64):
                        errors.append(
                            f"traceability: exact executable {field} must be a lowercase SHA-256 string"
                        )
                if artifact_receipt["artifact_sha256"] == artifact_receipt["receipt_sha256"]:
                    errors.append(
                        "traceability: executable artifact and receipt SHA-256 "
                        "identities must be distinct"
                    )
        if task_status.get("T-B0-P03") != "complete":
            errors.append("T-B0-P03: accepted technical state must be complete")
        for identifier in ("LRM-WS-004", "LRM-WS-005", "LRM-WS-010"):
            if requirement_status.get(identifier) != "complete":
                errors.append(f"{identifier}: accepted P03 requirement must be complete")
        if uat_status.get("UAT-042") != "complete":
            errors.append("UAT-042: accepted P03 UAT must be complete")
    if (
        task_status.get("T-B0-P09-OKF") != "complete"
        and gate_status.get("FG-B0-001") != "blocked"
    ):
        errors.append(
            "FG-B0-001: must remain blocked until its acceptance task "
            "T-B0-P09-OKF completes; P03 technical acceptance and D1-B "
            "observation cannot close the broader gate"
        )
    valid_receipt = isinstance(observation_receipt, dict)
    if observation_decision != "pending":
        if not valid_receipt or set(observation_receipt) != expected_receipt_fields:
            errors.append(
                "traceability: completed observation requires the exact receipt fields"
            )
            valid_receipt = False
        if valid_receipt:
            for field in ("record_sha256", "decision_sha256"):
                if not is_lower_hex(observation_receipt[field], 64):
                    errors.append(
                        f"traceability: observation {field} must be a lowercase SHA-256 string"
                    )
            if observation_receipt["record_sha256"] == observation_receipt["decision_sha256"]:
                errors.append(
                    "traceability: observation record and decision SHA-256 "
                    "identities must be distinct"
                )
            artifact_receipt = (
                identity.get("exact-executable-artifact-receipt")
                if isinstance(identity, dict)
                else None
            )
            if isinstance(artifact_receipt, dict) and set(artifact_receipt) == {
                "artifact_sha256",
                "receipt_sha256",
            }:
                sha256_evidence = (
                    artifact_receipt["artifact_sha256"],
                    artifact_receipt["receipt_sha256"],
                    observation_receipt["record_sha256"],
                    observation_receipt["decision_sha256"],
                )
                if len(set(sha256_evidence)) != len(sha256_evidence):
                    errors.append(
                        "traceability: artifact, qualification receipt, observation "
                        "record, and observation decision SHA-256 identities must be "
                        "pairwise distinct"
                    )
            if observation_receipt["observed_qualification_identity"] != identity:
                errors.append(
                    "traceability: observed qualification tuple must byte-equal "
                    "the technically accepted artifact identity"
                )
            if observation_receipt["decision"] != observation_decision:
                errors.append(
                    "traceability: observation receipt decision must equal "
                    "observation_decision"
                )
    if observation_decision == "pending":
        if observation_receipt is not None:
            errors.append("traceability: pending D1-B observation has no receipt")
        if replacement_task is not None or stopped_disposition is not None:
            errors.append(
                "traceability: pending observation has no replacement or stop disposition"
            )
        if p03d_eligibility != "blocked":
            errors.append("traceability: pending observation keeps P03D blocked")
        if task_status.get("T-B0-P03D") != "blocked":
            errors.append("T-B0-P03D: pending observation keeps the task blocked")
        if task_status.get("T-B0-P04") != "blocked":
            errors.append("T-B0-P04: pending observation keeps P04 blocked")
        if technical_acceptance == "accepted" and task_status.get(
            "T-B0-P03-OBS"
        ) != "current":
            errors.append("T-B0-P03-OBS: becomes current after technical acceptance")
        expected_pk010 = "current" if technical_acceptance == "accepted" else "blocked"
        if requirement_status.get("LRM-PK-010") != expected_pk010:
            errors.append(
                f"LRM-PK-010: pending observation requires {expected_pk010} status"
            )
        if gate_status.get("FG-B0-DESIGN-001") != "blocked":
            errors.append("FG-B0-DESIGN-001: pending observation keeps gate blocked")
    elif observation_decision == "Continue":
        if technical_acceptance != "accepted":
            errors.append("traceability: Continue cannot substitute for P03 acceptance")
        if replacement_task is not None or stopped_disposition is not None:
            errors.append("traceability: Continue has no replacement or stop disposition")
        if task_status.get("T-B0-P03-OBS") != "complete":
            errors.append("T-B0-P03-OBS: Continue requires completed observation")
        if requirement_status.get("LRM-PK-010") != "complete":
            errors.append("LRM-PK-010: Continue completes the observation requirement")
        if p03d_eligibility not in {"eligible", "satisfied"}:
            errors.append("traceability: Continue must make P03D eligible")
        expected_p03d_status = "complete" if p03d_eligibility == "satisfied" else "current"
        if task_status.get("T-B0-P03D") != expected_p03d_status:
            errors.append(
                f"T-B0-P03D: {p03d_eligibility} requires {expected_p03d_status} status"
            )
        if gate_status.get("FG-B0-DESIGN-001") != expected_p03d_status:
            errors.append(
                "FG-B0-DESIGN-001: status must track eligible/current or "
                "satisfied/complete P03D"
            )
        if (
            task_status.get("T-B0-P04") == "current"
            and task_status.get("T-B0-P03D") != "complete"
        ):
            errors.append("T-B0-P04: cannot become current before P03D completes")
    elif observation_decision == "Change":
        if technical_acceptance != "accepted":
            errors.append("traceability: Change requires prior P03 acceptance")
        if task_status.get("T-B0-P03-OBS") != "complete":
            errors.append("T-B0-P03-OBS: Change completes the observation record")
        if requirement_status.get("LRM-PK-010") != "complete":
            errors.append("LRM-PK-010: Change completes the observation requirement")
        if p03d_eligibility != "blocked":
            errors.append("traceability: Change must keep P03D ineligible")
        if task_status.get("T-B0-P03D") != "blocked" or task_status.get(
            "T-B0-P04"
        ) != "blocked":
            errors.append("traceability: Change keeps P03D and P04 blocked")
        if gate_status.get("FG-B0-DESIGN-001") != "blocked":
            errors.append("FG-B0-DESIGN-001: Change keeps design gate blocked")
        if (
            not isinstance(replacement_task, str)
            or replacement_task not in task_status
            or replacement_task in {"T-B0-P03-OBS", "T-B0-P03D", "T-B0-P04"}
            or task_status.get(replacement_task) != "current"
        ):
            errors.append("traceability: Change requires exactly one known current replacement_task")
        elif (
            "T-B0-P03-OBS" not in task_dependencies.get(replacement_task, set())
            or "p03-change-replacement" not in task_outputs.get(replacement_task, set())
            or not (
                task_requirements.get(replacement_task, set())
                or task_uat.get(replacement_task, set())
            )
        ):
            errors.append(
                "traceability: Change replacement_task must depend on OBS, output "
                "p03-change-replacement, and own an explicit requirement or UAT"
            )
        if stopped_disposition is not None:
            errors.append("traceability: Change must not claim a stopped disposition")
    elif observation_decision == "Stop":
        if technical_acceptance != "accepted":
            errors.append("traceability: Stop requires prior P03 acceptance")
        if task_status.get("T-B0-P03-OBS") != "complete":
            errors.append("T-B0-P03-OBS: Stop completes the observation record")
        if requirement_status.get("LRM-PK-010") != "complete":
            errors.append("LRM-PK-010: Stop completes the observation requirement")
        if p03d_eligibility != "blocked":
            errors.append("traceability: Stop must keep P03D ineligible")
        if task_status.get("T-B0-P03D") != "blocked" or task_status.get(
            "T-B0-P04"
        ) != "blocked":
            errors.append("traceability: Stop keeps P03D and P04 blocked")
        if gate_status.get("FG-B0-DESIGN-001") != "blocked":
            errors.append("FG-B0-DESIGN-001: Stop keeps design gate blocked")
        if replacement_task is not None:
            errors.append("traceability: Stop must not claim a replacement task")
        if not isinstance(stopped_disposition, dict) or set(stopped_disposition) != {
            "project_state",
            "preservation_receipt_sha256",
            "support_owner",
        }:
            errors.append(
                "traceability: Stop requires a structured preservation/support disposition"
            )
        else:
            if stopped_disposition["project_state"] != "stopped":
                errors.append("traceability: Stop project_state must be stopped")
            if not is_lower_hex(
                stopped_disposition["preservation_receipt_sha256"], 64
            ):
                errors.append(
                    "traceability: Stop preservation receipt must be a lowercase SHA-256 string"
                )
            support_owner = stopped_disposition["support_owner"]
            if support_owner is not None and (
                type(support_owner) is not str
                or support_owner not in task_status
                or task_status.get(support_owner) != "current"
                or "stopped-support" not in task_blocks.get(support_owner, "").lower()
            ):
                errors.append(
                    "traceability: Stop support_owner must be null for a terminal "
                    "stopped project or a dedicated current stopped-support task"
                )
    require_fragments(
        "LRM-WS-007",
        " ".join(
            str(requirement_by_id.get("LRM-WS-007", {}).get(field, ""))
            for field in ("statement", "acceptance")
        ),
        (
            "General and third-party post-A0 migrations",
            "LRM-WS-017",
            "UAT-066",
            "does not inherit this requirement",
            "where rollback is claimed",
            "explicit approved irreversibility boundary",
            "exclusively LRM-WS-017 and UAT-066",
        ),
        errors,
    )
    require_fragments(
        "LRM-UX-012",
        " ".join(
            str(requirement_by_id.get("LRM-UX-012", {}).get(field, ""))
            for field in ("statement", "acceptance")
        ),
        (
            "P04",
            "Workspace, People, and Health",
            "theme fixtures",
            "current-surface installed evidence",
            "without adding Events or A0 settings transfer",
        ),
        errors,
    )
    require_fragments(
        "LRM-UX-016",
        " ".join(
            str(requirement_by_id.get("LRM-UX-016", {}).get(field, ""))
            for field in ("statement", "acceptance")
        ),
        (
            "P11",
            "Overview, Directory, Events, Health, Settings",
            "Details, Cohort, Attendees, Readiness, and Brief",
            "installed B0 artifact",
            "system, light, dark, and high_contrast",
        ),
        errors,
    )
    require_fragments(
        "UAT-073",
        " ".join(
            str(case_by_id.get("UAT-073", {}).get(field, ""))
            for field in ("given", "when", "then")
        ),
        (
            "installed P04 macOS Tauri and WKWebView artifact",
            "en-XA at 400 percent reflow",
            "VoiceOver",
            "external child-process controller",
            "creates a workspace",
            "closes and reopens or resumes it",
            "opens the separate existing-workspace fixture",
            "writer-open failure through read-only Health",
            "lists and creates basic People",
            "before COMMIT",
            "after COMMIT",
            "system, light, dark, and high_contrast",
            "operating-system preference",
            "built-in choice and OS-following persist",
            "no external request",
            "no fault-injection seam",
        ),
        errors,
    )
    require_fragments(
        "LRM-PK-010",
        " ".join(
            str(requirement_by_id.get("LRM-PK-010", {}).get(field, ""))
            for field in ("statement", "acceptance")
        ),
        (
            "qualified-code SHA",
            "merge-result SHA",
            "attestation SHA",
            "exact executable artifact receipt",
            "without closing FG-B0-001",
            "synthetic or redacted workplace scenarios",
            "Continue, Change, or Stop",
            "real workplace personal data",
            "only a Continue decision tied to that exact attested artifact makes P03D eligible",
        ),
        errors,
    )
    require_fragments(
        "FG-R5-005",
        gate_blocks.get("FG-R5-005", ""),
        (
            "dry-run",
            "pre-migration backup",
            "deterministic repeat",
            "post-migration validation",
            "rollback or an explicit irreversibility statement",
            "exact pre-migration workspace",
        ),
        errors,
    )
    for relative in (
        "PROJECT_CONTEXT.md",
        "docs/product/working-state-delivery.md",
        "AI_BUILD_INSTRUCTIONS.md",
        "CHANGELOG.md",
        "docs/decisions/0015-observe-the-exact-p03-artifact-before-design-authority.md",
    ):
        try:
            source = (ROOT / relative).read_text(encoding="utf-8")
        except OSError as exc:
            errors.append(f"{relative}: cannot verify D1-B resume boundary: {exc}")
            continue
        require_fragments(
            relative,
            source,
            (
                "3499a6e9278fc72d2498a9978df59f30d03722e6",
                "29899084738",
                "29899498005",
                "release evidence",
            ),
            errors,
        )
    for relative in (
        "AGENTS.md",
        "PROJECT_CONTEXT.md",
        "docs/product/working-state-delivery.md",
        "AI_BUILD_INSTRUCTIONS.md",
        "SPEC.md",
        "README.md",
        "docs/product/roadmap.md",
        "docs/knowledge/KCS-0014-when-may-the-events-destination-be-enabled.md",
        "docs/evidence/ux/b0-events-design-contract-candidate.md",
        "docs/evidence/design/semantic-token-contrast-evidence.md",
        "docs/product/rice-prioritization.md",
        "docs/decisions/0015-observe-the-exact-p03-artifact-before-design-authority.md",
    ):
        try:
            source = (ROOT / relative).read_text(encoding="utf-8")
        except OSError as exc:
            errors.append(f"{relative}: cannot verify D1-B sequence: {exc}")
            continue
        require_fragments(
            relative,
            source,
            ("D1-B", "Continue", "P03D", "P04"),
            errors,
        )
    for relative in (
        "AGENTS.md",
        "PROJECT_CONTEXT.md",
        "docs/product/working-state-delivery.md",
        "docs/decisions/0016-scope-general-migration-safety-to-r5.md",
        "contexts/connections/README.md",
    ):
        try:
            source = (ROOT / relative).read_text(encoding="utf-8")
        except OSError as exc:
            errors.append(f"{relative}: cannot verify migration boundary: {exc}")
            continue
        require_fragments(
            relative,
            source,
            (
                "general and third-party",
                "LRM-WS-007",
                "LRM-WS-017",
                "UAT-066",
                "B0",
            ),
            errors,
        )
    for relative in (
        "SPEC.md",
        "README.md",
        "PROJECT_CONTEXT.md",
        "AI_BUILD_INSTRUCTIONS.md",
        "docs/product/roadmap.md",
    ):
        try:
            source = (ROOT / relative).read_text(encoding="utf-8")
        except OSError as exc:
            errors.append(f"{relative}: cannot verify P04/P11 theme ownership: {exc}")
            continue
        require_fragments(
            relative,
            source,
            ("P04", "Workspace", "People", "Health", "P11", "Details-to-Brief"),
            errors,
        )
    try:
        decisions_index = (ROOT / "docs/decisions/README.md").read_text(
            encoding="utf-8"
        )
    except OSError as exc:
        errors.append(f"docs/decisions/README.md: cannot verify ADR index: {exc}")
    else:
        require_fragments(
            "docs/decisions/README.md",
            decisions_index,
            ("0014", "unmerged proposal", "0015", "0016"),
            errors,
        )
    pr31 = next(
        (
            item
            for item in ownership.get("review_branch_dispositions", [])
            if item.get("reference") == "PR #31"
        ),
        {},
    )
    require_fragments(
        "PR #31 disposition",
        str(pr31.get("condition", "")),
        ("accepted P03 technical qualification", "T-B0-P03-OBS", "Continue", "P03D"),
        errors,
    )

    execution_status = {
        "G0": (ownership.get("milestones", []), "complete"),
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
            "P05/P05-OKF",
            "P06/P06-REPAIR",
            "`T-B0-P06-REPAIR`",
            "P09-OKF",
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
            "P05-OKF's pinned OKF v0.1 Draft strict writer/schema port as an `FG-B0-001` slice",
            "P06's tolerant Directory reader and domain-validity quarantine, which is accepted by `FG-R1-003`",
            "P09-OKF is the later acceptance task that closes `FG-B0-001`",
        ),
        "PROJECT_CONTEXT.md": (
            "G0, P00, P01, and P02 are complete",
            "G1 and P03 are current",
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
        build_guidance = (ROOT / "AI_BUILD_INSTRUCTIONS.md").read_text(
            encoding="utf-8"
        )
    except OSError as exc:
        errors.append(f"AI_BUILD_INSTRUCTIONS.md: cannot verify gate ownership: {exc}")
    else:
        validate_build_guidance_gate_ownership(build_guidance, errors)

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


@repository_custody_validation
def validate_traceability(
    requirements: list[dict],
    cases: list[dict],
    requirement_ids: set[str],
    case_ids: set[str],
    persona_ids: set[str],
    task_blocks: dict[str, str],
    gate_blocks: dict[str, str],
    errors: list[str],
    ownership_document: object | None = None,
    validate_projection: bool = True,
    validate_generated_outputs: bool = True,
    validate_repository_truth: bool = True,
) -> None:
    if ownership_document is None:
        try:
            ownership = load_json("spec/traceability-ownership.json")
        except (OSError, ValueError, json.JSONDecodeError) as exc:
            errors.append(f"traceability ownership: {exc}")
            return
    else:
        ownership = ownership_document

    ownership_schema_errors = traceability_ownership_schema_issues(ownership)
    errors.extend(ownership_schema_errors)
    if ownership_schema_errors:
        return

    if validate_repository_truth:
        validate_current_main_truth_sources(errors)

    errors.extend(restricted_yaml_block_issues(task_blocks, "tasks"))
    errors.extend(restricted_yaml_block_issues(gate_blocks, "gates"))
    try:
        gate_required_evidence = gate_evidence_required_map(gate_blocks)
    except ValueError as exc:
        errors.append(f"traceability: invalid gate evidence_required map: {exc}")
        gate_required_evidence = {identifier: frozenset() for identifier in gate_blocks}

    task_ids = set(task_blocks)
    gate_ids = set(gate_blocks)
    task_ownership = ownership.get("task_ownership", {})
    gate_ownership = ownership.get("gate_ownership", {})
    requirement_ownership = ownership.get("requirement_ownership", {})
    uat_ownership = ownership.get("uat_ownership", {})
    milestones = ownership.get("milestones", [])
    evidence_owners = ownership.get("evidence_owners", [])

    if validate_projection:
        validate_traceability_report_projection(
            requirements,
            cases,
            persona_ids,
            task_blocks,
            gate_blocks,
            ownership,
            errors,
        )

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

    # A repository release label may differ from execution ownership only when
    # that exact requirement or UAT case records its task and gate releases,
    # rationale, and authority. Release-pair wildcards would silently bless
    # unrelated future drift, so the exception registry is identifier-exact.
    def release_by_block(blocks: dict[str, str]) -> dict[str, str]:
        releases: dict[str, str] = {}
        for identifier, block in blocks.items():
            match = re.search(
                r"^\s{4}release:\s*([^\s#]+)", block, flags=re.MULTILINE
            )
            releases[identifier] = match.group(1) if match else ""
        return releases

    task_releases = release_by_block(task_blocks)
    gate_releases = release_by_block(gate_blocks)
    expected_cross_release: dict[str, dict[str, str]] = {}
    for artifact_type, artifacts, records in (
        ("requirement", requirements, requirement_ownership),
        ("uat", cases, uat_ownership),
    ):
        for artifact in artifacts:
            identifier = artifact.get("id")
            edge = records.get(identifier, {})
            task = edge.get("owning_task")
            gate = edge.get("owning_gate")
            repository_release = artifact.get("release")
            task_release = task_releases.get(task, "")
            gate_release = gate_releases.get(gate, "")
            if repository_release == task_release == gate_release:
                continue
            expected_cross_release[str(identifier)] = {
                "artifact_type": artifact_type,
                "repository_release": str(repository_release),
                "owning_task": str(task),
                "task_release": task_release,
                "owning_gate": str(gate),
                "gate_release": gate_release,
            }

    cross_release = ownership.get("cross_release_ownership_dispositions", {})
    if not isinstance(cross_release, dict):
        errors.append(
            "traceability: cross_release_ownership_dispositions must be an object"
        )
        cross_release = {}
    for identifier in sorted(set(expected_cross_release) - set(cross_release)):
        errors.append(
            f"{identifier}: cross-release ownership lacks an exact disposition"
        )
    for identifier in sorted(set(cross_release) - set(expected_cross_release)):
        errors.append(
            f"{identifier}: stale cross-release disposition without a release mismatch"
        )
    required_disposition_fields = {
        "artifact_type",
        "repository_release",
        "owning_task",
        "task_release",
        "owning_gate",
        "gate_release",
        "authority",
        "rationale",
    }
    for identifier, expected in expected_cross_release.items():
        disposition = cross_release.get(identifier)
        if not isinstance(disposition, dict):
            continue
        if set(disposition) != required_disposition_fields:
            errors.append(
                f"{identifier}: cross-release disposition fields must be exactly "
                f"{sorted(required_disposition_fields)}"
            )
        for field, value in expected.items():
            if disposition.get(field) != value:
                errors.append(
                    f"{identifier}: cross-release {field} must be {value!r}, "
                    f"not {disposition.get(field)!r}"
                )
        rationale = disposition.get("rationale")
        if not isinstance(rationale, str) or len(rationale.strip()) < 40:
            errors.append(
                f"{identifier}: cross-release disposition needs a specific rationale"
            )
        authority = disposition.get("authority")
        if not isinstance(authority, str) or not authority.strip():
            errors.append(
                f"{identifier}: cross-release disposition needs an authority path"
            )
            continue
        authority_path = repository_regular_file_path(authority)
        if authority_path is None:
            errors.append(
                f"{identifier}: cross-release authority must be a visible, "
                "non-ignored regular repository-owned file without symlink components"
            )

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

    # Evidence owned outside the consuming task's explicit dependency ancestry
    # is non-blocking design, regression, or acceptance input only when that
    # exact task/artifact pair is recorded. This covers later milestones and
    # same-milestone later owners alike, preventing an evidence list from
    # silently creating an undeclared execution prerequisite.
    expected_forward_evidence: dict[str, dict[str, str]] = {}
    for task, identifiers in task_evidence_dependencies.items():
        task_edge = task_ownership.get(task, {})
        if task_edge.get("status") == "superseded":
            continue
        task_milestone = str(task_edge.get("milestone", ""))
        task_ancestry = transitive_dependencies(task_dependencies, task) | {task}
        for identifier in identifiers:
            if identifier in requirement_ids:
                artifact_type = "requirement"
                artifact_edge = requirement_ownership.get(identifier, {})
            elif identifier in case_ids:
                artifact_type = "uat"
                artifact_edge = uat_ownership.get(identifier, {})
            else:
                continue
            artifact_milestone = str(artifact_edge.get("milestone", ""))
            artifact_owner_task = str(artifact_edge.get("owning_task", ""))
            if artifact_owner_task in task_ancestry:
                continue
            key = f"{task}::{identifier}"
            expected_forward_evidence[key] = {
                "task": task,
                "task_milestone": task_milestone,
                "artifact_type": artifact_type,
                "artifact_id": identifier,
                "artifact_owner_task": artifact_owner_task,
                "artifact_milestone": artifact_milestone,
                "relationship": "non-blocking-future-contract-input",
            }

    forward_evidence = ownership.get("forward_evidence_dispositions", {})
    if not isinstance(forward_evidence, dict):
        errors.append("traceability: forward_evidence_dispositions must be an object")
        forward_evidence = {}
    for key in sorted(set(expected_forward_evidence) - set(forward_evidence)):
        errors.append(f"{key}: forward evidence edge lacks an exact disposition")
    for key in sorted(set(forward_evidence) - set(expected_forward_evidence)):
        errors.append(f"{key}: stale forward evidence disposition without a live edge")
    required_forward_fields = {
        "task",
        "task_milestone",
        "artifact_type",
        "artifact_id",
        "artifact_owner_task",
        "artifact_milestone",
        "relationship",
        "authority",
        "rationale",
    }
    for key, expected in expected_forward_evidence.items():
        disposition = forward_evidence.get(key)
        if not isinstance(disposition, dict):
            continue
        if set(disposition) != required_forward_fields:
            errors.append(
                f"{key}: forward evidence disposition fields must be exactly "
                f"{sorted(required_forward_fields)}"
            )
        for field, value in expected.items():
            if disposition.get(field) != value:
                errors.append(
                    f"{key}: forward evidence {field} must be {value!r}, "
                    f"not {disposition.get(field)!r}"
                )
        rationale = disposition.get("rationale")
        if not isinstance(rationale, str) or len(rationale.strip()) < 40:
            errors.append(f"{key}: forward evidence disposition needs a specific rationale")
        authority = disposition.get("authority")
        if not isinstance(authority, str) or not authority.strip():
            errors.append(f"{key}: forward evidence disposition needs an authority path")
            continue
        authority_path = repository_regular_file_path(authority)
        if authority_path is None:
            errors.append(
                f"{key}: forward evidence authority must be a visible, non-ignored "
                "regular repository-owned file without symlink components"
            )

    # Every authority path is bound to an exact group of registry anchors and
    # to the content digest of the repository source that carries them. This
    # prevents a plausible-length rationale plus an unrelated existing file
    # from authorising release or milestone drift.
    expected_authority_anchors: dict[str, set[str]] = {}
    for identifier, disposition in cross_release.items():
        if isinstance(disposition, dict) and isinstance(
            disposition.get("authority"), str
        ):
            expected_authority_anchors.setdefault(
                disposition["authority"], set()
            ).add(f"cross-release::{identifier}")
    for key, disposition in forward_evidence.items():
        if isinstance(disposition, dict) and isinstance(
            disposition.get("authority"), str
        ):
            expected_authority_anchors.setdefault(
                disposition["authority"], set()
            ).add(f"forward-evidence::{key}")

    authority_sources = ownership.get("authority_sources", {})
    if not isinstance(authority_sources, dict):
        errors.append("traceability: authority_sources must be an object")
        authority_sources = {}
    for path in sorted(set(expected_authority_anchors) - set(authority_sources)):
        errors.append(f"{path}: cited authority lacks a bound source record")
    for path in sorted(set(authority_sources) - set(expected_authority_anchors)):
        errors.append(f"{path}: stale authority source without a live citation")
    required_authority_source_fields = {"kind", "sha256", "anchors"}
    normative_authorities = {"SPEC.md", "docs/product/working-state-delivery.md"}
    for path, expected_anchors in expected_authority_anchors.items():
        source = authority_sources.get(path)
        if not isinstance(source, dict):
            continue
        if set(source) != required_authority_source_fields:
            errors.append(
                f"{path}: authority source fields must be exactly "
                f"{sorted(required_authority_source_fields)}"
            )
        anchors = source.get("anchors")
        expected_sorted_anchors = sorted(expected_anchors)
        if anchors != expected_sorted_anchors:
            errors.append(
                f"{path}: authority anchors must exactly equal the sorted live "
                "identifier group"
            )
        authority_path = repository_regular_file_path(path)
        if authority_path is None:
            errors.append(
                f"{path}: authority source must be a visible, non-ignored regular "
                "repository-owned file without symlink components"
            )
            continue
        content = authority_path.read_text(encoding="utf-8")
        actual_sha256 = hashlib.sha256(content.encode("utf-8")).hexdigest()
        if source.get("sha256") != actual_sha256:
            errors.append(f"{path}: authority source SHA-256 is stale or incorrect")
        for anchor in expected_sorted_anchors:
            if content.count(f"`{anchor}`") != 1:
                errors.append(
                    f"{path}: authority must contain exactly one `{anchor}` anchor"
                )
        kind = source.get("kind")
        if kind == "accepted-decision":
            if not path.startswith("docs/decisions/") or not re.search(
                r"^- Status:\s*accepted\s*$", content, flags=re.MULTILINE
            ):
                errors.append(
                    f"{path}: accepted-decision authority must be an accepted ADR"
                )
        elif kind == "normative-contract":
            if path not in normative_authorities:
                errors.append(
                    f"{path}: normative-contract authority is not an approved "
                    "normative source"
                )
        else:
            errors.append(
                f"{path}: authority kind must be accepted-decision or normative-contract"
            )

    # Gate evidence_required is blocking by definition. Unlike a task's
    # explicitly dispositioned design/regression input, it may never point to
    # an owner in a later, unreachable milestone or it would create a hidden
    # release cycle. Parse only the evidence_required array, not explanatory
    # gate prose.
    for gate, block in gate_blocks.items():
        gate_edge = gate_ownership.get(gate, {})
        if gate_edge.get("status") == "superseded":
            continue
        gate_milestone = str(gate_edge.get("milestone", ""))
        reachable_milestones = transitive_dependencies(
            dependency_map, gate_milestone
        ) | {gate_milestone}
        contract_references = set(
            value
            for value in gate_required_evidence.get(gate, frozenset())
            if re.fullmatch(r"(?:LRM-[A-Z0-9-]+|UAT-[A-Z0-9-]+)", value)
        )
        acceptance_task = str(gate_edge.get("acceptance_task", ""))
        acceptance_task_ancestry = transitive_dependencies(
            task_dependencies, acceptance_task
        ) | {acceptance_task}
        for identifier in contract_references:
            if identifier in requirement_ids:
                artifact_edge = requirement_ownership.get(identifier, {})
            elif identifier in case_ids:
                artifact_edge = uat_ownership.get(identifier, {})
            else:
                errors.append(f"{gate}::{identifier}: unknown gate evidence contract")
                continue
            artifact_milestone = str(artifact_edge.get("milestone", ""))
            artifact_owner_task = str(artifact_edge.get("owning_task", ""))
            if artifact_milestone not in reachable_milestones:
                errors.append(
                    f"{gate}::{identifier}: blocking gate evidence in "
                    f"{gate_milestone} cannot depend on later or unreachable "
                    f"milestone {artifact_milestone}"
                )
            elif (
                artifact_milestone == gate_milestone
                and artifact_owner_task not in acceptance_task_ancestry
            ):
                errors.append(
                    f"{gate}::{identifier}: same-milestone evidence owner "
                    f"{artifact_owner_task} is not in acceptance-task ancestry"
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
            if owner_status != edge_status:
                errors.append(
                    f"{identifier}: status {edge_status} must match {task} "
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
                if identifier not in gate_required_evidence.get(gate, frozenset()):
                    errors.append(
                        f"{identifier}: owning gate {gate} does not name the UAT "
                        "in evidence_required"
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
            if edge.get("status") != task_edge.get("status"):
                errors.append(
                    f"{identifier}: status {edge.get('status')} must match acceptance "
                    f"task {task} status {task_edge.get('status')}"
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
        if milestone_status == "deferred" and status not in {
            "deferred",
            "superseded",
        }:
            errors.append(
                f"{identifier}: deferred milestone {milestone} requires a deferred "
                "or superseded task"
            )
        if milestone_status == "blocked" and status not in {
            "blocked",
            "superseded",
        } and not (identifier == "T-R2-007" and status == "deferred"):
            errors.append(
                f"{identifier}: blocked milestone {milestone} requires a blocked "
                "or superseded task; only T-R2-007 retains its identifier-exact "
                "deferred disposition"
            )
        if identifier == "T-R2-007" and status != "deferred":
            errors.append(
                "T-R2-007: status must remain deferred under its identifier-exact G7 disposition"
            )
        if milestone_status == "current" and status == "deferred":
            errors.append(
                f"{identifier}: current milestone {milestone} cannot contain a deferred task"
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
    resume_state = ownership.get("p03_resume_authority", {})
    resume_stopped_disposition = resume_state.get("stopped_disposition")
    terminal_stopped_project = (
        resume_state.get("observation_decision") == "Stop"
        and isinstance(resume_stopped_disposition, dict)
        and resume_stopped_disposition.get("project_state") == "stopped"
        and resume_stopped_disposition.get("support_owner") is None
    )
    if terminal_stopped_project and current_tasks:
        errors.append(
            "traceability: terminal Stop with null support_owner must leave no current task"
        )
    elif not current_tasks and not terminal_stopped_project:
        errors.append("traceability: current milestone has no current task")

    expected_proposals = CANONICAL_PROPOSAL_IDS
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
    for identifier in sorted(branch_ids - required_branch_dispositions):
        errors.append(f"traceability: unknown review-branch disposition {identifier}")
    for item in branch_records:
        if item.get("execution_status") not in {"blocked", "deferred"}:
            errors.append(
                f"{item.get('reference')}: review branch cannot be current before B0"
            )

    if validate_generated_outputs:
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

    try:
        requirements_doc = load_json("spec/requirements.json")
        uat_doc = load_json("spec/uat-cases.json")
        localization_doc = load_json("spec/localization-requirements.json")
    except (OSError, ValueError, json.JSONDecodeError) as exc:
        print("Specification validation failed:")
        print(f"- source JSON: {exc}")
        return 1
    for relative, document in (
        ("spec/requirements.json", requirements_doc),
        ("spec/uat-cases.json", uat_doc),
        ("spec/localization-requirements.json", localization_doc),
    ):
        errors.extend(json_catalog_root_issues(relative, document))
    errors.extend(
        json_catalog_bundle_issues(requirements_doc, uat_doc, localization_doc)
    )
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

    for req in requirements:
        if req.get("priority") not in VALID_PRIORITIES:
            errors.append(f"{req.get('id')}: invalid priority")
        if req.get("release") not in VALID_RELEASES:
            errors.append(f"{req.get('id')}: invalid release")
        for field in ("context", "statement", "acceptance"):
            if not str(req.get(field, "")).strip():
                errors.append(f"{req.get('id')}: missing {field}")

    for case in cases:
        if case.get("persona") not in persona_ids:
            errors.append(f"{case.get('id')}: unknown persona {case.get('persona')}")
        if case.get("release") not in VALID_RELEASES:
            errors.append(f"{case.get('id')}: invalid release")
        for field in ("title", "given", "when", "then"):
            if not str(case.get(field, "")).strip():
                errors.append(f"{case.get('id')}: missing {field}")

    gate_blocks: dict[str, str] = {}
    try:
        gate_path = repository_regular_file_path("spec/feature-gates.yaml")
        if gate_path is None:
            raise ValueError(
                "spec/feature-gates.yaml is not a visible, non-ignored regular "
                "repository-owned file without symlink components"
            )
        gate_blocks = parse_simple_yaml_records(gate_path, "gates")
        gate_ids = list(gate_blocks)
        duplicate_ids([{"id": value} for value in gate_ids], "feature gates", errors)
        for identifier, block in gate_blocks.items():
            release = re.search(
                r"^\s{4}release:\s*([^\s#]+)", block, flags=re.MULTILINE
            )
        if release is None or release.group(1) not in VALID_RELEASES | {"all"}:
                errors.append(f"{identifier}: invalid or missing gate release")
    except (OSError, ValueError) as exc:
        errors.append(f"feature gates: {exc}")

    task_blocks: dict[str, str] = {}
    try:
        task_path = repository_regular_file_path("spec/implementation-plan.yaml")
        if task_path is None:
            raise ValueError(
                "spec/implementation-plan.yaml is not a visible, non-ignored regular "
                "repository-owned file without symlink components"
            )
        task_text = task_path.read_text(encoding="utf-8")
        task_blocks = parse_simple_yaml_records(task_path, "tasks")
        task_ids = list(task_blocks)
        task_id_set = duplicate_ids([{"id": value} for value in task_ids], "tasks", errors)
        for identifier, block in task_blocks.items():
            release = re.search(
                r"^\s{4}release:\s*([^\s#]+)", block, flags=re.MULTILINE
            )
            if release is None or release.group(1) not in VALID_RELEASES:
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
            requirements,
            cases,
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
