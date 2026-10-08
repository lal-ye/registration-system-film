//! SVG recreations of the Swing views in `View/*.java`, drawn at final size rather than scaled
//! from the 450x320 originals. FlatLightLaf is a light look-and-feel, so these are white panels
//! with grey chrome and one rust accent where the film needs the eye to go.
//!
//! Every text argument is an owned `String`. `Svgr` implements `From<String>` and `From<&str>` but
//! not `From<&String>` or `From<&&str>`, and borrowing a local would tie the returned SVG to a
//! frame that is about to be dropped -- so these take values and copy them into the tree.
use crate::design::*;
use fframes::Svgr;

/// A text field. `value` is whatever has been typed so far.
pub fn field(x: f32, y: f32, w: f32, h: f32, value: String, size: usize, caret_op: f32) -> Svgr<'static> {
    let text_w = measure(&value, size);
    fframes::svgr!(
        <g>
            <rect x={x} y={y} width={w} height={h} rx="4" fill="#ffffff" stroke="#c9c5bc" stroke-width="1" />
            <text x={x + 14.0} y={y + h / 2.0 + size as f32 * 0.36} font-family={SANS}
                  font-weight={SANS_W} font-size={size} fill={INK}>{value}</text>
            <rect x={x + 18.0 + text_w} y={y + h / 2.0 - size as f32 * 0.55}
                  width="2" height={size as f32 * 1.15} fill={INK} opacity={caret_op} />
        </g>
    )
}

/// Rough advance width for Inter at `size`. Only used to place carets, never to lay out type.
fn measure(text: &str, size: usize) -> f32 {
    text.chars().count() as f32 * size as f32 * 0.56
}

/// A Swing button. `primary` is the one the film wants pressed.
pub fn button(x: f32, y: f32, w: f32, h: f32, label: String, primary: bool, pressed: f32) -> Svgr<'static> {
    let dy = 2.0 * pressed;
    let fill = if primary { RUST } else { "#e6e3dc" };
    let ink = if primary { "#ffffff" } else { INK };
    fframes::svgr!(
        <g opacity={1.0 - 0.12 * pressed}>
            <rect x={x} y={y + dy} width={w} height={h} rx="6" fill={fill} />
            <text x={x + w / 2.0} y={y + dy + h / 2.0 + UI as f32 * 0.36} text-anchor="middle"
                  font-family={SANS} font-weight={SANS_W} font-size={UI} fill={ink}>{label}</text>
        </g>
    )
}

/// A combo box.
pub fn combo(x: f32, y: f32, w: f32, h: f32, value: String) -> Svgr<'static> {
    fframes::svgr!(
        <g>
            <rect x={x} y={y} width={w} height={h} rx="4" fill="#ffffff" stroke="#c9c5bc" stroke-width="1" />
            <text x={x + 12.0} y={y + h / 2.0 + UI as f32 * 0.36} font-family={SANS}
                  font-weight={SANS_W} font-size={UI} fill={INK}>{value}</text>
            <path d={format!("M{} {} l7 0 l-3.5 5 z", x + w - 22.0, y + h / 2.0 - 3.0)} fill="#8a8479" />
        </g>
    )
}

/// A tab bar; the selected tab gets a rust underline.
pub fn tabs(
    x: f32,
    y: f32,
    labels: &[&'static str],
    widths: &[f32],
    selected: usize,
) -> Svgr<'static> {
    let mut out: Vec<Svgr> = Vec::new();
    let mut cx = x;
    for (i, (label, w)) in labels.iter().zip(widths).enumerate() {
        let on = i == selected;
        out.push(fframes::svgr!(<text x={cx} y={y + 30.0} font-family={SANS} font-weight={SANS_W}
            font-size={UI} fill={if on { INK } else { "#8a8479" }}>{*label}</text>));
        if on {
            out.push(fframes::svgr!(<rect x={cx} y={y + 42.0} width={w - 24.0} height="3" fill={RUST} />));
        }
        cx += w;
    }
    fframes::svgr!(<g>{out}<rect x={x} y={y + 42.0} width={cx - x} height="1" fill="#ddd9d0" /></g>)
}

