#!/usr/bin/env python3
"""Check the desktop shell's local-authority and packaging boundaries."""

from __future__ import annotations

import json
import re
import sys
import tomllib
from html.parser import HTMLParser
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DESKTOP = ROOT / "apps" / "desktop"
TAURI = DESKTOP / "src-tauri"
UI = DESKTOP / "ui"


class InterfaceParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self.ids: list[str] = []
        self.labels: list[str] = []
        self.inputs: list[str] = []
        self.route_controls: list[str] = []
        self.landmarks: set[str] = set()
        self.inline_handlers: list[str] = []
        self.external_urls: list[str] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        values = dict(attrs)
        if element_id := values.get("id"):
            self.ids.append(element_id)
        if tag == "label" and (target := values.get("for")):
            self.labels.append(target)
        if tag in {"input", "select", "textarea"} and (element_id := values.get("id")):
            self.inputs.append(element_id)
        if route := values.get("data-route"):
            self.route_controls.append(route)
        if tag in {"main", "nav", "header", "footer", "aside"}:
            self.landmarks.add(tag)
        for name, value in attrs:
            if name.startswith("on"):
                self.inline_handlers.append(name)
            if name in {"src", "href"} and value and re.match(r"https?://", value):
                self.external_urls.append(value)


def load_text(path: Path, errors: list[str]) -> str:
    if not path.is_file():
        errors.append(f"missing required file: {path.relative_to(ROOT)}")
        return ""
    return path.read_text(encoding="utf-8")


def relative_luminance(hex_color: str) -> float:
    channels = [int(hex_color[index : index + 2], 16) / 255 for index in (1, 3, 5)]
    linear = [
        channel / 12.92 if channel <= 0.04045 else ((channel + 0.055) / 1.055) ** 2.4
        for channel in channels
    ]
    return 0.2126 * linear[0] + 0.7152 * linear[1] + 0.0722 * linear[2]


def contrast_ratio(first: str, second: str) -> float:
    lighter, darker = sorted(
        [relative_luminance(first), relative_luminance(second)], reverse=True
    )
    return (lighter + 0.05) / (darker + 0.05)


def check_text_contrast(css: str, errors: list[str]) -> None:
    blocks = re.findall(r"(?:^|\n)\s*:root\s*\{([^}]+)\}", css, flags=re.DOTALL)
    if len(blocks) != 2:
        errors.append("desktop CSS must define one base and one dark-mode :root token block")
        return

    def variables(block: str) -> dict[str, str]:
        return {
            name: value.lower()
            for name, value in re.findall(r"--([a-z0-9-]+):\s*(#[0-9a-fA-F]{6})\s*;", block)
        }

    light = variables(blocks[0])
    dark = light | variables(blocks[1])
    pairs = [
        ("action-content", "action", "action button"),
        ("action-content", "action-hover", "action button hover"),
        ("accent-text", "accent-soft", "accent label"),
        ("sidebar-text", "sidebar-background", "sidebar navigation"),
        ("health-valid-text", "success-soft", "valid Health status"),
        ("health-error-text", "danger-soft", "error Health status"),
        ("muted", "surface", "muted text on cards"),
        ("muted", "canvas", "muted text on pages"),
    ]
    for mode, tokens in [("light", light), ("dark", dark)]:
        tokens = tokens | {"literal-white": "#ffffff"}
        for foreground, background, label in pairs:
            if foreground not in tokens or background not in tokens:
                errors.append(f"{mode} contrast check is missing tokens for {label}")
                continue
            ratio = contrast_ratio(tokens[foreground], tokens[background])
            if ratio < 4.5:
                errors.append(
                    f"{mode} {label} contrast is {ratio:.2f}:1; text requires at least 4.5:1"
                )


def check_events_navigation_boundary(
    route_controls: list[str], p11_status: str, errors: list[str]
) -> None:
    """Keep Events out of the DOM until its complete workflow is evidenced."""

    if p11_status == "complete":
        return
    if "events" in route_controls:
        errors.append(
            "desktop HTML includes an Events destination while T-B0-P11 is "
            f"{p11_status!r}; keep it absent until the complete workflow is evidenced"
        )


