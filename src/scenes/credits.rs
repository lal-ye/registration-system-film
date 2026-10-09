//! 91-96s. The title card at 6s, returned, with the names added. Nothing animates in one at a
//! time: the film decelerates into stillness.
use crate::design::*;
use crate::scenes::title::{AFFIL_1_Y, AFFIL_2_Y, NAME_Y};
use fframes::{Duration, FFramesContext, Frame, Scene, Svgr};

const TEAM: [(&str, &str); 5] = [
    ("Abenezer Kassahun", "RCD/1814/2017"),
    ("Abreham Tesfaw", "RCD/0398/2017"),
    ("Natal Tamrat", "RCD/0442/2017"),
    ("Mohammed Tekola", "RCD/0124/2017"),
    ("Robera Adugna", "RCD/0445/2017"),
];

#[derive(Debug)]
pub struct Credits;

impl Scene for Credits {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(6.0)
    }

    fn render_frame<'a>(&'a self, frame: Frame, _ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        // The film ends by going quiet rather than by cutting: everything holds, then fades.
        let fade = 1.0 - seg(&frame, 5.45, 0.55);
        let mut names: Vec<Svgr> = Vec::new();
        for (i, (name, id)) in TEAM.iter().enumerate() {
            let op = ramp(&frame, 0.625 + i as f32 * 0.22);
            let y = 646.0 + i as f32 * 40.0;
            names.push(fframes::svgr!(<g opacity={op}>
                <text x="800" y={y} text-anchor="end" font-family={SANS} font-weight={SANS_W}
                      font-size="21" fill={INK}>{*name}</text>
                <text x="828" y={y} font-family={MONO_F} font-weight={MONO_W} font-size="17"
                      fill="#8a8479">{*id}</text>
            </g>));
        }
        fframes::svgr!(<g opacity={fade}>
            {statement_c("University Registration System", TITLE, 960.0, NAME_Y, ramp(&frame, 0.225), INK)}
            {affil("ST. MARY'S UNIVERSITY  ·  DEPARTMENT OF COMPUTER SCIENCE", 20, 960.0, AFFIL_1_Y, ramp(&frame, 0.425), "#6d675d")}
            {affil("OBJECT-ORIENTED PROGRAMMING  ·  GROUP PROJECT", 20, 960.0, AFFIL_2_Y, ramp(&frame, 0.425), "#6d675d")}
            {names}
            <rect x="712" y="606" width="496" height="1" fill="#ddd9d0" opacity={ramp(&frame, 0.625)} />
            {mono_c("Music: \"Deliberate Thought\" by Kevin MacLeod  ·  CC BY 4.0", MONO_XS, 960.0, 906.0, ramp(&frame, 2.325), "#8a8479")}
            {mono_c("lal-ye / registration-system-film", MONO_XS, 960.0, 942.0, ramp(&frame, 2.625), "#8a8479")}
        </g>)
    }
}
