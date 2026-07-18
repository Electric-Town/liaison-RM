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


def validate_descriptor(path: Path, errors: list[str]) -> None:
    relative = path.relative_to(ROOT)
    try:
        descriptor = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        errors.append(f"{relative}: cannot parse JSON: {error}")
        return
    if not isinstance(descriptor, dict):
        errors.append(f"{relative}: descriptor must be an object")
        return

    require_exact_keys(
        descriptor,
        TOP_LEVEL_KEYS,
        TOP_LEVEL_KEYS,
        str(relative),
        errors,
    )

    if descriptor.get("schema") != "liaison/provider-descriptor@1":
        errors.append(f"{relative}: unsupported descriptor schema")

    provider_id = str(descriptor.get("provider_id", ""))
    if not PROVIDER_ID.fullmatch(provider_id) or provider_id.count(".") < 2:
        errors.append(
            f"{relative}: provider_id must contain at least three reverse-domain segments"
        )

    version = str(descriptor.get("provider_version", ""))
    if not PROVIDER_VERSION.fullmatch(version):
        errors.append(f"{relative}: invalid provider_version {version!r}")

    display_name = str(descriptor.get("display_name", "")).strip()
    if not display_name or len(display_name) > 120:
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
        name = str(contract.get("name", ""))
        version_value = contract.get("version")
        if not KEBAB_NAME.fullmatch(name):
            errors.append(f"{owner}: name must use kebab case")
        if not isinstance(version_value, int) or isinstance(version_value, bool) or version_value < 1:
            errors.append(f"{owner}: version must be a positive integer")
        contract_ids.append(f"{name}@{version_value}")

        operations = contract.get("operations")
        if not isinstance(operations, list) or not operations:
            errors.append(f"{owner}: operations must be a non-empty array")
            operations = []
        normalized_operations = [str(operation) for operation in operations]
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
        normalized_modes = [str(mode) for mode in safe_modes]
        unknown_modes = sorted(set(normalized_modes) - SAFE_MODES)
        if unknown_modes:
            errors.append(f"{owner}: unknown safe modes {unknown_modes}")
        duplicate_modes = duplicate_values(normalized_modes)
        if duplicate_modes:
            errors.append(f"{owner}: duplicate safe modes {duplicate_modes}")

        consistency = str(contract.get("consistency", "")).strip()
        if not consistency:
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
        key = str(field.get("key", ""))
        if not SNAKE_NAME.fullmatch(key):
            errors.append(f"{owner}: key must use snake case")
        field_names.append(key)
        value_type = str(field.get("value_type", ""))
        if value_type not in VALUE_TYPES:
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
        if not str(field.get("description", "")).strip():
            errors.append(f"{owner}: description is required")
    duplicate_fields = duplicate_values(field_names)
    if duplicate_fields:
        errors.append(f"{relative}: duplicate configuration fields {duplicate_fields}")

    destinations = descriptor.get("network_destinations")
    if not isinstance(destinations, list):
        errors.append(f"{relative}: network_destinations must be an array")
        destinations = []
    normalized_destinations = [str(destination).strip() for destination in destinations]
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
    status = str(conformance.get("status", ""))
    if status not in CONFORMANCE:
        errors.append(f"{relative}: unsupported conformance status {status!r}")
    evidence_ref = str(conformance.get("evidence_ref", "")).strip()
    if status != "not-tested" and not evidence_ref:
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


def main() -> int:
    errors: list[str] = []

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
        validate_descriptor(descriptor_path, errors)
        descriptor = json.loads(descriptor_path.read_text(encoding="utf-8"))
        if (
            descriptor.get("provider_id") == "org.electric-town.local-folder"
            and str(descriptor.get("provider_version")) != workspace_version
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
