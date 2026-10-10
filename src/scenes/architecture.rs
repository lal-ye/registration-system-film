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
/// One clock for the dot, the hop labels, the bolt and the cues: hop k
/// departs at DEPART + k * HOP and arrives 1.0 s later (the decel runs
/// 0.53 + k * HOP .. +1.5, so the bolt rides its last second).
const T0: f32 = 0.53;
const DEPART: f32 = 1.03;
const ARRIVE_LEN: f32 = 1.0;
/// Hop 4 is where the work leaves Java: arrival index 3 at this film time.
const SQL_ARRIVE: f32 = DEPART + 3.0 * HOP + ARRIVE_LEN;
/// SQL-boundary flicker after the arrival, then a dim ember.
const SQL_PAT: [f32; 8] = [1.0, 0.2, 0.8, 0.0, 0.5, 0.0, 0.15, 0.0];

fn depart(k: usize) -> f32 {
    DEPART + k as f32 * HOP
}

#[derive(Debug)]
pub struct Architecture;

impl Scene for Architecture {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(15.0)
    }

    // One short connector tick per hop departure (the dot's own clock) plus a
    // soft click on each arrival, and a sweep that carries the 48s cut.
    // Nothing here is longer than 0.35 s: the old full-file plays layered
    // into audible static, which is why they are gone.
    fn audio(&self) -> AudioMap<'_> {
        let mut tracks: Vec<AudioTrack> = Vec::new();
        for k in 0..HOPS.len() {
            tracks.push(
                AudioTrack::new("tick_hit.wav", Second(depart(k))..Second(depart(k) + 0.3))
                    .gain_db(-20.),
            );
            tracks.push(
                AudioTrack::new(
                    "click_hit.wav",
                    Second(depart(k) + ARRIVE_LEN)..Second(depart(k) + ARRIVE_LEN + 0.3),
                )
                .gain_db(-26.),
            );
        }
        tracks.push(
            AudioTrack::new("swoosh_hit.wav", Second(14.7)..Second(15.0))
                .gain_db(-16.)
                .fade_out(0.3),
        );
        AudioMap::from_iter(tracks)
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        // One hop per 4 beats. `step` is the index of the hop in progress, so the band highlight
        // and the dot are always reading the same clock.
        let step = ((frame.seconds() - T0) / HOP).floor().max(0.0);
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
        let hop_start = T0 + step * HOP;
        // The last hop holds its label to the end of the scene; the others hand off.
        let label_op = if idx == HOPS.len() - 1 {
            ramp(&frame, hop_start)
        } else {
            ramp(&frame, hop_start) * (1.0 - seg(&frame, hop_start + HOP - 0.35, 0.35))
        };
        // Decelerate into each node: constant velocity reads as a machine, and the
        // arrival is what makes the stop legible.
        let eased = decel(&frame, T0 + step * HOP, 1.5);
        let (_, label) = HOPS[idx];
        let (py, _) = HOPS[idx.saturating_sub(1)];
        let y = py + (HOPS[idx].0 - py) * eased;
        // Hop 4 is where the work leaves Java. Before the arrival the dot goes
        // dark exactly as before; after it the dark flickers, then a dim ember.
        let t = frame.seconds();
        let dark = if idx == 3 && t < SQL_ARRIVE {
            seg(&frame, 4.83, 0.9)
        } else if idx == 3 {
            let ph = ((t - SQL_ARRIVE) / 0.09).floor() as usize;
            if ph < SQL_PAT.len() { SQL_PAT[ph] } else { 0.12 }
        } else {
            0.0
        };
        let dot_op = 1.0 - dark;

        // Electricity on the rail, keyed to the same depart/arrive clock as the
        // cues. Bolt while the hop is in flight (it dies with the dot at the
        // SQL boundary); burst, ring and squash on each arrival. All windows
        // are guarded so no zero-size geometry is ever emitted, and nothing is
        // visible at the Architecture@6s snapshot frame.
        let s = idx;
        let seed = fidx(&frame) / 2;
        if s >= 1 {
            let age = t - depart(s);
            if age > 0.005 && age < ARRIVE_LEN {
                let fade =
                    (age / 0.12).min(1.0) * ((ARRIVE_LEN - age) / 0.15).min(1.0).max(0.0);
                let p = fade * dot_op;
                if p > 0.01 {
                    let (py0, _) = HOPS[s - 1];
                    let y1 = HOPS[s].0;
                    let mut d = format!("M {RAIL_X} {py0}");
                    for j in 1..7 {
                        let yy = py0 + (y1 - py0) * j as f32 / 7.0;
                        let xx =
                            RAIL_X + (frand(seed, s as u64 * 16 + j as u64) - 0.5) * 28.0;
                        d.push_str(&format!(" L {xx:.1} {yy:.1}"));
                    }
                    out.push(fframes::svgr!(<g opacity={p}>
                        <path d={d.clone()} stroke={RUST} stroke-width="5" stroke-linecap="round" fill="none" opacity="0.25" />
                        <path d={d.clone()} stroke={RUST} stroke-width="2.5" stroke-linecap="round" fill="none" opacity="0.45" />
                        <path d={d} stroke="#d4694e" stroke-width="1" stroke-linecap="round" fill="none" opacity="0.9" />
                    </g>));
                }
            }
            // The previous hop's arrival point and age.
            let at = depart(s) - HOP + ARRIVE_LEN;
            let bage = t - at;
            if bage > 0.005 && bage < 0.35 {
                let k = bage / 0.35;
                let (ay, _) = HOPS[s - 1];
                let mut d = String::new();
                for j in 0..9 {
                    let ang =
                        -std::f32::consts::PI * (0.15 + 0.7 * j as f32 / 8.0);
                    let sp = 120.0 + 160.0 * frand(seed, 100 + j as u64);
                    let dx = ang.cos() * sp * bage;
                    let dy = ang.sin() * sp * bage + 320.0 * bage * bage;
                    d.push_str(&format!("M {RAIL_X} {ay} l {dx:.1} {dy:.1} "));
                }
                let bop = 0.85 * (1.0 - k);
                let r = 10.0 + 110.0 * k;
                let rop = 0.5 * (1.0 - k);
                let sq = 1.0 - 0.35 * (1.0 - k);
                let rx = 15.0 * 1.25;
                let ry = 15.0 * sq;
                out.push(fframes::svgr!(<g>
                    <path d={d} stroke={RUST} stroke-width="3" stroke-linecap="round" fill="none" opacity={bop} />
                    <circle cx={RAIL_X} cy={ay} r={r} fill="none" stroke={RUST} stroke-width="2" opacity={rop} />
                    <ellipse cx={RAIL_X} cy={y} rx={rx} ry={ry} fill={RUST} opacity={dot_op} />
                </g>));
            }
        }

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
