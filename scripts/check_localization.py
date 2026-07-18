#!/usr/bin/env python3
"""Validate locale catalogs, placeholders, Unicode, review state, and traceability."""
from __future__ import annotations

import json
import re
import sys
import unicodedata
from pathlib import Path

from jsonschema import Draft202012Validator, FormatChecker

ROOT = Path(__file__).resolve().parents[1]
LOCALES = ROOT / "examples/locales"
SCHEMA = json.loads((ROOT / "schemas/locale-catalog.schema.json").read_text(encoding="utf-8"))
TRACEABILITY = ROOT / "spec/localization-requirements.json"
PLACEHOLDER = re.compile(r"\{([a-z][a-z0-9_]*)\}")
HTML = re.compile(r"</?[a-z][^>]*>", re.IGNORECASE)
REQUIREMENT_ID = re.compile(r"^LRM-L10N-[0-9]{3}$")
UAT_ID = re.compile(r"^UAT-LOC-[0-9]{3}$")
RELEASE_LOCALES = {"ga-IE", "ja-JP", "pt-BR"}


def validate_traceability(errors: list[str]) -> tuple[int, int]:
    try:
        document = json.loads(TRACEABILITY.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as error:
        errors.append(f"localization traceability cannot be read: {error}")
        return 0, 0

    if document.get("schema_version") != 1:
        errors.append("localization traceability uses an unsupported schema_version")
    if document.get("context") != "localization":
        errors.append("localization traceability context must be localization")

    requirements = document.get("requirements", [])
    uat_cases = document.get("uat", [])
    if not isinstance(requirements, list) or not requirements:
        errors.append("localization traceability requires at least one requirement")
        requirements = []
    if not isinstance(uat_cases, list) or not uat_cases:
        errors.append("localization traceability requires at least one UAT case")
        uat_cases = []

    requirement_ids = [item.get("id", "") for item in requirements if isinstance(item, dict)]
    uat_ids = [item.get("id", "") for item in uat_cases if isinstance(item, dict)]
    if len(requirement_ids) != len(set(requirement_ids)):
        errors.append("localization requirement IDs are not unique")
    if len(uat_ids) != len(set(uat_ids)):
        errors.append("localization UAT IDs are not unique")
    for identifier in requirement_ids:
        if not REQUIREMENT_ID.fullmatch(identifier):
            errors.append(f"invalid localization requirement ID: {identifier}")
    for identifier in uat_ids:
        if not UAT_ID.fullmatch(identifier):
            errors.append(f"invalid localization UAT ID: {identifier}")

    for item in requirements:
        if not isinstance(item, dict):
            errors.append("localization requirement must be an object")
            continue
        for key in ("id", "release", "priority", "statement", "acceptance"):
            if not str(item.get(key, "")).strip():
                errors.append(f"{item.get('id', '<unknown>')}: missing {key}")
    for item in uat_cases:
        if not isinstance(item, dict):
            errors.append("localization UAT case must be an object")
            continue
        for key in ("id", "persona", "release", "title", "given", "when", "then"):
            if not str(item.get(key, "")).strip():
                errors.append(f"{item.get('id', '<unknown>')}: missing {key}")

    return len(requirements), len(uat_cases)


def main() -> int:
    errors: list[str] = []
    catalogs: dict[str, dict] = {}
    validator = Draft202012Validator(SCHEMA, format_checker=FormatChecker())

    for path in sorted(LOCALES.glob("*.json")):
        try:
            raw = path.read_text(encoding="utf-8")
            document = json.loads(raw)
        except (UnicodeDecodeError, json.JSONDecodeError) as error:
            errors.append(f"{path.name}: cannot decode catalog: {error}")
            continue

        locale = document.get("locale", path.stem)
        if locale in catalogs:
            errors.append(f"{path.name}: duplicate locale {locale}")
        catalogs[locale] = document

        for error in validator.iter_errors(document):
            location = ".".join(str(item) for item in error.absolute_path) or "$"
            errors.append(f"{path.name}:{location}: {error.message}")
        if raw != unicodedata.normalize("NFC", raw):
            errors.append(f"{path.name}: content is not NFC-normalized")
        if path.stem != locale:
            errors.append(f"{path.name}: filename does not match locale {locale}")

    source = catalogs.get("en-IE")
    if source is None:
        errors.append("en-IE source catalog is missing")
    else:
        source_keys = set(source["messages"])
        for locale, catalog in sorted(catalogs.items()):
            keys = set(catalog["messages"])
            missing = sorted(source_keys - keys)
            extra = sorted(keys - source_keys)
            if missing:
                errors.append(f"{locale}: missing source keys {missing}")
            if extra:
                errors.append(f"{locale}: extra keys without source definitions {extra}")
            for key in sorted(source_keys & keys):
                source_value = source["messages"][key]
                value = catalog["messages"][key]
                if set(PLACEHOLDER.findall(source_value)) != set(PLACEHOLDER.findall(value)):
                    errors.append(f"{locale}:{key}: placeholder set differs from source")
                if HTML.search(value):
                    errors.append(f"{locale}:{key}: HTML is not allowed in ordinary messages")
                if not value.strip():
                    errors.append(f"{locale}:{key}: value is blank")

            status = catalog["status"]
            review = catalog.get("review", {})
            if status == "release-ready":
                if not review.get("reviewer") or not review.get("reviewed_at"):
                    errors.append(f"{locale}: release-ready catalog lacks named dated review")
                if locale in RELEASE_LOCALES:
                    copied = sorted(
                        key
                        for key in source_keys
                        if catalog["messages"].get(key) == source["messages"].get(key)
                        and key != "app.name"
                    )
                    if copied:
                        errors.append(f"{locale}: release-ready catalog still copies source text for {copied}")

        pseudo = catalogs.get("en-XA")
        if pseudo is None:
            errors.append("en-XA expansion catalog is missing")
        else:
            if pseudo.get("status") != "test-only":
                errors.append("en-XA must remain test-only")
            for key, source_value in source["messages"].items():
                source_visible = PLACEHOLDER.sub("", source_value)
                pseudo_visible = PLACEHOLDER.sub("", pseudo["messages"][key])
                if len(pseudo_visible) < max(len(source_visible) + 6, int(len(source_visible) * 1.30)):
                    errors.append(f"en-XA:{key}: pseudolocale does not provide useful expansion")

    required_catalogs = {"en-IE", "en-XA", "ga-IE", "ja-JP", "pt-BR"}
    absent = sorted(required_catalogs - catalogs.keys())
    if absent:
        errors.append(f"required architecture catalogs are missing: {absent}")

    requirement_count, uat_count = validate_traceability(errors)

    if errors:
        print("Localization validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    source_count = len(source["messages"]) if source else 0
    print(
        "Localization validation passed: "
        f"{len(catalogs)} catalogs, {source_count} keys, "
        f"{requirement_count} requirements, {uat_count} UAT cases"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
