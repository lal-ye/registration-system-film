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