/// A table: a grey header row, hairline rules, mono cells. `rows` are pre-formatted.
pub fn table(
    x: f32,
    y: f32,
    w: f32,
    cols: &[f32],
    header: &[&'static str],
    rows: Vec<Vec<String>>,
    row_h: f32,
    opacity: f32,
) -> Svgr<'static> {
    let mut out: Vec<Svgr> = vec![fframes::svgr!(
        <rect x={x} y={y} width={w} height={row_h} fill="#eeebe4" />
    )];
    let mut cx = x + 14.0;
    for (c, head) in cols.iter().zip(header) {
        out.push(fframes::svgr!(<text x={cx} y={y + row_h * 0.66} font-family={SANS} font-weight={SANS_W}
            font-size={UI_SM} fill="#6d675d">{*head}</text>));
        cx += c;
    }
    for (r, row) in rows.into_iter().enumerate() {
        let ry = y + row_h * (r as f32 + 1.0);
        if r % 2 == 1 {
            out.push(fframes::svgr!(<rect x={x} y={ry} width={w} height={row_h} fill="#f7f5f1" />));
        }
        let mut ccx = x + 14.0;
        for (i, cell) in row.into_iter().enumerate() {
            out.push(fframes::svgr!(<text x={ccx} y={ry + row_h * 0.66} font-family={MONO_F}
                font-weight={MONO_W} font-size={MONO_XS} fill={INK}>{cell}</text>));
            ccx += cols.get(i).copied().unwrap_or(120.0);
        }
        out.push(fframes::svgr!(<rect x={x} y={ry + row_h} width={w} height="1" fill="#e6e3dc" />));
    }
    fframes::svgr!(<g opacity={opacity}>{out}</g>)
}

/// `View/LoginFrame.java` at 1.6x: title, ID field, password field, error label, two buttons.
pub fn login_window(typing: (String, String), caret_id: f32, caret_pw: f32, pressed: f32) -> Svgr<'static> {
    let (x, y, w, h) = (600.0_f32, 250.0_f32, 720.0_f32, 512.0_f32);
    let (id_text, pw_text) = typing;
    fframes::svgr!(
        <g>
            {window_card(x, y, w, h, 1.0)}
            {title_bar(x, y, w, "University Registration System — Login")}
            {sans("University Registration System", 26, x + w / 2.0, y + 104.0, 1.0, INK)}
            {sans("Student / Instructor ID:", 21, x + 64.0, y + 180.0, 1.0, INK)}
            {field(x + 380.0, y + 152.0, 276.0, 40.0, id_text, 19, caret_id)}
            {sans("Password:", 21, x + 64.0, y + 255.0, 1.0, INK)}
            {field(x + 380.0, y + 227.0, 276.0, 40.0, pw_text, 19, caret_pw)}
            {sans(" ", 19, x + 64.0, y + 316.0, 1.0, INK)}
            {button(x + 100.0, y + 346.0, 200.0, 58.0, "Login".to_string(), true, pressed)}
            {button(x + 320.0, y + 346.0, 336.0, 58.0, "Register as Student".to_string(), false, 0.0)}
        </g>
    )
}

/// `View/StudentDashboardFrame.java` at ~1.05x, My Enrollments selected.
pub fn student_dashboard(enrolled: Vec<Vec<String>>, drop_op: f32) -> Svgr<'static> {
    let (x, y, w, h) = (461.0_f32, 110.0_f32, 997.0_f32, 651.0_f32);
    fframes::svgr!(
        <g>
            {window_card(x, y, w, h, 1.0)}
            {title_bar(x, y, w, "Student Dashboard")}
            {sans("Welcome, Abebe Kebede", 20, x + 30.0, y + 78.0, 1.0, INK)}
            {button(x + w - 128.0, y + 48.0, 98.0, 40.0, "Logout".to_string(), false, 0.0)}
            {tabs(x + 30.0, y + 108.0,
                  &["My Enrollments", "Available Courses", "My Schedule", "My Grades"],
                  &[250.0, 260.0, 210.0, 200.0], 0)}
            {table(x + 30.0, y + 172.0, w - 60.0, &[180.0, 300.0, 140.0, 140.0],
                   &["ID", "Course", "Section", "Status"], enrolled, 40.0, 1.0)}
            {button(x + 30.0, y + h - 78.0, 250.0, 46.0, "Drop Selected Course".to_string(), false, 0.0)}
            <rect x={x + 30.0} y={y + h - 78.0} width={250.0} height="46" rx="6" fill="none"
                  stroke={RUST} stroke-width="2" opacity={drop_op} />
        </g>
    )
}

