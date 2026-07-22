#!/usr/bin/env python3
"""Enforce Liaison RM's initial dependency and naming boundaries."""

from __future__ import annotations

import re
import sys
import tomllib
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
FORBIDDEN_CRATE_DEPENDENCIES = {
    "contexts/people": {"liaison-workspace"},
    "adapters/vault-markdown": {"liaison-workspace-session-local"},
    "adapters/workspace-session-local": {"liaison-vault-markdown"},
}
DEPENDENCY_SECTIONS = ("dependencies", "dev-dependencies", "build-dependencies")


def effective_dependency_names(
    manifest: dict, workspace_dependencies: dict | None = None
) -> set[str]:
    """Return package names across direct, aliased, and target dependency tables."""

    names: set[str] = set()
    workspace_dependencies = workspace_dependencies or {}

    def collect(table: dict) -> None:
        for alias, specification in table.items():
            if isinstance(specification, dict):
                package = specification.get("package")
                if package is None and specification.get("workspace") is True:
                    inherited = workspace_dependencies.get(alias, {})
                    if isinstance(inherited, dict):
                        package = inherited.get("package")
                names.add(str(package or alias))
            else:
                names.add(alias)

    for section in DEPENDENCY_SECTIONS:
        table = manifest.get(section, {})
        if isinstance(table, dict):
            collect(table)
    targets = manifest.get("target", {})
    if isinstance(targets, dict):
        for target in targets.values():
            if not isinstance(target, dict):
                continue
            for section in DEPENDENCY_SECTIONS:
                table = target.get(section, {})
                if isinstance(table, dict):
                    collect(table)
    return names


def main() -> int:
    errors: list[str] = []
    try:
        root_manifest = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as error:
        print(f"Architecture check failed: cannot parse root Cargo.toml: {error}", file=sys.stderr)
        return 1
    workspace_dependencies = root_manifest.get("workspace", {}).get("dependencies", {})

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
        if cargo == ROOT / "Cargo.toml" or "target" in cargo.parts or ".claude" in cargo.parts:
            continue
        text = cargo.read_text(encoding="utf-8")
        if "[lints]\nworkspace = true" not in text:
            errors.append(
                f"{cargo.relative_to(ROOT)}: crate does not inherit workspace lints"
            )

    for crate, forbidden_dependencies in FORBIDDEN_CRATE_DEPENDENCIES.items():
        cargo = ROOT / crate / "Cargo.toml"
        if not cargo.exists():
            errors.append(f"{crate}: missing Cargo.toml for dependency-boundary check")
            continue
        text = cargo.read_text(encoding="utf-8")
        try:
            dependencies = effective_dependency_names(
                tomllib.loads(text), workspace_dependencies
            )
        except tomllib.TOMLDecodeError as error:
            errors.append(f"{crate}: cannot parse Cargo.toml for boundary check: {error}")
            continue
        for dependency in sorted(forbidden_dependencies):
            if dependency in dependencies:
                errors.append(
                    f"{crate}: forbidden dependency on {dependency} crosses the authority boundary"
                )

    alias_probe = tomllib.loads(
        '[target."cfg(unix)".dev-dependencies]\n'
        'renamed = { workspace = true }\n'
    )
    workspace_alias_probe = tomllib.loads(
        '[workspace.dependencies]\n'
        'renamed = { package = "liaison-vault-markdown", path = "example" }\n'
    )["workspace"]["dependencies"]
    if "liaison-vault-markdown" not in effective_dependency_names(
        alias_probe, workspace_alias_probe
    ):
        errors.append(
            "architecture checker failed its inherited aliased target-dependency probe"
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
