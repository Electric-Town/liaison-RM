#!/usr/bin/env python3
"""Validate provider package descriptors and evidence links."""

from __future__ import annotations

import json
import re
import sys
import tomllib
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
PROVIDERS = ROOT / "providers"
PROVIDER_ID = re.compile(r"^[a-z0-9]+(?:[.-][a-z0-9]+)+$")
PROVIDER_VERSION = re.compile(r"^\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?$")
SNAKE_NAME = re.compile(r"^[a-z][a-z0-9_]*$")
KEBAB_NAME = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
SAFE_MODES = {"import", "export", "backup", "single-writer", "multi-writer"}
VALUE_TYPES = {"string", "integer", "boolean", "string-list", "secret-ref"}
CONFORMANCE = {"not-tested", "passed", "passed-with-limits", "failed"}
TOP_LEVEL_KEYS = {
    "schema",
    "provider_id",
    "provider_version",
    "display_name",
    "contracts",
    "configuration_fields",
    "network_destinations",
    "conformance",
}
CONTRACT_KEYS = {
    "name",
    "version",
    "operations",
    "safe_modes",
    "limits",
    "consistency",
}
FIELD_KEYS = {
    "key",
    "value_type",
    "secret",
    "required",
    "description",
}
CONFORMANCE_KEYS = {"suite_version", "status", "evidence_ref"}


class DuplicateJsonKeyError(ValueError):
    """Raised when a JSON object repeats a key at any nesting depth."""


def reject_duplicate_json_keys(
    pairs: list[tuple[str, Any]],
) -> dict[str, Any]:
    """Build a JSON object while rejecting last-wins duplicate-key input."""

    value: dict[str, Any] = {}
    for key, item in pairs:
        if key in value:
            raise DuplicateJsonKeyError(f"duplicate JSON key {key!r}")
        value[key] = item
    return value


def load_json_document(path: Path) -> Any:
    """Load JSON with duplicate-key rejection for every object in the document."""

    return json.loads(
        path.read_text(encoding="utf-8"),
        object_pairs_hook=reject_duplicate_json_keys,
    )


def string_field(
    value: dict[str, Any],
    field: str,
    owner: str,
    errors: list[str],
) -> str | None:
    """Return a descriptor string without coercing JSON scalar types."""

    item = value.get(field)
    if not isinstance(item, str):
        errors.append(f"{owner}: {field} must be a string")
        return None
    return item


def duplicate_values(values: list[str]) -> list[str]:
    return sorted({value for value in values if values.count(value) > 1})


def require_exact_keys(
    value: dict[str, Any],
    expected: set[str],
    required: set[str],
    owner: str,
    errors: list[str],
) -> None:
    missing = sorted(required - value.keys())
    extra = sorted(value.keys() - expected)
    if missing:
        errors.append(f"{owner}: missing keys {missing}")
    if extra:
        errors.append(f"{owner}: unsupported keys {extra}")


