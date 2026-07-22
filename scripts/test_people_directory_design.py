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

        for name, email in [
            ("Synthetic Alpha", "alpha@example.test"),
            ("Synthetic Bravo", "bravo@example.test"),
            ("Synthetic Charlie", "charlie@example.test"),
            (
                "Synthetic Catherine With A Deliberately Long Local Profile Name",
                "catherine-with-a-deliberately-long-local-profile-name@example.test",
            ),
        ]:
            page.get_by_role("button", name="Add person").click()
            page.get_by_label("Display name").fill(name)
            page.get_by_label("Primary email optional").fill(email)
            page.get_by_role("button", name="Create profile").click()
            page.locator("#live-status").get_by_text(f"Saved {name}", exact=False).wait_for()
            page.get_by_role(
                "button", name=f"Open local record for {name}", exact=True
            ).wait_for()

        assert page.locator("#people-count").inner_text() == "4 people"
        page.get_by_role("searchbox", name="Search people").fill("bravo")
        assert page.locator("#people-count-summary").inner_text() == "Showing 1 of 4 people."
        assert page.locator(".person-row").count() == 1
        assert page.locator("#person-detail").count() == 0
        assert page.locator("#person-detail-dialog").count() == 0
        directory_width = page.locator(".directory-workspace").evaluate(
            "element => Math.round(element.getBoundingClientRect().width)"
        )
        results_width = page.locator(".directory-results").evaluate(
            "element => Math.round(element.getBoundingClientRect().width)"
        )
        assert abs(directory_width - results_width) <= 2

        page.get_by_role("searchbox", name="Search people").fill("not in this workspace")
        assert page.locator("#people-count-summary").inner_text() == "Showing 0 of 4 people."
        assert page.get_by_text(
            "No people match “not in this workspace”. Try another name, email, "
            "phone, or alias, or clear the search.",
            exact=True,
        ).is_visible()

        page.get_by_role("button", name="Clear search").click()
        open_bravo = page.get_by_role(
            "button", name="Open local record for Synthetic Bravo"
        )
        open_bravo.click()
        page.get_by_role("heading", name="Synthetic Bravo", exact=True).wait_for()
        assert page.locator("[data-page='person']").is_visible()
        assert page.locator("#person-contact-details").get_by_text(
            "bravo@example.test", exact=True
        ).count() == 1
        assert page.locator("#person-record-details").get_by_text(
            "Revision 1", exact=True
        ).count() == 1
        assert page.get_by_role("button", name="Back to People").is_visible()

        page.set_viewport_size({"width": 320, "height": 900})
        assert page.evaluate(
            "document.documentElement.scrollWidth <= document.documentElement.clientWidth"
        )
        assert page.locator("#person-page-heading").inner_text() == "Synthetic Bravo"
        assert page.locator("#person-page-heading").evaluate(
            "heading => heading === document.activeElement"
        )
        page.get_by_role("button", name="Back to People").click()
        page.get_by_role("heading", name="People", exact=True).wait_for()
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

        SCREENSHOT.parent.mkdir(parents=True, exist_ok=True)
        page.screenshot(path=str(SCREENSHOT), full_page=True)
        assert external_requests == []
        browser.close()

    print(
        "People directory regression passed: full-width table, search, separate "
        "read-only Person surface, 320 CSS-pixel reflow, focus return, and zero "
        "external requests"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
