#!/usr/bin/env python3
"""Validate the P04 typed frontend boundary without executing product code."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
UI = ROOT / "apps/desktop/ui-react"
PACKAGE = UI / "package.json"
CONTRACT = UI / "src/application-contract.ts"
TRANSPORT = UI / "src/transport.ts"
APP = UI / "src/App.tsx"
README = UI / "README.md"
DESIGN = ROOT / "DESIGN.md"

REQUIRED_ROUTES = ("overview", "directory", "events", "health", "settings")
REQUIRED_STAGES = ("details", "cohort", "attendees", "readiness", "brief")
REQUIRED_PHASES = (
    "idle",
    "validating",
    "staging",
    "commit-decided",
    "publishing",
    "recovering",
    "complete",
    "conflict",
    "failed",
)
FORBIDDEN_AUTHORITY = (
    "localStorage",
    "sessionStorage",
    "indexedDB",
    "fetch(",
    "XMLHttpRequest",
    "WebSocket",
    "EventSource",
    "sendBeacon",
)


def fail(message: str) -> None:
    print(f"frontend-contract: {message}")
    raise SystemExit(1)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing required file: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def quoted_values(source: str, constant: str) -> tuple[str, ...]:
    match = re.search(
        rf"export const {re.escape(constant)} = \[(?P<body>.*?)\] as const;",
        source,
        flags=re.DOTALL,
    )
    if match is None:
        fail(f"missing {constant} tuple")
    return tuple(re.findall(r'"([a-z0-9-]+)"', match.group("body")))


def main() -> int:
    package = json.loads(read(PACKAGE))
    contract = read(CONTRACT)
    transport = read(TRANSPORT)
    app = read(APP)
    readme = read(README)
    design = read(DESIGN)

    if package.get("private") is not True:
        fail("frontend package must remain private")
    if package.get("engines", {}).get("node") != ">=22.0.0":
        fail("Node 22 floor is not pinned")
    for command in ("typecheck", "test", "build", "check"):
        if command not in package.get("scripts", {}):
            fail(f"missing npm script: {command}")
    for group in ("dependencies", "devDependencies"):
        for name, version in package.get(group, {}).items():
            if not re.fullmatch(r"\d+\.\d+\.\d+", version):
                fail(f"{name} is not pinned to an exact semantic version: {version}")

    if quoted_values(contract, "B0_ROUTES") != REQUIRED_ROUTES:
        fail("B0 route identities drifted from DESIGN.md")
    if quoted_values(contract, "EVENT_STAGES") != REQUIRED_STAGES:
        fail("Events stage identities drifted from DESIGN.md")
    if quoted_values(contract, "OPERATION_PHASES") != REQUIRED_PHASES:
        fail("P03 operation phases drifted from the typed presentation contract")

    combined = "\n".join((contract, transport, app))
    for token in FORBIDDEN_AUTHORITY:
        if token in combined:
            fail(f"forbidden browser or network authority introduced: {token}")

    if 'liaison/application-response@1' not in contract:
        fail("versioned application response envelope is missing")
    if "CommandTransport" not in contract or "createTauriTransport" not in transport:
        fail("typed injected transport seam is missing")
    if "ui-react-dist" not in readme or "legacy shell remains active" not in readme.lower():
        fail("parallel migration and rollback boundary is not documented")

    for label in ("Overview", "Directory", "Events", "Health", "Settings"):
        if label not in design:
            fail(f"DESIGN.md does not contain stable route label: {label}")

    print(
        "frontend-contract: passed — private pinned package, typed response boundary, "
        "stable routes/stages/phases, no browser or network authority"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
