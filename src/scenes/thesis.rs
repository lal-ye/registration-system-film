//! 8-10s. The problem the next 78 seconds answer: three roles, one set of rules, and rules need
//! somewhere to live. The three constants are real, raised from the database.
use crate::design::*;
use fframes::{Duration, FFramesContext, Frame, Scene, Svgr};

const RULES: [&str; 3] = ["PREREQUISITES_NOT_MET", "ROOM_CONFLICT", "INSTRUCTOR_CONFLICT"];

#[derive(Debug)]
pub struct Thesis;

impl Scene for Thesis {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(2.0)
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
