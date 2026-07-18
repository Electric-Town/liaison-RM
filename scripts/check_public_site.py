#!/usr/bin/env python3
"""Validate the static public site without fetching third-party resources."""

from __future__ import annotations

import json
import re
import sys
import xml.etree.ElementTree as ET
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import urlparse

ROOT = Path(__file__).resolve().parents[1]
SITE = ROOT / "site"
SITE_URL = "https://electric-town.github.io/liaison-RM/"

REQUIRED_FILES = (
    "index.html",
    "404.html",
    "robots.txt",
    "sitemap.xml",
    "site.webmanifest",
    "assets/site.css",
    "assets/favicon.svg",
    "assets/social-card.svg",
    "assets/social-card.png",
    "assets/desktop-workspace-health.png",
)

PUBLIC_COPY_FILES = (
    SITE / "index.html",
    SITE / "404.html",
    ROOT / "README.md",
    ROOT / "docs/repository-metadata.md",
)

DISALLOWED_WORDS = {
    "best-in-class",
    "crucial",
    "delve",
    "enterprise-ready",
    "foster",
    "future-proof",
    "intricate",
    "intuitive",
    "landscape",
    "leverage",
    "multifaceted",
    "nuanced",
    "pivotal",
    "revolutionary",
    "robust",
    "seamless",
    "showcase",
    "tapestry",
    "underscore",
    "unleash",
    "vibrant",
}


class DocumentParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.start_tags: list[tuple[str, dict[str, str]]] = []
        self.json_ld: list[str] = []
        self._in_json_ld = False
        self._json_parts: list[str] = []
        self.title_parts: list[str] = []
        self._in_title = False

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        values = {key: value or "" for key, value in attrs}
        self.start_tags.append((tag, values))
        if tag == "script" and values.get("type") == "application/ld+json":
            self._in_json_ld = True
            self._json_parts = []
        if tag == "title":
            self._in_title = True

    def handle_endtag(self, tag: str) -> None:
        if tag == "script" and self._in_json_ld:
            self.json_ld.append("".join(self._json_parts))
            self._in_json_ld = False
        if tag == "title":
            self._in_title = False

    def handle_data(self, data: str) -> None:
        if self._in_json_ld:
            self._json_parts.append(data)
        if self._in_title:
            self.title_parts.append(data)


def attr_values(parser: DocumentParser, tag: str, name: str) -> list[str]:
    return [attrs.get(name, "") for item_tag, attrs in parser.start_tags if item_tag == tag]


def tags(parser: DocumentParser, tag: str) -> list[dict[str, str]]:
    return [attrs for item_tag, attrs in parser.start_tags if item_tag == tag]


def local_target(value: str) -> Path | None:
    if not value or value.startswith(("#", "mailto:", "data:")):
        return None
    parsed = urlparse(value)
    if parsed.scheme or parsed.netloc:
        return None
    path = parsed.path.lstrip("/")
    if path.startswith("liaison-RM/"):
        path = path.removeprefix("liaison-RM/")
    return SITE / path


def check_required_files(errors: list[str]) -> None:
    for relative in REQUIRED_FILES:
        if not (SITE / relative).is_file():
            errors.append(f"site: missing required file: {relative}")


def check_index(errors: list[str]) -> None:
    index = SITE / "index.html"
    parser = DocumentParser()
    parser.feed(index.read_text(encoding="utf-8"))

    html_tags = tags(parser, "html")
    if len(html_tags) != 1 or html_tags[0].get("lang") != "en-IE":
        errors.append("site/index.html: html lang must be en-IE")

    if len(tags(parser, "h1")) != 1:
        errors.append("site/index.html: expected exactly one h1")
    if not any(item.get("id") == "main-content" for item in tags(parser, "main")):
        errors.append("site/index.html: missing main#main-content")
    if not any(item.get("href") == "#main-content" for item in tags(parser, "a")):
        errors.append("site/index.html: missing skip link to #main-content")
    if not any(item.get("aria-label") for item in tags(parser, "nav")):
        errors.append("site/index.html: navigation needs an accessible name")

    title = "".join(parser.title_parts).strip()
    if not 20 <= len(title) <= 65:
        errors.append(f"site/index.html: title length is {len(title)}, expected 20-65")

    meta = tags(parser, "meta")
    descriptions = [item.get("content", "") for item in meta if item.get("name") == "description"]
    if len(descriptions) != 1 or not 100 <= len(descriptions[0]) <= 170:
        errors.append("site/index.html: meta description must be 100-170 characters")

    canonical = [
        item.get("href")
        for item in tags(parser, "link")
        if "canonical" in item.get("rel", "").split()
    ]
    if canonical != [SITE_URL]:
        errors.append("site/index.html: canonical URL does not match the Pages URL")

    hreflangs = {
        item.get("hreflang"): item.get("href")
        for item in tags(parser, "link")
        if "alternate" in item.get("rel", "").split() and item.get("hreflang")
    }
    expected_hreflangs = {"en-IE": SITE_URL, "x-default": SITE_URL}
    if hreflangs != expected_hreflangs:
        errors.append(f"site/index.html: hreflang set must be {expected_hreflangs}")

    property_meta = {item.get("property"): item.get("content") for item in meta if item.get("property")}
    for required in ("og:title", "og:description", "og:url", "og:image", "og:image:alt"):
        if not property_meta.get(required):
            errors.append(f"site/index.html: missing {required}")
    if property_meta.get("og:url") != SITE_URL:
        errors.append("site/index.html: og:url does not match the canonical URL")

    scripts = tags(parser, "script")
    if any(item.get("type") != "application/ld+json" for item in scripts):
        errors.append("site/index.html: executable scripts are not allowed")
    if len(parser.json_ld) != 1:
        errors.append("site/index.html: expected one JSON-LD block")
    else:
        try:
            structured = json.loads(parser.json_ld[0])
        except json.JSONDecodeError as exc:
            errors.append(f"site/index.html: invalid JSON-LD: {exc}")
        else:
            graph = structured.get("@graph", [])
            types = {item.get("@type") for item in graph if isinstance(item, dict)}
            if not {"WebSite", "Organization", "SoftwareSourceCode"}.issubset(types):
                errors.append("site/index.html: JSON-LD graph is missing required public entities")
            for item in graph:
                if item.get("@type") in {"WebSite", "SoftwareSourceCode"} and item.get("inLanguage") != "en-IE":
                    errors.append(f"site/index.html: {item.get('@type')} inLanguage must be en-IE")

    for image in tags(parser, "img"):
        if not image.get("alt"):
            errors.append("site/index.html: every img needs non-empty alt text")
        if not image.get("width") or not image.get("height"):
            errors.append(f"site/index.html: image needs width and height: {image.get('src', '')}")

    for src in attr_values(parser, "img", "src"):
        if urlparse(src).scheme:
            errors.append(f"site/index.html: image must be local: {src}")
    for item in tags(parser, "link"):
        rel = set(item.get("rel", "").split())
        if rel.intersection({"stylesheet", "icon", "manifest"}) and urlparse(item.get("href", "")).scheme:
            errors.append(f"site/index.html: site asset must be local: {item.get('href')}")

    local_refs: set[Path] = set()
    for tag, attrs in parser.start_tags:
        if tag in {"img", "script"}:
            target = local_target(attrs.get("src", ""))
            if target:
                local_refs.add(target)
        if tag == "link" and set(attrs.get("rel", "").split()).intersection({"stylesheet", "icon", "manifest"}):
            target = local_target(attrs.get("href", ""))
            if target:
                local_refs.add(target)
    for target in sorted(local_refs):
        if not target.is_file():
            errors.append(f"site/index.html: missing local asset: {target.relative_to(ROOT)}")


