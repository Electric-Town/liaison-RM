#!/usr/bin/env python3
"""Validate Liaison RM locale catalogues and placeholder parity."""

from __future__ import annotations

import json
import re
import sys
import unicodedata
from pathlib import Path

from jsonschema import Draft202012Validator, FormatChecker

ROOT = Path(__file__).resolve().parents[1]
LOCALES = ROOT / "examples/locales"
SCHEMA = ROOT / "schemas/locale-catalog.schema.json"
PLACEHOLDER = re.compile(r"\{([a-z][a-z0-9_]*)\}")


def load_json(path: Path):
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def main() -> int:
    errors: list[str] = []
    schema = load_json(SCHEMA)
    validator = Draft202012Validator(schema, format_checker=FormatChecker())

    catalogues: dict[str, dict] = {}
    for path in sorted(LOCALES.glob("*.json")):
        try:
            catalogue = load_json(path)
        except (OSError, json.JSONDecodeError) as error:
            errors.append(f"{path.relative_to(ROOT)}: cannot parse JSON: {error}")
            continue
        for error in sorted(validator.iter_errors(catalogue), key=str):
            location = ".".join(str(item) for item in error.absolute_path) or "<root>"
            errors.append(f"{path.name} {location}: {error.message}")
        locale = catalogue.get("locale")
        if locale in catalogues:
            errors.append(f"duplicate locale catalogue: {locale}")
        catalogues[locale] = catalogue
        if path.stem != locale:
            errors.append(f"{path.name}: filename does not match locale {locale!r}")
        for key, value in catalogue.get("messages", {}).items():
            if unicodedata.normalize("NFC", value) != value:
                errors.append(f"{locale} {key}: message is not NFC-normalized")
            if "\n\n\n" in value:
                errors.append(f"{locale} {key}: message contains excessive blank lines")

    source = catalogues.get("en-IE")
    if source is None:
        errors.append("missing en-IE source catalogue")
        source_messages: dict[str, str] = {}
        source_revision = None
    else:
        if source.get("status") != "source":
            errors.append("en-IE must have source status")
        source_messages = source.get("messages", {})
        source_revision = source.get("source_revision")

    source_keys = set(source_messages)
    for locale, catalogue in sorted(catalogues.items()):
        messages = catalogue.get("messages", {})
        keys = set(messages)
        missing = sorted(source_keys - keys)
        extra = sorted(keys - source_keys)
        if missing:
            errors.append(f"{locale}: missing keys {missing}")
        if extra:
            errors.append(f"{locale}: extra keys {extra}")
        if catalogue.get("source_locale") != "en-IE":
            errors.append(f"{locale}: source_locale must be en-IE")
        if catalogue.get("source_revision") != source_revision:
            errors.append(f"{locale}: source_revision differs from en-IE")
        for key in sorted(source_keys & keys):
            expected = sorted(PLACEHOLDER.findall(source_messages[key]))
            actual = sorted(PLACEHOLDER.findall(messages[key]))
            if actual != expected:
                errors.append(
                    f"{locale} {key}: placeholders {actual} do not match source {expected}"
                )

    pseudolocale = catalogues.get("en-XA")
    if pseudolocale is None:
        errors.append("missing en-XA expansion pseudolocale")
    else:
        if pseudolocale.get("status") != "pseudolocale":
            errors.append("en-XA must have pseudolocale status")
        for key, source_value in source_messages.items():
            pseudo_value = pseudolocale.get("messages", {}).get(key, "")
            source_letters = len(re.sub(r"\{[^}]+\}", "", source_value))
            pseudo_letters = len(re.sub(r"\{[^}]+\}", "", pseudo_value))
            if source_letters >= 8 and pseudo_letters < int(source_letters * 1.35):
                errors.append(f"en-XA {key}: expansion is below 35%")

    for locale in ("ga-IE", "ja-JP", "pt-BR"):
        catalogue = catalogues.get(locale)
        if catalogue is None:
            errors.append(f"missing structural fixture: {locale}")
        elif catalogue.get("status") != "human-review-required":
            errors.append(f"{locale}: fixture must remain human-review-required")

    if errors:
        print("Localization validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print(
        "Localization validation passed: "
        f"{len(catalogues)} catalogues, {len(source_keys)} keys"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
