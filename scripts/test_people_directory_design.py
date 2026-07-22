#!/usr/bin/env python3
"""Focused browser regression for the approved People directory treatment."""

from __future__ import annotations

import os
import shutil
from pathlib import Path

from playwright.sync_api import sync_playwright

from test_desktop_ui import load_page


SCREENSHOT = Path(
    os.environ.get(
        "PEOPLE_DIRECTORY_SCREENSHOT",
        "artifacts/desktop-screens/people-directory-regression.png",
    )
)


def main() -> int:
    external_requests: list[str] = []

    def assert_only_route(page, route: str) -> None:
        assert page.locator("[data-page]:not([hidden])").count() == 1
        assert page.locator(f"[data-page='{route}']:not([hidden])").count() == 1

    def assert_within_viewport(locator, viewport_width: int) -> None:
        box = locator.bounding_box()
        assert box is not None
        assert box["width"] > 0
        assert box["x"] >= -0.5
        assert box["x"] + box["width"] <= viewport_width + 0.5

    with sync_playwright() as playwright:
        executable = os.environ.get("CHROMIUM_PATH") or shutil.which("chromium")
        browser = playwright.chromium.launch(headless=True, executable_path=executable)
        page = browser.new_page(viewport={"width": 1280, "height": 900})
        page.set_default_timeout(8_000)
        page.on(
            "request",
            lambda request: external_requests.append(request.url)
            if not request.url.startswith("file://")
            else None,
        )
        load_page(page)

        create_workspace = page.get_by_role("button", name="Create local workspace")
        create_workspace.click()
        page.locator("#live-status").get_by_text(
            "Workspace setup did not complete", exact=False
        ).wait_for()
        create_workspace.click()
        page.get_by_role("heading", name="People", exact=True).wait_for()
        assert_only_route(page, "people")

        for name, email in [
            ("Synthetic Alpha", "alpha@example.test"),
            ("Synthetic Bravo", "bravo@example.test"),
            ("Synthetic Charlie", "charlie@example.test"),
            (
                "Synthetic Catherine With A Deliberately Long Local Profile Name",
                "catherine-with-a-deliberately-long-local-profile-name@example.test",
            ),
        ]:
            add_person = page.get_by_role("button", name="Add person")
            add_person.click()
            page.wait_for_function(
                "document.getElementById('person-name') === document.activeElement"
            )
            page.get_by_label("Display name").fill(name)
            page.get_by_label("Primary email optional").fill(email)
            page.get_by_role("button", name="Create profile").click()
            page.locator("#live-status").get_by_text(f"Saved {name}", exact=False).wait_for()
            page.get_by_role(
                "button", name=f"Open local record for {name}", exact=True
            ).wait_for()
            assert page.locator("#person-dialog").is_hidden()
            page.wait_for_function(
                "document.getElementById('add-person') === document.activeElement"
            )
            assert_only_route(page, "people")
            assert page.locator(".person-row.is-selected").count() == 0
            assert page.locator("[data-person-open][aria-current='true']").count() == 0

        assert page.locator("#people-count").inner_text() == "4 people"
        people_width = page.locator("[data-page='people']").evaluate(
            "element => Math.round(element.getBoundingClientRect().width)"
        )
        assert page.locator("#person-detail").count() == 0
        assert page.locator("#person-detail-dialog").count() == 0
        directory_width = page.locator(".directory-workspace").evaluate(
            "element => Math.round(element.getBoundingClientRect().width)"
        )
        results_width = page.locator(".directory-results").evaluate(
            "element => Math.round(element.getBoundingClientRect().width)"
        )
        assert abs(people_width - directory_width) <= 2
        assert abs(directory_width - results_width) <= 2

        page.evaluate(
            """
            async () => {
              const sessionId = window.__liaisonBridgeState.currentSession();
              const response = await window.__TAURI__.core.invoke("list_people", {
                request: { sessionId },
              });
              const bravo = response.value.find(
                (person) => person.display_name === "Synthetic Bravo",
              );
              bravo.aliases.push("Bee Bravo");
              bravo.emails.push({ value: "bravo.work@example.test", label: "work" });
              bravo.phones.push({ value: "+353 1 555 0102", label: "mobile" });
              bravo.phones.push({ value: "+353 1 555 0199", label: "office" });
              window.__liaisonBridgeState.resetOperationMetrics();
              window.__liaisonBridgeState.delayNext("list_people");
            }
            """
        )
        refresh = page.get_by_role("button", name="Refresh")
        refresh.click()
        page.wait_for_function(
            'window.__liaisonBridgeState.isDelayed("list_people")'
        )
        assert refresh.inner_text() == "Refreshing…"
        assert refresh.is_disabled()
        assert page.locator("#main-content").get_attribute("aria-busy") == "true"
        assert page.evaluate(
            'window.__liaisonBridgeState.invocationCount("list_people")'
        ) == 1
        assert page.evaluate(
            'window.__liaisonBridgeState.release("list_people")'
        ) is True
        page.locator("#live-status").get_by_text(
            "Refreshed 4 local profiles", exact=False
        ).wait_for()
        page.wait_for_function("!document.getElementById('refresh-people').disabled")
        assert refresh.inner_text() == "Refresh"
        assert page.locator("#main-content").get_attribute("aria-busy") is None
        assert page.locator("#people-count-summary").inner_text() == (
            "4 people in this workspace."
        )
        assert page.locator("#people-refresh-notice").is_hidden()
        assert page.get_by_role("columnheader", name="Record state").count() == 1
        assert page.get_by_role("columnheader", name="Information state").count() == 0

        search = page.get_by_role("searchbox", name="Search people")
        for query in [
            "Synthetic Bravo",
            "bravo@example.test",
            "bravo.work@example.test",
            "Bee Bravo",
            "+353 1 555 0102",
            "+353 1 555 0199",
        ]:
            search.fill(query)
            assert page.locator("#people-count-summary").inner_text() == (
                "Showing 1 of 4 people."
            )
            assert page.locator(".person-row").count() == 1
            assert page.get_by_role(
                "button", name="Open local record for Synthetic Bravo"
            ).count() == 1

        search.fill("not in this workspace")
        assert page.locator("#people-count-summary").inner_text() == "Showing 0 of 4 people."
        assert page.get_by_text(
            "No people match “not in this workspace”. Try another name, email, "
            "phone, or alias, or clear the search.",
            exact=True,
        ).is_visible()

        page.get_by_role("button", name="Clear search").click()
        assert search.input_value() == ""
        search.fill("bravo@example.test")
        open_bravo = page.get_by_role(
            "button", name="Open local record for Synthetic Bravo"
        )
        open_bravo.focus()
        page.keyboard.press("Enter")
        page.get_by_role("heading", name="Synthetic Bravo", exact=True).wait_for()
        assert_only_route(page, "person")
        assert page.locator("[data-page='person']").is_visible()
        assert page.locator(".read-only-badge").get_by_text(
            "Read-only local record", exact=True
        ).count() == 1
        assert page.locator("[data-page='person']").locator(
            "form, input, textarea, select"
        ).count() == 0
        assert page.locator("[data-page='person']").get_by_role("button").count() == 1
        assert page.locator("#person-contact-details").get_by_text(
            "bravo@example.test", exact=True
        ).count() == 1
        assert page.locator("#person-contact-details").get_by_text(
            "bravo.work@example.test", exact=True
        ).count() == 1
        assert page.locator("#person-contact-details").get_by_text(
            "+353 1 555 0102", exact=True
        ).count() == 1
        assert page.locator("#person-contact-details").get_by_text(
            "+353 1 555 0199", exact=True
        ).count() == 1
        assert page.locator("#person-contact-details").get_by_text(
            "Email · primary", exact=True
        ).count() == 1
        assert page.locator("#person-contact-details").get_by_text(
            "Email · work", exact=True
        ).count() == 1
        assert page.locator("#person-contact-details").get_by_text(
            "Phone · mobile", exact=True
        ).count() == 1
        assert page.locator("#person-contact-details").get_by_text(
            "Phone · office", exact=True
        ).count() == 1
        assert page.locator("#person-record-details").get_by_text(
            "Revision 1", exact=True
        ).count() == 1
        assert page.locator("#person-record-details").get_by_text(
            "Record state", exact=True
        ).count() == 1
        assert page.get_by_role("button", name="Back to People").is_visible()
        assert page.locator("[data-route='people']").get_attribute("aria-current") == "page"
        assert page.locator("#person-page-heading").get_attribute("aria-describedby") == (
            "person-record-mode person-page-summary"
        )
        assert page.locator("#person-operation-status").inner_text() == (
            "Viewing Synthetic Bravo as a read-only local record."
        )
        assert "IBM Plex Mono" not in page.locator(
            "#person-contact-details div:last-child dd"
        ).evaluate("element => getComputedStyle(element).fontFamily")
        assert "IBM Plex Mono" in page.locator(
            "#person-record-details .is-provenance dd"
        ).first.evaluate("element => getComputedStyle(element).fontFamily")

        future_controls = [
            "Columns",
            "Advanced filters",
            "Filter by CSV",
            "Import",
            "Export",
            "Saved views",
            "Edit",
            "Add note",
            "Notes",
            "Interactions",
            "Important dates",
            "Commitments",
            "Workplace",
            "Custom fields",
        ]
        current_people_surfaces = page.locator(
            "[data-page='people'], [data-page='person']"
        )
        for label in future_controls:
            assert current_people_surfaces.get_by_role(
                "button", name=label, exact=True
            ).count() == 0
            assert current_people_surfaces.get_by_role(
                "heading", name=label, exact=True
            ).count() == 0

        page.set_viewport_size({"width": 320, "height": 900})
        assert page.evaluate(
            "document.documentElement.scrollWidth <= document.documentElement.clientWidth"
        )
        assert page.locator("#person-page-heading").inner_text() == "Synthetic Bravo"
        assert page.locator("#person-page-heading").evaluate(
            "heading => heading === document.activeElement"
        )
        for locator in [
            page.get_by_role("button", name="Back to People"),
            page.locator("#person-page-heading"),
            page.locator("#person-contact-details"),
            page.locator("#person-record-details"),
        ]:
            assert_within_viewport(locator, 320)
        page.get_by_role("button", name="Back to People").focus()
        page.keyboard.press("Enter")
        page.get_by_role("heading", name="People", exact=True).wait_for()
        assert_only_route(page, "people")
        assert search.input_value() == "bravo@example.test"
        assert page.locator("#people-count-summary").inner_text() == (
            "Showing 1 of 4 people."
        )
        assert page.locator("#people-operation-status").inner_text() == (
            "Returned to People. Showing 1 of 4 people."
        )
        assert page.locator(".person-row").count() == 1
        page.wait_for_function(
            "document.activeElement?.getAttribute('aria-label') === "
            "'Open local record for Synthetic Bravo'"
        )
        assert page.get_by_role(
            "button", name="Open local record for Synthetic Bravo"
        ).evaluate(
            "button => button === document.activeElement"
        )
        assert page.get_by_role(
            "button", name="Open local record for Synthetic Bravo"
        ).get_attribute("aria-current") == "true"
        assert page.evaluate(
            "document.documentElement.scrollWidth <= document.documentElement.clientWidth"
        )
        assert page.get_by_role(
            "table", name="People returned by the open local workspace"
        ).count() == 1
        assert page.locator("#people-table th[scope='col']").count() == 5
        for locator in [
            search,
            page.get_by_role("button", name="Add person"),
            page.get_by_role("button", name="Refresh"),
            page.get_by_role("button", name="Clear search"),
            page.get_by_role("button", name="Open local record for Synthetic Bravo"),
        ]:
            assert_within_viewport(locator, 320)

        page.get_by_role("button", name="Clear search").click()
        long_name = "Synthetic Catherine With A Deliberately Long Local Profile Name"
        long_email = "catherine-with-a-deliberately-long-local-profile-name@example.test"
        long_row_button = page.get_by_role(
            "button", name=f"Open local record for {long_name}"
        )
        assert_within_viewport(long_row_button, 320)
        long_row = long_row_button.locator("xpath=ancestor::tr")
        assert long_row.get_by_text(long_name, exact=True).evaluate(
            "element => element.scrollWidth <= element.clientWidth"
        )
        assert long_row.get_by_text(long_email, exact=True).evaluate(
            "element => element.scrollWidth <= element.clientWidth"
        )
        long_row_button.focus()
        page.keyboard.press("Space")
        page.get_by_role("heading", name=long_name, exact=True).wait_for()
        assert_only_route(page, "person")
        assert page.evaluate("window.scrollY") == 0
        assert page.get_by_role("button", name="Back to People").bounding_box()["y"] >= 0
        assert page.locator("#person-contact-details").get_by_text(
            long_email, exact=True
        ).count() == 1
        assert page.evaluate(
            "document.documentElement.scrollWidth <= document.documentElement.clientWidth"
        )
        assert_within_viewport(page.locator("#person-page-heading"), 320)
        assert_within_viewport(page.locator("#person-contact-details"), 320)
        assert page.locator("#person-page-heading").evaluate(
            "element => element.scrollWidth <= element.clientWidth"
        )
        assert page.locator("#person-contact-details").get_by_text(
            long_email, exact=True
        ).evaluate("element => element.scrollWidth <= element.clientWidth")
        assert page.locator("#person-operation-status").inner_text() == (
            f"Viewing {long_name} as a read-only local record."
        )
        assert_within_viewport(page.locator("#person-operation-status"), 320)

        page.get_by_role("button", name="Back to People").click()
        page.wait_for_function(
            "document.activeElement?.getAttribute('aria-label') === "
            f"'Open local record for {long_name}'"
        )
        row_count_before_failure = page.locator(".person-row").count()
        page.evaluate(
            """
            () => {
              const original = window.__TAURI__.core.invoke;
              window.__TAURI__.core.invoke = async (command, payload) => {
                if (command === "list_people") {
                  window.__TAURI__.core.invoke = original;
                  throw new Error("synthetic refresh failure");
                }
                return original(command, payload);
              };
            }
            """
        )
        page.get_by_role("button", name="Refresh").click()
        page.locator("#live-status").get_by_text(
            "People were not refreshed", exact=False
        ).wait_for()
        assert page.locator(".person-row").count() == row_count_before_failure
        assert page.get_by_role("button", name="Refresh").is_enabled()
        assert page.locator("#main-content").get_attribute("aria-busy") is None
        assert page.locator("#people-refresh-notice").is_visible()
        assert "Directory may be stale" in page.locator(
            "#people-refresh-notice"
        ).inner_text()
        assert "People were not refreshed" in page.locator(
            "#people-operation-status"
        ).inner_text()
        assert page.locator("#people-table").get_attribute("aria-describedby") == (
            "people-count-summary people-refresh-notice"
        )
        assert_within_viewport(page.locator("#people-operation-status"), 320)

        page.get_by_role("button", name="Refresh").click()
        page.locator("#live-status").get_by_text(
            "Refreshed 4 local profiles", exact=False
        ).wait_for()
        assert page.locator("#people-refresh-notice").is_hidden()
        assert page.locator(".person-row").count() == 4

        SCREENSHOT.parent.mkdir(parents=True, exist_ok=True)
        page.screenshot(path=str(SCREENSHOT), full_page=True)
        assert external_requests == []
        browser.close()

    print(
        "People directory regression passed: full-width table, field search, "
        "refresh recovery, separate read-only Person surface, keyboard/focus "
        "continuity, 320 CSS-pixel long-content reflow, and zero external requests"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
