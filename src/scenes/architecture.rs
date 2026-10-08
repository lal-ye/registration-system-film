//! 35-50s. The stack, drawn in our own type at full size, with one pulse of light going down it.
//! The dot performs MVC rather than asserting it, and goes dark at the SQL boundary.
use crate::design::*;
use fframes::{Duration, FFramesContext, Frame, Scene, Svgr};

struct Band {
    label: &'static str,
    y: f32,
    h: f32,
    lines: [&'static str; 2],
}

const BANDS: [Band; 4] = [
    Band { label: "VIEW", y: 130.0, h: 140.0, lines: ["LoginFrame · StudentDashboardFrame · InstructorDashboardFrame · HoDDashboardFrame · RegistrationFrame", ""] },
    Band { label: "CONTROLLER", y: 300.0, h: 140.0, lines: ["LoginController · StudentDashboardController · InstructorDashboardController · HoDDashboardController · RegistrationController", ""] },
    Band { label: "DAO", y: 470.0, h: 150.0, lines: ["UserDAO · StudentDAO · InstructorDAO · CourseDAO · RegistrationDAO · ScheduleDAO · GradeDAO", "seven interfaces, seven implementations"] },
    Band { label: "MODEL / UTIL", y: 650.0, h: 150.0, lines: ["Assessment · Course · EnrolledStudentRecord · Instructor · Registration · Schedule", "Section · Student · User · GradeUtils · DBConnection"] },
];

const DB_Y: f32 = 832.0;

/// Where the dot stands for each hop, and what that hop is called.
const HOPS: [(f32, f32, &str); 7] = [
    (520.0, 200.0, "LoginFrame — Enroll in Selected Course"),
    (760.0, 370.0, "StudentDashboardController"),
    (960.0, 545.0, "RegistrationDAO.enroll(1814, \"CS101\")"),
    (960.0, DB_Y + 28.0, "{call sp_enroll_student(?, ?)}"),
    (1120.0, 545.0, "EnrollResult.SUCCESS"),
    (860.0, 725.0, "new Registration(...)"),
    (700.0, 200.0, "StudentDashboardFrame.repaint()"),
];

#[derive(Debug)]
pub struct Architecture;

impl Scene for Architecture {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(15.0)
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        // One hop per 2 s. `step` is the index of the hop in progress, so the band highlight
        // and the dot are always reading the same clock.
        let step = ((frame.seconds() - 0.6) / 2.0).floor().max(0.0);
        let mut out: Vec<Svgr> = Vec::new();
        for (i, band) in BANDS.iter().enumerate() {
            let flash = (1.0 - ((i as f32 - step) * 1.6).abs().min(1.0)) * 0.09;
            out.push(fframes::svgr!(
                <g>
                    <rect x="192" y={band.y} width="1536" height={band.h} rx="8" fill={CARD} opacity={0.62 + flash} />
                    <rect x="192" y={band.y} width="4" height={band.h} rx="2" fill={RUST} opacity={flash * 4.0} />
                    <text x="224" y={band.y + 34.0} font-family={SANS} font-weight={SANS_W} font-size={UI_SM}
                          letter-spacing="2.4" fill="#8a8479">{band.label}</text>
                    <text x="224" y={band.y + 82.0} font-family={MONO_F} font-weight={MONO_W} font-size={MONO_SM}
                          fill={INK}>{band.lines[0]}</text>
                    {mono(band.lines[1], MONO_SM, 224.0, band.y + 116.0, 1.0, "#6d675d")}
                </g>
            ));
        }
        out.push(fframes::svgr!(
            <g>
                <rect x="192" y={DB_Y} width="1536" height="56" rx="8" fill={INK} opacity="0.07" />
                <text x="224" y={DB_Y + 34.0} font-family={MONO_F} font-weight={MONO_W} font-size={MONO_SM}
                      fill="#6d675d">"MySQL · universitydb · 12 tables · 7 stored procedures"</text>
            </g>
        ));

        let idx = (step as usize).min(HOPS.len() - 1);
        let eased = seg(&frame, 0.6 + step * 2.0, 1.5);
        let (cx, cy, label) = HOPS[idx];
        let (px, py, _) = HOPS[idx.saturating_sub(1)];
        let x = px + (cx - px) * eased;
        let y = py + (cy - py) * eased;
        // Hop 4 is where the work leaves Java: the dot goes dark and nothing else moves.
        let dark = if idx == 3 { seg(&frame, 4.9, 0.9) } else { 0.0 };
        let dot_op = 1.0 - dark;

        out.push(fframes::svgr!(<g>
            <circle cx={x} cy={y} r="15" fill={RUST} opacity={dot_op} />
            <circle cx={x} cy={y} r="27" fill={RUST} opacity={0.16 * dot_op} />
            <text x="192" y="964" font-family={MONO_F} font-weight={MONO_W} font-size={MONO_LG}
                  fill={INK} opacity={ramp(&frame, 0.6 + step * 2.0) * (1.0 - seg(&frame, 0.9 + step * 2.0, 0.35))}>{label}</text>
        </g>));
        fframes::svgr!(<g>{out}</g>)
    }
}