/// `View/InstructorDashboardFrame.java` at 1.0x, Grade Entry tab, mid-grade-entry.
pub fn grade_entry(
    course: String,
    student: String,
    scores: [String; 3],
    caret: f32,
    result: String,
    result_op: f32,
    pressed: f32,
) -> Svgr<'static> {
    let (x, y, w, h) = (435.0_f32, 130.0_f32, 1050.0_f32, 640.0_f32);
    let rows = ["Midterm", "Project", "Final"];
    let mut score_fields: Vec<Svgr> = Vec::new();
    for (i, name) in rows.iter().enumerate() {
        let ry = y + 250.0 + i as f32 * 62.0;
        score_fields.push(fframes::svgr!(<text x={x + 220.0} y={ry + 30.0} font-family={SANS}
            font-weight={SANS_W} font-size={UI} fill={INK}>{*name}</text>));
        score_fields.push(fframes::svgr!(<text x={x + 360.0} y={ry + 30.0} text-anchor="end"
            font-family={MONO_F} font-weight={MONO_W} font-size={MONO_XS} fill="#6d675d">{"/ 30"}</text>));
        let value = scores[i].clone();
        score_fields.push(field(x + 400.0, ry + 4.0, 130.0, 40.0, value, 18, if i == 2 { caret } else { 0.0 }));
    }
    fframes::svgr!(
        <g>
            {window_card(x, y, w, h, 1.0)}
            {title_bar(x, y, w, "Instructor Dashboard")}
            {sans("Welcome, Dr. Mohammed Tekola", 20, x + 30.0, y + 78.0, 1.0, INK)}
            {button(x + w - 128.0, y + 48.0, 98.0, 40.0, "Logout".to_string(), false, 0.0)}
            {tabs(x + 30.0, y + 108.0, &["My Schedule", "My Students", "Grade Entry"], &[230.0, 240.0, 230.0], 2)}
            {sans("Course:", 18, x + 40.0, y + 200.0, 1.0, INK)}
            {combo(x + 110.0, y + 174.0, 240.0, 40.0, course)}
            {sans("Student:", 18, x + 390.0, y + 200.0, 1.0, INK)}
            {combo(x + 470.0, y + 174.0, 300.0, 40.0, student)}
            {button(x + 800.0, y + 174.0, 190.0, 40.0, "Load Scores".to_string(), false, 0.0)}
            <rect x={x + 40.0} y={y + 232.0} width={w - 80.0} height="200" rx="6" fill="#f7f5f1" stroke="#e6e3dc" stroke-width="1" />
            {score_fields}
            {result_line(result, 20, x + 40.0, y + 486.0, result_op)}
            {button(x + 40.0, y + h - 84.0, 380.0, 52.0, "Save Scores & Calculate Grade".to_string(), true, pressed)}
        </g>
    )
}

/// `gradeResultLabel`. The real code sets `new Color(0,120,0)`; kept, so the only green in the
/// film is the one line Swing actually paints green.
fn result_line(text: String, size: usize, x: f32, y: f32, opacity: f32) -> Svgr<'static> {
    fframes::svgr!(
        <g opacity={opacity}>
            <text x={x} y={y} font-family={SANS} font-weight={SANS_W} font-size={size}
                  fill="#0d7a34">{text}</text>
        </g>
    )
}
