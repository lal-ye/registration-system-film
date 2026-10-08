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
