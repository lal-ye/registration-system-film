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
            AudioTrack::new("click_hit.wav", Second(0.05)..Second(0.3)).gain_db(-26.),
            AudioTrack::new("tick_hit.wav", Second(0.45)..Second(0.7)).gain_db(-24.),
            AudioTrack::new("tick_hit.wav", Second(0.61)..Second(0.86)).gain_db(-24.),
            AudioTrack::new("tick_hit.wav", Second(0.77)..Second(1.02)).gain_db(-24.),
            AudioTrack::new("swoosh_hit.wav", Second(1.7)..Second(2.0))
                .gain_db(-16.)
                .fade_out(0.3),
        ])
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let lines: Vec<Svgr> = RULES
            .iter()
            .enumerate()
            .map(|(i, rule)| mono(rule, MONO_MD, 196.0, 566.0 + i as f32 * 44.0, ramp(&frame, 0.45 + i as f32 * 0.16), INK))
            .collect();
        let mut out: Vec<Svgr> = vec![fframes::svgr!(<g>
            <g opacity={ramp(&frame, 0.05)} transform={Transform::translate(0.0, rise(&frame, 0.05, 26.0))}>
                {statement("shared rules. one place.", STATEMENT, 192.0, 452.0, 1.0, INK)}
            </g>
            {lines}
        </g>)];
        // No exit wipe: the geometry as written covered the full frame at the
        // wrong end of the sweep (a paper flash, not a wipe). The cut to Roles
        // is carried by the swoosh cue and Roles' card 1, which is already
        // ramping at t=0.
        fframes::svgr!(<g>{out}</g>)
    }
}
