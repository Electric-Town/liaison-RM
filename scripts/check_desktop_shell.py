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


def main() -> int:
    errors: list[str] = []

    root_cargo = tomllib.loads(load_text(ROOT / "Cargo.toml", errors) or "{}")
    members = root_cargo.get("workspace", {}).get("members", [])
    if "apps/desktop/src-tauri" not in members:
        errors.append("Cargo workspace does not include apps/desktop/src-tauri")

    desktop_cargo = tomllib.loads(load_text(TAURI / "Cargo.toml", errors) or "{}")
    dependencies = desktop_cargo.get("dependencies", {})
    for required in ["liaison-workspace", "liaison-people", "liaison-vault-markdown", "tauri"]:
        if required not in dependencies:
            errors.append(f"desktop crate is missing dependency {required}")
    for forbidden in ["reqwest", "ureq", "hyper", "rusqlite", "sqlx"]:
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

    javascript = load_text(UI / "app.js", errors)
    for forbidden in ["fetch(", "XMLHttpRequest", "WebSocket", "EventSource", "sendBeacon", "localStorage", "indexedDB"]:
        if forbidden in javascript:
            errors.append(f"desktop JavaScript contains forbidden authority token {forbidden!r}")
    if "window.__TAURI__?.core?.invoke" not in javascript:
        errors.append("desktop JavaScript does not use the Tauri command bridge")
    if "textContent" not in javascript or "innerHTML =" not in javascript:
        errors.append("desktop JavaScript must use safe text rendering and a fixed static template")

    for icon in ["32x32.png", "128x128.png", "128x128@2x.png", "icon.icns"]:
        if not (TAURI / "icons" / icon).is_file():
            errors.append(f"desktop icon is missing: {icon}")

    mac_workflow = load_text(ROOT / ".github" / "workflows" / "macos-desktop.yml", errors)
    for required in [
        "universal-apple-darwin",
        "aarch64-apple-darwin",
        "x86_64-apple-darwin",
        "codesign --verify",
        "hdiutil verify",
        "actions/upload-artifact@v4",
    ]:
        if required not in mac_workflow:
            errors.append(f"macOS workflow is missing {required!r}")

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

    print("Desktop shell check passed: local authority, accessibility, and Mac gates are present")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
