# University Registration System — 96s Film: Full Build Record

One place holding everything that made the video: the tools, the scene-by-scene
map, and the complete code. The film lives at `~/fframes/registration-system-film`
(repo `lal-ye/registration-system-film`); the verified final encode is
`~/fframes/registration-system-film-out/out.mp4`
(h264, 1920×1080, 30 fps, 2880 frames = 96.000 s, AAC audio 96 s).

Source of truth for wording: `University_Registration_System_Report.pdf`
(Actor table p4, class diagram p8, architecture §§5.1–5.5, auth logic §6.6,
conclusion p27, team list p1).

---

## 1. Tools used

| Tool | What it did |
|---|---|
| **fframes 1.2.0** (Rust, CPU backend, `compile-time-svgtree` + `cli` features, `h264` + `libav-agree-gpl` on non-Windows) | The whole video: `render_frame` returns an SVG tree per frame; scenes, `timeline!`-style runtime animation, audio map, ffmpeg encode. |
| **GitHub Actions** (`.github/workflows/render.yml`, full text in §4) | All rendering — this machine has no GPU and ~5 GB free disk, so nothing was ever built locally. Pipeline per run: `timeline` → `inspect` (gates, exits 2 on problems) → `strip -n 12` contact sheet → `frame 1s,50%,end` stills → full `render` → `out.mp4` + `review-images` artifacts. `FFMPEG_FORCE_BUILD=1`, `FFMPEG_MARCH=x86-64`, `FFMPEG_MTUNE=generic`, `libx264-dev` installed (prebuilt-ffmpeg/x264-163-vs-164 ABI fix). |
| **`~/bin/ffr.sh` + `~/bin/gh-artifact.sh`** | Commit → push → wait for the right run → download artifacts (`gh run download` stalls on this machine) → `ffprobe`-verify frame count = duration × fps. `--review-only` for PNG-only iterations. |
| **Python `fontTools`** (`measure_text.py`, `measure_ui.py`) | Measured real glyph advance widths from the TTFs so no line can overflow its box. Drove the `condensed()` auto-shrink in `design.rs` and the login-card widening (720 → 840). |
| **`beat_align.py`** | Snapped scene onsets to the bed's beat grid (136 BPM, period 0.4412 s, phase 0.234 s): one ±0.2 s shift per scene, intra-group spacing untouched. |
| **ffmpeg (local, SFX surgery)** | Trimmed every SFX to a short hit with the transient at t=0 and a faded tail (stereo 44.1 kHz WAV, peak ≈ −3 dBFS): `click_hit.wav` 0.15 s, `beep_hit.wav` 0.125 s, `tick_hit.wav` 0.14 s (attack only — the source file's 0.9 s tail is sustained −9…−12 dB static, the root cause of the static complaint), `swoosh_hit.wav` 1.36 s, `whoosh_rise.wav` 3.35 s, `chime_hit.wav` 5.85 s wash (the chime has no transient anywhere — slow swell, so pad use only), `tada_hit.wav` 3.15 s. Synthesised `bell_hit.wav` (numpy: 880 Hz partials 1/2.76/5.40/8.93 + 220 Hz sub + 8 ms noise strike, 1.2 s decay) as the anchor the chime moments were missing. Cut `phrase_loop.wav` (17.49 s): bed 2-bar phrase 69.06–72.59 s on the 0.4412 s beat grid, tiled 5× with 40 ms equal-power crossfades. Full-decode + `atrim` was required (input `-ss` on MP3 re-adds ~48 ms decoder-delay silence). `silencedetect`/`astats`/`volumedetect` + a >6 kHz highpass check diagnosed the mix (arch region HF mean −37 dB vs bed-only −66 dB, +29 dB from the layered tick). |
| **ffmpeg / ffprobe (local)** | `silencedetect` proved bed.mp3 is truly silent 75.79 s → 96 s (so SFX carry Payoff + Credits); `volumedetect` on the final encode proved the SFX are in the mix (chime region max −9.8 dB, whoosh tail −20.2 dB); `ffprobe` verified 2880 frames / 96.000 s. |
| **Lightpanda browser + curl** | Re-found the 7 picked SFX on orangefreesounds (category pages are Cloudflare-walled; resolved via site search + HTML grep) and downloaded the MP3s into `media/`. |
| **Music / SFX sources** | Bed: "Deliberate Thought" by Kevin MacLeod (CC BY 4.0, normalised to −16 LUFS, 2 s fade-in). SFX: orangefreesounds.com (CC BY-NC 4.0, class-only non-commercial use): Plastic Button Click, UI Button Click Beep, Glitch Button Click, Quick Swoosh, Futuristic Rising Whoosh, Soothing Chime, Soft Ta-Da — all cleaned to short `*_hit.wav` files (see ffmpeg row; original MP3s deleted), plus synthesised `bell_hit.wav` and bed-derived `phrase_loop.wav`. |
| **Fonts in `media/`** | `InstrumentSerif-Italic.ttf` (statements), `Inter-Bold.ttf` (system/UI), `JetBrainsMono-Regular.ttf` (proof/code). |

Render loop used throughout: push → CI review-only → judge `strip.png` + full-size
stills as images → fix → repeat; full encode only when stills passed.

---

## 2. How the video is mapped out

### 2.1 Design lock (global)

- Canvas 1920×1080, 30 fps. Palette: paper `#f4f1ea`, ink `#14110d`, rust
  `#b4472a`, card white — nothing else has a colour (except the one green line
  Swing itself paints: the grade-result label `#0d7a34`).
- Type roles: Instrument Serif = ideas/statements, Inter Bold = the system/UI,
  JetBrains Mono = proof/code. Statements set at x=192 (1536 px available),
  auto-condensed by measured serif advance (0.411 em) so long lines shrink
  instead of clipping.
- CPU-safe "life" only (no blur/SkSL/SVG filters): vector paper grain
  (`grain.rs`: blotches + ~2600 edge-biased specks = the vignette), stacked-rect
  soft shadows, rust hairline rules, halo pulse rings on the architecture dot.
- Motion grammar: entrances spring/ease-out 300–600 ms, exits faster; shared
  runtimes `EASE / SOFT / SNAP / DECEL / SPRING / LINEAR` + helpers `ramp`,
  `scrub`, `seg`, `decel`, `rise`, `in_out`, `retire`, `after`, `typed`, `caret`.
- Bookend: cold open resolves `97.0` → giant `A+` at the same page position
  (`LETTER_X/Y = 960/700`); payoff returns the same letter; credits reuses the
  title card geometry (`NAME_Y / AFFIL_1_Y / AFFIL_2_Y` shared consts).

### 2.2 Beat sheet (film time = cumulative)

| # | Scene (file) | Film time | Dur | What happens |
|---|---|---|---|---|
| 1 | ColdOpen (`cold_open.rs`) | 0–4 | 4 s | Mono `97.0` (real weighted total, unexplained) with blinking caret → cross-dissolves into giant serif `A+`. |
| 2 | Rewind (`rewind.rs`) | 4–6 | 2 s | One word: `how.` + rust rule. The cut turning a result into a question. |
| 3 | Title (`title.rs`) | 6–8 | 2 s | `University Registration System` + St. Mary's / CS / OOP affiliations. Credits card reuses this exact geometry. |
| 4 | Thesis (`thesis.rs`) | 8–10 | 2 s | `shared rules. one place.` + 3 real DB rule constants (`PREREQUISITES_NOT_MET`, `ROOM_CONFLICT`, `INSTRUCTOR_CONFLICT`). Swoosh @1.7 rings across the cut. |
| 5 | Roles (`roles.rs`) | 10–22 | 12 s | 3 roles × 4 s, `in_out(at, 3.15, 0.40)`: Student → `EnrollResult enroll(…)`; Instructor → `sp_updategrade(?, ?, ?)`; Head of Department → `sp_create_course_full(…)`; each with a grey Actor-table description line. Click under each card. |
| 6 | Login (`login.rs`) | 22–33 | 11 s | True-cps typed ID `1814` (keys @0.75–1.35) + masked password (@2.29–3.01), press-peak click @4.6, release @5.0, dashboard cross-dissolve @5.15 (beep + swoosh), then the 4-line auth decision strip @7.75+i·0.2 (per-line ticks): student branch `taken` (rust), instructor + HoD branches dimmed `not reached`. Divider at y=792 clears the card shadow. |
| 7 | Architecture (`architecture.rs`) | 33–48 | 15 s | VIEW / CONTROLLER / DAO / MODEL·UTIL bands (real class lists) + MySQL strip. A rust dot rides a right gutter rail through 7 hops on one clock (`T0 = 0.53`, `depart(k) = 1.03+k·HOP`, arrive +1.0 s; `HOP = 1.7648 s` = 4 beats @136 BPM); hop labels hold the full hop (last holds to end); electrified rail (bolt arc in flight, spark-burst + ring + squash per arrival); dot goes dark at the SQL hop, then flickers to a dim ember. Short tick per departure + click per arrival, swoosh @14.7 into the cut. |
| 8 | MovementOne (`movement_one.rs`) | 48–62 | 14 s | `one fact, one place.` hard-on at t=0 (no blank 48 s boundary frame) + entrance paper-wipe 0–0.3 → `User.java` gender-guard proof with per-keystroke taps (throw = tick @3.24; Encapsulation tag with wash + bell @3.26) → abstract-`User` + near-empty Student/Instructor boxes (field ticks @7.51/7.81; Inheritance tag with wash + bell @8.11) → landing `illegal states don't exist.` (click @10.56). |
| 9 | MovementTwo (`movement_two.rs`) | 62–76 | 14 s | `the machinery stays hidden.` → 7 `XDAO → XDAOImp` pairs ticking in on the beat (Abstraction tag with wash + bell @5.855) → interface/implementation crossing with centre divider (swoosh @7.355, divider tick @7.455; Polymorphism + overriding tags; cross fully retires ~11.25) → landing `same call. no idea which.` @11.35 (wash + bell) + ink flash @12.0, and the rising whoosh 11.0–14.0 (faded) peaking exactly on the 76 s cut. |
| 10 | Payoff (`payoff.rs`) | 76–90 | 14 s | Grade-entry window: scores 29/20/48 typed @true-cps onsets (real maxima 30/20/50), save-press click @3.725, release @4.07, bell CONFIRM on the green `Scores saved. Weighted total: 97.0% → Grade: A+` @4.325; window recedes @7.825 (swoosh), the `A+` letter arrives @8.625 with the full chime + bell (bed is silent here — SFX carry it) + `MVC + DAO` caption @9.4 (tick). |
| 11 | Credits (`credits.rs`) | 90–96 | 6 s | Title geometry + 5 names/IDs, music + SFX + repo credit lines; everything holds, fade floors at **0.8** so the last frame stays legible. Ta-da @0.6. |

Durations sum: 4+2+2+2+12+11+15+14+14+14+6 = **96 s**.

### 2.3 Audio map (all scene-relative; signed dB; cleaned `*_hit.wav` assets only)

Film-global (`lib.rs`): `bed.mp3` 0 → 76.2, `fade_out(1.8)`; `phrase_loop.wav`
74.36 → 92.0, `fade_in(1.8)` + `fade_out(4.0)` — the 2-bar loop crossfades in
under the bed's own fade and carries 76.2–91.85, so the last 20 s are scored,
not just SFX. Every cue ≤ 0.35 s except named washes; every range end carries a
fade (no hard-truncated slices — the second static source).

ColdOpen bell@2.4 −24 (A+ ramp). Rewind click@0.15 −26, swoosh@0.9 −22
(`fade_out` 0.3). Title click@0.1 −26. Thesis click@0.05 −26, ticks@0.45/0.61/0.77
−24 (rule draws), swoosh@1.7 −16 (`fade_out` 0.3). Roles clicks@0/3.91/7.91 −20 +
ticks@+1.1 −26 (desc lines). Login: ID keys@0.75/0.95/1.15/1.35 −24 (true-cps),
MASK keys@2.29/2.44/2.58/2.72/2.87/3.01 −24, press-peak click@4.6 −20, release
click@5.0 −24, swoosh@5.0 −16, beep@5.15 −18 (dashboard ramp), strip
ticks@7.75/7.95 −22 + @8.15/8.35 −28. Architecture: `tick_hit`@depart(k) =
1.03+k·HOP −20 + `click_hit`@arrive(k) = depart+1.0 −26 (k = 0..6), swoosh@14.7
−16 (`fade_out` 0.3). MovementOne: click@0.05 −24; guard line clicks@2.96/3.10/
3.38/3.52 −24 + tick@3.24 (throw) −22; wash 2.96–4.0 −16 (`fade_out` 0.4) +
bell@3.26 −20; ticks@7.51/7.81 −22; wash 7.81–8.9 −16 + bell@8.11 −20;
click@10.56 −24 (landing). MovementTwo: click@0.105 −26; 7 row
ticks@2.855+i·0.42 −24; wash 5.55–6.9 −16 + bell@5.855 −20; swoosh@7.355 −18
(cross start); tick@7.455 −20 (divider); wash 11.05–12.6 −16 + bell@11.35 −16
(landing); whoosh@11.0–14.0 −10 (`fade_out` 1.0). Payoff: score
keys@1.39/1.56/2.09/2.26/2.79/2.96 −24 (true-cps), press click@3.725 −20,
release@4.07 −24, CONFIRM bell@4.325 −20 (green line), swoosh@7.825 −18
(recede), chime 8.6–13.5 −12 (`fade_out` 0.5) + bell@8.625 −14 (A+ arrival),
tick@9.4 −26 (caption). Credits tada@0.6 −12 (`fade_in` 0.02).

### 2.4 Fix history (what the judging passes changed)

`4900e78` PDF-v2 wording + timing (roles `in_out` 3.6/0.45 → 3.15/0.40, arch
2.0 s → 1.7648 s beat grid, `mono_c` centring, login 13 → 11 s, payoff 13 →
14 s, credits 5 → 6 s, OOP tag captions from the report's conclusion). `32d4161`
+ `0cf5ec5` screening fixes (credits fade → floor 0.8, MovementOne thesis
hard-on at t=0, login card 720 → 840 centred + `sans_c` title, MovementTwo
cross retires before landing). `945b663` SFX kit + credits attribution line.
Sound+motion pass (static fix + retime + electricity): `typed()`/`caret()` now
true-cps with `typed_onsets()` (old version saturated via a 1 s scrub runtime —
keystroke cues moved to real onsets), arch hop clock unified to `T0 = 0.53`
(`depart(k) = 1.03+k·HOP`, `arrive = depart+1.0`), 7 overlapping full-file
`tick.mp3` plays replaced by 0.14 s attack-only `tick_hit` + arrival clicks,
all slices given fades, bed extended past its 75.79 s silence by the
`phrase_loop` crossfade, arch rail electrified (2nd-frame bolt arc, 9-spark
arrival bursts, shockwave rings, arrival squash, SQL flicker→ember — all
snapshot-safe), Thesis exit wipe 1.7–2.0, MovementOne entrance wipe 0–0.3,
MovementTwo ink flash @12.0, paper-boil skipped (would break all snapshots).
(`7d651ef`, `5a66fb1`, `2cddd04` are full-encode trigger commits.)

---

## 3. Full code

> Revision note: the excerpts below were captured before the sound+motion
> pass. They are current for all visuals and wording; only the `audio()`
> functions differ — the present ones are given in full in §2.3 (summary) and
> in the working tree. `media/` now holds `bed.mp3`, the nine `*.wav` assets
> (`click_hit`, `beep_hit`, `tick_hit`, `swoosh_hit`, `whoosh_rise`,
> `chime_hit`, `tada_hit`, `bell_hit`, `phrase_loop`) and the three TTFs; the
> seven original SFX MP3s were deleted.

### 3.1 `src/lib.rs` — the video

```rust
//! "University Registration System" — a 96-second film about an object-oriented Java group
//! project, built with fframes. One scene per beat; the music bed is declared once on the video,
//! so it runs continuously across all of them.
pub mod design;
pub mod grain;
pub mod scenes;
pub mod ui;

use fframes::{
    AudioMap, AudioTimestamp::*, AudioTrack, Color, Duration, FFramesContext, Frame, Scene, Scenes,
    Svgr, Video, include_media_dir,
};
use scenes::{architecture, cold_open, credits, login, movement_one, movement_two, payoff, rewind,
             roles, thesis, title};

// Every file of `media/` is embedded in the binary. Fonts are addressed by family name.
include_media_dir!(pub struct RegistrationSystemFilmMedia, "media");

use design::*;

pub struct RegistrationSystemFilmVideo<'a> {
    pub media: &'a RegistrationSystemFilmMedia,
    beats: [Box<dyn Scene + 'a>; 11],
}

impl<'a> RegistrationSystemFilmVideo<'a> {
    pub fn new(media: &'a RegistrationSystemFilmMedia) -> Self {
        Self {
            media,
            beats: [
                Box::new(cold_open::ColdOpen),
                Box::new(rewind::Rewind),
                Box::new(title::Title),
                Box::new(thesis::Thesis),
                Box::new(roles::Roles),
                Box::new(login::Login),
                Box::new(architecture::Architecture),
                Box::new(movement_one::MovementOne),
                Box::new(movement_two::MovementTwo),
                Box::new(payoff::Payoff),
                Box::new(credits::Credits),
            ],
        }
    }
}

impl std::fmt::Debug for RegistrationSystemFilmVideo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegistrationSystemFilmVideo").finish()
    }
}

impl Video for RegistrationSystemFilmVideo<'_> {
    const FPS: usize = 30;
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;
    const BACKGROUND_COLOR: Color = Color::WHITE;

    /// The sum of the beat durations: 4 + 2 + 2 + 2 + 12 + 11 + 15 + 14 + 14 + 14 + 6 = 96 s.
    fn duration(&self) -> Duration<'_> {
        Duration::Auto
    }

    // bed.mp3 is already loudness-normalised to -16 LUFS with a 2 s fade in, so no gain is
    // applied here: the template's gain_db(-16) would attenuate it a second time.
    fn audio(&self) -> AudioMap<'_> {
        AudioMap::from([
            AudioTrack::new("bed.mp3", Second(0.0)..Eof).fade_out(4.0),
        ])
    }

    fn define_scenes(&self) -> Scenes<'_> {
        Scenes::from(self.beats.iter().map(|b| b.as_ref() as &dyn Scene).collect::<Vec<_>>())
    }

    fn render_frame<'a>(&'a self, frame: Frame, ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        fframes::svgr!(
            <svg xmlns="http://www.w3.org/2000/svg" width={WIDTH} height={HEIGHT} viewBox="0 0 1920 1080">
                <rect width="1920" height="1080" fill={PAPER} />
                {paper()}
                {ctx.render_scenes(&frame)}
            </svg>
        )
    }
}
```

### 3.2 `src/design.rs` — design system (palette, type, motion helpers)

```rust
//! The film's design system. Every scene draws through this module, which is what keeps the
//! palette and the type scale from eroding one file at a time.
use crate::grain::{BLOTCH, SPECKS};
use fframes::{
    AnimateRuntimeInput, Frame, Svgr, animation::{AnimationRuntime, Easing},
};
use std::sync::LazyLock;

/// Re-exported so every scene gets it from `design::*` rather than importing fframes itself.
pub use fframes::Transform;

pub const WIDTH: usize = 1920;
pub const HEIGHT: usize = 1080;

/// Warm paper, warm ink, one rust accent. Nothing else has a colour.
pub const PAPER: &str = "#f4f1ea";
pub const INK: &str = "#14110d";
pub const RUST: &str = "#b4472a";
pub const CARD: &str = "#ffffff";

/// Serif states ideas, Inter labels the system, mono carries proof.
pub const SERIF: &str = "Instrument Serif";
pub const SANS: &str = "Inter";
pub const MONO_F: &str = "JetBrains Mono";
pub const SANS_W: u16 = 700;
pub const MONO_W: u16 = 400;

/// Type scale. Statements are huge and rare; evidence is small and mono.
pub const STATEMENT: usize = 190;
pub const TITLE: usize = 110;
pub const ROLE: usize = 150;
pub const MONO_HERO: usize = 32;
pub const MONO_LG: usize = 27;
pub const MONO_MD: usize = 23;
pub const MONO_SM: usize = 19;
pub const MONO_XS: usize = 16;
pub const UI: usize = 15;
pub const UI_SM: usize = 13;

/// The letter's page position, shared by the cold open and the payoff so the bookend closes.
pub const LETTER_X: f32 = 960.0;
pub const LETTER_Y: f32 = 700.0;
pub const LETTER_SIZE: usize = 420;

/// `Easing::Linear` is avoided deliberately: a linear cubic-bezier is the same curve and is a
/// variant the template is known to use.
static LINEAR_EASE: LazyLock<Easing> = LazyLock::new(|| Easing::CubicBezier(0.0, 0.0, 1.0, 1.0));
pub static EASE: LazyLock<AnimationRuntime> =
    LazyLock::new(|| AnimationRuntime::new(0.8, &Easing::CubicBezier(0.16, 1.0, 0.3, 1.0)));
pub static SOFT: LazyLock<AnimationRuntime> =
    LazyLock::new(|| AnimationRuntime::new(1.2, &Easing::CubicBezier(0.33, 0.0, 0.15, 1.0)));
pub static SNAP: LazyLock<AnimationRuntime> =
    LazyLock::new(|| AnimationRuntime::new(0.5, &Easing::CubicBezier(0.2, 0.0, 0.0, 1.0)));
pub static LINEAR: LazyLock<AnimationRuntime> =
    LazyLock::new(|| AnimationRuntime::new(1.0, &LINEAR_EASE));
/// Decelerates hard and stops: the curve title work uses so arrivals settle.
pub static DECEL: LazyLock<AnimationRuntime> =
    LazyLock::new(|| AnimationRuntime::new(1.0, &Easing::CubicBezier(0.12, 0.0, 0.0, 1.0)));
pub static SPRING: LazyLock<AnimationRuntime> = LazyLock::new(|| {
    AnimationRuntime::new(3.0, &Easing::Spring { mass: 1.0, stiffness: 180.0, damping: 20.0 })
});

/// 0 to 1 over the easing curve, starting at `start` seconds into the scene.
pub fn ramp(frame: &Frame, start: f32) -> f32 {
    frame.animate_runtime(AnimateRuntimeInput { on_second: start, from: 0.0, to: 1.0, animation_runtime: &EASE })
}

/// 0 to 1 at a constant rate, for scrubbing and counting.
pub fn scrub(frame: &Frame, start: f32) -> f32 {
    frame.animate_runtime(AnimateRuntimeInput { on_second: start, from: 0.0, to: 1.0, animation_runtime: &LINEAR })
}

/// 0 to 1 at a constant rate across an `over`-second window starting at `start`. Used for a
/// multi-second move where the shared 1 s runtimes would be the wrong length.
///
/// Computed from the clock rather than from `scrub`, because `scrub` saturates at 1.0 after
/// its runtime's own 1 s and would silently stop short on any window longer than that.
pub fn seg(frame: &Frame, start: f32, over: f32) -> f32 {
    ((frame.seconds() - start) / over.max(0.001)).clamp(0.0, 1.0)
}

/// 0 -> 1 across `over` seconds, arriving slowly. Anything that travels between
/// two points and then stops should decelerate into place; at constant speed it
/// reads as a machine, and the stop is what makes it read as an arrival.
///
/// The shared runtimes are exactly 1 s and saturate, so for a longer window the
/// ramp is started early instead of being divided down: the shape is kept and the
/// arrival still lands on `start + over`.
pub fn decel(frame: &Frame, start: f32, over: f32) -> f32 {
    frame.animate_runtime(AnimateRuntimeInput {
        on_second: start + (over - 1.0).max(0.0),
        from: 0.0,
        to: 1.0,
        animation_runtime: &DECEL,
    })
}

/// Vertical offset that springs from `from` to 0 at `start`.
pub fn rise(frame: &Frame, start: f32, from: f32) -> f32 {
    frame.animate_runtime(AnimateRuntimeInput { on_second: start, from, to: 0.0, animation_runtime: &SPRING })
}

/// Fades up, holds, then fades out: a statement that will be replaced.
pub fn in_out(frame: &Frame, start: f32, hold: f32, out: f32) -> f32 {
    ramp(frame, start).min(1.0 - ramp(frame, start + hold + out))
}

/// 1 until `start`, then linearly down to `to` over `over` seconds. Retires a line of evidence.
pub fn retire(frame: &Frame, start: f32, over: f32, to: f32) -> f32 {
    let t = ((frame.seconds() - start) / over).clamp(0.0, 1.0);
    1.0 + (to - 1.0) * t
}

/// True once `frame` is past `start`. Cheaper than comparing opacities.
pub fn after(frame: &Frame, start: f32) -> bool {
    frame.seconds() >= start
}

/// The paper ground: mottling plus ~2600 edge-biased specks, all pure vector so nothing can fail
/// `inspect`. The edge bias *is* the vignette, which is why no gradient is needed.
pub fn paper<'a>() -> Svgr<'a> {
    fframes::svgr!(
        <g>
            {BLOTCH}
            <path d={SPECKS} fill={INK} opacity="0.032" />
        </g>
    )
}

/// A soft ink shadow under a white card, faked with stacked rects because the CPU backend
/// cannot do blur filters.
pub fn soft_shadow<'a>(x: f32, y: f32, w: f32, h: f32, r: f32) -> Svgr<'a> {
    let layers: Vec<Svgr> = [(3.0, 4.0, 0.030), (6.0, 8.0, 0.026), (10.0, 13.0, 0.020), (16.0, 20.0, 0.013)]
        .into_iter()
        .map(|(dx, dy, op)| {
            fframes::svgr!(<rect x={x + dx} y={y + dy} width={w} height={h} rx={r} fill={INK} opacity={format!("{op}")} />)
        })
        .collect();
    fframes::svgr!(<g>{layers}</g>)
}

/// A white window card floating on the paper.
pub fn window_card<'a>(x: f32, y: f32, w: f32, h: f32, opacity: f32) -> Svgr<'a> {
    fframes::svgr!(
        <g opacity={opacity}>
            {soft_shadow(x, y, w, h, 10.0)}
            <rect x={x} y={y} width={w} height={h} rx="10" fill={CARD} />
        </g>
    )
}

/// The title bar of a recreated Swing frame.
pub fn title_bar<'a>(x: f32, y: f32, w: f32, title: &'a str) -> Svgr<'a> {
    fframes::svgr!(
        <g font-family={SANS} font-weight={SANS_W}>
            <path d={format!("M{x} {y}h{w}v34h-{w}z")} fill="#eceae4" />
            <circle cx={x + 22.0} cy={y + 17.0} r="5.5" fill="#c9c5bc" />
            <circle cx={x + 41.0} cy={y + 17.0} r="5.5" fill="#c9c5bc" />
            <circle cx={x + 60.0} cy={y + 17.0} r="5.5" fill="#c9c5bc" />
            <text x={x + w / 2.0} y={y + 22.0} text-anchor="middle" font-size={UI_SM} fill="#6d675d">{title}</text>
        </g>
    )
}

/// Widest a left-set statement may be: canvas minus the left margin it starts
/// at, minus an equal right margin.
pub const STMT_AVAIL: f32 = 1536.0;

/// Advance width of one character of the serif, as a fraction of the em.
/// Measured from the real font (Instrument Serif Italic, upem 1000).
const SERIF_ADVANCE: f32 = 0.411;

/// The size a statement will actually be set at: the size it wants, condensed to fit if the
/// line would otherwise bleed off the page. Statements are one idea at one size, so a long
/// line must shrink rather than clip.
fn condensed(text: &str, size: usize) -> usize {
    let chars = text.chars().filter(|c| *c != ' ').count() as f32;
    let spaces = text.chars().filter(|c| *c == ' ').count() as f32;
    let est = (chars * SERIF_ADVANCE + spaces * 0.22) * size as f32;
    if est > STMT_AVAIL { (STMT_AVAIL / est * size as f32) as usize } else { size }
}

/// A statement: the voice, on paper, alone.
pub fn statement<'a>(text: &'a str, size: usize, x: f32, y: f32, opacity: f32, fill: &'a str) -> Svgr<'a> {
    let fitted = condensed(text, size);
    fframes::svgr!(<g opacity={opacity}>
        <text x={x} y={y} font-family={SERIF} font-size={fitted} fill={fill}>{text}</text>
    </g>)
}

/// One line of mono: the proof.
pub fn mono<'a>(text: &'a str, size: usize, x: f32, y: f32, opacity: f32, fill: &'a str) -> Svgr<'a> {
    fframes::svgr!(<g opacity={opacity}>
        <text x={x} y={y} font-family={MONO_F} font-weight={MONO_W} font-size={size} fill={fill}>{text}</text>
    </g>)
}

/// One line of Inter: the system.
pub fn sans<'a>(text: &'a str, size: usize, x: f32, y: f32, opacity: f32, fill: &'a str) -> Svgr<'a> {
    fframes::svgr!(<g opacity={opacity}>
        <text x={x} y={y} font-family={SANS} font-weight={SANS_W} font-size={size} fill={fill}>{text}</text>
    </g>)
}

/// One line of Inter, centred on `x`.
pub fn sans_c<'a>(text: &'a str, size: usize, x: f32, y: f32, opacity: f32, fill: &'a str) -> Svgr<'a> {
    fframes::svgr!(<g opacity={opacity}>
        <text x={x} y={y} text-anchor="middle" font-family={SANS} font-weight={SANS_W}
              font-size={size} fill={fill}>{text}</text>
    </g>)
}

/// A statement centred on `x`.
pub fn statement_c<'a>(text: &'a str, size: usize, x: f32, y: f32, opacity: f32, fill: &'a str) -> Svgr<'a> {
    let fitted = condensed(text, size);
    fframes::svgr!(<g opacity={opacity}>
        <text x={x} y={y} text-anchor="middle" font-family={SERIF} font-size={fitted} fill={fill}>{text}</text>
    </g>)
}

/// Mono, centred: file paths and credit lines.
pub fn mono_c<'a>(text: &'a str, size: usize, x: f32, y: f32, opacity: f32, fill: &'a str) -> Svgr<'a> {
    fframes::svgr!(<g opacity={opacity}>
        <text x={x} y={y} text-anchor="middle" font-family={MONO_F} font-weight={MONO_W}
              font-size={size} fill={fill}>{text}</text>
    </g>)
}

/// Inter, centred, letterspaced: affiliation lines and nothing else.
pub fn affil<'a>(text: &'a str, size: usize, x: f32, y: f32, opacity: f32, fill: &'a str) -> Svgr<'a> {
    fframes::svgr!(<g opacity={opacity}>
        <text x={x} y={y} text-anchor="middle" font-family={SANS} font-weight={SANS_W}
              font-size={size} letter-spacing="3.4" fill={fill}>{text}</text>
    </g>)
}

/// The letter, at the one page position it ever occupies.
pub fn letter(opacity: f32, fill: &str) -> Svgr<'_> {
    fframes::svgr!(
        <g opacity={opacity}>
            <text x={LETTER_X} y={LETTER_Y} text-anchor="middle" font-family={SERIF}
                  font-size={LETTER_SIZE} letter-spacing="-18" fill={fill}>"A+"</text>
        </g>
    )
}

/// Reveals `text` a character at a time. `per_sec` is characters per second.
pub fn typed(frame: &Frame, start: f32, per_sec: f32, text: &str) -> String {
    let shown = (scrub(frame, start) * per_sec * text.chars().count() as f32).floor().max(0.0);
    text.chars().take(shown as usize).collect()
}

/// A caret that blinks while `text` is still being typed, and vanishes when it is not.
pub fn caret(frame: &Frame, start: f32, per_sec: f32, text: &str) -> f32 {
    let done = scrub(frame, start) * per_sec * text.chars().count() as f32;
    if done >= text.chars().count() as f32 {
        0.0
    } else if (frame.seconds() * 2.2).fract() < 0.55 {
        1.0
    } else {
        0.12
    }
}

/// A small rust hairline, the film's only marker of "this is the thing that matters".
pub fn rule<'a>(x: f32, y: f32, w: f32, opacity: f32) -> Svgr<'a> {
    fframes::svgr!(<rect x={x} y={y} width={w} height="3" fill={RUST} opacity={opacity} />)
}
```

### 3.3 `src/ui.rs` — Swing recreations at final size

```rust
//! SVG recreations of the Swing views in `View/*.java`, drawn at final size rather than scaled
//! from the 450x320 originals. FlatLightLaf is a light look-and-feel, so these are white panels
//! with grey chrome and one rust accent where the film needs the eye to go.
//!
//! Every text argument is an owned `String`. `Svgr` implements `From<String>` and `From<&str>` but
//! not `From<&String>` or `From<&&str>`, and borrowing a local would tie the returned SVG to a
//! frame that is about to be dropped -- so these take values and copy them into the tree.
use crate::design::*;
use fframes::Svgr;

/// A text field. `value` is whatever has been typed so far.
pub fn field(x: f32, y: f32, w: f32, h: f32, value: String, size: usize, caret_op: f32) -> Svgr<'static> {
    let text_w = measure(&value, size);
    fframes::svgr!(
        <g>
            <rect x={x} y={y} width={w} height={h} rx="4" fill="#ffffff" stroke="#c9c5bc" stroke-width="1" />
            <text x={x + 14.0} y={y + h / 2.0 + size as f32 * 0.36} font-family={SANS}
                  font-weight={SANS_W} font-size={size} fill={INK}>{value}</text>
            <rect x={x + 18.0 + text_w} y={y + h / 2.0 - size as f32 * 0.55}
                  width="2" height={size as f32 * 1.15} fill={INK} opacity={caret_op} />
        </g>
    )
}

/// Rough advance width for Inter at `size`. Only used to place carets, never to lay out type.
fn measure(text: &str, size: usize) -> f32 {
    text.chars().count() as f32 * size as f32 * 0.56
}

/// A Swing button. `primary` is the one the film wants pressed.
pub fn button(x: f32, y: f32, w: f32, h: f32, label: String, primary: bool, pressed: f32) -> Svgr<'static> {
    let dy = 2.0 * pressed;
    let fill = if primary { RUST } else { "#e6e3dc" };
    let ink = if primary { "#ffffff" } else { INK };
    fframes::svgr!(
        <g opacity={1.0 - 0.12 * pressed}>
            <rect x={x} y={y + dy} width={w} height={h} rx="6" fill={fill} />
            <text x={x + w / 2.0} y={y + dy + h / 2.0 + UI as f32 * 0.36} text-anchor="middle"
                  font-family={SANS} font-weight={SANS_W} font-size={UI} fill={ink}>{label}</text>
        </g>
    )
}

/// A combo box.
pub fn combo(x: f32, y: f32, w: f32, h: f32, value: String) -> Svgr<'static> {
    fframes::svgr!(
        <g>
            <rect x={x} y={y} width={w} height={h} rx="4" fill="#ffffff" stroke="#c9c5bc" stroke-width="1" />
            <text x={x + 12.0} y={y + h / 2.0 + UI as f32 * 0.36} font-family={SANS}
                  font-weight={SANS_W} font-size={UI} fill={INK}>{value}</text>
            <path d={format!("M{} {} l7 0 l-3.5 5 z", x + w - 22.0, y + h / 2.0 - 3.0)} fill="#8a8479" />
        </g>
    )
}

/// A tab bar; the selected tab gets a rust underline.
pub fn tabs(
    x: f32,
    y: f32,
    labels: &[&'static str],
    widths: &[f32],
    selected: usize,
) -> Svgr<'static> {
    let mut out: Vec<Svgr> = Vec::new();
    let mut cx = x;
    for (i, (label, w)) in labels.iter().zip(widths).enumerate() {
        let on = i == selected;
        out.push(fframes::svgr!(<text x={cx} y={y + 30.0} font-family={SANS} font-weight={SANS_W}
            font-size={UI} fill={if on { INK } else { "#8a8479" }}>{*label}</text>));
        if on {
            out.push(fframes::svgr!(<rect x={cx} y={y + 42.0} width={w - 24.0} height="3" fill={RUST} />));
        }
        cx += w;
    }
    fframes::svgr!(<g>{out}<rect x={x} y={y + 42.0} width={cx - x} height="1" fill="#ddd9d0" /></g>)
}

/// A table: a grey header row, hairline rules, mono cells. `rows` are pre-formatted.
pub fn table(
    x: f32,
    y: f32,
    w: f32,
    cols: &[f32],
    header: &[&'static str],
    rows: Vec<Vec<String>>,
    row_h: f32,
    opacity: f32,
) -> Svgr<'static> {
    let mut out: Vec<Svgr> = vec![fframes::svgr!(
        <rect x={x} y={y} width={w} height={row_h} fill="#eeebe4" />
    )];
    let mut cx = x + 14.0;
    for (c, head) in cols.iter().zip(header) {
        out.push(fframes::svgr!(<text x={cx} y={y + row_h * 0.66} font-family={SANS} font-weight={SANS_W}
            font-size={UI_SM} fill="#6d675d">{*head}</text>));
        cx += c;
    }
    for (r, row) in rows.into_iter().enumerate() {
        let ry = y + row_h * (r as f32 + 1.0);
        if r % 2 == 1 {
            out.push(fframes::svgr!(<rect x={x} y={ry} width={w} height={row_h} fill="#f7f5f1" />));
        }
        let mut ccx = x + 14.0;
        for (i, cell) in row.into_iter().enumerate() {
            out.push(fframes::svgr!(<text x={ccx} y={ry + row_h * 0.66} font-family={MONO_F}
                font-weight={MONO_W} font-size={MONO_XS} fill={INK}>{cell}</text>));
            ccx += cols.get(i).copied().unwrap_or(120.0);
        }
        out.push(fframes::svgr!(<rect x={x} y={ry + row_h} width={w} height="1" fill="#e6e3dc" />));
    }
    fframes::svgr!(<g opacity={opacity}>{out}</g>)
}

/// `View/LoginFrame.java` at 1.6x: title, ID field, password field, error label, two buttons.
/// The card is centred on 960 and the title is centred text, so nothing can bleed past an edge.
pub fn login_window(typing: (String, String), caret_id: f32, caret_pw: f32, pressed: f32) -> Svgr<'static> {
    let (x, y, w, h) = (540.0_f32, 250.0_f32, 840.0_f32, 520.0_f32);
    let (id_text, pw_text) = typing;
    fframes::svgr!(
        <g>
            {window_card(x, y, w, h, 1.0)}
            {title_bar(x, y, w, "University Registration System — Login")}
            {sans_c("University Registration System", 26, x + w / 2.0, y + 104.0, 1.0, INK)}
            {sans("Student / Instructor ID:", 21, x + 104.0, y + 180.0, 1.0, INK)}
            {field(x + 420.0, y + 152.0, 276.0, 40.0, id_text, 19, caret_id)}
            {sans("Password:", 21, x + 104.0, y + 255.0, 1.0, INK)}
            {field(x + 420.0, y + 227.0, 276.0, 40.0, pw_text, 19, caret_pw)}
            {sans(" ", 19, x + 104.0, y + 316.0, 1.0, INK)}
            {button(x + 140.0, y + 346.0, 200.0, 58.0, "Login".to_string(), true, pressed)}
            {button(x + 360.0, y + 346.0, 336.0, 58.0, "Register as Student".to_string(), false, 0.0)}
        </g>
    )
}

/// `View/StudentDashboardFrame.java` at ~1.05x, My Enrollments selected.
pub fn student_dashboard(enrolled: Vec<Vec<String>>, drop_op: f32) -> Svgr<'static> {
    let (x, y, w, h) = (461.0_f32, 110.0_f32, 997.0_f32, 651.0_f32);
    fframes::svgr!(
        <g>
            {window_card(x, y, w, h, 1.0)}
            {title_bar(x, y, w, "Student Dashboard")}
            {sans("Welcome, Abebe Kebede", 20, x + 30.0, y + 78.0, 1.0, INK)}
            {button(x + w - 128.0, y + 48.0, 98.0, 40.0, "Logout".to_string(), false, 0.0)}
            {tabs(x + 30.0, y + 108.0,
                  &["My Enrollments", "Available Courses", "My Schedule", "My Grades"],
                  &[250.0, 260.0, 210.0, 200.0], 0)}
            {table(x + 30.0, y + 172.0, w - 60.0, &[180.0, 300.0, 140.0, 140.0],
                   &["ID", "Course", "Section", "Status"], enrolled, 40.0, 1.0)}
            {button(x + 30.0, y + h - 78.0, 250.0, 46.0, "Drop Selected Course".to_string(), false, 0.0)}
            <rect x={x + 30.0} y={y + h - 78.0} width={250.0} height="46" rx="6" fill="none"
                  stroke={RUST} stroke-width="2" opacity={drop_op} />
        </g>
    )
}

/// `View/InstructorDashboardFrame.java` at 1.0x, Grade Entry tab, mid-grade-entry.
pub fn grade_entry(
    course: String,
    student: String,
    scores: [String; 3],
    caret: f32,
    result: String,
    result_op: f32,
    pressed: f32,
) -> Svgr<'static> {
    let (x, y, w, h) = (435.0_f32, 130.0_f32, 1050.0_f32, 640.0_f32);
    // Real assessment maxima from sql/seed_data.sql: Midterm 30, Project 20, Final 50.
    let rows = [("Midterm", 30u32), ("Project", 20), ("Final", 50)];
    let mut score_fields: Vec<Svgr> = Vec::new();
    for (i, (name, max)) in rows.iter().enumerate() {
        let ry = y + 250.0 + i as f32 * 62.0;
        score_fields.push(fframes::svgr!(<text x={x + 220.0} y={ry + 30.0} font-family={SANS}
            font-weight={SANS_W} font-size={UI} fill={INK}>{*name}</text>));
        let limit = format!("/ {max}");
        score_fields.push(fframes::svgr!(<text x={x + 360.0} y={ry + 30.0} text-anchor="end"
            font-family={MONO_F} font-weight={MONO_W} font-size={MONO_XS} fill="#6d675d">{limit}</text>));
        let value = scores[i].clone();
        score_fields.push(field(x + 400.0, ry + 4.0, 130.0, 40.0, value, 18, if i == 2 { caret } else { 0.0 }));
    }
    fframes::svgr!(
        <g>
            {window_card(x, y, w, h, 1.0)}
            {title_bar(x, y, w, "Instructor Dashboard")}
            {sans("Welcome, Dr. Mohammed Tekola", 20, x + 30.0, y + 78.0, 1.0, INK)}
            {button(x + w - 128.0, y + 48.0, 98.0, 40.0, "Logout".to_string(), false, 0.0)}
            {tabs(x + 30.0, y + 108.0, &["My Schedule", "My Students", "Grade Entry"], &[230.0, 240.0, 230.0], 2)}
            {sans("Course:", 18, x + 40.0, y + 200.0, 1.0, INK)}
            {combo(x + 110.0, y + 174.0, 320.0, 40.0, course)}
            {sans("Student:", 18, x + 455.0, y + 200.0, 1.0, INK)}
            {combo(x + 535.0, y + 174.0, 250.0, 40.0, student)}
            {button(x + 800.0, y + 174.0, 190.0, 40.0, "Load Scores".to_string(), false, 0.0)}
            <rect x={x + 40.0} y={y + 232.0} width={w - 80.0} height="200" rx="6" fill="#f7f5f1" stroke="#e6e3dc" stroke-width="1" />
            {score_fields}
            {result_line(result, 20, x + 40.0, y + 486.0, result_op)}
            {button(x + 40.0, y + h - 84.0, 380.0, 52.0, "Save Scores & Calculate Grade".to_string(), true, pressed)}
        </g>
    )
}

/// `gradeResultLabel`. The real code sets `new Color(0,120,0)`; kept, so the only green in the
/// film is the one line Swing actually paints green.
fn result_line(text: String, size: usize, x: f32, y: f32, opacity: f32) -> Svgr<'static> {
    fframes::svgr!(
        <g opacity={opacity}>
            <text x={x} y={y} font-family={SANS} font-weight={SANS_W} font-size={size}
                  fill="#0d7a34">{text}</text>
        </g>
    )
}
```

### 3.4 `src/grain.rs` — generated paper grain (summarised, not pasted)

Generated file — do not hand-edit (regen snippet per its header). Structure:

```rust
//! Generated paper grain. Regenerate with the python snippet in README.md; do not hand-edit.

/// Large, very low opacity ellipses: the low-frequency blotch of real paper.
pub const BLOTCH: &str = r##"<g><ellipse cx="..." ... opacity="0.013"/>... (25 ellipses)</g>"##;

/// ~2600 specks in a single path element (edge-biased density is the vignette).
pub const SPECKS: &str = r##"M102 868h3v3h-3zM416 465h5v1h-5zM720 ... (thousands of 2-8px rects)"##;
```

### 3.5 `src/main.rs` — CLI + encoder settings

```rust
use fframes::{EncoderOptions, RenderOptions, StaticMediaProvider, cli};
use registration_system_film::{ RegistrationSystemFilmMedia, RegistrationSystemFilmVideo };
use fframes::cli::clap; // the derive below expands to `clap::...`
use std::process::ExitCode;

/// Flags of this video next to the standard ones of `fframes::cli` (render, frame, strip,
/// inspect, audio, ...). Run `cargo run --release -- --help`.
#[derive(Debug, clap::Args)]
struct VideoArgs {
}

fn main() -> ExitCode {
    let args = cli::parse::<VideoArgs>();
    let media = RegistrationSystemFilmMedia::prepare().expect("media");
    let video = RegistrationSystemFilmVideo::new(&media);

    cli::new(
        &video,
        RenderOptions {
            media: Some(&media),
            video_encoder_options: EncoderOptions {
                preferred_encoder: Some("libx264"),
                codec_params: Some(&[("crf", "20"), ("preset", "medium"), ("tune", "animation")]),
                ..Default::default()
            },
            ..Default::default()
        },
    )
        .args(args)
        .run()
}
```

### 3.6 `src/scenes/mod.rs`

```rust
//! One scene per beat. `frame.seconds()` is seconds into the scene, so the numbers in each
//! `render_frame` match the beat sheet directly.
pub mod architecture;
pub mod cold_open;
pub mod credits;
pub mod login;
pub mod movement_one;
pub mod movement_two;
pub mod payoff;
pub mod rewind;
pub mod roles;
pub mod thesis;
pub mod title;
```

### 3.7 Scenes

#### `cold_open.rs` (0–4 s)

```rust
//! 0-4s. One number on paper: 97.0, the film's real weighted total, legible and unexplained.
//! It resolves into the enormous `A+`, which is a mystery here and a repayment at 86s.
use crate::design::*;
use fframes::{Duration, FFramesContext, Frame, Scene, Svgr};

/// Advance width of one character of the mono, as a fraction of the em.
const MONO_ADV: f32 = 0.6;
const TOTAL: &str = "97.0";
const FIGURE: usize = 300;

/// When the number starts leaving and the letter starts arriving: one cross-dissolve, so the
/// second reads as the explanation of the first rather than a replacement for it.
const HANDOFF: f32 = 2.0;

#[derive(Debug)]
pub struct ColdOpen;

impl Scene for ColdOpen {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(4.0)
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        // The figure is simply there from the first frame and is otherwise motionless: an
        // empty page is the point, and an empty *first* frame would be a fault.
        let num_op = 1.0 - seg(&frame, HANDOFF, 0.8);
        // The letter takes the same page position, so the two cross-dissolve in place.
        let a_op = ramp(&frame, 2.4);

        let half = TOTAL.chars().count() as f32 * MONO_ADV * FIGURE as f32 / 2.0;
        let mut figure: Vec<Svgr> = vec![fframes::svgr!(<text x={LETTER_X} y={LETTER_Y} text-anchor="middle"
            font-family={MONO_F} font-weight={MONO_W} font-size={FIGURE} letter-spacing="-6"
            fill={INK}>{TOTAL}</text>)];
        // A caret, so the number looks computed rather than printed. It blinks only while the
        // number is still the thing on screen.
        if frame.seconds() < HANDOFF && (frame.seconds() * 2.2).fract() < 0.55 {
            figure.push(fframes::svgr!(<rect x={LETTER_X + half + 14.0} y={LETTER_Y - 196.0}
                width="10" height="230" fill={INK} opacity="0.7" />));
        }

        fframes::svgr!(<g>
            <g opacity={num_op}>
                {figure}
            </g>
            {letter(a_op, INK)}
        </g>)
    }
}
```

#### `rewind.rs` (4–6 s)

```rust
//! 4-6s. One word. The cut that turns a result into a question.
use crate::design::*;
use fframes::{Duration, FFramesContext, Frame, Scene, Svgr};

#[derive(Debug)]
pub struct Rewind;

impl Scene for Rewind {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(2.0)
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        fframes::svgr!(<g>
            <g opacity={ramp(&frame, 0.15)} transform={Transform::translate(0.0, rise(&frame, 0.15, 26.0))}>
                {statement("how.", STATEMENT, 192.0, 620.0, 1.0, INK)}
                {rule(196.0, 672.0, 190.0 * ramp(&frame, 0.9), ramp(&frame, 0.9))}
            </g>
        </g>)
    }
}
```

#### `title.rs` (6–8 s)

```rust
//! 6-8s. The name. The credits card at 91s is this same composition with the names added.
use crate::design::*;
use fframes::{Duration, FFramesContext, Frame, Scene, Svgr};

/// The card's fixed geometry, shared by the title scene and the credits scene so the return
/// lands on exactly the same page.
pub const NAME_Y: f32 = 452.0;
pub const AFFIL_1_Y: f32 = 524.0;
pub const AFFIL_2_Y: f32 = 560.0;

#[derive(Debug)]
pub struct Title;

impl Scene for Title {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(2.0)
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = ramp(&frame, 0.1);
        fframes::svgr!(<g opacity={t}>
            {statement_c("University Registration System", TITLE, 960.0, NAME_Y, 1.0, INK)}
            <g opacity={ramp(&frame, 0.55)}>
                {affil("ST. MARY'S UNIVERSITY  ·  DEPARTMENT OF COMPUTER SCIENCE", 20, 960.0, AFFIL_1_Y, 1.0, "#6d675d")}
                {affil("OBJECT-ORIENTED PROGRAMMING  ·  GROUP PROJECT", 20, 960.0, AFFIL_2_Y, 1.0, "#6d675d")}
            </g>
        </g>)
    }
}
```

#### `thesis.rs` (8–10 s)

```rust
//! 8-10s. The problem the next 78 seconds answer: three roles, one set of rules, and rules need
//! somewhere to live. The three constants are real, raised from the database.
use crate::design::*;
use fframes::{AudioMap, AudioTimestamp::*, AudioTrack, Duration, FFramesContext, Frame, Scene, Svgr};

const RULES: [&str; 3] = ["PREREQUISITES_NOT_MET", "ROOM_CONFLICT", "INSTRUCTOR_CONFLICT"];

#[derive(Debug)]
pub struct Thesis;

impl Scene for Thesis {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(2.0)
    }

    // A sweep that starts under the thesis and rings across the cut into Roles.
    fn audio(&self) -> AudioMap<'_> {
        AudioMap::from([
            AudioTrack::new("swoosh.mp3", Second(1.7)..Eof).gain_db(-16.),
        ])
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let lines: Vec<Svgr> = RULES
            .iter()
            .enumerate()
            .map(|(i, rule)| mono(rule, MONO_MD, 196.0, 566.0 + i as f32 * 44.0, ramp(&frame, 0.45 + i as f32 * 0.16), INK))
            .collect();
        fframes::svgr!(<g>
            <g opacity={ramp(&frame, 0.05)} transform={Transform::translate(0.0, rise(&frame, 0.05, 26.0))}>
                {statement("shared rules. one place.", STATEMENT, 192.0, 452.0, 1.0, INK)}
            </g>
            {lines}
        </g>)
    }
}
```

#### `roles.rs` (10–22 s)

```rust
//! 10-22s. Three roles, three identical frames. A name lands alone, then the one method that
//! role exists to call. Two of the three are not Java at all.
use crate::design::*;
use fframes::{AudioMap, AudioTimestamp::*, AudioTrack, Duration, FFramesContext, Frame, Scene, Svgr};

const ROLES: [(&str, &str, &str); 3] = [
    ("Student", "EnrollResult enroll(int studentId, String courseCode)", "self-register · enroll · drop · schedule · grades"),
    ("Instructor", "sp_updategrade(?, ?, ?)", "teaching schedule · class rosters · assessment marks"),
    ("Head of Department", "sp_create_course_full(?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", "elevated instructor · add courses · department reports"),
];

#[derive(Debug)]
pub struct Roles;

impl Scene for Roles {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(12.0)
    }

    // One soft tap as each role card lands (scene-relative 0.0 / 3.91 / 7.91).
    fn audio(&self) -> AudioMap<'_> {
        AudioMap::from([
            AudioTrack::new("click.mp3", Second(0.0)..Eof).gain_db(-20.),
            AudioTrack::new("click.mp3", Second(3.91)..Eof).gain_db(-20.),
            AudioTrack::new("click.mp3", Second(7.91)..Eof).gain_db(-20.),
        ])
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let mut out: Vec<Svgr> = Vec::new();
        for (i, (role, call, desc)) in ROLES.iter().enumerate() {
            let at = (-0.09 + i as f32 * 4.0).max(0.0);
            let op = in_out(&frame, at, 3.15, 0.40);
            if op <= 0.01 {
                continue;
            }
            out.push(fframes::svgr!(<g opacity={op}>
                {statement(role, ROLE, 192.0, 468.0, 1.0, INK)}
                {mono(call, MONO_MD, 196.0, 574.0, ramp(&frame, at + 0.9), RUST)}
                {rule(196.0, 620.0, 96.0 * ramp(&frame, at + 0.9), ramp(&frame, at + 0.9))}
                {mono(desc, MONO_XS, 196.0, 668.0, ramp(&frame, at + 1.1), "#6d675d")}
            </g>));
        }
        fframes::svgr!(<g>{out}</g>)
    }
}
```

#### `login.rs` (22–33 s)

```rust
//! 22-35s. One login, then the three decisions the code made. The dimmed lines are the point:
//! it is a chain, and for this login the third branch was never reached.
use crate::design::*;
use crate::ui::*;
use fframes::{AudioMap, AudioTimestamp::*, AudioTrack, Duration, FFramesContext, Frame, Scene, Svgr};

const ID: &str = "1814";
const MASK: &str = "••••••";

#[derive(Debug)]
pub struct Login;

impl Scene for Login {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(11.0)
    }

    // Keystroke taps under the typing, a confirm beep on the button press at
    // 4.25, a sweep as the dashboard arrives at 5.15, one soft tick as the
    // decision strip starts at 7.75.
    fn audio(&self) -> AudioMap<'_> {
        AudioMap::from([
            AudioTrack::new("click.mp3", Second(0.8)..Eof).gain_db(-22.),
            AudioTrack::new("click.mp3", Second(1.6)..Eof).gain_db(-22.),
            AudioTrack::new("click.mp3", Second(2.4)..Eof).gain_db(-22.),
            AudioTrack::new("beep.mp3", Second(4.25)..Eof).gain_db(-18.),
            AudioTrack::new("swoosh.mp3", Second(5.0)..Eof).gain_db(-16.),
            AudioTrack::new("tick.mp3", Second(7.75)..Eof).gain_db(-22.),
        ])
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let id = typed(&frame, 0.55, 5.0, ID);
        let pw = typed(&frame, 2.15, 7.0, MASK);
        let press = (t - 4.25).clamp(0.0, 0.35) / 0.35 * (1.0 - ((t - 4.6).clamp(0.0, 0.4) / 0.4));
        let press = press.clamp(0.0, 1.0);
        let login_op = (1.0 - ramp(&frame, 4.75)).clamp(0.0, 1.0);
        let dash_op = ramp(&frame, 5.15);

        let enrolled: Vec<Vec<String>> = [
            ["1001", "CS101", "SEC-1", "Enrolled"],
            ["1001", "DB301", "SEC-1", "Enrolled"],
            ["1001", "ME101", "SEC-2", "Enrolled"],
        ]
        .iter()
        .map(|r| r.iter().map(|s| (*s).to_string()).collect())
        .collect();

        let strip = [
            ("if (authenticateStudent(1814, ••••••) != null)", "taken", RUST, 1.0_f64),
            ("    return new StudentDashboardFrame();", "", INK, 1.0_f64),
            ("if (authenticateInstructor(1814, ••••••) != null)", "not reached", INK, 0.3_f64),
            ("if (isHeadOfDepartment(1814))", "not reached", INK, 0.3_f64),
        ];
        let mut lines: Vec<Svgr> = Vec::new();
        for (i, (code, tag, fill, dim)) in strip.iter().enumerate() {
            let at = 7.75 + i as f32 * 0.2;
            let op = ramp(&frame, at) * *dim as f32;
            lines.push(fframes::svgr!(<g opacity={op}>
                <text x="470" y={812.0 + i as f32 * 46.0} font-family={MONO_F} font-weight={MONO_W}
                      font-size={MONO_MD} fill={*fill}>{*code}</text>
                <text x="1450" y={812.0 + i as f32 * 46.0} font-family={MONO_F} font-weight={MONO_W}
                      font-size={MONO_XS} fill={*fill}>{*tag}</text>
            </g>));
        }

        fframes::svgr!(<g>
            <g opacity={login_op} transform={Transform::translate(0.0, rise(&frame, 0.0, 40.0) * login_op)}>
                {login_window((id, pw), caret(&frame, 0.55, 5.0, ID), caret(&frame, 2.15, 7.0, MASK), press)}
            </g>
            <g opacity={dash_op} transform={Transform::translate(0.0, rise(&frame, 5.15, 46.0) * (1.0 - dash_op))}>
                {student_dashboard(enrolled, 0.35 + 0.3 * (frame.seconds() * 0.7).sin())}
            </g>
            <rect x="440" y="792" width="1040" height="1" fill="#ddd9d0" opacity={ramp(&frame, 7.45)} />
            {lines}
        </g>)
    }
}
```

#### `architecture.rs` (33–48 s)

```rust
//! 35-50s. The stack, drawn in our own type at full size, with one pulse of light going down it.
//! The dot performs MVC rather than asserting it, and goes dark at the SQL boundary.
use crate::design::*;
use fframes::{AudioMap, AudioTimestamp::*, AudioTrack, Duration, FFramesContext, Frame, Scene, Svgr};

struct Band {
    label: &'static str,
    y: f32,
    h: f32,
    lines: [&'static str; 2],
}

const BANDS: [Band; 4] = [
    Band { label: "VIEW", y: 130.0, h: 140.0, lines: ["LoginFrame · StudentDashboardFrame · InstructorDashboardFrame · HoDDashboardFrame · RegistrationFrame", ""] },
    Band { label: "CONTROLLER", y: 300.0, h: 140.0, lines: ["LoginController · StudentDashboardController · InstructorDashboardController · HoDDashboardController · RegistrationController", ""] },
    Band { label: "DAO", y: 470.0, h: 150.0, lines: ["UserDAO · StudentDAO · InstructorDAO · CourseDAO · RegistrationDAO · ScheduleDAO · GradeDAO", "seven interfaces, seven implementations"] },
    Band { label: "MODEL / UTIL", y: 650.0, h: 150.0, lines: ["Assessment · Course · EnrolledStudentRecord · Instructor · Registration · Schedule", "Section · Student · User · GradeUtils · DBConnection"] },
];

const DB_Y: f32 = 832.0;

/// The dot runs on a rail in the right-hand gutter, clear of every band, so it
/// never sits on top of the class name it is meant to light up. The bands run
/// x=192..1728, so 1806 is outside all of them.
const RAIL_X: f32 = 1806.0;

/// Where the dot stands for each hop, and what that hop is called. Only the
/// height varies: the rail is fixed, so the travel reads as "down the stack".
const HOPS: [(f32, &str); 7] = [
    (200.0, "LoginFrame — Enroll in Selected Course"),
    (370.0, "StudentDashboardController"),
    (545.0, "RegistrationDAO.enroll(1814, \"CS101\")"),
    (DB_Y + 28.0, "{call sp_enroll_student(?, ?)}"),
    (545.0, "EnrollResult.SUCCESS"),
    (725.0, "new Registration(...)"),
    (200.0, "StudentDashboardFrame.repaint()"),
];

/// Hop spacing snapped to the bed's beat grid: 4 beats at 136 BPM
/// (4 x 0.4412 s), so every hop lands on a beat.
const HOP: f32 = 1.7648;

#[derive(Debug)]
pub struct Architecture;

impl Scene for Architecture {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(15.0)
    }

    // One connector tick per hop label (hop starts at 0.53 + k * HOP, same
    // clock the dot runs on), plus a sweep that carries the 48s cut.
    fn audio(&self) -> AudioMap<'_> {
        AudioMap::from([
            AudioTrack::new("tick.mp3", Second(0.53)..Eof).gain_db(-20.),
            AudioTrack::new("tick.mp3", Second(0.53 + HOP)..Eof).gain_db(-20.),
            AudioTrack::new("tick.mp3", Second(0.53 + 2.0 * HOP)..Eof).gain_db(-20.),
            AudioTrack::new("tick.mp3", Second(0.53 + 3.0 * HOP)..Eof).gain_db(-20.),
            AudioTrack::new("tick.mp3", Second(0.53 + 4.0 * HOP)..Eof).gain_db(-20.),
            AudioTrack::new("tick.mp3", Second(0.53 + 5.0 * HOP)..Eof).gain_db(-20.),
            AudioTrack::new("tick.mp3", Second(0.53 + 6.0 * HOP)..Eof).gain_db(-20.),
            AudioTrack::new("swoosh.mp3", Second(14.7)..Eof).gain_db(-16.),
        ])
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        // One hop per 4 beats. `step` is the index of the hop in progress, so the band highlight
        // and the dot are always reading the same clock.
        let step = ((frame.seconds() - 0.6) / HOP).floor().max(0.0);
        let mut out: Vec<Svgr> = Vec::new();
        for (i, band) in BANDS.iter().enumerate() {
            let flash = (1.0 - ((i as f32 - step) * 1.6).abs().min(1.0)) * 0.09;
            out.push(fframes::svgr!(
                <g>
                    <rect x="192" y={band.y} width="1536" height={band.h} rx="8" fill={CARD} opacity={0.62 + flash} />
                    <rect x="192" y={band.y} width="4" height={band.h} rx="2" fill={RUST} opacity={flash * 4.0} />
                    <text x="224" y={band.y + 34.0} font-family={SANS} font-weight={SANS_W} font-size={UI_SM}
                          letter-spacing="2.4" fill="#8a8479">{band.label}</text>
                    <text x="224" y={band.y + 82.0} font-family={MONO_F} font-weight={MONO_W} font-size={MONO_SM}
                          fill={INK}>{band.lines[0]}</text>
                    {mono(band.lines[1], MONO_SM, 224.0, band.y + 116.0, 1.0, "#6d675d")}
                </g>
            ));
        }
        out.push(fframes::svgr!(
            <g>
                <rect x="192" y={DB_Y} width="1536" height="56" rx="8" fill={INK} opacity="0.07" />
                <text x="224" y={DB_Y + 34.0} font-family={MONO_F} font-weight={MONO_W} font-size={MONO_SM}
                      fill="#6d675d">"MySQL · universitydb · 12 tables · 7 stored procedures"</text>
            </g>
        ));

        let idx = (step as usize).min(HOPS.len() - 1);
        let hop_start = 0.53 + step * HOP;
        // The last hop holds its label to the end of the scene; the others hand off.
        let label_op = if idx == HOPS.len() - 1 {
            ramp(&frame, hop_start)
        } else {
            ramp(&frame, hop_start) * (1.0 - seg(&frame, hop_start + HOP - 0.35, 0.35))
        };
        // Decelerate into each node: constant velocity reads as a machine, and the
        // arrival is what makes the stop legible.
        let eased = decel(&frame, 0.53 + step * HOP, 1.5);
        let (_, label) = HOPS[idx];
        let (py, _) = HOPS[idx.saturating_sub(1)];
        let y = py + (HOPS[idx].0 - py) * eased;
        // Hop 4 is where the work leaves Java: the dot goes dark and nothing else moves.
        let dark = if idx == 3 { seg(&frame, 4.83, 0.9) } else { 0.0 };
        let dot_op = 1.0 - dark;

        out.push(fframes::svgr!(<g>
            <line x1={RAIL_X} y1="130" x2={RAIL_X} y2={DB_Y + 56.0} stroke={INK} stroke-width="1" opacity="0.18" />
            <circle cx={RAIL_X} cy={y} r="15" fill={RUST} opacity={dot_op} />
            <circle cx={RAIL_X} cy={y} r="27" fill={RUST} opacity={0.16 * dot_op} />
            <text x="192" y="964" font-family={MONO_F} font-weight={MONO_W} font-size={MONO_LG}
                  fill={INK} opacity={label_op}>{label}</text>
        </g>));
        fframes::svgr!(<g>{out}</g>)
    }
}
```

#### `movement_one.rs` (48–62 s)

```rust
//! 50-64s. Movement I: a fact lives in exactly one place. A rule lives in the class; a field
//! lives in the base class. Each proof is ~4 s, the landing ~3 s.
use crate::design::*;
use fframes::{AudioMap, AudioTimestamp::*, AudioTrack, Duration, FFramesContext, Frame, Scene, Svgr};

const GUARD: [&str; 5] = [
    "public void setGender(String gender) {",
    "    if (gender == null || (!gender.equals(\"M\") && !gender.equals(\"F\"))) {",
    "        throw new IllegalArgumentException(\"Gender must be 'M' or 'F'.\");",
    "    }",
    "    this.gender = gender;",
];

#[derive(Debug)]
pub struct MovementOne;

impl Scene for MovementOne {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(14.0)
    }

    // Bell pings (attack slices of the chime, tail cut) under the two OOP tag
    // captions at 3.26 and 8.11.
    fn audio(&self) -> AudioMap<'_> {
        AudioMap::from([
            AudioTrack::new("chime.mp3", Second(3.1)..Second(4.4)).offset(0.3).gain_db(-14.),
            AudioTrack::new("chime.mp3", Second(7.95)..Second(9.25)).offset(0.3).gain_db(-14.),
        ])
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let mut out: Vec<Svgr> = Vec::new();

        // 0-2.9s: the thesis, alone on paper. Hard-on at t=0 with no fade-in: the cut
        // from Architecture is the entrance, so the 48s boundary frame lands on content,
        // never on blank paper. Only the exit fades, handing off to the guard proof.
        let thesis_op = 1.0 - ramp(&frame, 2.32);
        if thesis_op > 0.01 {
            out.push(fframes::svgr!(<g opacity={thesis_op}>
                {statement("one fact, one place.", STATEMENT, 192.0, 540.0, 1.0, INK)}
            </g>));
        }

        // 2.75-6.75s: proof. A rule that cannot be stepped over, from inside the class.
        let guard_op = in_out(&frame, 2.86, 3.5, 0.5);
        if guard_op > 0.01 {
            let mut lines: Vec<Svgr> = Vec::new();
            for (i, line) in GUARD.iter().enumerate() {
                let op = ramp(&frame, 2.96 + i as f32 * 0.14) * guard_op;
                let is_throw = i == 2;
                lines.push(fframes::svgr!(<text x={196.0 + i as f32 * 18.0} y={430.0 + i as f32 * 44.0}
                    font-family={MONO_F} font-weight={MONO_W} font-size={MONO_MD}
                    fill={if is_throw { RUST } else { INK }} opacity={op}>{*line}</text>));
            }
            out.push(fframes::svgr!(<g>
                {lines}
                {rule(232.0, 534.0, 1021.0 * ramp(&frame, 3.36), ramp(&frame, 3.36) * guard_op)}
                {mono("Model/User.java — Encapsulation: private fields, getters, setters", MONO_XS, 196.0, 690.0, ramp(&frame, 3.26) * guard_op, "#6d675d")}
            </g>));
        }

        // 6.6-10.6s: proof. Inheritance made visible as absence: two nearly empty boxes.
        let boxes_op = in_out(&frame, 6.71, 3.5, 0.5);
        if boxes_op > 0.01 {
            let links: Vec<Svgr> = vec![
                fframes::svgr!(<rect x="899" y="432" width="2" height="34" fill="#c9c5bc" />),
                fframes::svgr!(<rect x="318" y="465" width="990" height="2" fill="#c9c5bc" />),
                fframes::svgr!(<rect x="317" y="465" width="2" height="27" fill="#c9c5bc" />),
                fframes::svgr!(<rect x="1307" y="465" width="2" height="27" fill="#c9c5bc" />),
            ];
            out.push(fframes::svgr!(<g opacity={boxes_op}>
                <rect x="480" y="262" width="960" height="170" rx="8" fill={CARD} opacity="0.75" />
                {mono("User", MONO_LG, 512.0, 312.0, 1.0, INK)}
                {sans("abstract class", UI, 640.0, 310.0, 1.0, "#8a8479")}
                {mono("id · firstName · lastName · gender · dob · password", MONO_MD, 512.0, 372.0, 1.0, INK)}
                {links}
                <rect x="280" y="492" width="580" height="150" rx="8" fill={CARD} opacity="0.75" />
                {mono("Student", MONO_LG, 312.0, 542.0, 1.0, INK)}
                {mono("+ sectionId", MONO_MD, 312.0, 604.0, ramp(&frame, 7.51), RUST)}
                <rect x="1060" y="492" width="580" height="150" rx="8" fill={CARD} opacity="0.75" />
                {mono("Instructor", MONO_LG, 1092.0, 542.0, 1.0, INK)}
                {mono("+ deptId, boolean isHoD", MONO_MD, 1092.0, 604.0, ramp(&frame, 7.81), RUST)}
                {mono_c("The User abstract class is the parent of Student and Instructor.", MONO_XS, 960.0, 700.0, ramp(&frame, 8.11), "#6d675d")}
            </g>));
        }

        // 10.45-14s: the landing.
        let landing_op = ramp(&frame, 10.56);
        out.push(fframes::svgr!(<g opacity={landing_op} transform={Transform::translate(0.0, rise(&frame, 10.56, 26.0))}>
            {statement("illegal states don't exist.", STATEMENT, 192.0, 540.0, 1.0, INK)}
        </g>));
        fframes::svgr!(<g>{out}</g>)
    }
}
```

#### `movement_two.rs` (62–76 s)

```rust
//! 64-78s. Movement II: the caller never sees the machinery. Seven pairs are visibly one rule;
//! the crossing is the same rule in code, and it is checkable in the repo.
use crate::design::*;
use fframes::{AudioMap, AudioTimestamp::*, AudioTrack, Duration, FFramesContext, Frame, Scene, Svgr};

const PAIRS: [&str; 7] = [
    "UserDAO          →  UserDAOImp",
    "StudentDAO       →  StudentDAOImp",
    "InstructorDAO    →  InstructorDAOImp",
    "CourseDAO        →  CourseDAOImp",
    "RegistrationDAO  →  RegistrationDAOImp",
    "ScheduleDAO      →  ScheduleDAOImp",
    "GradeDAO         →  GradeDAOImp",
];

const LEFT: [&str; 6] = [
    "interface RegistrationDAO {",
    "    enum EnrollResult { SUCCESS, ALREADY_ENROLLED,",
    "                      PREREQUISITES_NOT_MET, ERROR }",
    "",
    "    EnrollResult enroll(int studentId,",
    "                        String courseCode);",
];

const RIGHT: [&str; 6] = [
    "class RegistrationDAOImp implements RegistrationDAO {",
    "    String call = \"{call sp_enroll_student(?, ?)}\";",
    "    try (Connection c = DBConnection.getConnection();",
    "         CallableStatement s = c.prepareCall(call)) { ... }",
    "    catch (SQLException e)",
    "        if (msg.contains(\"PREREQUISITES_NOT_MET\")) ...",
];

#[derive(Debug)]
pub struct MovementTwo;

impl Scene for MovementTwo {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(14.0)
    }

    // Tick as the pairs block starts, bell ping under the pairs caption at
    // 5.855, sweep into the crossing at 7.355, bell ping under the landing at
    // 11.35, and the rising whoosh (11.0-14.0) that peaks exactly on the 76s
    // cut into Payoff — where the bed has gone silent.
    fn audio(&self) -> AudioMap<'_> {
        AudioMap::from([
            AudioTrack::new("tick.mp3", Second(2.7)..Eof).gain_db(-22.),
            AudioTrack::new("chime.mp3", Second(5.7)..Second(7.0)).offset(0.3).gain_db(-14.),
            AudioTrack::new("swoosh.mp3", Second(7.2)..Eof).gain_db(-14.),
            AudioTrack::new("chime.mp3", Second(11.2)..Second(12.5)).offset(0.3).gain_db(-14.),
            AudioTrack::new("whoosh.mp3", Second(11.0)..Second(14.0)).gain_db(-10.),
        ])
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let mut out: Vec<Svgr> = Vec::new();

        let thesis_op = in_out(&frame, 0.105, 2.25, 0.5);
        if thesis_op > 0.01 {
            out.push(fframes::svgr!(<g opacity={thesis_op}>
                {statement("the machinery stays hidden.", STATEMENT, 192.0, 540.0, 1.0, INK)}
            </g>));
        }

        // Seven repetitions of one shape, ticking in on a beat. The block is centred: the
        // longest row is 38 mono chars, so 698 is the left edge that puts its middle on the page.
        let pairs_op = in_out(&frame, 2.705, 4.3, 0.5);
        if pairs_op > 0.01 {
            let mut rows: Vec<Svgr> = Vec::new();
            for (i, pair) in PAIRS.iter().enumerate() {
                let op = ramp(&frame, 2.855 + i as f32 * 0.42) * pairs_op;
                rows.push(fframes::svgr!(<text x="698" y={330.0 + i as f32 * 66.0} font-family={MONO_F}
                    font-weight={MONO_W} font-size={MONO_MD} fill={INK} opacity={op}>{*pair}</text>));
            }
            out.push(fframes::svgr!(<g>
                {rows}
                {mono_c("seven interfaces, seven implementations · interfaces hide implementation details", MONO_XS, 960.0, 812.0, ramp(&frame, 5.855) * pairs_op, "#6d675d")}
            </g>));
        }

        // The crossing. A hard rule, the contract on one side, the machinery on the other.
        // Retires fully before the landing arrives, so the statement never sits on the code.
        let cross_op = in_out(&frame, 7.355, 2.7, 0.4);
        if cross_op > 0.01 {
            let mut l: Vec<Svgr> = Vec::new();
            let mut r: Vec<Svgr> = Vec::new();
            for i in 0..LEFT.len() {
                l.push(fframes::svgr!(<text x="196" y={290.0 + i as f32 * 46.0} font-family={MONO_F}
                    font-weight={MONO_W} font-size={MONO_MD} fill={INK}
                    opacity={ramp(&frame, 7.455 + i as f32 * 0.1) * cross_op}>{LEFT[i]}</text>));
                r.push(fframes::svgr!(<text x="1000" y={290.0 + i as f32 * 46.0} font-family={MONO_F}
                    font-weight={MONO_W} font-size={MONO_MD} fill={INK}
                    opacity={ramp(&frame, 7.555 + i as f32 * 0.1) * cross_op}>{RIGHT[i]}</text>));
            }
            out.push(fframes::svgr!(<g opacity={cross_op}>
                {mono("DAO/RegistrationDAO.java — interface-based polymorphism", MONO_XS, 196.0, 236.0, 1.0, "#6d675d")}
                {mono("DAO/RegistrationDAOImp.java — method overriding", MONO_XS, 1000.0, 236.0, 1.0, "#6d675d")}
                {l}
                {r}
                <rect x="958" y="212" width="3" height="300" fill={INK} opacity={ramp(&frame, 7.455)} />
            </g>));
        }

        let landing_op = ramp(&frame, 11.35);
        out.push(fframes::svgr!(<g opacity={landing_op} transform={Transform::translate(0.0, rise(&frame, 11.35, 26.0))}>
            {statement("same call. no idea which.", STATEMENT, 192.0, 540.0, 1.0, INK)}
        </g>));
        fframes::svgr!(<g>{out}</g>)
    }
}
```

#### `payoff.rs` (76–90 s)

```rust
//! 78-91s. The one frame where every layer participates at once: View, Controller, DAO,
//! Assessment, GradeUtils and a stored procedure. The label resolves to the letter the film
//! opened on, and the window recedes so the letter is the last thing standing.
use crate::design::*;
use crate::ui::*;
use fframes::{AudioMap, AudioTimestamp::*, AudioTrack, Duration, FFramesContext, Frame, Scene, Svgr};

const RESULT: &str = "Scores saved. Weighted total: 97.0% → Grade: A+";

#[derive(Debug)]
pub struct Payoff;

impl Scene for Payoff {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(14.0)
    }

    // The bed is silent from here to the end of the film, so SFX carry the
    // scene: taps under the score typing, a confirm beep on the save press at
    // ~3.5, and the full chime as the A+ letter arrives at 8.625 (capped
    // before the credits cut so its tail never fights the ta-da).
    fn audio(&self) -> AudioMap<'_> {
        AudioMap::from([
            AudioTrack::new("click.mp3", Second(1.4)..Eof).gain_db(-22.),
            AudioTrack::new("click.mp3", Second(2.1)..Eof).gain_db(-22.),
            AudioTrack::new("click.mp3", Second(2.8)..Eof).gain_db(-22.),
            AudioTrack::new("beep.mp3", Second(3.5)..Eof).gain_db(-18.),
            AudioTrack::new("chime.mp3", Second(8.6)..Second(13.5)).gain_db(-12.),
        ])
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let scores = [
            typed(&frame, 1.225, 6.0, "29"),
            typed(&frame, 1.925, 6.0, "20"),
            typed(&frame, 2.625, 6.0, "48"),
        ];
        let press = (1.0 - ((t - 3.725).clamp(0.0, 0.35) / 0.35)) * ((t - 3.375).clamp(0.0, 0.35) / 0.35);
        let press = press.clamp(0.0, 1.0);
        let result_op = ramp(&frame, 4.325);
        // The window is fully gone before the letter begins to arrive, so the letter never
        // lands on top of the label that produced it and no empty beat opens between them.
        let recede = seg(&frame, 7.825, 0.8);
        let window_op = 1.0 - recede;
        let drift = recede * 26.0;
        let a_op = ramp(&frame, 8.625);

        fframes::svgr!(
        <g>
            <g opacity={window_op} transform={Transform::translate(0.0, drift)}>
                {grade_entry("CS101 — Introduction to Programming".to_string(),
                             "1001 · Abebe Kebede".to_string(), scores,
                             caret(&frame, 2.625, 6.0, "48"), RESULT.to_string(), result_op, press)}
            </g>
            {letter(a_op, INK)}
            {mono_c("MVC + DAO — separation of concerns · maintainable · extensible", MONO_XS, 960.0, 920.0, ramp(&frame, 9.4), "#6d675d")}
        </g>)
    }
}
```

#### `credits.rs` (90–96 s)

```rust
//! 91-96s. The title card at 6s, returned, with the names added. Nothing animates in one at a
//! time: the film decelerates into stillness.
use crate::design::*;
use crate::scenes::title::{AFFIL_1_Y, AFFIL_2_Y, NAME_Y};
use fframes::{AudioMap, AudioTimestamp::*, AudioTrack, Duration, FFramesContext, Frame, Scene, Svgr};

const TEAM: [(&str, &str); 5] = [
    ("Abenezer Kassahun", "RCD/1814/2017"),
    ("Abreham Tesfaw", "RCD/0398/2017"),
    ("Natal Tamrat", "RCD/0442/2017"),
    ("Mohammed Tekola", "RCD/0124/2017"),
    ("Robera Adugna", "RCD/0445/2017"),
];

#[derive(Debug)]
pub struct Credits;

impl Scene for Credits {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(6.0)
    }

    // The bed is silent here too: a soft ta-da as the names start arriving.
    fn audio(&self) -> AudioMap<'_> {
        AudioMap::from([
            AudioTrack::new("tada.mp3", Second(0.6)..Eof).gain_db(-12.),
        ])
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        // The film ends by going quiet rather than by cutting: everything holds, then dips
        // slightly. The fade floors at 0.8 so the names are fully legible on the last frame
        // while the bed's own fade-out supplies the sense of an ending.
        let fade = 1.0 - 0.2 * seg(&frame, 5.45, 0.55);
        let mut names: Vec<Svgr> = Vec::new();
        for (i, (name, id)) in TEAM.iter().enumerate() {
            let op = ramp(&frame, 0.625 + i as f32 * 0.22);
            let y = 646.0 + i as f32 * 40.0;
            names.push(fframes::svgr!(<g opacity={op}>
                <text x="800" y={y} text-anchor="end" font-family={SANS} font-weight={SANS_W}
                      font-size="21" fill={INK}>{*name}</text>
                <text x="828" y={y} font-family={MONO_F} font-weight={MONO_W} font-size="17"
                      fill="#8a8479">{*id}</text>
            </g>));
        }
        fframes::svgr!(<g opacity={fade}>
            {statement_c("University Registration System", TITLE, 960.0, NAME_Y, ramp(&frame, 0.225), INK)}
            {affil("ST. MARY'S UNIVERSITY  ·  DEPARTMENT OF COMPUTER SCIENCE", 20, 960.0, AFFIL_1_Y, ramp(&frame, 0.425), "#6d675d")}
            {affil("OBJECT-ORIENTED PROGRAMMING  ·  GROUP PROJECT", 20, 960.0, AFFIL_2_Y, ramp(&frame, 0.425), "#6d675d")}
            {names}
            <rect x="712" y="606" width="496" height="1" fill="#ddd9d0" opacity={ramp(&frame, 0.625)} />
            {mono_c("Music: \"Deliberate Thought\" by Kevin MacLeod  ·  CC BY 4.0", MONO_XS, 960.0, 906.0, ramp(&frame, 2.325), "#8a8479")}
            {mono_c("SFX: orangefreesounds.com  ·  CC BY-NC 4.0", MONO_XS, 960.0, 928.0, ramp(&frame, 2.475), "#8a8479")}
            {mono_c("lal-ye / registration-system-film", MONO_XS, 960.0, 950.0, ramp(&frame, 2.625), "#8a8479")}
        </g>)
    }
}
```

### 3.8 `Cargo.toml`

```toml
[package]
name = "registration-system-film"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
fframes = { version = "=1.2.0", features = ["compile-time-svgtree", "cli"] }

[target.'cfg(not(windows))'.dependencies]
fframes = { version = "=1.2.0", features = ["h264", "libav-agree-gpl"] }

[lib]
name = "registration_system_film"
path = "src/lib.rs"
doctest = false

[[bin]]
name = "registration-system-film"
path = "src/main.rs"

[workspace]

[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

### 3.9 `tests/frames.rs` — visual regression + full inspect

```rust
//! Visual regression: renders a few frames and compares them with the approved PNGs in
//! `_frame_snapshots/`. The first run only creates them, so look at the PNGs before you
//! commit them. `FFRAMES_UPDATE_SNAPSHOTS=1 cargo test` accepts intentional changes; failing
//! frames leave `.actual.png` and `.diff.png` files.
use fframes::{CpuFrameRenderer, Previewer, RenderOptions, StaticMediaProvider, snapshot};
use registration_system_film::{ RegistrationSystemFilmMedia, RegistrationSystemFilmVideo };

#[test]
fn key_frames_match_snapshots() {
    let media = RegistrationSystemFilmMedia::prepare().unwrap();
    let video = RegistrationSystemFilmVideo::new(&media);
    let options = RenderOptions {
        media: Some(&media),
        scale_resolution: 0.5,
        ..Default::default()
    };
    let mut previewer = Previewer::new(&video, &options).unwrap();

    snapshot::assert_frames(
        &mut previewer,
        &mut CpuFrameRenderer::default(),
        // Settled frames: the middle of a scene, not the end of it where it fades out.
        &[
            "ColdOpen@2s",
            "Title@1s",
            "Roles@6s",
            "Login@9s",
            "Architecture@6s",
            "MovementOne@8s",
            "MovementTwo@8s",
            "Payoff@5s",
            "Credits@4s",
        ],
        &snapshot::SnapshotOptions::default(),
    );
}

#[test]
fn no_problems_in_any_frame() {
    // Converting a frame without rasterizing it is fast, so every frame is checked.
    let media = RegistrationSystemFilmMedia::prepare().unwrap();
    let video = RegistrationSystemFilmVideo::new(&media);
    let options = RenderOptions {
        media: Some(&media),
        ..Default::default()
    };
    let mut previewer = Previewer::new(&video, &options).unwrap();

    let duration = previewer.timeline().duration_in_frames;
    for frame in 0..duration {
        let report = previewer.inspect(frame).unwrap();
        let problems: Vec<_> = report
            .diagnostics
            .iter()
            .filter(|d| d.severity >= fframes::diagnostics::Severity::Warning)
            .map(|d| d.message.as_str())
            .collect();
        assert!(problems.is_empty(), "frame {frame} ({:.2}s): {problems:?}", report.seconds);
    }
}
```

### 3.10 `beat_align.py` — beat-grid onset alignment

```python
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
```

(`measure_text.py` / `measure_ui.py`: fontTools scripts that print true advance
widths of every statement line and every boxed UI string against its container;
their findings are compiled into `SERIF_ADVANCE` / `condensed()` and the
840-wide centred login card. Full scripts in the repo root.)

### 3.11 `.github/workflows/render.yml` — CI render pipeline

```yaml
name: Render video

# Renders this fframes project on a free GitHub runner and uploads the mp4 plus
# review PNGs. Rendering happens here rather than locally because a full build
# (including a static ffmpeg) needs gigabytes of disk.
#
# The binary name is discovered from Cargo.toml rather than hardcoded, so this
# file works unchanged in any fframes project.
on:
  push:
    branches: [main]
  pull_request:
  workflow_dispatch:
    inputs:
      draft:
        description: Draft render only (one scene, half resolution, fast preset)
        type: boolean
        default: false
      skip_render:
        description: Review PNGs only (skip the mp4 encode)
        type: boolean
        default: false

# Least privilege: this workflow only reads the repo and uploads artifacts.
permissions:
  contents: read

# A render is minutes of CPU; never run two for the same ref at once.
concurrency:
  group: render-${{ github.ref }}
  cancel-in-progress: true

env:
  CARGO_TERM_COLOR: always
  CARGO_INCREMENTAL: "0"
  RUST_BACKTRACE: "1"
  # ffmpeg-sys-fframes downloads a prebuilt static FFmpeg for Linux x86_64, but
  # that archive links x264/x265 dynamically and records the -L paths of the
  # machine that built it. Those paths do not exist on a runner, so build.rs
  # falls back to pkg-config and picks up the runner's system x264 -- which is
  # a different ABI than the prebuilt's libavcodec was compiled against
  # (prebuilt wants x264_encoder_open_163, Ubuntu 24.04 ships 164), and the
  # final link fails with "undefined symbol: x264_encoder_open_163".
  # Building FFmpeg from source makes x264 come from the same bundled sources
  # as libavcodec, so the pair always matches whatever the runner is.
  FFMPEG_FORCE_BUILD: "1"
  # A source build otherwise gets -march=native, and rust-cache would then
  # restore artifacts built on a different runner's CPU, which can SIGILL.
  # Pin a portable baseline instead.
  FFMPEG_MARCH: "x86-64"
  FFMPEG_MTUNE: "generic"

jobs:
  render:
    # Free for public repositories. The CPU backend needs no GPU, so a plain
    # runner is all this requires.
    runs-on: ubuntu-latest
    timeout-minutes: 60

    steps:
      - name: Checkout
        uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1

      # FFmpeg is compiled from source here (see the FFMPEG_* env vars), which
      # is why the assemblers and dev headers are needed.
      #
      # libx264-dev is REQUIRED even though the prebuilt static ffmpeg is not
      # used: the `h264` feature makes ffmpeg-sys-fframes pass
      # --enable-libx264 to ./configure, and configure aborts with
      # "ERROR: x264 not found using pkg-config" without it. x264 always comes
      # from the system (ffmpeg-sys-fframes never vendors it); what
      # FFMPEG_FORCE_BUILD buys is that libavcodec.a is then compiled against
      # the *same* x264 the final link resolves, which is the whole point --
      # the prebuilt archive was built against x264 163 while Ubuntu noble ships
      # 164, producing `undefined symbol: x264_encoder_open_163`.
      - name: Install system dependencies
        run: |
          set -euo pipefail
          export DEBIAN_FRONTEND=noninteractive
          sudo apt-get update -qq
          sudo apt-get install -y --no-install-recommends \
            yasm nasm \
            clang libclang-dev \
            ninja-build \
            libx264-dev \
            pkg-config

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@89b12181fb390509a0842a86cc55eeb8eb928c1d # stable

      # Without this every push is a cold build, and the cold build is where the
      # static ffmpeg lands.
      - name: Cache cargo build
        uses: Swatinem/rust-cache@6323deb102c322ba6fcbdcafc7e3dddab59af2b6 # v2.9.2
        with:
          workspaces: ". -> target"
          key: ${{ github.job }}
          shared-key: fframes-render

      # Read the binary name from the manifest so the rest of the workflow is
      # project-agnostic. CARGO_PKG_NAME and the default bin target agree for the
      # single-binary projects cargo fframes new generates.
      - name: Resolve binary name
        id: bin
        run: |
          set -euo pipefail
          NAME="$(awk -F'"' '/^\[package\]/{f=1} f && /^name[[:space:]]*=/{print $2; exit}' Cargo.toml)"
          if [ -z "${NAME}" ]; then
            echo "::error::no [package] name in Cargo.toml"
            exit 1
          fi
          echo "name=${NAME}" >> "${GITHUB_OUTPUT}"
          echo "Using binary: ${NAME}"

      - name: Build
        run: cargo build --release --locked

      - name: Timeline
        run: "./target/release/${{ steps.bin.outputs.name }} timeline"

      # inspect reports missing media or fonts, text clipped by the canvas,
      # invalid SVG and panics, and exits 2 when it finds any of them.
      - name: Inspect every frame
        run: "./target/release/${{ steps.bin.outputs.name }} inspect"

      # A contact sheet is the cheapest way to see whether the motion works
      # without watching the video. `strip` defaults to strip.png, `frame` to
      # frames/; collect them under review/ so one artifact holds them all.
      - name: Contact sheet
        run: "./target/release/${{ steps.bin.outputs.name }} strip -n 12"

      # Full-size stills at the moments that matter.
      - name: Still frames
        run: "./target/release/${{ steps.bin.outputs.name }} frame 1s,50%,end"

      - name: Collect review images
        run: |
          set -euo pipefail
          mkdir -p review
          for f in strip.png frames onion.png; do
            if [ -e "${f}" ]; then
              mkdir -p "review/$(dirname "${f}")"
              mv "${f}" "review/${f}"
            fi
          done
          find review -type f | sort

      - name: Upload review images
        uses: actions/upload-artifact@cf430e030ddbb5b0abf93d22962f4752f3646cd9 # v7.0.2
        with:
          name: review-images
          path: review/
          retention-days: 14
          if-no-files-found: error

      # A full render of every scene at 1080p.
      - name: Render
        if: ${{ !inputs.draft && !inputs.skip_render }}
        run: "./target/release/${{ steps.bin.outputs.name }} render"

      # One scene at half resolution with the fast preset: seconds, not minutes.
      # Proves the encoder and the file writer work end to end.
      - name: Draft render
        if: ${{ inputs.draft && !inputs.skip_render }}
        run: "./target/release/${{ steps.bin.outputs.name }} render --draft"

      - name: Upload video
        if: ${{ !inputs.skip_render }}
        uses: actions/upload-artifact@cf430e030ddbb5b0abf93d22962f4752f3646cd9 # v7.0.2
        with:
          name: video
          path: out.mp4
          retention-days: 14
          if-no-files-found: error
```

---

## 4. Reproduce / verify

```sh
~/bin/ffr.sh ~/fframes/registration-system-film --review-only  # PNGs only, ~85 s warm
~/bin/ffr.sh ~/fframes/registration-system-film                # full 1080p render
ffprobe -v error -select_streams v:0 \
  -show_entries stream=codec_name,width,height,r_frame_rate,nb_frames \
  -show_entries format=duration -of default=nw=1 out.mp4
# expect: h264, 1920x1080, 30fps, nb_frames=2880, duration=96.000000
```

`media/` contents (all embedded in the binary via `include_media_dir!`):
`bed.mp3`, `click_hit.wav`, `beep_hit.wav`, `tick_hit.wav`, `swoosh_hit.wav`, `whoosh_rise.wav`,
`chime_hit.wav`, `tada_hit.wav`, `bell_hit.wav`, `phrase_loop.wav`,
`InstrumentSerif-Italic.ttf`, `Inter-Bold.ttf`, `JetBrainsMono-Regular.ttf`.
