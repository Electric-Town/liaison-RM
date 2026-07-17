#!/usr/bin/env python3
"""Screen-integrity and browser-level checks for the Liaison RM review prototype."""
from __future__ import annotations

import os
from pathlib import Path
from xml.etree import ElementTree

from playwright.sync_api import Page, sync_playwright

ROOT = Path(__file__).resolve().parents[1]
PROTOTYPE = ROOT / "docs" / "prototypes" / "liaison-rm-review.html"
SCREENS = ROOT / "docs" / "prototypes" / "screens"
EXPECTED_SCREENS = {
    "dashboard.svg": (1200, 800),
    "people.svg": (1200, 800),
    "events.svg": (1200, 800),
    "network.svg": (1200, 800),
    "settings.svg": (1200, 800),
    "mobile-dashboard.svg": (390, 844),
}
ROUTES = ("home", "people", "events", "network", "settings")


def check_committed_screens() -> None:
    for name, dimensions in EXPECTED_SCREENS.items():
        path = SCREENS / name
        assert path.exists(), f"missing review screen: {path}"
        root = ElementTree.parse(path).getroot()
        assert root.tag.endswith("svg"), f"not an SVG: {name}"
        assert int(root.attrib["width"]) == dimensions[0], name
        assert int(root.attrib["height"]) == dimensions[1], name
        titles = [node.text for node in root if node.tag.endswith("title")]
        descriptions = [node.text for node in root if node.tag.endswith("desc")]
        assert titles and titles[0] and titles[0].strip(), f"missing title: {name}"
        assert descriptions and descriptions[0] and descriptions[0].strip(), f"missing description: {name}"
        assert path.stat().st_size > 1_000, f"screen is unexpectedly small: {name}"


def launch(playwright):
    options = {"headless": True, "args": ["--no-sandbox", "--disable-dev-shm-usage"]}
    executable = os.environ.get("CHROMIUM_PATH")
    if executable:
        options["executable_path"] = executable
    return playwright.chromium.launch(**options)


def load(page: Page) -> None:
    page.set_content(PROTOTYPE.read_text(encoding="utf-8"), wait_until="load")
    page.wait_for_timeout(50)


def check_accessible_structure(page: Page) -> None:
    assert page.locator("main").count() == 1
    assert page.locator("nav").count() == 1
    assert page.locator("a.skip").get_attribute("href") == "#main"
    assert page.locator("button").evaluate_all(
        "nodes => nodes.filter(n => !(n.getAttribute('aria-label') || n.textContent.trim())).length"
    ) == 0
    assert page.locator("input,select,textarea").evaluate_all(
        """nodes => nodes.filter(n => {
          if (n.getAttribute('aria-label') || n.getAttribute('aria-labelledby')) return false;
          if (n.id && document.querySelector(`label[for="${CSS.escape(n.id)}"]`)) return false;
          if (n.closest('label')) return false;
          return true;
        }).length"""
    ) == 0
    duplicates = page.locator("[id]").evaluate_all(
        "nodes => { const ids=nodes.map(n=>n.id); return ids.filter((id,i)=>ids.indexOf(id)!==i); }"
    )
    assert duplicates == [], duplicates


def check_routes(page: Page) -> None:
    for route in ROUTES:
        page.locator(f'.nav [data-route="{route}"]').click()
        visible = page.locator("[data-view]:visible")
        assert visible.count() == 1
        assert visible.get_attribute("data-view") == route
        assert page.locator(f'.nav [data-route="{route}"]').get_attribute("aria-current") is not None
        assert page.evaluate("document.activeElement.tagName") == "H1"


def check_profile_tabs(page: Page) -> None:
    page.locator('.nav [data-route="people"]').click()
    notes = page.get_by_role("tab", name="Notes")
    notes.click()
    assert page.locator("#notes").is_visible()
    notes.focus()
    notes.press("ArrowRight")
    assert page.get_by_role("tab", name="Timeline").get_attribute("aria-selected") == "true"
    page.get_by_role("tab", name="Timeline").press("End")
    assert page.get_by_role("tab", name="Relationships").get_attribute("aria-selected") == "true"


def check_event_flow(page: Page) -> None:
    page.locator('.nav [data-route="events"]').click()
    page.locator(".resolve").first.click()
    assert page.locator("#covered").inner_text() == "42"
    assert page.locator("#gaps").inner_text() == "3 unknown"
    assert page.locator("#toast").is_visible()


def check_graph_fallback(page: Page) -> None:
    page.locator('.nav [data-route="network"]').click()
    page.locator("#tablebutton").click()
    assert page.locator("#networktable").is_visible()
    assert page.locator("#networktable table").count() == 1
    assert page.locator("#tablebutton").get_attribute("aria-pressed") == "true"
    page.locator("#graphbutton").click()
    assert page.locator("#graph").is_visible()


def check_feature_gate(page: Page) -> None:
    page.locator('.nav [data-route="settings"]').click()
    toggle = page.get_by_role("switch", name="Connected-local features")
    assert toggle.get_attribute("aria-checked") == "false"
    toggle.click()
    assert toggle.get_attribute("aria-checked") == "true"
    assert page.locator("#toast").is_visible()


def check_mobile(browser) -> None:
    page = browser.new_page(viewport={"width": 390, "height": 844})
    load(page)
    assert page.locator("#menu").is_visible()
    page.locator("#menu").click()
    assert page.locator("#menu").get_attribute("aria-expanded") == "true"
    page.keyboard.press("Escape")
    assert page.locator("#menu").get_attribute("aria-expanded") == "false"
    assert not page.evaluate("document.documentElement.scrollWidth > document.documentElement.clientWidth")
    page.close()


def main() -> None:
    assert PROTOTYPE.exists(), PROTOTYPE
    check_committed_screens()
    with sync_playwright() as playwright:
        browser = launch(playwright)
        page = browser.new_page(viewport={"width": 1200, "height": 800})
        load(page)
        check_accessible_structure(page)
        check_routes(page)
        check_profile_tabs(page)
        check_event_flow(page)
        check_graph_fallback(page)
        check_feature_gate(page)
        page.close()
        check_mobile(browser)
        browser.close()
    print("Prototype tests passed: six screens, landmarks, labels, routes, focus, tabs, event coverage, graph fallback, feature gates, and mobile overflow")


if __name__ == "__main__":
    main()