def check_css(errors: list[str]) -> None:
    css = (SITE / "assets/site.css").read_text(encoding="utf-8")
    required = (
        ":focus-visible",
        "prefers-reduced-motion",
        "min-width: 320px",
        "[lang=\"ja\"]",
        "line-break: strict",
        "word-break: keep-all",
    )
    for token in required:
        if token not in css:
            errors.append(f"site/assets/site.css: missing resilience rule: {token}")
    if "@import" in css or re.search(r"url\(\s*['\"]?https?://", css, re.IGNORECASE):
        errors.append("site/assets/site.css: external styles or assets are not allowed")


def check_manifest_and_discovery(errors: list[str]) -> None:
    manifest_path = SITE / "site.webmanifest"
    try:
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        errors.append(f"site/site.webmanifest: invalid JSON: {exc}")
    else:
        if manifest.get("lang") != "en-IE":
            errors.append("site/site.webmanifest: lang must be en-IE")
        if manifest.get("start_url") != "/liaison-RM/" or manifest.get("scope") != "/liaison-RM/":
            errors.append("site/site.webmanifest: start_url and scope must match the repository path")
        for icon in manifest.get("icons", []):
            target = local_target(icon.get("src", ""))
            if target and not target.is_file():
                errors.append(f"site/site.webmanifest: missing icon: {icon.get('src')}")

    try:
        sitemap = ET.parse(SITE / "sitemap.xml")
    except ET.ParseError as exc:
        errors.append(f"site/sitemap.xml: invalid XML: {exc}")
    else:
        locs = [item.text for item in sitemap.findall("{http://www.sitemaps.org/schemas/sitemap/0.9}url/{http://www.sitemaps.org/schemas/sitemap/0.9}loc")]
        if locs != [SITE_URL]:
            errors.append("site/sitemap.xml: expected only the canonical homepage URL")

    robots = (SITE / "robots.txt").read_text(encoding="utf-8")
    if f"Sitemap: {SITE_URL}sitemap.xml" not in robots:
        errors.append("site/robots.txt: sitemap URL does not match the Pages URL")


def check_public_copy(errors: list[str]) -> None:
    for path in PUBLIC_COPY_FILES:
        if not path.is_file():
            errors.append(f"public copy: missing file: {path.relative_to(ROOT)}")
            continue
        text = path.read_text(encoding="utf-8")
        lower = text.lower()
        if "\u2014" in text:
            errors.append(f"{path.relative_to(ROOT)}: replace the em dash with plainer punctuation")
        for phrase in ("in today’s digital", "in today's digital", "lorem ipsum", "tbd", "todo"):
            if phrase in lower:
                errors.append(f"{path.relative_to(ROOT)}: remove placeholder or stock phrase '{phrase}'")
        for word in sorted(DISALLOWED_WORDS):
            if re.search(rf"(?<![\w-]){re.escape(word)}(?![\w-])", lower):
                errors.append(f"{path.relative_to(ROOT)}: replace stock word '{word}' with a concrete claim")


def main() -> int:
    errors: list[str] = []
    check_required_files(errors)
    check_index(errors)
    check_css(errors)
    check_manifest_and_discovery(errors)
    check_public_copy(errors)
    if errors:
        print("Public site validation failed:")
        for error in sorted(set(errors)):
            print(f"- {error}")
        return 1
    print("Public site validation passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
