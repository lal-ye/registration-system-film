//! 22-35s. One login, then the three decisions the code made. The dimmed lines are the point:
//! it is a chain, and for this login the third branch was never reached.
use crate::design::*;
use crate::ui::*;
use fframes::{Duration, FFramesContext, Frame, Scene, Svgr};

const ID: &str = "1814";
const MASK: &str = "••••••";

#[derive(Debug)]
pub struct Login;

impl Scene for Login {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(13.0)
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let t = frame.seconds();
        let id = typed(&frame, 0.55, 5.0, ID);
        let pw = typed(&frame, 2.15, 7.0, MASK);
        let press = (t - 4.25).clamp(0.0, 0.35) / 0.35 * (1.0 - ((t - 4.6).clamp(0.0, 0.4) / 0.4));
        let press = press.clamp(0.0, 1.0);
        let login_op = (1.0 - ramp(&frame, 4.75)).clamp(0.0, 1.0);
        let dash_op = ramp(&frame, 5.15);

        let enrolled: Vec<Vec<String>> = [
            ["1001", "CS101", "SEC-1", "Enrolled"],
            ["1001", "DB301", "SEC-1", "Enrolled"],
            ["1001", "ME101", "SEC-2", "Enrolled"],
        ]
        .iter()
        .map(|r| r.iter().map(|s| (*s).to_string()).collect())
        .collect();

        let strip = [
            ("if (authenticateStudent(1814, ••••••) != null)", "taken", RUST, 1.0_f64),
            ("    return new StudentDashboardFrame();", "", INK, 1.0_f64),
            ("if (authenticateInstructor(1814, ••••••) != null)", "not reached", INK, 0.3_f64),
            ("if (isHeadOfDepartment(1814))", "not reached", INK, 0.3_f64),
        ];
        let mut lines: Vec<Svgr> = Vec::new();
        for (i, (code, tag, fill, dim)) in strip.iter().enumerate() {
            let at = 7.75 + i as f32 * 0.2;
            let op = ramp(&frame, at) * *dim as f32;
            lines.push(fframes::svgr!(<g opacity={op}>
                <text x="470" y={812.0 + i as f32 * 46.0} font-family={MONO_F} font-weight={MONO_W}
                      font-size={MONO_MD} fill={*fill}>{*code}</text>
                <text x="1450" y={812.0 + i as f32 * 46.0} font-family={MONO_F} font-weight={MONO_W}
                      font-size={MONO_XS} fill={*fill}>{*tag}</text>
            </g>));
        }

        fframes::svgr!(<g>
            <g opacity={login_op} transform={Transform::translate(0.0, rise(&frame, 0.0, 40.0) * login_op)}>
                {login_window((id, pw), caret(&frame, 0.55, 5.0, ID), caret(&frame, 2.15, 7.0, MASK), press)}
            </g>
            <g opacity={dash_op} transform={Transform::translate(0.0, rise(&frame, 5.15, 46.0) * (1.0 - dash_op))}>
                {student_dashboard(enrolled, 0.35 + 0.3 * (frame.seconds() * 0.7).sin())}
            </g>
            <rect x="440" y="762" width="1040" height="1" fill="#ddd9d0" opacity={ramp(&frame, 7.45)} />
            {lines}
        </g>)
    }
}
