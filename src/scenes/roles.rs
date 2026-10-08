//! 10-22s. Three roles, three identical frames. A name lands alone, then the one method that
//! role exists to call. Two of the three are not Java at all.
use crate::design::*;
use fframes::{Duration, FFramesContext, Frame, Scene, Svgr};

const ROLES: [(&str, &str); 3] = [
    ("Student", "EnrollResult enroll(int studentId, String courseCode)"),
    ("Instructor", "sp_updategrade(?, ?, ?)"),
    ("Head of Department", "sp_create_course_full(?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"),
];

#[derive(Debug)]
pub struct Roles;

impl Scene for Roles {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(12.0)
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let mut out: Vec<Svgr> = Vec::new();
        for (i, (role, call)) in ROLES.iter().enumerate() {
            let at = i as f32 * 4.0;
            let op = in_out(&frame, at, 3.1, 0.45);
            if op <= 0.01 {
                continue;
            }
            out.push(fframes::svgr!(<g opacity={op}>
                {statement(role, ROLE, 192.0, 468.0, 1.0, INK)}
                {mono(call, MONO_MD, 196.0, 574.0, ramp(&frame, at + 0.9), RUST)}
                {rule(196.0, 620.0, 96.0 * ramp(&frame, at + 0.9), ramp(&frame, at + 0.9))}
            </g>));
        }
        fframes::svgr!(<g>{out}</g>)
    }
}
