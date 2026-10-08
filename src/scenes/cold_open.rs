//! 0-4s. Chaos you can tell is code and cannot read, collapsing into one enormous letter.
//! The letter is a mystery here; at 86s it is a repayment.
use crate::design::*;
use fframes::{Duration, FFramesContext, Frame, Scene, Svgr};

const FRAGMENTS: [&str; 28] = [
    "private final int",
    "setGender(",
    "sp_enroll_student(?, ?)",
    "getConnection()",
    "implements UserDAO",
    "SELECT * FROM registration",
    "throw new IllegalArgumentException",
    "public void setMaxScore(",
    "jdbc:mysql://localhost:3306",
    "weightedToLetter(",
    "CallableStatement stmt =",
    "signal sqlstate '45000'",
    "INSERT INTO student_score",
    "prepareCall(call)",
    "isHeadOfDepartment(",
    "public abstract class User",
    "sp_get_weighted_total(?, ?)",
    "catch (SQLException e)",
    "return EnrollResult.SUCCESS;",
    "private String gender;",
    "ON DUPLICATE KEY UPDATE",
    "sp_updategrade(?, ?, ?)",
    "sp_create_schedule",
    "FlatLightLaf.setup()",
    "new UserDAOImp()",
    "ResultSet rs = stmt.",
    "setSectionId(int sectionId)",
    "REGISTRATION TABLE",
];

/// A deterministic 0..1 hash, so the chaos is identical on every frame of every run.
fn h(n: u32) -> f32 {
    let mut x = n.wrapping_mul(2654435761);
    x ^= x >> 15;
    x = x.wrapping_mul(2246822519);
    x ^= x >> 13;
    (x % 10_000) as f32 / 10_000.0
}

#[derive(Debug)]
pub struct ColdOpen;

impl Scene for ColdOpen {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(4.0)
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let mut bits: Vec<Svgr> = Vec::with_capacity(200);
        for i in 0..196u32 {
            let text = FRAGMENTS[(i as usize) % FRAGMENTS.len()];
            let (a, b, c) = (h(i * 7 + 1), h(i * 7 + 2), h(i * 7 + 3));
            // Where it starts, scattered well past the edges so the frame looks over-full.
            let sx = -140.0 + a * 2200.0;
            let sy = -40.0 + b * 1160.0;
            // Where it ends: a tight column, because that is where the letter will be.
            let ex = LETTER_X - 40.0 + c * 80.0;
            let ey = LETTER_Y - 40.0 + h(i * 7 + 4) * 80.0;
            // Each fragment starts drifting toward the column at its own moment.
            let pull = frame.animate_runtime(fframes::AnimateRuntimeInput {
                on_second: 0.3 + c * 1.3,
                from: 0.0,
                to: 1.0,
                animation_runtime: &EASE,
            });
            let x = sx + (ex - sx) * pull;
            let y = sy + (ey - sy) * pull;
            let gone = scrub(&frame, 2.2 + h(i * 7 + 5) * 1.2);
            let op = (0.14 + a * 0.16) * (1.0 - gone);
            if op > 0.004 {
                bits.push(fframes::svgr!(<text x={x} y={y} font-family={MONO_F} font-weight={MONO_W}
                    font-size="13" fill={INK} opacity={op}>{text}</text>));
            }
        }
        let a_op = ramp(&frame, 3.1);
        fframes::svgr!(<g>
            {bits}
            {letter(a_op, INK)}
        </g>)
    }
}
