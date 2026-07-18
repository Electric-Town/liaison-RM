#!/usr/bin/env python3
"""Generate deterministic Liaison RM application icons."""

from __future__ import annotations

import argparse
import filecmp
import tempfile
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
DESTINATION = ROOT / "apps" / "desktop" / "src-tauri" / "icons"


def base_icon(size: int) -> Image.Image:
    image = Image.new("RGBA", (size, size), (23, 32, 51, 255))
    draw = ImageDraw.Draw(image)
    margin = round(size * 0.13)
    radius = round(size * 0.18)
    draw.rounded_rectangle(
        (margin, margin, size - margin, size - margin),
        radius=radius,
        fill=(49, 87, 200, 255),
    )
    stroke = max(2, round(size * 0.055))
    left = round(size * 0.32)
    right = round(size * 0.68)
    top = round(size * 0.29)
    bottom = round(size * 0.71)
    middle = round(size * 0.5)
    draw.line((left, top, left, bottom, middle, bottom), fill="white", width=stroke, joint="curve")
    draw.arc((middle - stroke, top, right, bottom), start=270, end=90, fill="white", width=stroke)
    draw.line((middle, top, middle, bottom), fill="white", width=stroke)
    return image


def write_assets(destination: Path) -> None:
    destination.mkdir(parents=True, exist_ok=True)
    base_icon(32).save(destination / "32x32.png", format="PNG", optimize=False)
    base_icon(128).save(destination / "128x128.png", format="PNG", optimize=False)
    base_icon(256).save(destination / "128x128@2x.png", format="PNG", optimize=False)
    base_icon(1024).save(destination / "icon.icns", format="ICNS")


def check_assets() -> int:
    with tempfile.TemporaryDirectory() as temporary:
        generated = Path(temporary)
        write_assets(generated)
        expected = ["32x32.png", "128x128.png", "128x128@2x.png", "icon.icns"]
        mismatches = [
            name for name in expected
            if not (DESTINATION / name).is_file()
            or not filecmp.cmp(generated / name, DESTINATION / name, shallow=False)
        ]
        if mismatches:
            print(f"Desktop assets differ from generator output: {', '.join(mismatches)}")
            return 1
    print("Desktop asset check passed")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    if args.check:
        return check_assets()
    write_assets(DESTINATION)
    print(f"Generated desktop assets in {DESTINATION.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