def self_test_events_navigation_boundary(errors: list[str]) -> None:
    """Prove that the policy rejects the broken premature-navigation shape."""

    visible_parser = InterfaceParser()
    visible_parser.feed(
        '<nav><button type="button" data-route="events">Events</button></nav>'
    )
    visible_errors: list[str] = []
    check_events_navigation_boundary(
        visible_parser.route_controls, "blocked", visible_errors
    )
    if not visible_errors:
        errors.append(
            "Events navigation boundary self-test did not reject a visible blocked route"
        )

    unrelated_parser = InterfaceParser()
    unrelated_parser.feed('<button type="button" data-route="people">People</button>')
    unrelated_errors: list[str] = []
    check_events_navigation_boundary(
        unrelated_parser.route_controls, "blocked", unrelated_errors
    )
    if unrelated_errors:
        errors.append(
            "Events navigation boundary self-test rejected an unrelated route"
        )

    complete_errors: list[str] = []
    check_events_navigation_boundary(["events"], "complete", complete_errors)
    if complete_errors:
        errors.append("Events navigation boundary self-test rejected a completed P11 route")


def main() -> int:
    errors: list[str] = []

    self_test_events_navigation_boundary(errors)

    root_cargo = tomllib.loads(load_text(ROOT / "Cargo.toml", errors) or "{}")
    members = root_cargo.get("workspace", {}).get("members", [])
    if "apps/desktop/src-tauri" not in members:
        errors.append("Cargo workspace does not include apps/desktop/src-tauri")

    desktop_cargo = tomllib.loads(load_text(TAURI / "Cargo.toml", errors) or "{}")
    dependencies = desktop_cargo.get("dependencies", {})
    for required in ["liaison-application", "tauri"]:
        if required not in dependencies:
            errors.append(f"desktop crate is missing dependency {required}")
    for forbidden in [
        "liaison-workspace",
        "liaison-people",
        "liaison-vault-markdown",
        "liaison-shared-kernel",
        "reqwest",
        "ureq",
        "hyper",
        "rusqlite",
        "sqlx",
    ]:
        if forbidden in dependencies:
            errors.append(f"desktop crate includes forbidden dependency {forbidden}")

    config_text = load_text(TAURI / "tauri.conf.json", errors)
    try:
        config = json.loads(config_text)
    except json.JSONDecodeError as error:
        errors.append(f"tauri.conf.json is invalid JSON: {error}")
        config = {}
    if config.get("identifier") != "io.github.electric-town.liaison-rm":
        errors.append("Tauri identifier is not the stable Electric Town identifier")
    if config.get("build", {}).get("frontendDist") != "../ui":
        errors.append("Tauri frontendDist must point to the committed local UI")
    if config.get("app", {}).get("withGlobalTauri") is not True:
        errors.append("Tauri global bridge must be explicitly enabled for the static UI")
    csp = config.get("app", {}).get("security", {}).get("csp", "")
    if "default-src 'self'" not in csp or "object-src 'none'" not in csp:
        errors.append("Tauri CSP is missing local-only default and object restrictions")
    for unsafe_destination in ["https:", "http:", "ws:", "wss:", "*"]:
        sanitized = csp.replace("http://ipc.localhost", "").replace("http://asset.localhost", "")
        if unsafe_destination in sanitized:
            errors.append(f"Tauri CSP permits undeclared destination {unsafe_destination}")
    targets = set(config.get("bundle", {}).get("targets", []))
    if not {"app", "dmg"}.issubset(targets):
        errors.append("Tauri bundle targets must include app and dmg")

    rust = load_text(TAURI / "src" / "lib.rs", errors)
    for forbidden in ["reqwest", "ureq", "WebSocket", "telemetry", "analytics", "http://", "https://"]:
        if forbidden in rust:
            errors.append(f"desktop Rust source contains forbidden network/telemetry token {forbidden!r}")
    for forbidden in [
        "MarkdownVault",
        "InitialiseWorkspace::new",
        "ValidateWorkspace::new",
        "CreatePerson::new",
        "ListPeople::new",
        "Result<WorkspaceView, String>",
        "map_err(|error| error.to_string())",
    ]:
        if forbidden in rust:
            errors.append(f"desktop Rust source bypasses the application boundary with {forbidden!r}")
    for required in [
        "State<'_, LiaisonApplication>",
        ".manage(LiaisonApplication::new())",
        "CommandResult<",
        "ApplicationError",
        "WorkspaceSessionId",
    ]:
        if required not in rust:
            errors.append(f"desktop Rust source is missing application-boundary token {required!r}")
    for command in [
        "app_status",
        "default_workspace_path",
        "initialise_workspace",
        "open_workspace",
        "list_people",
        "create_person",
        "validate_workspace",
    ]:
        if command not in rust:
            errors.append(f"desktop Rust source is missing command {command}")

    html = load_text(UI / "index.html", errors)
    parser = InterfaceParser()
    parser.feed(html)
    duplicates = sorted({value for value in parser.ids if parser.ids.count(value) > 1})
    if duplicates:
        errors.append(f"desktop HTML contains duplicate IDs: {duplicates}")
    unlabeled = sorted(set(parser.inputs) - set(parser.labels))
    if unlabeled:
        errors.append(f"desktop controls are missing explicit labels: {unlabeled}")
    missing_landmarks = sorted({"main", "nav", "header", "footer"} - parser.landmarks)
    if missing_landmarks:
        errors.append(f"desktop HTML is missing landmarks: {missing_landmarks}")
    if parser.inline_handlers:
        errors.append(f"desktop HTML contains inline event handlers: {parser.inline_handlers}")
    if parser.external_urls:
        errors.append(f"desktop HTML loads external URLs: {parser.external_urls}")
    if 'href="#main-content"' not in html:
        errors.append("desktop HTML is missing skip navigation")
    if 'role="status" aria-live="polite"' not in html:
        errors.append("desktop HTML is missing a polite live status region")

    traceability_text = load_text(ROOT / "spec" / "traceability-ownership.json", errors)
    try:
        traceability = json.loads(traceability_text)
    except json.JSONDecodeError as error:
        errors.append(f"traceability-ownership.json is invalid JSON: {error}")
        traceability = {}
    p11_status = (
        traceability.get("task_ownership", {})
        .get("T-B0-P11", {})
        .get("status")
    )
    if not isinstance(p11_status, str):
        errors.append("traceability is missing the T-B0-P11 delivery status")
    else:
        check_events_navigation_boundary(parser.route_controls, p11_status, errors)

    javascript = load_text(UI / "app.js", errors)
    for forbidden in ["fetch(", "XMLHttpRequest", "WebSocket", "EventSource", "sendBeacon", "localStorage", "indexedDB"]:
        if forbidden in javascript:
            errors.append(f"desktop JavaScript contains forbidden authority token {forbidden!r}")
    if "window.__TAURI__?.core?.invoke" not in javascript:
        errors.append("desktop JavaScript does not use the Tauri command bridge")
    if "textContent" not in javascript or "replaceChildren" not in javascript:
        errors.append("desktop JavaScript must use DOM text rendering")
    for unsafe_sink in ["innerHTML", "outerHTML", "insertAdjacentHTML", "document.write"]:
        if unsafe_sink in javascript:
            errors.append(f"desktop JavaScript contains unsafe HTML sink {unsafe_sink!r}")
    for required in [
        "commandValue",
        "APPLICATION_CONTRACT_VERSION",
        "sessionId: operation.sessionId",
        "operationOwnsCurrentSession(operation)",
        "app.product_state",
        "app.connection_state",
        "app.release_evidence",
        "Recovery:",
    ]:
        if required not in javascript:
            errors.append(f"desktop JavaScript is missing bridge-parity token {required!r}")
    for forbidden in ["workspacePath:", "error.details", "console.log", "console.error"]:
        if forbidden in javascript:
            errors.append(f"desktop JavaScript leaks an obsolete or private boundary token {forbidden!r}")

    css = load_text(UI / "styles.css", errors)
    check_text_contrast(css, errors)

    for icon in ["32x32.png", "128x128.png", "128x128@2x.png", "icon.icns", "icon.ico"]:
        if not (TAURI / "icons" / icon).is_file():
            errors.append(f"desktop icon is missing: {icon}")

    mac_workflow = load_text(ROOT / ".github" / "workflows" / "macos-desktop.yml", errors)
    for required in [
        "universal-apple-darwin",
        "aarch64-apple-darwin",
        "x86_64-apple-darwin",
        "codesign --verify",
        "hdiutil verify",
        "cd artifacts/macos",
        "shasum -a 256 -c SHA256SUMS",
        "actions/upload-artifact@v4",
    ]:
        if required not in mac_workflow:
            errors.append(f"macOS workflow is missing {required!r}")

    windows_workflow = load_text(ROOT / ".github" / "workflows" / "windows-desktop.yml", errors)
    for required in [
        "cargo tauri build --bundles nsis",
        "cd artifacts/windows",
        "sha256sum -c SHA256SUMS",
        "actions/upload-artifact@v4",
    ]:
        if required not in windows_workflow:
            errors.append(f"Windows workflow is missing {required!r}")

    release_workflow = load_text(ROOT / ".github" / "workflows" / "macos-release.yml", errors)
    for required in [
        "APPLE_CERTIFICATE",
        "APPLE_SIGNING_IDENTITY",
        "APPLE_ID",
        "APPLE_PASSWORD",
        "APPLE_TEAM_ID",
        "stapler validate",
        "spctl --assess",
    ]:
        if required not in release_workflow:
            errors.append(f"macOS release workflow is missing {required!r}")

    if errors:
        print("Desktop shell check failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print("Desktop shell check passed: local authority, structural accessibility, contrast, and Mac gates are present")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
