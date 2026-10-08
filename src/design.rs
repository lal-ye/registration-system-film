//! The film's design system. Every scene draws through this module, which is what keeps the
//! palette and the type scale from eroding one file at a time.
use crate::grain::{BLOTCH, SPECKS};
use fframes::{
    AnimateRuntimeInput, Frame, Svgr, Transform, animation::{AnimationRuntime, Easing},
};
use std::sync::LazyLock;

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
pub fn seg(frame: &Frame, start: f32, over: f32) -> f32 {
    (scrub(frame, start) / over.max(0.001)).clamp(0.0, 1.0)
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

/// A statement: the voice, on paper, alone.
pub fn statement<'a>(text: &'a str, size: usize, x: f32, y: f32, opacity: f32, fill: &'a str) -> Svgr<'a> {
    fframes::svgr!(<g opacity={opacity}>
        <text x={x} y={y} font-family={SERIF} font-size={size} fill={fill}>{text}</text>
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
    fframes::svgr!(<g opacity={opacity}>
        <text x={x} y={y} text-anchor="middle" font-family={SERIF} font-size={size} fill={fill}>{text}</text>
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