#!/usr/bin/env python3
"""Validate provider package descriptors and declared capability boundaries."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from urllib.parse import urlparse

ROOT = Path(__file__).resolve().parents[1]
PROVIDERS = ROOT / "providers"
KNOWN_CONTRACTS = {"object-store@1"}
KNOWN_SAFE_MODES = {
    "backup",
    "restore",
    "single-writer-publication",
    "immutable-transport",
    "multi-writer-synchronisation",
    "contacts-import",
    "contacts-synchronisation",
    "calendar-import",
    "email-metadata-import",
}
OBJECT_STORE_OPERATIONS = {
    "put-immutable",
    "get",
    "head",
    "list",
    "delete-if-digest",
    "replace-manifest-if-revision",
}
SECRET_LIKE_KEYS = {"password", "token", "secret", "client_secret", "private_key", "access_key"}


def read_json(path: Path, errors: list[str]):
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        errors.append(f"{path.relative_to(ROOT)}: invalid JSON: {exc}")
        return None


def validate_provider(directory: Path, errors: list[str], warnings: list[str]) -> None:
    descriptor_path = directory / "descriptor.json"
    readme_path = directory / "README.md"
    if not descriptor_path.is_file():
        errors.append(f"{directory.relative_to(ROOT)}: missing descriptor.json")
        return
    if not readme_path.is_file():
        errors.append(f"{directory.relative_to(ROOT)}: missing README.md")

    descriptor = read_json(descriptor_path, errors)
    if not isinstance(descriptor, dict):
        return

    provider_id = descriptor.get("provider_id")
    if provider_id != directory.name:
        errors.append(
            f"{descriptor_path.relative_to(ROOT)}: provider_id must match directory name"
        )
    if not isinstance(descriptor.get("provider_version"), str) or not descriptor["provider_version"].strip():
        errors.append(f"{descriptor_path.relative_to(ROOT)}: provider_version is required")
    if not isinstance(descriptor.get("display_name"), str) or not descriptor["display_name"].strip():
        errors.append(f"{descriptor_path.relative_to(ROOT)}: display_name is required")

    schema_value = descriptor.get("configuration_schema")
    if not isinstance(schema_value, str) or not schema_value:
        errors.append(f"{descriptor_path.relative_to(ROOT)}: configuration_schema is required")
    else:
        schema_path = ROOT / schema_value
        if not schema_path.is_file():
            errors.append(
                f"{descriptor_path.relative_to(ROOT)}: configuration schema does not exist: {schema_value}"
            )
        else:
            schema = read_json(schema_path, errors)
            if isinstance(schema, dict):
                properties = schema.get("properties", {})
                if isinstance(properties, dict) and SECRET_LIKE_KEYS & {key.lower() for key in properties}:
                    errors.append(
                        f"{schema_path.relative_to(ROOT)}: secret values must be opaque secret references, not provider configuration properties"
                    )

    contracts = descriptor.get("contracts")
    if not isinstance(contracts, list) or not contracts:
        errors.append(f"{descriptor_path.relative_to(ROOT)}: at least one contract is required")
    else:
        seen = set()
        for claim in contracts:
            if not isinstance(claim, dict):
                errors.append(f"{descriptor_path.relative_to(ROOT)}: contract claim must be an object")
                continue
            contract = claim.get("contract")
            if contract not in KNOWN_CONTRACTS:
                errors.append(
                    f"{descriptor_path.relative_to(ROOT)}: unknown contract: {contract!r}"
                )
            if contract in seen:
                errors.append(
                    f"{descriptor_path.relative_to(ROOT)}: duplicate contract claim: {contract}"
                )
            seen.add(contract)
            operations = claim.get("operations")
            if not isinstance(operations, list) or not operations:
                errors.append(
                    f"{descriptor_path.relative_to(ROOT)}: contract {contract} needs operations"
                )
            elif contract == "object-store@1":
                operation_set = set(operations)
                unknown = operation_set - OBJECT_STORE_OPERATIONS
                missing = OBJECT_STORE_OPERATIONS - operation_set
                if unknown:
                    errors.append(
                        f"{descriptor_path.relative_to(ROOT)}: unknown object-store operations: {sorted(unknown)}"
                    )
                if missing:
                    errors.append(
                        f"{descriptor_path.relative_to(ROOT)}: object-store claim omits required operations: {sorted(missing)}"
                    )

    safe_modes = descriptor.get("safe_modes")
    if not isinstance(safe_modes, list):
        errors.append(f"{descriptor_path.relative_to(ROOT)}: safe_modes must be a list")
    else:
        unknown_modes = set(safe_modes) - KNOWN_SAFE_MODES
        if unknown_modes:
            errors.append(
                f"{descriptor_path.relative_to(ROOT)}: unknown safe modes: {sorted(unknown_modes)}"
            )
        if "multi-writer-synchronisation" in safe_modes and descriptor.get("conformance") is None:
            errors.append(
                f"{descriptor_path.relative_to(ROOT)}: multi-writer synchronisation requires conformance evidence"
            )

    for slot in descriptor.get("secret_slots", []):
        if not isinstance(slot, str) or not slot.strip() or any(character.isspace() for character in slot):
            errors.append(f"{descriptor_path.relative_to(ROOT)}: invalid secret slot: {slot!r}")

    for destination in descriptor.get("destinations", []):
        if not isinstance(destination, str) or not destination:
            errors.append(f"{descriptor_path.relative_to(ROOT)}: invalid destination")
            continue
        parsed = urlparse(destination)
        if parsed.scheme not in {"https", "http"} or not parsed.hostname:
            errors.append(
                f"{descriptor_path.relative_to(ROOT)}: destination must be an explicit HTTP(S) origin: {destination}"
            )

    if descriptor.get("conformance") is None:
        warnings.append(
            f"{descriptor_path.relative_to(ROOT)}: no accepted conformance report; provider must remain draft and unavailable for release"
        )


def main() -> int:
    errors: list[str] = []
    warnings: list[str] = []
    if not PROVIDERS.is_dir():
        errors.append("providers directory is missing")
    else:
        for directory in sorted(path for path in PROVIDERS.iterdir() if path.is_dir()):
            validate_provider(directory, errors, warnings)

    wit = ROOT / "interfaces/wit/liaison-provider.wit"
    if not wit.is_file():
        errors.append("missing interfaces/wit/liaison-provider.wit")
    elif "interface object-store-v1" not in wit.read_text(encoding="utf-8"):
        errors.append("WIT package does not declare object-store-v1")

    for warning in warnings:
        print(f"WARNING: {warning}")

    if errors:
        print("Provider package check failed:")
        for error in sorted(set(errors)):
            print(f"- {error}")
        return 1

    provider_count = len([path for path in PROVIDERS.iterdir() if path.is_dir()])
    print(f"Provider package check passed: {provider_count} provider package(s)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
