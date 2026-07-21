#!/usr/bin/env python3
"""Validate the P04 Workspace, Directory, and Health parity slice."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
UI = ROOT / "apps/desktop/ui-react/src"
ROUTES = UI / "routes"

REQUIRED_ROUTE_FILES = (
    "OverviewRoute.tsx",
    "DirectoryRoute.tsx",
    "EventsRoute.tsx",
    "HealthRoute.tsx",
    "SettingsRoute.tsx",
)
REQUIRED_COMMANDS = (
    "initialise_workspace",
    "open_workspace",
    "close_workspace",
    "list_people",
    "create_person",
    "validate_workspace",
    "recover_workspace",
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
FORBIDDEN_DERIVATIONS = (
    "relationshipStrength",
    "relationship_strength",
    "employeeScore",
    "employee_score",
    "socialCredit",
    "social_credit",
)


def fail(message: str) -> None:
    print(f"parity-routes: {message}")
    raise SystemExit(1)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing required file: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def main() -> int:
    sources: list[str] = []
    for filename in REQUIRED_ROUTE_FILES:
        source = read(ROUTES / filename)
        sources.append(source)
        if 'id="route-heading"' not in source or "tabIndex={-1}" not in source:
            fail(f"route does not expose a focusable stable heading: {filename}")

    contract = read(UI / "application-contract.ts")
    controller = read(UI / "useDesktopController.ts")
    app = read(UI / "App.tsx")
    route_css = read(ROUTES / "routes.css")
    sources.extend((contract, controller, app, route_css))
    combined = "\n".join(sources)

    for command in REQUIRED_COMMANDS:
        if f'"{command}"' not in contract:
            fail(f"typed application contract is missing command: {command}")
        if f'transport.invoke("{command}"' not in controller:
            fail(f"workflow controller does not invoke typed command: {command}")

    for token in FORBIDDEN_AUTHORITY:
        if token in combined:
            fail(f"browser or network authority introduced: {token}")
    for token in FORBIDDEN_DERIVATIONS:
        if token in combined:
            fail(f"forbidden human-value derivation introduced: {token}")

    if 'document.getElementById("route-heading")?.focus()' not in app:
        fail("route changes do not move focus to the current heading")
    if "useDesktopController(transport)" not in app:
        fail("App does not use the typed workflow controller")

    directory = read(ROUTES / "DirectoryRoute.tsx")
    for fragment in (
        'type="search"',
        'aria-pressed={person.personId === selectedId}',
        'controller.createPerson',
        'controller.refreshPeople',
    ):
        if fragment not in directory:
            fail(f"Directory parity is missing: {fragment}")

    settings = read(ROUTES / "SettingsRoute.tsx")
    for fragment in (
        "controller.openWorkspace",
        "controller.initialiseWorkspace",
        "controller.closeWorkspace",
        'value="airgap"',
        'value="connected-local"',
    ):
        if fragment not in settings:
            fail(f"Workspace parity is missing: {fragment}")

    health = read(ROUTES / "HealthRoute.tsx")
    for fragment in (
        "controller.validateWorkspace",
        "controller.recoverWorkspace",
        "finding.recovery",
        "finding.code",
        "finding.path",
    ):
        if fragment not in health:
            fail(f"Health parity is missing: {fragment}")

    events = read(ROUTES / "EventsRoute.tsx")
    if "remain compiled out" not in events or "EVENT_STAGES.map" not in events:
        fail("Events route does not preserve honest compiled-out stage semantics")

    if re.search(r"#[0-9a-fA-F]{3,8}\b|\brgba?\(|\bhsla?\(", route_css):
        fail("route CSS contains raw colors instead of semantic tokens")
    if "grid-template-columns: minmax(0, 1fr)" not in route_css:
        fail("narrow route reflow is missing")
    if "@media (forced-colors: active)" not in route_css:
        fail("forced-colours route behavior is missing")

    print(
        "parity-routes: passed — typed local Workspace, Person, Health, recovery, "
        "focus, narrow reflow, and compiled-out Events boundaries"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