def validate_descriptor(path: Path, errors: list[str]) -> dict[str, Any] | None:
    relative = path.relative_to(ROOT)
    try:
        descriptor = load_json_document(path)
    except (OSError, json.JSONDecodeError, DuplicateJsonKeyError) as error:
        errors.append(f"{relative}: cannot parse JSON: {error}")
        return None
    if not isinstance(descriptor, dict):
        errors.append(f"{relative}: descriptor must be an object")
        return None

    descriptor_owner = str(relative)

    require_exact_keys(
        descriptor,
        TOP_LEVEL_KEYS,
        TOP_LEVEL_KEYS,
        descriptor_owner,
        errors,
    )

    schema = string_field(descriptor, "schema", descriptor_owner, errors)
    if schema is not None and schema != "liaison/provider-descriptor@1":
        errors.append(f"{relative}: unsupported descriptor schema")

    provider_id_value = string_field(
        descriptor, "provider_id", descriptor_owner, errors
    )
    provider_id = provider_id_value or ""
    if provider_id_value is not None and (
        not PROVIDER_ID.fullmatch(provider_id) or provider_id.count(".") < 2
    ):
        errors.append(
            f"{relative}: provider_id must contain at least three reverse-domain segments"
        )

    version = string_field(descriptor, "provider_version", descriptor_owner, errors)
    if version is not None and not PROVIDER_VERSION.fullmatch(version):
        errors.append(f"{relative}: invalid provider_version {version!r}")

    display_name_value = string_field(
        descriptor, "display_name", descriptor_owner, errors
    )
    display_name = display_name_value.strip() if display_name_value is not None else ""
    if display_name_value is not None and (
        not display_name or len(display_name) > 120
    ):
        errors.append(f"{relative}: display_name must contain 1 to 120 characters")

    contracts = descriptor.get("contracts")
    if not isinstance(contracts, list) or not contracts:
        errors.append(f"{relative}: contracts must be a non-empty array")
        contracts = []
    contract_ids: list[str] = []
    all_operations: set[str] = set()
    for index, contract in enumerate(contracts):
        owner = f"{relative}: contracts[{index}]"
        if not isinstance(contract, dict):
            errors.append(f"{owner}: contract must be an object")
            continue
        require_exact_keys(
            contract,
            CONTRACT_KEYS,
            {"name", "version", "operations", "safe_modes", "consistency"},
            owner,
            errors,
        )
        name_value = string_field(contract, "name", owner, errors)
        name = name_value or ""
        version_value = contract.get("version")
        if name_value is not None and not KEBAB_NAME.fullmatch(name):
            errors.append(f"{owner}: name must use kebab case")
        if not isinstance(version_value, int) or isinstance(version_value, bool) or version_value < 1:
            errors.append(f"{owner}: version must be a positive integer")
        contract_ids.append(f"{name}@{version_value}")

        operations = contract.get("operations")
        if not isinstance(operations, list) or not operations:
            errors.append(f"{owner}: operations must be a non-empty array")
            operations = []
        normalized_operations: list[str] = []
        for operation_index, operation in enumerate(operations):
            if not isinstance(operation, str):
                errors.append(
                    f"{owner}: operations[{operation_index}] must be a string"
                )
                continue
            normalized_operations.append(operation)
        if any(not KEBAB_NAME.fullmatch(operation) for operation in normalized_operations):
            errors.append(f"{owner}: operation names must use kebab case")
        duplicate_operations = duplicate_values(normalized_operations)
        if duplicate_operations:
            errors.append(f"{owner}: duplicate operations {duplicate_operations}")
        all_operations.update(normalized_operations)

        safe_modes = contract.get("safe_modes")
        if not isinstance(safe_modes, list) or not safe_modes:
            errors.append(f"{owner}: safe_modes must be a non-empty array")
            safe_modes = []
        normalized_modes: list[str] = []
        for mode_index, mode in enumerate(safe_modes):
            if not isinstance(mode, str):
                errors.append(f"{owner}: safe_modes[{mode_index}] must be a string")
                continue
            normalized_modes.append(mode)
        unknown_modes = sorted(set(normalized_modes) - SAFE_MODES)
        if unknown_modes:
            errors.append(f"{owner}: unknown safe modes {unknown_modes}")
        duplicate_modes = duplicate_values(normalized_modes)
        if duplicate_modes:
            errors.append(f"{owner}: duplicate safe modes {duplicate_modes}")

        consistency_value = string_field(contract, "consistency", owner, errors)
        if consistency_value is not None and not consistency_value.strip():
            errors.append(f"{owner}: consistency statement is required")

    duplicate_contracts = duplicate_values(contract_ids)
    if duplicate_contracts:
        errors.append(f"{relative}: duplicate contracts {duplicate_contracts}")

    fields = descriptor.get("configuration_fields")
    if not isinstance(fields, list):
        errors.append(f"{relative}: configuration_fields must be an array")
        fields = []
    field_names: list[str] = []
    for index, field in enumerate(fields):
        owner = f"{relative}: configuration_fields[{index}]"
        if not isinstance(field, dict):
            errors.append(f"{owner}: field must be an object")
            continue
        require_exact_keys(
            field,
            FIELD_KEYS,
            {"key", "value_type", "secret"},
            owner,
            errors,
        )
        key_value = string_field(field, "key", owner, errors)
        key = key_value or ""
        if key_value is not None and not SNAKE_NAME.fullmatch(key):
            errors.append(f"{owner}: key must use snake case")
        field_names.append(key)
        value_type_value = string_field(field, "value_type", owner, errors)
        value_type = value_type_value or ""
        if value_type_value is not None and value_type not in VALUE_TYPES:
            errors.append(f"{owner}: unsupported value_type {value_type!r}")
        secret = field.get("secret")
        if not isinstance(secret, bool):
            errors.append(f"{owner}: secret must be boolean")
        elif secret != (value_type == "secret-ref"):
            errors.append(
                f"{owner}: secret fields must use secret-ref and other fields must not"
            )
        if "required" in field and not isinstance(field["required"], bool):
            errors.append(f"{owner}: required must be boolean")
        description = string_field(field, "description", owner, errors)
        if description is not None and not description.strip():
            errors.append(f"{owner}: description is required")
    duplicate_fields = duplicate_values(field_names)
    if duplicate_fields:
        errors.append(f"{relative}: duplicate configuration fields {duplicate_fields}")

    destinations = descriptor.get("network_destinations")
    if not isinstance(destinations, list):
        errors.append(f"{relative}: network_destinations must be an array")
        destinations = []
    normalized_destinations: list[str] = []
    for destination_index, destination in enumerate(destinations):
        if not isinstance(destination, str):
            errors.append(
                f"{relative}: network_destinations[{destination_index}] must be a string"
            )
            continue
        normalized_destinations.append(destination.strip())
    if any(not destination for destination in normalized_destinations):
        errors.append(f"{relative}: network destination cannot be empty")
    duplicate_destinations = duplicate_values(normalized_destinations)
    if duplicate_destinations:
        errors.append(f"{relative}: duplicate network destinations {duplicate_destinations}")

    conformance = descriptor.get("conformance")
    if not isinstance(conformance, dict):
        errors.append(f"{relative}: conformance must be an object")
        conformance = {}
    require_exact_keys(
        conformance,
        CONFORMANCE_KEYS,
        {"suite_version", "status"},
        f"{relative}: conformance",
        errors,
    )
    suite_version = conformance.get("suite_version")
    if not isinstance(suite_version, int) or isinstance(suite_version, bool) or suite_version < 1:
        errors.append(f"{relative}: conformance suite_version must be positive")
    status_value = string_field(
        conformance, "status", f"{relative}: conformance", errors
    )
    status = status_value or ""
    if status_value is not None and status not in CONFORMANCE:
        errors.append(f"{relative}: unsupported conformance status {status!r}")
    evidence_ref_value = (
        string_field(
            conformance, "evidence_ref", f"{relative}: conformance", errors
        )
        if "evidence_ref" in conformance
        else None
    )
    evidence_ref = evidence_ref_value.strip() if evidence_ref_value is not None else ""
    if status_value is not None and status != "not-tested" and not evidence_ref:
        errors.append(f"{relative}: tested provider needs evidence_ref")
    if evidence_ref:
        evidence_path = ROOT / evidence_ref
        if not evidence_path.is_file():
            errors.append(f"{relative}: evidence_ref does not exist: {evidence_ref}")

    readme = path.parent / "README.md"
    if not readme.is_file():
        errors.append(f"{path.parent.relative_to(ROOT)}: missing README.md")
    evidence_directory = path.parent / "evidence"
    if not evidence_directory.is_dir():
        errors.append(f"{path.parent.relative_to(ROOT)}: missing evidence directory")

    # Prevent descriptor drift for the checked-in local reference implementation.
    if provider_id == "org.electric-town.local-folder":
        source = (ROOT / "adapters/object-store-local/src/lib.rs").read_text(
            encoding="utf-8"
        )
        wit = (ROOT / "interfaces/wit/liaison-provider.wit").read_text(
            encoding="utf-8"
        )
        if provider_id not in source:
            errors.append(f"{relative}: provider ID differs from Rust descriptor")
        for operation in all_operations:
            rust_operation = operation.replace("-", "_")
            if operation not in source and rust_operation not in source:
                errors.append(
                    f"{relative}: operation {operation!r} is absent from Rust adapter"
                )
            if operation not in wit:
                errors.append(f"{relative}: operation {operation!r} is absent from WIT")

    return descriptor


