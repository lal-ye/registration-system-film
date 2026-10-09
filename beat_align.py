#!/usr/bin/env python3
"""Nudge each scene's felt onsets onto the bed's beat grid.

Deliberate Thought runs at 136 BPM; `grid.py` found period 0.4412 s with a beat at
0.234 s of the trimmed file, so beats fall at 0.234 + k*0.4412.

For each scene we search a single shift in [-0.2, +0.2] s -- the budget the
grilling decision allowed -- and keep the one that puts the most onsets on a
beat. Intra-group spacing is never touched, so the seven DAO rows stay seven rows.
"""
PERIOD, PHASE = 0.4412, 0.234
BUDGET = 0.2


def beats(t):
    """Signed distance from t to the nearest beat."""
    k = round((t - PHASE) / PERIOD)
    return t - (PHASE + k * PERIOD)


def best(onsets):
    scored = []
    for step in range(-int(BUDGET / 0.005), int(BUDGET / 0.005) + 1):
        shift = step * 0.005
        err = [beats(t + shift) for t in onsets]
        scored.append((sum(abs(e) for e in err), shift, err))
    scored.sort()
    return scored[0]


SCENES = {
    "cold_open":    [1.9],
    "roles":        [10.0, 13.9, 17.9],
    "login":        [22.7, 24.3, 26.4, 27.3, 29.6, 29.9],
    "architecture": [33.6 + i * 1.7648 for i in range(7)],
    "movement_one": [48.3, 50.9, 54.7],
    "movement_two": [62.1, 64.7, 69.4],
    "payoff":       [77.2, 78.6, 79.4, 80.3, 84.6],
    "credits":      [90.6, 92.3, 92.6],
}

total_before = total_after = 0.0
for name, onsets in SCENES.items():
    before = sum(abs(beats(t)) for t in onsets)
    cost, shift, err = best(onsets)
    total_before += before
    total_after += cost
    worst = max(abs(e) for e in err)
    print(
        f"{name:<13} shift {shift:+.3f}s  mean err {before/len(onsets):.3f} -> "
        f"{cost/len(onsets):.3f}s  worst {worst:.3f}s"
    )
print(f"\ntotal misalignment {total_before:.2f}s -> {total_after:.2f}s")