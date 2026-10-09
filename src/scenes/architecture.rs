//! 35-50s. The stack, drawn in our own type at full size, with one pulse of light going down it.
//! The dot performs MVC rather than asserting it, and goes dark at the SQL boundary.
use crate::design::*;
use fframes::{AudioMap, AudioTimestamp::*, AudioTrack, Duration, FFramesContext, Frame, Scene, Svgr};

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

/// The dot runs on a rail in the right-hand gutter, clear of every band, so it
/// never sits on top of the class name it is meant to light up. The bands run
/// x=192..1728, so 1806 is outside all of them.
const RAIL_X: f32 = 1806.0;

/// Where the dot stands for each hop, and what that hop is called. Only the
/// height varies: the rail is fixed, so the travel reads as "down the stack".
const HOPS: [(f32, &str); 7] = [
    (200.0, "LoginFrame — Enroll in Selected Course"),
    (370.0, "StudentDashboardController"),
    (545.0, "RegistrationDAO.enroll(1814, \"CS101\")"),
    (DB_Y + 28.0, "{call sp_enroll_student(?, ?)}"),
    (545.0, "EnrollResult.SUCCESS"),
    (725.0, "new Registration(...)"),
    (200.0, "StudentDashboardFrame.repaint()"),
];

/// Hop spacing snapped to the bed's beat grid: 4 beats at 136 BPM
/// (4 x 0.4412 s), so every hop lands on a beat.
const HOP: f32 = 1.7648;

#[derive(Debug)]
pub struct Architecture;

impl Scene for Architecture {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(15.0)
    }

    // One connector tick per hop label (hop starts at 0.53 + k * HOP, same
    // clock the dot runs on), plus a sweep that carries the 48s cut.
    fn audio(&self) -> AudioMap<'_> {
        AudioMap::from([
            AudioTrack::new("tick.mp3", Second(0.53)..Eof).gain_db(-20.),
            AudioTrack::new("tick.mp3", Second(0.53 + HOP)..Eof).gain_db(-20.),
            AudioTrack::new("tick.mp3", Second(0.53 + 2.0 * HOP)..Eof).gain_db(-20.),
            AudioTrack::new("tick.mp3", Second(0.53 + 3.0 * HOP)..Eof).gain_db(-20.),
            AudioTrack::new("tick.mp3", Second(0.53 + 4.0 * HOP)..Eof).gain_db(-20.),
            AudioTrack::new("tick.mp3", Second(0.53 + 5.0 * HOP)..Eof).gain_db(-20.),
            AudioTrack::new("tick.mp3", Second(0.53 + 6.0 * HOP)..Eof).gain_db(-20.),
            AudioTrack::new("swoosh.mp3", Second(14.7)..Eof).gain_db(-16.),
        ])
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        // One hop per 4 beats. `step` is the index of the hop in progress, so the band highlight
        // and the dot are always reading the same clock.
        let step = ((frame.seconds() - 0.6) / HOP).floor().max(0.0);
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
        let hop_start = 0.53 + step * HOP;
        // The last hop holds its label to the end of the scene; the others hand off.
        let label_op = if idx == HOPS.len() - 1 {
            ramp(&frame, hop_start)
        } else {
            ramp(&frame, hop_start) * (1.0 - seg(&frame, hop_start + HOP - 0.35, 0.35))
        };
        // Decelerate into each node: constant velocity reads as a machine, and the
        // arrival is what makes the stop legible.
        let eased = decel(&frame, 0.53 + step * HOP, 1.5);
        let (_, label) = HOPS[idx];
        let (py, _) = HOPS[idx.saturating_sub(1)];
        let y = py + (HOPS[idx].0 - py) * eased;
        // Hop 4 is where the work leaves Java: the dot goes dark and nothing else moves.
        let dark = if idx == 3 { seg(&frame, 4.83, 0.9) } else { 0.0 };
        let dot_op = 1.0 - dark;

        out.push(fframes::svgr!(<g>
            <line x1={RAIL_X} y1="130" x2={RAIL_X} y2={DB_Y + 56.0} stroke={INK} stroke-width="1" opacity="0.18" />
            <circle cx={RAIL_X} cy={y} r="15" fill={RUST} opacity={dot_op} />
            <circle cx={RAIL_X} cy={y} r="27" fill={RUST} opacity={0.16 * dot_op} />
            <text x="192" y="964" font-family={MONO_F} font-weight={MONO_W} font-size={MONO_LG}
                  fill={INK} opacity={label_op}>{label}</text>
        </g>));
        fframes::svgr!(<g>{out}</g>)
    }
}