def main() -> int:
    errors: list[str] = []

    provider_plan = (PROVIDERS / "README.md").read_text(encoding="utf-8")
    expected_google_drive_row = (
        "| Google Drive | object-store@1 where conformance permits | R5 |"
    )
    google_drive_rows = [
        line.strip()
        for line in provider_plan.splitlines()
        if re.match(r"^\|\s*Google Drive\s*\|", line, flags=re.IGNORECASE)
    ]
    if google_drive_rows != [expected_google_drive_row]:
        errors.append(
            "providers/README.md: Google Drive must have exactly one plan row "
            f"equal to {expected_google_drive_row!r}, not {google_drive_rows!r}"
        )

    root_cargo = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    workspace_version = str(root_cargo["workspace"]["package"]["version"])

    provider_directories = sorted(
        directory
        for directory in PROVIDERS.iterdir()
        if directory.is_dir() and not directory.name.startswith("_")
    )
    if not provider_directories:
        errors.append("providers: no provider packages found")

    for directory in provider_directories:
        descriptor_path = directory / "descriptor.json"
        if not descriptor_path.is_file():
            errors.append(f"{directory.relative_to(ROOT)}: missing descriptor.json")
            continue
        descriptor = validate_descriptor(descriptor_path, errors)
        if descriptor is not None and (
            descriptor.get("provider_id") == "org.electric-town.local-folder"
            and descriptor.get("provider_version") != workspace_version
        ):
            errors.append(
                f"{descriptor_path.relative_to(ROOT)}: version differs from workspace"
            )

    if errors:
        print("Provider check failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print(f"Provider check passed: {len(provider_directories)} provider package(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
