//! 4-6s. One word. The cut that turns a result into a question.
use crate::design::*;
use fframes::{AudioMap, AudioTimestamp::*, AudioTrack, Duration, FFramesContext, Frame, Scene, Svgr};

#[derive(Debug)]
pub struct Rewind;

impl Scene for Rewind {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(2.0)
    }

    fn audio(&self) -> AudioMap<'_> {
        // Rule ramp at 0.9: click on the stroke, swish under the sweep.
        AudioMap::from([
            AudioTrack::new("click_hit.wav", Second(0.15)..Second(0.4)).gain_db(-26.),
            AudioTrack::new("swoosh_hit.wav", Second(0.9)..Second(2.0))
                .gain_db(-22.)
                .fade_out(0.3),
        ])
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
