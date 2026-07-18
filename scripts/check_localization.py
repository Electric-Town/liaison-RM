#!/usr/bin/env python3
"""Validate locale catalogs, placeholders, Unicode, review state, and expansion."""
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
PLACEHOLDER = re.compile(r"\{([a-z][a-z0-9_]*)\}")
HTML = re.compile(r"</?[a-z][^>]*>", re.IGNORECASE)
RELEASE_LOCALES = {"ga-IE", "ja-JP", "pt-BR"}


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

    if errors:
        print("Localization validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    source_count = len(source["messages"]) if source else 0
    print(f"Localization validation passed: {len(catalogs)} catalogs, {source_count} keys")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
