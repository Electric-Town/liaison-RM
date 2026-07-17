#!/usr/bin/env python3
"""Enforce Liaison RM's initial dependency and naming boundaries."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GENERIC_MODULE_NAMES = {
    "helper.rs",
    "helpers.rs",
    "manager.rs",
    "managers.rs",
    "service.rs",
    "services.rs",
    "utils.rs",
}
INFRASTRUCTURE_TOKENS = {
    "tauri",
    "reqwest",
    "rusqlite",
    "sqlx",
    "aws_sdk",
    "google_",
    "webdav",
    "liaison_vault_markdown",
}
CONTEXT_IMPORT = re.compile(r"\bliaison_(?:workspace|people|relationships|events|connections)\b")


def main() -> int:
    errors: list[str] = []

    for path in sorted(ROOT.rglob("*.rs")):
        if ".git" in path.parts or "target" in path.parts:
            continue
        relative = path.relative_to(ROOT)
        text = path.read_text(encoding="utf-8")
        lowered = text.casefold()

        if path.name in GENERIC_MODULE_NAMES:
            errors.append(
                f"{relative}: generic module name hides the owning domain concept"
            )
        if re.search(r"\bunsafe\s*(?:fn|impl|trait|extern|\{)", text):
            errors.append(f"{relative}: unsafe code is prohibited")

        if relative.parts[:1] == ("crates",) and relative.parts[1:2] == (
            "liaison-shared-kernel",
        ):
            if CONTEXT_IMPORT.search(lowered):
                errors.append(f"{relative}: shared kernel imports a bounded context")

        if relative.parts[:1] == ("contexts",):
            for token in INFRASTRUCTURE_TOKENS:
                if token in lowered:
                    errors.append(
                        f"{relative}: bounded context imports infrastructure token {token!r}"
                    )
            if "/domain" in relative.as_posix() or path.name == "domain.rs":
                if "std::fs" in text or "std::net" in text or "std::process::Command" in text:
                    errors.append(
                        f"{relative}: domain module performs filesystem, network, or process I/O"
                    )

    contexts = ROOT / "contexts"
    if contexts.exists():
        for context in sorted(path for path in contexts.iterdir() if path.is_dir()):
            if not (context / "README.md").exists():
                errors.append(f"{context.relative_to(ROOT)}: missing context README.md")
            if not (context / "Cargo.toml").exists():
                errors.append(f"{context.relative_to(ROOT)}: missing Cargo.toml")

    for cargo in sorted(ROOT.rglob("Cargo.toml")):
        if cargo == ROOT / "Cargo.toml" or "target" in cargo.parts:
            continue
        text = cargo.read_text(encoding="utf-8")
        if "[lints]\nworkspace = true" not in text:
            errors.append(
                f"{cargo.relative_to(ROOT)}: crate does not inherit workspace lints"
            )

    if errors:
        print("Architecture check failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print("Architecture check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
