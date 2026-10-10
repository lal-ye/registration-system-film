//! 0-4s. One number on paper: 97.0, the film's real weighted total, legible and unexplained.
//! It resolves into the enormous `A+`, which is a mystery here and a repayment at 86s.
use crate::design::*;
use fframes::{AudioMap, AudioTimestamp::*, AudioTrack, Duration, FFramesContext, Frame, Scene, Svgr};

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

    fn audio(&self) -> AudioMap<'_> {
        // Chime motif lands exactly as the A+ ramps in at 2.4s.
        AudioMap::from([AudioTrack::new("bell_hit.wav", Second(2.4)..Second(3.8)).gain_db(-24.)])
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