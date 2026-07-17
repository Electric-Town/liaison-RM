#!/usr/bin/env python3
"""Check repository boundaries that can be evaluated without compiling Rust."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CONTEXTS = ROOT / "contexts"

FORBIDDEN_CONTEXT_DEPENDENCIES = {
    "tauri",
    "reqwest",
    "hyper",
    "axum",
    "actix-web",
    "rusqlite",
    "sqlx",
    "diesel",
    "sea-orm",
    "aws-sdk-s3",
    "google-drive3",
    "webdav-client",
}

FORBIDDEN_CONTEXT_SOURCE = {
    "std::fs",
    "std::net",
    "tokio::net",
    "reqwest::",
    "tauri::",
    "rusqlite::",
    "sqlx::",
    "aws_sdk_",
    "google_drive",
}

PROVIDER_NAMES = {
    "google drive",
    "gmail",
    "amazon s3",
    "aws s3",
    "minio",
    "azure blob",
    "dropbox",
}


def relative(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def context_directories() -> list[Path]:
    if not CONTEXTS.is_dir():
        return []
    return sorted(path for path in CONTEXTS.iterdir() if path.is_dir())


def check_context_shape(errors: list[str]) -> None:
    for context in context_directories():
        for required in ["README.md", "Cargo.toml", "src/lib.rs"]:
            path = context / required
            if not path.is_file():
                errors.append(f"{relative(context)}: missing {required}")


def check_context_dependencies(errors: list[str]) -> None:
    dependency_pattern = re.compile(r"^([A-Za-z0-9_-]+)\s*=", re.MULTILINE)
    for context in context_directories():
        cargo = context / "Cargo.toml"
        if not cargo.is_file():
            continue
        dependencies = {name.lower() for name in dependency_pattern.findall(cargo.read_text(encoding="utf-8"))}
        for forbidden in sorted(FORBIDDEN_CONTEXT_DEPENDENCIES & dependencies):
            errors.append(
                f"{relative(cargo)}: bounded context depends on external mechanism '{forbidden}'"
            )


def check_context_source(errors: list[str]) -> None:
    for context in context_directories():
        for source in (context / "src").rglob("*.rs") if (context / "src").is_dir() else []:
            text = source.read_text(encoding="utf-8")
            lower = text.lower()
            for token in sorted(FORBIDDEN_CONTEXT_SOURCE):
                if token.lower() in lower:
                    errors.append(
                        f"{relative(source)}: bounded context uses external mechanism '{token}'"
                    )
            for provider in sorted(PROVIDER_NAMES):
                if provider in lower:
                    errors.append(
                        f"{relative(source)}: provider name '{provider}' leaked into bounded-context source"
                    )
            if re.search(r"\bunsafe\b", text):
                errors.append(f"{relative(source)}: unsafe code is not permitted in a context")


def check_application_storage_bypass(errors: list[str]) -> None:
    for application_root in [ROOT / "apps"]:
        if not application_root.is_dir():
            continue
        for source in application_root.rglob("*.rs"):
            text = source.read_text(encoding="utf-8")
            for token in ["std::fs", "rusqlite::", "sqlx::", "serde_yaml::"]:
                if token in text:
                    errors.append(
                        f"{relative(source)}: application bypasses an application service through '{token}'"
                    )


def check_workspace_members(errors: list[str]) -> None:
    cargo = ROOT / "Cargo.toml"
    if not cargo.is_file():
        errors.append("missing root Cargo.toml")
        return
    text = cargo.read_text(encoding="utf-8")
    members_block = re.search(r"members\s*=\s*\[(.*?)\]", text, flags=re.DOTALL)
    if not members_block:
        errors.append("Cargo.toml: workspace members list is missing")
        return
    declared = set(re.findall(r'"([^"]+)"', members_block.group(1)))
    discovered = {
        relative(path.parent)
        for root_name in ["apps", "adapters", "contexts", "crates"]
        for path in (ROOT / root_name).glob("*/Cargo.toml")
    }
    for missing in sorted(discovered - declared):
        errors.append(f"Cargo.toml: crate is not a workspace member: {missing}")
    for absent in sorted(declared - discovered):
        errors.append(f"Cargo.toml: declared workspace member does not exist: {absent}")


def main() -> int:
    errors: list[str] = []
    check_context_shape(errors)
    check_context_dependencies(errors)
    check_context_source(errors)
    check_application_storage_bypass(errors)
    check_workspace_members(errors)

    if errors:
        print("Architecture check failed:")
        for error in sorted(set(errors)):
            print(f"- {error}")
        return 1

    print(
        f"Architecture check passed: {len(context_directories())} bounded contexts are structurally valid"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
