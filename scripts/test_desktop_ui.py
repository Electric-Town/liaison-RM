#!/usr/bin/env python3
"""Browser acceptance tests for the static Tauri desktop interface."""

from __future__ import annotations

import json
import os
import shutil
from pathlib import Path

from playwright.sync_api import Page, sync_playwright

ROOT = Path(__file__).resolve().parents[1]
UI = ROOT / "apps" / "desktop" / "ui"
SCREENSHOTS = Path(os.environ.get("DESKTOP_SCREENSHOT_DIR", "artifacts/desktop-screens"))
REPORT = Path(os.environ.get("DESKTOP_TEST_REPORT", "artifacts/desktop-ui-report.json"))

BRIDGE = r"""
(() => {
  const people = [];
  let workspace = null;
  window.__TAURI__ = {
    core: {
      invoke: async (command, payload = {}) => {
        switch (command) {
          case "app_status":
            return {
              version: "0.1.0",
              local_authority: true,
              network_clients_compiled: false,
              canonical_storage: "Markdown/YAML and documented JSONL",
            };
          case "default_workspace_path":
            return "/Users/tester/Documents/Liaison RM";
          case "initialise_workspace":
            workspace = {
              path: payload.request.path,
              workspace_id: "01900000-0000-7000-8000-000000000001",
              name: payload.request.name,
              profile: payload.request.profile,
              build_profile: "airgap",
              locale: "en-IE",
              people_count: people.length,
            };
            return workspace;
          case "open_workspace":
            if (!workspace) throw new Error("workspace does not exist");
            return { ...workspace, path: payload.path, people_count: people.length };
          case "list_people":
            return people.slice();
          case "create_person": {
            const person = {
              id: `01900000-0000-7000-8000-${String(people.length + 2).padStart(12, "0")}`,
              display_name: payload.request.displayName,
              primary_email: payload.request.email,
              revision: payload.request.email ? 2 : 1,
            };
            people.push(person);
            return person;
          }
          case "validate_workspace":
            return { valid: true, schema_version: 1, finding_count: 0, findings: [] };
          default:
            throw new Error(`Unexpected command: ${command}`);
        }
      },
    },
  };
})();
"""


def load_page(page: Page) -> None:
    html = (UI / "index.html").read_text(encoding="utf-8")
    css = (UI / "styles.css").read_text(encoding="utf-8")
    javascript = (UI / "app.js").read_text(encoding="utf-8")
    html = html.replace('<link rel="stylesheet" href="styles.css">', f"<style>{css}</style>")
    html = html.replace('<script src="app.js" defer></script>', "")
    page.set_content(html, wait_until="load")
    page.evaluate(BRIDGE)
    page.add_script_tag(content=javascript)


def test_desktop(page: Page, external_requests: list[str]) -> None:
    page.on(
        "request",
        lambda request: external_requests.append(request.url)
        if not request.url.startswith("file://")
        else None,
    )
    load_page(page)

    page.get_by_role("heading", name="Choose where Liaison keeps your files").wait_for()
    assert page.locator("header").count() == 1
    assert page.locator("nav").count() == 1
    assert page.locator("main").count() == 1
    assert page.locator("footer").count() == 1
    assert page.get_by_role("link", name="Skip to main content").count() == 1
    assert page.get_by_label("Absolute folder path").input_value() == "/Users/tester/Documents/Liaison RM"
    assert "Local authority" in page.get_by_label("Storage status").inner_text()

    page.get_by_label("Workspace name").fill("Family relationships")
    page.get_by_label("Workspace profile").select_option("family")
    page.get_by_role("button", name="Create local workspace").click()
    page.get_by_role("heading", name="Remember useful context without scoring people").wait_for()
    assert page.get_by_role("heading", name="Remember useful context without scoring people").evaluate("el => el === document.activeElement")

    page.get_by_label("Display name").fill("Alex Murphy")
    page.get_by_label("Email optional").fill("alex@example.test")
    page.get_by_role("button", name="Save Markdown profile").click()
    page.get_by_text("Alex Murphy", exact=True).wait_for()
    assert page.get_by_text("alex@example.test", exact=True).count() == 1
    assert "Saved Alex Murphy" in page.locator("#live-status").inner_text()

    page.get_by_role("button", name="Health").click()
    page.get_by_role("heading", name="Validate the open-file workspace").wait_for()
    page.get_by_role("button", name="Run validation").click()
    page.get_by_text("Workspace is valid", exact=True).wait_for()
    assert "Schema 1" in page.locator("#validation-summary").inner_text()

    SCREENSHOTS.mkdir(parents=True, exist_ok=True)
    page.screenshot(path=str(SCREENSHOTS / "desktop-workspace-health.png"), full_page=True)
    assert external_requests == []


def test_mobile(browser) -> None:
    page = browser.new_page(viewport={"width": 390, "height": 844}, reduced_motion="reduce")
    page.set_default_timeout(8_000)
    load_page(page)
    page.get_by_role("button", name="People").click()
    page.get_by_role("heading", name="Remember useful context without scoring people").wait_for()
    overflow = page.evaluate("document.documentElement.scrollWidth > document.documentElement.clientWidth")
    assert overflow is False
    assert page.get_by_role("button", name="Workspace").is_visible()
    assert page.get_by_role("button", name="People").is_visible()
    assert page.get_by_role("button", name="Health").is_visible()
    SCREENSHOTS.mkdir(parents=True, exist_ok=True)
    page.screenshot(path=str(SCREENSHOTS / "mobile-people.png"), full_page=True)
    page.close()


def main() -> int:
    results = {
        "desktop_workflow": False,
        "mobile_reflow": False,
        "external_requests": [],
    }
    with sync_playwright() as playwright:
        executable = os.environ.get("CHROMIUM_PATH") or shutil.which("chromium")
        browser = playwright.chromium.launch(headless=True, executable_path=executable)
        page = browser.new_page(viewport={"width": 1280, "height": 900})
        page.set_default_timeout(8_000)
        external_requests: list[str] = []
        test_desktop(page, external_requests)
        results["desktop_workflow"] = True
        results["external_requests"] = external_requests
        test_mobile(browser)
        results["mobile_reflow"] = True
        page.close()
        browser.close()

    REPORT.parent.mkdir(parents=True, exist_ok=True)
    REPORT.write_text(json.dumps(results, indent=2) + "\n", encoding="utf-8")
    print("Desktop UI tests passed: workspace, person, validation, focus, mobile reflow, and zero external requests")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
