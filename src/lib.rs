//! "University Registration System" — a 96-second film about an object-oriented Java group
//! project, built with fframes. One scene per beat; the music bed is declared once on the video,
//! so it runs continuously across all of them.
pub mod design;
pub mod grain;
pub mod scenes;
pub mod ui;

use fframes::{
    AudioMap, AudioTimestamp::*, AudioTrack, Color, Duration, FFramesContext, Frame, Scene, Scenes,
    Svgr, Video, include_media_dir,
};
use scenes::{architecture, cold_open, credits, login, movement_one, movement_two, payoff, rewind,
             roles, thesis, title};

// Every file of `media/` is embedded in the binary. Fonts are addressed by family name.
include_media_dir!(pub struct RegistrationSystemFilmMedia, "media");

use design::*;

pub struct RegistrationSystemFilmVideo<'a> {
    pub media: &'a RegistrationSystemFilmMedia,
    beats: [Box<dyn Scene + 'a>; 11],
}

impl<'a> RegistrationSystemFilmVideo<'a> {
    pub fn new(media: &'a RegistrationSystemFilmMedia) -> Self {
        Self {
            media,
            beats: [
                Box::new(cold_open::ColdOpen),
                Box::new(rewind::Rewind),
                Box::new(title::Title),
                Box::new(thesis::Thesis),
                Box::new(roles::Roles),
                Box::new(login::Login),
                Box::new(architecture::Architecture),
                Box::new(movement_one::MovementOne),
                Box::new(movement_two::MovementTwo),
                Box::new(payoff::Payoff),
                Box::new(credits::Credits),
            ],
        }
    }
}

impl std::fmt::Debug for RegistrationSystemFilmVideo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegistrationSystemFilmVideo").finish()
    }
}

impl Video for RegistrationSystemFilmVideo<'_> {
    const FPS: usize = 30;
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;
    const BACKGROUND_COLOR: Color = Color::WHITE;

    /// The sum of the beat durations: 4 + 2 + 2 + 2 + 12 + 11 + 15 + 14 + 14 + 14 + 6 = 96 s.
    fn duration(&self) -> Duration<'_> {
        Duration::Auto
    }

    // bed.mp3 is already loudness-normalised to -16 LUFS with a 2 s fade in, so no gain is
    // applied here: the template's gain_db(-16) would attenuate it a second time.
    fn audio(&self) -> AudioMap<'_> {
        AudioMap::from([
            AudioTrack::new("bed.mp3", Second(0.0)..Eof).fade_out(4.0),
        ])
    }

    fn define_scenes(&self) -> Scenes<'_> {
        Scenes::from(self.beats.iter().map(|b| b.as_ref() as &dyn Scene).collect::<Vec<_>>())
    }

    fn render_frame<'a>(&'a self, frame: Frame, ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        fframes::svgr!(
            <svg xmlns="http://www.w3.org/2000/svg" width={WIDTH} height={HEIGHT} viewBox="0 0 1920 1080">
                <rect width="1920" height="1080" fill={PAPER} />
                {paper()}
                {ctx.render_scenes(&frame)}
            </svg>
        )
    }
}
