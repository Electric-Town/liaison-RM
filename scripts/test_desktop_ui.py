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
  const peopleByPath = new Map();
  let workspace = null;
  const sessions = new Map();
  const closedSessions = [];
  const delayedCommands = new Set();
  const delayResolvers = new Map();
  const commandCounts = new Map();
  let inFlightCommands = 0;
  let maxInFlightCommands = 0;
  let nextSessionNumber = 101;
  let failNextCloseFor = null;
  let failNextCloseCount = 0;
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
    if (!sessions.has(sessionId)) {
      throw applicationError(
        "application.workspace-session-not-found",
        "the workspace session is not open",
        "open the workspace again and retry the operation",
        { private_diagnostic: "must not be rendered" },
      );
    }
  };
  const sessionPeople = (sessionId) => {
    requireSession(sessionId);
    const path = sessions.get(sessionId).workspace.path;
    if (!peopleByPath.has(path)) peopleByPath.set(path, []);
    return peopleByPath.get(path);
  };
  const waitForConfiguredDelay = async (command) => {
    if (!delayedCommands.delete(command)) return;
    await new Promise((resolve) => delayResolvers.set(command, resolve));
  };
  const openResult = (path, name, profile) => {
    if (!peopleByPath.has(path)) peopleByPath.set(path, []);
    const sessionId = `01900000-0000-7000-8000-${String(nextSessionNumber++).padStart(12, "0")}`;
    const opened = {
      workspace: {
        session_id: sessionId,
        path,
        workspace_id: "01900000-0000-7000-8000-000000000001",
        schema_version: 1,
        name,
        profile,
        build_profile: "connected-local",
        locale: "en-IE",
        enabled_modules: ["people"],
      },
      people: peopleByPath.get(path).slice(),
      validation: null,
    };
    workspace = opened;
    opened.validation = validation();
    sessions.set(sessionId, opened);
    return opened;
  };
  window.__liaisonBridgeState = {
    activeSessions: () => Array.from(sessions.keys()),
    closedSessions: () => closedSessions.slice(),
    currentSession: () => workspace?.workspace.session_id || null,
    failNextClose: (sessionId) => { failNextCloseFor = sessionId; },
    failNextCloses: (count) => { failNextCloseCount = count; },
    delayNext: (command) => delayedCommands.add(command),
    release: (command) => {
      const resolve = delayResolvers.get(command);
      if (!resolve) return false;
      delayResolvers.delete(command);
      resolve();
      return true;
    },
    isDelayed: (command) => delayResolvers.has(command),
    invocationCount: (command) => commandCounts.get(command) || 0,
    inFlightCommands: () => inFlightCommands,
    maxInFlightCommands: () => maxInFlightCommands,
    resetOperationMetrics: () => {
      commandCounts.clear();
      maxInFlightCommands = inFlightCommands;
    },
  };
  window.__TAURI__ = {
    core: {
      invoke: async (command, payload = {}) => {
        commandCounts.set(command, (commandCounts.get(command) || 0) + 1);
        inFlightCommands += 1;
        maxInFlightCommands = Math.max(maxInFlightCommands, inFlightCommands);
        try {
          await waitForConfiguredDelay(command);
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
            return result(openResult(payload.request.path, payload.request.name, payload.request.profile));
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
            return result(openResult(payload.path, "Opened workspace", "workplace"));
          case "close_workspace": {
            const sessionId = payload.request.sessionId;
            requireSession(sessionId);
            if (failNextCloseFor === sessionId || failNextCloseCount > 0) {
              if (failNextCloseCount > 0) failNextCloseCount -= 1;
              failNextCloseFor = null;
              throw applicationError(
                "workspace.authority-unavailable",
                "the current workspace could not be closed safely",
                "keep the current workspace selected and retry the switch",
              );
            }
            sessions.delete(sessionId);
            closedSessions.push(sessionId);
            return result({ session_id: sessionId });
          }
          case "list_people": {
            return result(sessionPeople(payload.request.sessionId).slice());
          }
          case "create_person": {
            const people = sessionPeople(payload.request.sessionId);
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
          case "inspect_workspace_health":
            validationNeedsAttention = payload.path.includes("needs-attention");
            return result(validation());
          default:
            throw new Error(`Unexpected command: ${command}`);
          }
        } finally {
          inFlightCommands -= 1;
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
    assert page.locator(".topbar").count() == 1
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

    page.get_by_label("Absolute folder path").fill("/Users/tester/Documents/health-only-needs-attention")
    page.get_by_role("button", name="Run read-only Health").click()
    page.get_by_role("heading", name="Validate the open-file workspace").wait_for()
    page.get_by_text("Workspace needs attention", exact=True).wait_for()
    assert page.locator("#validation-scope").inner_text() == (
        "Read-only folder: /Users/tester/Documents/health-only-needs-attention"
    )
    assert "without changing files" in page.locator("#live-status").inner_text()
    assert page.evaluate("window.__liaisonBridgeState.activeSessions().length") == 0
    page.get_by_role("button", name="Workspace").click()

    page.get_by_label("Workspace name").fill("Family relationships")
    page.get_by_label("Workspace profile").select_option("family")
    page.get_by_role("button", name="Create local workspace").click()
    page.locator("#live-status").get_by_text("Workspace setup did not complete", exact=False).wait_for()
    assert page.get_by_role("button", name="Create local workspace").is_enabled()
    page.get_by_role("button", name="Create local workspace").click()
    page.get_by_role("heading", name="People", exact=True).wait_for()
    assert page.get_by_role("heading", name="People", exact=True).evaluate("el => el === document.activeElement")
    assert page.locator("#people-workspace-warning").is_hidden()
    first_session = page.evaluate("window.__liaisonBridgeState.currentSession()")
    assert page.evaluate("window.__liaisonBridgeState.activeSessions().length") == 1

    page.get_by_role("button", name="Add person").click()
    page.get_by_label("Display name").fill("Alex Murphy")
    page.get_by_label("Primary email optional").fill("alex@example.test")
    page.get_by_label("Primary email optional").press("Enter")
    page.locator("#people-table").get_by_text("Alex Murphy", exact=True).wait_for()
    assert page.locator("#people-table").get_by_text("alex@example.test", exact=True).count() == 1
    assert "Saved Alex Murphy" in page.locator("#live-status").inner_text()
    page.get_by_role("button", name="Open local record for Alex Murphy").click()
    page.get_by_role("heading", name="Alex Murphy", exact=True).wait_for()
    assert page.locator("#person-contact-details").get_by_text(
        "alex@example.test", exact=True
    ).count() == 1
    assert page.locator("#person-record-details").get_by_text(
        "Revision 1", exact=True
    ).count() == 1
    page.get_by_role("button", name="Back to People").click()
    page.get_by_role("heading", name="People", exact=True).wait_for()
    page.wait_for_function(
        "document.activeElement?.getAttribute('aria-label') === "
        "'Open local record for Alex Murphy'"
    )

    page.get_by_role("button", name="Workspace").click()
    page.get_by_label("Absolute folder path").fill("/Users/tester/Documents/needs-attention")
    page.get_by_role("button", name="Run read-only Health").click()
    page.get_by_role("heading", name="Validate the open-file workspace").wait_for()
    assert page.locator("#validation-scope").inner_text() == (
        "Read-only folder: /Users/tester/Documents/needs-attention"
    )
    assert page.locator("#workspace-path-label").inner_text() == "Family relationships · Family"
    assert page.evaluate("window.__liaisonBridgeState.activeSessions()") == [first_session]
    page.get_by_role("button", name="Workspace").click()
    page.get_by_role("button", name="Open existing workspace").click()
    page.locator("#live-status").get_by_text("review Health before editing", exact=False).wait_for()
    assert page.get_by_role("button", name="Open existing workspace").is_enabled()
    second_session = page.evaluate("window.__liaisonBridgeState.currentSession()")
    assert second_session != first_session
    assert page.evaluate("window.__liaisonBridgeState.activeSessions()") == [second_session]
    assert first_session in page.evaluate("window.__liaisonBridgeState.closedSessions()")

    page.get_by_role("button", name="Health", exact=True).click()
    page.get_by_role("heading", name="Validate the open-file workspace").wait_for()
    page.get_by_role("button", name="Run validation").click()
    page.get_by_text("Workspace needs attention", exact=True).wait_for()
    assert "people.invalid-record" in page.locator("#validation-findings").inner_text()
    assert "Recovery:" in page.locator("#validation-findings").inner_text()

    page.get_by_role("button", name="Workspace").click()
    page.get_by_label("Absolute folder path").fill("/Users/tester/Documents/created-replacement")
    page.get_by_label("Workspace name").fill("Created replacement")
    page.get_by_role("button", name="Create local workspace").click()
    page.get_by_role("heading", name="People", exact=True).wait_for()
    third_session = page.evaluate("window.__liaisonBridgeState.currentSession()")
    assert third_session != second_session
    assert page.evaluate("window.__liaisonBridgeState.activeSessions()") == [third_session]
    assert second_session in page.evaluate("window.__liaisonBridgeState.closedSessions()")

    page.get_by_role("button", name="Workspace").click()
    page.evaluate("sessionId => window.__liaisonBridgeState.failNextClose(sessionId)", third_session)
    page.get_by_label("Absolute folder path").fill("/Users/tester/Documents/rolled-back-replacement")
    page.get_by_role("button", name="Open existing workspace").click()
    page.locator("#live-status").get_by_text("Workspace switch did not complete", exact=False).wait_for()
    assert page.locator("#workspace-path-label").inner_text() == "Created replacement · Family"
    assert page.evaluate("window.__liaisonBridgeState.activeSessions()") == [third_session]

    page.evaluate(
        "window.__liaisonBridgeState.resetOperationMetrics(); "
        "window.__liaisonBridgeState.failNextCloses(2);"
    )
    page.get_by_label("Absolute folder path").fill(
        "/Users/tester/Documents/restart-required-replacement"
    )
    page.get_by_role("button", name="Open existing workspace").click()
    page.locator("#live-status").get_by_text(
        "Restart Liaison RM before retrying", exact=False
    ).wait_for()
    restart_status = page.locator("#live-status").inner_text()
    assert "may still hold writer authority" in restart_status
    assert page.locator("#workspace-path-label").inner_text() == "Created replacement · Family"
    active_after_cleanup_failure = page.evaluate(
        "window.__liaisonBridgeState.activeSessions()"
    )
    assert third_session in active_after_cleanup_failure
    assert len(active_after_cleanup_failure) == 2
    assert page.locator("[data-native-operation]:not(:disabled)").count() == 0
    page.locator("#open-workspace").dispatch_event("click")
    assert page.evaluate(
        'window.__liaisonBridgeState.invocationCount("open_workspace")'
    ) == 1

    SCREENSHOTS.mkdir(parents=True, exist_ok=True)
    page.screenshot(path=str(SCREENSHOTS / "desktop-workspace-health.png"), full_page=True)
    assert external_requests == []


def initialise_overlap_workspace(page: Page, path: str) -> str:
    load_page(page)
    page.wait_for_function(
        "!document.querySelector('[data-native-operation]').disabled"
    )
    page.get_by_label("Absolute folder path").fill(path)
    page.get_by_label("Workspace name").fill("Overlap source")
    page.get_by_role("button", name="Create local workspace").click()
    page.locator("#live-status").get_by_text(
        "Workspace setup did not complete", exact=False
    ).wait_for()
    page.get_by_role("button", name="Create local workspace").click()
    page.get_by_role("heading", name="People", exact=True).wait_for()
    return page.evaluate("window.__liaisonBridgeState.currentSession()")


def test_overlapping_workspace_switches(browser, external_requests: list[str]) -> None:
    page = browser.new_page(viewport={"width": 1280, "height": 900})
    page.set_default_timeout(8_000)
    page.on(
        "request",
        lambda request: external_requests.append(request.url)
        if not request.url.startswith("file://")
        else None,
    )
    first_session = initialise_overlap_workspace(
        page, "/Users/tester/Documents/overlap-switch-source"
    )
    page.get_by_role("button", name="Workspace").click()
    page.get_by_label("Absolute folder path").fill(
        "/Users/tester/Documents/overlap-switch-target"
    )
    page.evaluate(
        """
        window.__liaisonBridgeState.resetOperationMetrics();
        window.__liaisonBridgeState.delayNext("open_workspace");
        """
    )
    open_button = page.get_by_role("button", name="Open existing workspace")
    open_button.click()
    page.wait_for_function(
        'window.__liaisonBridgeState.isDelayed("open_workspace")'
    )
    assert page.locator("#main-content").get_attribute("aria-busy") == "true"
    assert page.locator("[data-native-operation]:not(:disabled)").count() == 0

    # A synthetic second event covers keyboard/programmatic re-entry even though
    # every visible native-operation control is disabled during the first switch.
    page.locator("#open-workspace").dispatch_event("click")
    assert page.evaluate(
        'window.__liaisonBridgeState.invocationCount("open_workspace")'
    ) == 1
    assert page.evaluate("window.__liaisonBridgeState.inFlightCommands()") == 1
    assert page.evaluate("window.__liaisonBridgeState.maxInFlightCommands()") == 1

    assert page.evaluate(
        'window.__liaisonBridgeState.release("open_workspace")'
    ) is True
    page.locator("#live-status").get_by_text("Opened workspace", exact=False).wait_for()
    page.wait_for_function("!document.querySelector('#open-workspace').disabled")
    replacement_session = page.evaluate(
        "window.__liaisonBridgeState.currentSession()"
    )
    assert replacement_session != first_session
    assert page.evaluate("window.__liaisonBridgeState.activeSessions()") == [
        replacement_session
    ]
    assert first_session in page.evaluate(
        "window.__liaisonBridgeState.closedSessions()"
    )
    assert page.evaluate(
        'window.__liaisonBridgeState.invocationCount("open_workspace")'
    ) == 1
    assert page.locator("[data-native-operation]:disabled").count() == 0
    assert page.locator("#main-content").get_attribute("aria-busy") is None
    assert open_button.evaluate("element => element === document.activeElement")
    page.close()


def test_person_result_cannot_cross_workspace(browser, external_requests: list[str]) -> None:
    page = browser.new_page(viewport={"width": 1280, "height": 900})
    page.set_default_timeout(8_000)
    page.on(
        "request",
        lambda request: external_requests.append(request.url)
        if not request.url.startswith("file://")
        else None,
    )
    source_session = initialise_overlap_workspace(
        page, "/Users/tester/Documents/overlap-person-source"
    )
    page.get_by_role("button", name="Add person").click()
    page.get_by_label("Display name").fill("Delayed Person")
    page.evaluate(
        """
        window.__liaisonBridgeState.resetOperationMetrics();
        window.__liaisonBridgeState.delayNext("create_person");
        """
    )
    page.get_by_role("button", name="Create profile").click()
    page.wait_for_function(
        'window.__liaisonBridgeState.isDelayed("create_person")'
    )
    assert page.locator("#main-content").get_attribute("aria-busy") == "true"
    assert page.locator("[data-native-operation]:not(:disabled)").count() == 0

    # Native-operation fields are disabled for users. Force a value and event
    # to prove script/programmatic re-entry is rejected by the synchronous
    # guard as well.
    page.evaluate(
        """
        document.getElementById("workspace-path").value =
          "/Users/tester/Documents/overlap-person-target";
        """
    )
    page.locator("#open-workspace").dispatch_event("click")
    assert page.evaluate(
        'window.__liaisonBridgeState.invocationCount("open_workspace")'
    ) == 0
    assert page.evaluate("window.__liaisonBridgeState.activeSessions()") == [
        source_session
    ]
    assert page.evaluate("window.__liaisonBridgeState.maxInFlightCommands()") == 1

    assert page.evaluate(
        'window.__liaisonBridgeState.release("create_person")'
    ) is True
    page.locator("#live-status").get_by_text("Saved Delayed Person", exact=False).wait_for()
    page.wait_for_function("!document.querySelector('#open-workspace').disabled")
    page.get_by_role("button", name="Workspace").click()
    page.get_by_role("button", name="Open existing workspace").click()
    page.locator("#live-status").get_by_text("Opened workspace", exact=False).wait_for()
    target_session = page.evaluate("window.__liaisonBridgeState.currentSession()")
    assert target_session != source_session
    assert page.evaluate("window.__liaisonBridgeState.activeSessions()") == [
        target_session
    ]
    assert source_session in page.evaluate(
        "window.__liaisonBridgeState.closedSessions()"
    )
    assert page.locator("#people-count").inner_text() == "0 people"
    assert page.get_by_text("Delayed Person", exact=True).count() == 0
    assert "Delayed Person" not in page.locator("#live-status").inner_text()
    assert page.evaluate("window.__liaisonBridgeState.maxInFlightCommands()") == 1
    assert page.locator("[data-native-operation]:disabled").count() == 0
    assert page.locator("#main-content").get_attribute("aria-busy") is None
    page.close()


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
    page.get_by_role("button", name="Sections").click()
    page.get_by_role("button", name="People", exact=True).click()
    page.get_by_role("heading", name="People", exact=True).wait_for()
    overflow = page.evaluate("document.documentElement.scrollWidth > document.documentElement.clientWidth")
    assert overflow is False
    assert page.get_by_role("button", name="Sections").is_visible()
    assert page.locator("#current-section-label").inner_text() == "People"
    page.get_by_role("button", name="Sections").click()
    assert page.get_by_role("button", name="Workspace", exact=True).is_visible()
    assert page.get_by_role("button", name="People", exact=True).is_visible()
    assert page.get_by_role("button", name="Health", exact=True).is_visible()
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
    assert styles == {"background": "rgb(120, 200, 224)", "color": "rgb(10, 24, 26)"}
    sidebar = page.locator(".sidebar").evaluate(
        "element => ({ background: getComputedStyle(element).backgroundColor, color: getComputedStyle(element).color })"
    )
    assert sidebar == {"background": "rgb(22, 35, 30)", "color": "rgb(244, 241, 232)"}
    SCREENSHOTS.mkdir(parents=True, exist_ok=True)
    page.screenshot(path=str(SCREENSHOTS / "dark-workspace.png"), full_page=True)
    page.close()


def main() -> int:
    results = {
        "desktop_workflow": False,
        "native_switch_serialization": False,
        "native_session_result_serialization": False,
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
        test_overlapping_workspace_switches(browser, external_requests)
        results["native_switch_serialization"] = True
        test_person_result_cannot_cross_workspace(browser, external_requests)
        results["native_session_result_serialization"] = True
        test_mobile(browser, external_requests)
        results["mobile_reflow"] = True
        test_dark_mode(browser, external_requests)
        results["dark_mode"] = True
        assert external_requests == []
        page.close()
        browser.close()

    REPORT.parent.mkdir(parents=True, exist_ok=True)
    REPORT.write_text(json.dumps(results, indent=2) + "\n", encoding="utf-8")
    print("Desktop UI tests passed: globally serialized native operations, workspace switching/rollback, dual-close restart recovery, stale-person isolation, full-canvas People and separate Person navigation, validation, focus recovery, mobile reflow, dark mode, and zero external requests")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
