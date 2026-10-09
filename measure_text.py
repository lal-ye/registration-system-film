#!/usr/bin/env python3
"""Measure the real advance width of every statement line in Instrument Serif.

The film sets statements at x=192 on a 1920 canvas, so a line may use
1920 - 192 - 192 = 1536 px before it clips. This prints each line's true width
so we know which overflow and by how much.
"""
import sys
from fontTools.ttLib import TTFont

FONT = "media/InstrumentSerif-Italic.ttf"
UNITS = 1000.0          # unitsPerEm

LINES = [
    ("how.", 190),
    ("shared rules. one place.", 190),
    ("Student", 150),
    ("Instructor", 150),
    ("Head of Department", 150),
    ("one fact, one place.", 190),
    ("illegal states don't exist.", 190),
    ("the machinery stays hidden.", 190),
    ("same call. no idea which.", 190),
    ("University Registration System", 110),
]

AVAIL = 1920.0 - 192.0 - 192.0


def width_of(font, text, size):
    cmap = font.getBestCmap()
    hmtx = font["hmtx"]
    upem = font["head"].unitsPerEm
    total = 0
    missing = []
    for ch in text:
        gname = cmap.get(ord(ch))
        if gname is None:
            missing.append(ch)
            continue
        total += hmtx[gname][0]
    return total / upem * size, missing


def main():
    f = TTFont(FONT)
    print(f"upem {f['head'].unitsPerEm}   italicAngle {f['post'].italicAngle}")
    print(f"available width at x=192 with 192 right margin: {AVAIL:.0f}px\n")
    worst = 0.0
    for text, size in LINES:
        w, missing = width_of(f, text, size)
        over = w - AVAIL
        flag = f"OVERFLOW +{over:.0f}px" if over > 0 else f"fits ({AVAIL-w:.0f}px spare)"
        worst = max(worst, over)
        mm = f"  MISSING GLYPHS: {missing}" if missing else ""
        print(f"{w:7.0f}px  {size:3d}pt  {text!r:42s} {flag}{mm}")
    print(f"\nworst overflow: {worst:.0f}px")
    if worst > 0:
        # what size makes the longest line fit?
        longest = max(LINES, key=lambda t: width_of(f, t[0], t[1])[0])
        w, _ = width_of(f, longest[0], longest[1])
        fitted = longest[1] * AVAIL / w
        print(f"longest line {longest[0]!r} needs size <= {fitted:.1f}pt to fit")


main()