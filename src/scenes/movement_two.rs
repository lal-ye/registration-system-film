//! 64-78s. Movement II: the caller never sees the machinery. Seven pairs are visibly one rule;
//! the crossing is the same rule in code, and it is checkable in the repo.
use crate::design::*;
use fframes::{AudioMap, AudioTimestamp::*, AudioTrack, Duration, FFramesContext, Frame, Scene, Svgr};

const PAIRS: [&str; 7] = [
    "UserDAO          →  UserDAOImp",
    "StudentDAO       →  StudentDAOImp",
    "InstructorDAO    →  InstructorDAOImp",
    "CourseDAO        →  CourseDAOImp",
    "RegistrationDAO  →  RegistrationDAOImp",
    "ScheduleDAO      →  ScheduleDAOImp",
    "GradeDAO         →  GradeDAOImp",
];

const LEFT: [&str; 6] = [
    "interface RegistrationDAO {",
    "    enum EnrollResult { SUCCESS, ALREADY_ENROLLED,",
    "                      PREREQUISITES_NOT_MET, ERROR }",
    "",
    "    EnrollResult enroll(int studentId,",
    "                        String courseCode);",
];

const RIGHT: [&str; 6] = [
    "class RegistrationDAOImp implements RegistrationDAO {",
    "    String call = \"{call sp_enroll_student(?, ?)}\";",
    "    try (Connection c = DBConnection.getConnection();",
    "         CallableStatement s = c.prepareCall(call)) { ... }",
    "    catch (SQLException e)",
    "        if (msg.contains(\"PREREQUISITES_NOT_MET\")) ...",
];

#[derive(Debug)]
pub struct MovementTwo;

impl Scene for MovementTwo {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(14.0)
    }

    // Ticks under the seven arriving pair rows (2.855 + i * 0.42), a wash +
    // bell anchor under the pairs caption (5.855), a sweep into the crossing
    // (7.355) with a tick on the divider (7.455), a wash + bell under the
    // landing (11.35), and the rising whoosh (11.0-14.0, faded) that peaks
    // exactly on the 76s cut into Payoff — where the bed has gone silent.
    fn audio(&self) -> AudioMap<'_> {
        AudioMap::from([
            AudioTrack::new("click_hit.wav", Second(0.105)..Second(0.355)).gain_db(-26.),
            AudioTrack::new("tick_hit.wav", Second(2.855)..Second(3.105)).gain_db(-24.),
            AudioTrack::new("tick_hit.wav", Second(3.275)..Second(3.525)).gain_db(-24.),
            AudioTrack::new("tick_hit.wav", Second(3.695)..Second(3.945)).gain_db(-24.),
            AudioTrack::new("tick_hit.wav", Second(4.115)..Second(4.365)).gain_db(-24.),
            AudioTrack::new("tick_hit.wav", Second(4.535)..Second(4.785)).gain_db(-24.),
            AudioTrack::new("tick_hit.wav", Second(4.955)..Second(5.205)).gain_db(-24.),
            AudioTrack::new("tick_hit.wav", Second(5.375)..Second(5.625)).gain_db(-24.),
            AudioTrack::new("chime_hit.wav", Second(5.55)..Second(6.9))
                .gain_db(-16.)
                .fade_out(0.4),
            AudioTrack::new("bell_hit.wav", Second(5.855)..Second(7.1)).gain_db(-20.),
            AudioTrack::new("swoosh_hit.wav", Second(7.355)..Second(8.5)).gain_db(-18.),
            AudioTrack::new("tick_hit.wav", Second(7.455)..Second(7.705)).gain_db(-20.),
            AudioTrack::new("chime_hit.wav", Second(11.05)..Second(12.6))
                .gain_db(-16.)
                .fade_out(0.4),
            AudioTrack::new("bell_hit.wav", Second(11.35)..Second(12.6)).gain_db(-16.),
            AudioTrack::new("whoosh_rise.wav", Second(11.0)..Second(14.0))
                .gain_db(-10.)
                .fade_out(1.0),
        ])
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let mut out: Vec<Svgr> = Vec::new();

        let thesis_op = in_out(&frame, 0.105, 2.25, 0.5);
        if thesis_op > 0.01 {
            out.push(fframes::svgr!(<g opacity={thesis_op}>
                {statement("the machinery stays hidden.", STATEMENT, 192.0, 540.0, 1.0, INK)}
            </g>));
        }

        // Seven repetitions of one shape, ticking in on a beat. The block is centred: the
        // longest row is 38 mono chars, so 698 is the left edge that puts its middle on the page.
        let pairs_op = in_out(&frame, 2.705, 4.3, 0.5);
        if pairs_op > 0.01 {
            let mut rows: Vec<Svgr> = Vec::new();
            for (i, pair) in PAIRS.iter().enumerate() {
                let op = ramp(&frame, 2.855 + i as f32 * 0.42) * pairs_op;
                rows.push(fframes::svgr!(<text x="698" y={330.0 + i as f32 * 66.0} font-family={MONO_F}
                    font-weight={MONO_W} font-size={MONO_MD} fill={INK} opacity={op}>{*pair}</text>));
            }
            out.push(fframes::svgr!(<g>
                {rows}
                {mono_c("seven interfaces, seven implementations · interfaces hide implementation details", MONO_XS, 960.0, 812.0, ramp(&frame, 5.855) * pairs_op, "#6d675d")}
            </g>));
        }

        // The crossing. A hard rule, the contract on one side, the machinery on the other.
        // Retires fully before the landing arrives, so the statement never sits on the code.
        let cross_op = in_out(&frame, 7.355, 2.7, 0.4);
        if cross_op > 0.01 {
            let mut l: Vec<Svgr> = Vec::new();
            let mut r: Vec<Svgr> = Vec::new();
            for i in 0..LEFT.len() {
                l.push(fframes::svgr!(<text x="196" y={290.0 + i as f32 * 46.0} font-family={MONO_F}
                    font-weight={MONO_W} font-size={MONO_MD} fill={INK}
                    opacity={ramp(&frame, 7.455 + i as f32 * 0.1) * cross_op}>{LEFT[i]}</text>));
                r.push(fframes::svgr!(<text x="1000" y={290.0 + i as f32 * 46.0} font-family={MONO_F}
                    font-weight={MONO_W} font-size={MONO_MD} fill={INK}
                    opacity={ramp(&frame, 7.555 + i as f32 * 0.1) * cross_op}>{RIGHT[i]}</text>));
            }
            out.push(fframes::svgr!(<g opacity={cross_op}>
                {mono("DAO/RegistrationDAO.java — interface-based polymorphism", MONO_XS, 196.0, 236.0, 1.0, "#6d675d")}
                {mono("DAO/RegistrationDAOImp.java — method overriding", MONO_XS, 1000.0, 236.0, 1.0, "#6d675d")}
                {l}
                {r}
                <rect x="958" y="212" width="3" height="300" fill={INK} opacity={ramp(&frame, 7.455)} />
            </g>));
        }

        let landing_op = ramp(&frame, 11.35);
        out.push(fframes::svgr!(<g opacity={landing_op} transform={Transform::translate(0.0, rise(&frame, 11.35, 26.0))}>
            {statement("same call. no idea which.", STATEMENT, 192.0, 540.0, 1.0, INK)}
        </g>));
        // No wipe or flash: the landing line and whoosh carry the settle.
        fframes::svgr!(<g>{out}</g>)
    }
}
