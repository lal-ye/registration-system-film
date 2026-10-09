//! 50-64s. Movement I: a fact lives in exactly one place. A rule lives in the class; a field
//! lives in the base class. Each proof is ~4 s, the landing ~3 s.
use crate::design::*;
use fframes::{Duration, FFramesContext, Frame, Scene, Svgr};

const GUARD: [&str; 5] = [
    "public void setGender(String gender) {",
    "    if (gender == null || (!gender.equals(\"M\") && !gender.equals(\"F\"))) {",
    "        throw new IllegalArgumentException(\"Gender must be 'M' or 'F'.\");",
    "    }",
    "    this.gender = gender;",
];

#[derive(Debug)]
pub struct MovementOne;

impl Scene for MovementOne {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(14.0)
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let mut out: Vec<Svgr> = Vec::new();

        // 0-2.9s: the thesis, alone on paper.
        let thesis_op = in_out(&frame, 0.26, 2.25, 0.5);
        if thesis_op > 0.01 {
            out.push(fframes::svgr!(<g opacity={thesis_op}>
                {statement("one fact, one place.", STATEMENT, 192.0, 540.0, 1.0, INK)}
            </g>));
        }

        // 2.75-6.75s: proof. A rule that cannot be stepped over, from inside the class.
        let guard_op = in_out(&frame, 2.86, 3.5, 0.5);
        if guard_op > 0.01 {
            let mut lines: Vec<Svgr> = Vec::new();
            for (i, line) in GUARD.iter().enumerate() {
                let op = ramp(&frame, 2.96 + i as f32 * 0.14) * guard_op;
                let is_throw = i == 2;
                lines.push(fframes::svgr!(<text x={196.0 + i as f32 * 18.0} y={430.0 + i as f32 * 44.0}
                    font-family={MONO_F} font-weight={MONO_W} font-size={MONO_MD}
                    fill={if is_throw { RUST } else { INK }} opacity={op}>{*line}</text>));
            }
            out.push(fframes::svgr!(<g>
                {lines}
                {rule(232.0, 534.0, 1021.0 * ramp(&frame, 3.36), ramp(&frame, 3.36) * guard_op)}
                {mono("Model/User.java — Encapsulation: private fields, getters, setters", MONO_XS, 196.0, 690.0, ramp(&frame, 3.26) * guard_op, "#6d675d")}
            </g>));
        }

        // 6.6-10.6s: proof. Inheritance made visible as absence: two nearly empty boxes.
        let boxes_op = in_out(&frame, 6.71, 3.5, 0.5);
        if boxes_op > 0.01 {
            let links: Vec<Svgr> = vec![
                fframes::svgr!(<rect x="899" y="432" width="2" height="34" fill="#c9c5bc" />),
                fframes::svgr!(<rect x="318" y="465" width="990" height="2" fill="#c9c5bc" />),
                fframes::svgr!(<rect x="317" y="465" width="2" height="27" fill="#c9c5bc" />),
                fframes::svgr!(<rect x="1307" y="465" width="2" height="27" fill="#c9c5bc" />),
            ];
            out.push(fframes::svgr!(<g opacity={boxes_op}>
                <rect x="480" y="262" width="960" height="170" rx="8" fill={CARD} opacity="0.75" />
                {mono("User", MONO_LG, 512.0, 312.0, 1.0, INK)}
                {sans("abstract class", UI, 640.0, 310.0, 1.0, "#8a8479")}
                {mono("id · firstName · lastName · gender · dob · password", MONO_MD, 512.0, 372.0, 1.0, INK)}
                {links}
                <rect x="280" y="492" width="580" height="150" rx="8" fill={CARD} opacity="0.75" />
                {mono("Student", MONO_LG, 312.0, 542.0, 1.0, INK)}
                {mono("+ sectionId", MONO_MD, 312.0, 604.0, ramp(&frame, 7.51), RUST)}
                <rect x="1060" y="492" width="580" height="150" rx="8" fill={CARD} opacity="0.75" />
                {mono("Instructor", MONO_LG, 1092.0, 542.0, 1.0, INK)}
                {mono("+ deptId, boolean isHoD", MONO_MD, 1092.0, 604.0, ramp(&frame, 7.81), RUST)}
                {mono_c("The User abstract class is the parent of Student and Instructor.", MONO_XS, 960.0, 700.0, ramp(&frame, 8.11), "#6d675d")}
            </g>));
        }

        // 10.45-14s: the landing.
        let landing_op = ramp(&frame, 10.56);
        out.push(fframes::svgr!(<g opacity={landing_op} transform={Transform::translate(0.0, rise(&frame, 10.56, 26.0))}>
            {statement("illegal states don't exist.", STATEMENT, 192.0, 540.0, 1.0, INK)}
        </g>));
        fframes::svgr!(<g>{out}</g>)
    }
}
