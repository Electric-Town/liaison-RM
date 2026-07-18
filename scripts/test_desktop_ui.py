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
  let commandNumber = 1;
  let initialiseAttempts = 0;
  let validationNeedsAttention = false;
  const nextCommandId = () => `01900000-0000-7000-8000-${String(commandNumber++).padStart(12, "0")}`;
  const result = (value) => ({
    contract_version: 1,
    command_id: nextCommandId(),
    completed_at: "2026-07-18T12:00:00Z",
    value,
  });
  const applicationError = (code, message, recovery, details = {}) => ({
    contract_version: 1,
    code,
    message,
    recovery,
    details,
    correlation_id: nextCommandId(),
  });
  const validation = () => ({
    contract_version: 1,
    workspace_id: workspace?.workspace.workspace_id || "01900000-0000-7000-8000-000000000001",
    schema_version: 1,
    valid: !validationNeedsAttention,
    findings: validationNeedsAttention ? [{
      contract_version: 1,
      code: "people.invalid-record",
      severity: "error",
      path: "people/invalid.md",
      message: "person record format or schema is invalid",
      recovery: "repair or restore the file, then run validation again",
    }] : [],
  });
  const requireSession = (sessionId) => {
    if (!workspace || sessionId !== workspace.workspace.session_id) {
      throw applicationError(
        "application.workspace-session-not-found",
        "the workspace session is not open",
        "open the workspace again and retry the operation",
        { private_diagnostic: "must not be rendered" },
      );
    }
  };
  window.__TAURI__ = {
    core: {
      invoke: async (command, payload = {}) => {
        switch (command) {
          case "app_status":
            return result({
              version: "0.1.0-alpha.1",
              product_state: "local-authoritative review build",
              authority_model: "canonical records stay in the selected local workspace",
              connection_state: "no connection configured",
              release_evidence: "not yet release-proven",
              canonical_storage: "Markdown/YAML and documented JSONL",
            });
          case "default_workspace_path":
            return result("/Users/tester/Documents/Liaison RM");
          case "initialise_workspace":
            initialiseAttempts += 1;
            if (initialiseAttempts === 1) {
              throw applicationError(
                "workspace.storage-error",
                "the workspace storage operation failed",
                "choose a verified-empty folder and retry",
              );
            }
            validationNeedsAttention = false;
            workspace = {
              workspace: {
                session_id: "01900000-0000-7000-8000-000000000101",
                path: payload.request.path,
                workspace_id: "01900000-0000-7000-8000-000000000001",
                schema_version: 1,
                name: payload.request.name,
                profile: payload.request.profile,
                build_profile: "connected-local",
                locale: "en-IE",
              },
              people: people.slice(),
              validation: null,
            };
            workspace.validation = validation();
            return result(workspace);
          case "open_workspace":
            if (!workspace) {
              throw applicationError(
                "workspace.not-found",
                "workspace does not exist",
                "choose an existing Liaison workspace or initialise a new one",
                { private_diagnostic: "must not be rendered" },
              );
            }
            validationNeedsAttention = payload.path.includes("needs-attention");
            workspace.workspace.path = payload.path;
            workspace.people = people.slice();
            workspace.validation = validation();
            return result(workspace);
          case "list_people": {
            requireSession(payload.request.sessionId);
            return result(people.slice());
          }
          case "create_person": {
            requireSession(payload.request.sessionId);
            const person = {
              id: `01900000-0000-7000-8000-${String(people.length + 2).padStart(12, "0")}`,
              revision: 1,
              display_name: payload.request.displayName,
              aliases: [],
              emails: payload.request.email ? [{ value: payload.request.email, label: "primary" }] : [],
              phones: [],
              birthday: null,
              archived: false,
            };
            people.push(person);
            return result(person);
          }
          case "validate_workspace":
            requireSession(payload.request.sessionId);
            return result(validation());
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
    authority = page.get_by_label("Storage status").inner_text()
    assert "local-authoritative review build" in authority
    assert "no connection configured" in authority
    assert "not yet release-proven" in page.locator("#live-status").inner_text()

    page.evaluate(
        """
        window.__liaisonOriginalInvoke = window.__TAURI__.core.invoke;
        window.__TAURI__.core.invoke = async (command, payload) => {
          const response = await window.__liaisonOriginalInvoke(command, payload);
          return command === "default_workspace_path" ? { ...response, contract_version: 2 } : response;
        };
        void 0;
        """
    )
    page.get_by_role("button", name="Use Documents").click()
    page.locator("#live-status").get_by_text("unexpected result", exact=False).wait_for()
    page.evaluate("window.__TAURI__.core.invoke = window.__liaisonOriginalInvoke; void 0;")

    page.get_by_role("button", name="Open existing workspace").click()
    page.locator("#live-status").get_by_text("workspace does not exist", exact=False).wait_for()
    error_status = page.locator("#live-status").inner_text()
    assert "Recovery: choose an existing Liaison workspace" in error_status
    assert "private_diagnostic" not in error_status
    assert page.get_by_role("button", name="Open existing workspace").is_enabled()

    page.get_by_label("Workspace name").fill("Family relationships")
    page.get_by_label("Workspace profile").select_option("family")
    page.get_by_role("button", name="Create local workspace").click()
    page.locator("#live-status").get_by_text("Workspace setup did not complete", exact=False).wait_for()
    assert page.get_by_role("button", name="Create local workspace").is_enabled()
    page.get_by_role("button", name="Create local workspace").click()
    page.get_by_role("heading", name="Remember useful context without scoring people").wait_for()
    assert page.get_by_role("heading", name="Remember useful context without scoring people").evaluate("el => el === document.activeElement")
    assert page.locator("#people-workspace-warning").is_hidden()

    page.get_by_label("Display name").fill("Alex Murphy")
    page.get_by_label("Email optional").fill("alex@example.test")
    page.get_by_label("Email optional").press("Enter")
    page.get_by_text("Alex Murphy", exact=True).wait_for()
    assert page.get_by_text("alex@example.test", exact=True).count() == 1
    assert page.get_by_text("Revision 1", exact=True).count() == 1
    assert "Saved Alex Murphy" in page.locator("#live-status").inner_text()

    page.get_by_role("button", name="Workspace").click()
    page.get_by_label("Absolute folder path").fill("/Users/tester/Documents/needs-attention")
    page.get_by_role("button", name="Open existing workspace").click()
    page.locator("#live-status").get_by_text("review Health before editing", exact=False).wait_for()
    assert page.get_by_role("button", name="Open existing workspace").is_enabled()

    page.get_by_role("button", name="Health").click()
    page.get_by_role("heading", name="Validate the open-file workspace").wait_for()
    page.get_by_role("button", name="Run validation").click()
    page.get_by_text("Workspace needs attention", exact=True).wait_for()
    assert "people.invalid-record" in page.locator("#validation-findings").inner_text()
    assert "Recovery:" in page.locator("#validation-findings").inner_text()

    SCREENSHOTS.mkdir(parents=True, exist_ok=True)
    page.screenshot(path=str(SCREENSHOTS / "desktop-workspace-health.png"), full_page=True)
    assert external_requests == []


def test_mobile(browser, external_requests: list[str]) -> None:
    page = browser.new_page(viewport={"width": 390, "height": 844}, reduced_motion="reduce")
    page.set_default_timeout(8_000)
    page.on(
        "request",
        lambda request: external_requests.append(request.url)
        if not request.url.startswith("file://")
        else None,
    )
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


def test_dark_mode(browser, external_requests: list[str]) -> None:
    page = browser.new_page(viewport={"width": 1280, "height": 900}, color_scheme="dark")
    page.set_default_timeout(8_000)
    page.on(
        "request",
        lambda request: external_requests.append(request.url)
        if not request.url.startswith("file://")
        else None,
    )
    load_page(page)
    page.get_by_role("heading", name="Choose where Liaison keeps your files").wait_for()
    styles = page.locator(".primary-button").first.evaluate(
        "element => ({ background: getComputedStyle(element).backgroundColor, color: getComputedStyle(element).color })"
    )
    assert styles == {"background": "rgb(49, 87, 200)", "color": "rgb(255, 255, 255)"}
    sidebar = page.locator(".sidebar").evaluate(
        "element => ({ background: getComputedStyle(element).backgroundColor, color: getComputedStyle(element).color })"
    )
    assert sidebar == {"background": "rgb(17, 25, 42)", "color": "rgb(255, 255, 255)"}
    SCREENSHOTS.mkdir(parents=True, exist_ok=True)
    page.screenshot(path=str(SCREENSHOTS / "dark-workspace.png"), full_page=True)
    page.close()


def main() -> int:
    results = {
        "desktop_workflow": False,
        "mobile_reflow": False,
        "dark_mode": False,
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
        test_mobile(browser, external_requests)
        results["mobile_reflow"] = True
        test_dark_mode(browser, external_requests)
        results["dark_mode"] = True
        assert external_requests == []
        page.close()
        browser.close()

    REPORT.parent.mkdir(parents=True, exist_ok=True)
    REPORT.write_text(json.dumps(results, indent=2) + "\n", encoding="utf-8")
    print("Desktop UI tests passed: workspace, person, validation, focus, mobile reflow, dark mode, and zero external requests")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
