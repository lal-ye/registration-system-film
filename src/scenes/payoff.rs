//! 78-91s. The one frame where every layer participates at once: View, Controller, DAO,
//! Assessment, GradeUtils and a stored procedure. The label resolves to the letter the film
//! opened on, and the window recedes so the letter is the last thing standing.
use crate::design::*;
use crate::ui::*;
use fframes::{Duration, FFramesContext, Frame, Scene, Svgr};

const RESULT: &str = "Scores saved. Weighted total: 97.0% → Grade: A+";

#[derive(Debug)]
pub struct Payoff;

impl Scene for Payoff {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(13.0)
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let scores = [
            typed(&frame, 1.2, 6.0, "29"),
            typed(&frame, 1.9, 6.0, "20"),
            typed(&frame, 2.6, 6.0, "48"),
        ];
        let press = (1.0 - ((t - 3.7).clamp(0.0, 0.35) / 0.35)) * ((t - 3.35).clamp(0.0, 0.35) / 0.35);
        let press = press.clamp(0.0, 1.0);
        let result_op = ramp(&frame, 4.3);
        // The window recedes, then the letter takes the page.
        let recede = seg(&frame, 8.0, 1.4);
        let window_op = 1.0 - recede;
        let drift = recede * 26.0;
        let a_op = ramp(&frame, 9.6);

        fframes::svgr!(
        <g>
            <g opacity={window_op} transform={Transform::translate(0.0, drift)}>
                {grade_entry("CS101 — Introduction to Programming", "1001 · Abebe Kebede", scores,
                             caret(&frame, 2.6, 6.0, "48"), RESULT, result_op, press)}
            </g>
            {letter(a_op, INK)}
        </g>)
    }
}
