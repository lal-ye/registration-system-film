#!/usr/bin/env python3
"""Measure UI strings in the real fonts, so windows can't overflow.

The Swing recreations are drawn as SVG at final size, so a string that is too
long for its box will visibly run over its neighbour. This checks every string
that has a container: combo values, tab labels, buttons, and the mono lines in
the two movements.
"""
from fontTools.ttLib import TTFont

FONTS = {
    "Inter-Bold": "media/Inter-Bold.ttf",
    "JetBrainsMono-Regular": "media/JetBrainsMono-Regular.ttf",
    "InstrumentSerif-Italic": "media/InstrumentSerif-Italic.ttf",
}

_cache = {}


def font(name):
    if name not in _cache:
        f = TTFont(FONTS[name])
        cmap = f.getBestCmap()
        hmtx = f["hmtx"]
        upem = f["head"].unitsPerEm
        adv = {}
        for cp, gname in cmap.items():
            adv[cp] = hmtx[gname][0] / upem
        _cache[name] = adv
    return _cache[name]


def width(name, text, size):
    adv = font(name)
    return sum(adv.get(ord(c), adv.get(ord("n"), 0.5)) for c in text) * size


def check(label, fname, text, size, box, pad=0.0):
    w = width(fname, text, size)
    avail = box - pad
    flag = "" if w <= avail else f"  OVERFLOW +{w - avail:.0f}px"
    print(f"{w:7.0f}px / {avail:5.0f}px  {label:34s} {text!r}{flag}")
    return w <= avail


print("=== combo values (Inter Bold 15), box 240 with 12px left pad + 24 for arrow")
check("course combo (240)", "Inter-Bold",
      "CS101 — Introduction to Programming", 15, 240, 36)
check("student combo (300)", "Inter-Bold", "1001 — Abebe Kebede", 15, 300, 36)

print("\n=== buttons (Inter Bold 15)")
check("Save Scores & Calc (380)", "Inter-Bold",
      "Save Scores & Calculate Grade", 15, 380, 24)
check("Register as Student (190)", "Inter-Bold", "Register as Student", 15, 190, 24)
check("Enroll in Selected (260)", "Inter-Bold", "Enroll in Selected Course", 15, 260, 24)
check("Drop Selected (250)", "Inter-Bold", "Drop Selected Course", 15, 250, 24)

print("\n=== result line (Inter Bold 20), window inner width")
check("grade result", "Inter-Bold",
      "Scores saved. Weighted total: 97.0% → Grade: A+", 20, 970)

print("\n=== movement two: mono 19 on the LEFT half of a split screen")
for line in [
    "EnrollResult enroll(int studentId,",
    "                    String courseCode);",
    "public EnrollResult enroll(int studentId,",
    "                            String courseCode) {",
]:
    check("split line", "JetBrainsMono-Regular", line, 19, 900)

print("\n=== movement two: mono 19 on the RIGHT half")
for line in [
    'String call = "{call sp_enroll_student(?, ?)}";',
    "try (Connection conn = DBConnection",
    "         .getConnection();",
    "     CallableStatement stmt =",
    "         conn.prepareCall(call)) {",
    "} catch (SQLException e) {",
    "  if (msg.contains(",
    '      "PREREQUISITES_NOT_MET"))',
    "    return EnrollResult.SUCCESS;",
]:
    check("split line", "JetBrainsMono-Regular", line, 19, 900)

print("\n=== architecture dot labels (mono 27)")
for line in [
    'RegistrationDAO.enroll(1814, "CS101")',
    "{call sp_enroll_student(?, ?)}",
    "StudentDashboardFrame.repaint()",
    "new Registration(...)",
    "StudentDashboardController",
]:
    check("dot label", "JetBrainsMono-Regular", line, 27, 1300)

print("\n=== branch strip (mono 19)")
for line in [
    "if (authenticateStudent(1814, ••••••) != null)",
    "    return new StudentDashboardFrame();",
    "if (authenticateInstructor(1814, ••••••) != null)",
    "if (isHeadOfDepartment(1814))",
]:
    check("branch", "JetBrainsMono-Regular", line, 19, 1500)