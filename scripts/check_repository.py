#!/usr/bin/env python3
"""Run repository-level policy checks that do not require a compiled application."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

REQUIRED_FILES = [
    "README.md",
    "AGENTS.md",
    "CONTRIBUTING.md",
    "GOVERNANCE.md",
    "SECURITY.md",
    "CODE_OF_CONDUCT.md",
    "CHANGELOG.md",
    ".github/pull_request_template.md",
    "docs/standards/domain-driven-design.md",
    "docs/standards/knowledge-centered-work.md",
    "docs/standards/ux-review.md",
    "docs/standards/content-quality.md",
]

VAGUE_TERMS = {
    "seamless",
    "robust",
    "powerful",
    "intuitive",
    "future-proof",
    "enterprise-ready",
    "best-in-class",
    "revolutionary",
}

EXCLUDED_DIRS = {".git", "target", "node_modules", "dist", "vendor"}


def text_files() -> list[Path]:
    paths: list[Path] = []
    for path in ROOT.rglob("*"):
        if not path.is_file() or any(part in EXCLUDED_DIRS for part in path.parts):
            continue
        if path.suffix.lower() in {".md", ".txt", ".rs", ".py", ".toml", ".yaml", ".yml", ".json", ".wit"}:
            paths.append(path)
    return paths


def check_required_files(errors: list[str]) -> None:
    for relative in REQUIRED_FILES:
        if not (ROOT / relative).is_file():
            errors.append(f"missing required file: {relative}")


def check_markdown_links(errors: list[str]) -> None:
    link_pattern = re.compile(r"\[[^\]]+\]\(([^)]+)\)")
    for path in ROOT.rglob("*.md"):
        if any(part in EXCLUDED_DIRS for part in path.parts):
            continue
        text = path.read_text(encoding="utf-8")
        for target in link_pattern.findall(text):
            if target.startswith(("http://", "https://", "mailto:", "#")):
                continue
            target = target.split("#", 1)[0]
            if not target:
                continue
            resolved = (path.parent / target).resolve()
            try:
                resolved.relative_to(ROOT.resolve())
            except ValueError:
                errors.append(f"{path.relative_to(ROOT)}: link escapes repository: {target}")
                continue
            if not resolved.exists():
                errors.append(f"{path.relative_to(ROOT)}: broken link: {target}")


def check_content(errors: list[str]) -> None:
    duplicate_paragraphs: dict[str, Path] = {}
    for path in text_files():
        if path.suffix.lower() not in {".md", ".txt"}:
            continue
        text = path.read_text(encoding="utf-8")
        lower = text.lower()
        for term in VAGUE_TERMS:
            if term in lower and not path.name == "content-quality.md":
                errors.append(
                    f"{path.relative_to(ROOT)}: define or remove vague term '{term}'"
                )
        paragraphs = [
            re.sub(r"\s+", " ", item.strip()).lower()
            for item in re.split(r"\n\s*\n", text)
            if len(item.strip()) >= 180 and not item.lstrip().startswith("```")
        ]
        for paragraph in paragraphs:
            previous = duplicate_paragraphs.get(paragraph)
            if previous and previous != path:
                errors.append(
                    f"duplicate long paragraph in {previous.relative_to(ROOT)} and {path.relative_to(ROOT)}"
                )
            else:
                duplicate_paragraphs[paragraph] = path


def check_json(errors: list[str]) -> None:
    for path in ROOT.rglob("*.json"):
        if any(part in EXCLUDED_DIRS for part in path.parts):
            continue
        try:
            json.loads(path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as exc:
            errors.append(f"{path.relative_to(ROOT)}: invalid JSON: {exc}")


def main() -> int:
    errors: list[str] = []
    check_required_files(errors)
    check_markdown_links(errors)
    check_content(errors)
    check_json(errors)
    if errors:
        print("Repository policy check failed:")
        for error in sorted(set(errors)):
            print(f"- {error}")
        return 1
    print("Repository policy check passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
