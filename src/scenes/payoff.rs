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
    // scene: taps under the score typing (true-cps onsets), a click at the
    // save-press peak (3.725), release (4.07), a bell CONFIRM on the result
    // line (4.325, previously silent), a sweep as the window recedes (7.825),
    // and the full chime as the A+ letter arrives at 8.625 (capped before
    // the credits cut so its tail never fights the ta-da).
    fn audio(&self) -> AudioMap<'_> {
        AudioMap::from([
            AudioTrack::new("click_hit.wav", Second(1.39)..Second(1.64)).gain_db(-24.),
            AudioTrack::new("click_hit.wav", Second(1.56)..Second(1.81)).gain_db(-24.),
            AudioTrack::new("click_hit.wav", Second(2.09)..Second(2.34)).gain_db(-24.),
            AudioTrack::new("click_hit.wav", Second(2.26)..Second(2.51)).gain_db(-24.),
            AudioTrack::new("click_hit.wav", Second(2.79)..Second(3.04)).gain_db(-24.),
            AudioTrack::new("click_hit.wav", Second(2.96)..Second(3.21)).gain_db(-24.),
            AudioTrack::new("click_hit.wav", Second(3.725)..Second(3.975)).gain_db(-20.),
            AudioTrack::new("click_hit.wav", Second(4.07)..Second(4.32)).gain_db(-24.),
            AudioTrack::new("bell_hit.wav", Second(4.325)..Second(5.6)).gain_db(-20.),
            AudioTrack::new("swoosh_hit.wav", Second(7.825)..Second(8.9)).gain_db(-18.),
            AudioTrack::new("chime_hit.wav", Second(8.6)..Second(13.5))
                .gain_db(-12.)
                .fade_out(0.5),
            AudioTrack::new("bell_hit.wav", Second(8.625)..Second(9.9)).gain_db(-14.),
            AudioTrack::new("tick_hit.wav", Second(9.4)..Second(9.65)).gain_db(-26.),
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
