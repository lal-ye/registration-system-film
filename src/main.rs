use fframes::{EncoderOptions, RenderOptions, StaticMediaProvider, cli};
use registration_system_film::{ RegistrationSystemFilmMedia, RegistrationSystemFilmVideo };
use fframes::cli::clap; // the derive below expands to `clap::...`
use std::process::ExitCode;

/// Flags of this video next to the standard ones of `fframes::cli` (render, frame, strip,
/// inspect, audio, ...). Run `cargo run --release -- --help`.
#[derive(Debug, clap::Args)]
struct VideoArgs {
}

fn main() -> ExitCode {
    let args = cli::parse::<VideoArgs>();
    let media = RegistrationSystemFilmMedia::prepare().expect("media");
    let video = RegistrationSystemFilmVideo::new(&media);

    cli::new(
        &video,
        RenderOptions {
            media: Some(&media),
            video_encoder_options: EncoderOptions {
                preferred_encoder: Some("libx264"),
                codec_params: Some(&[("crf", "20"), ("preset", "medium"), ("tune", "animation")]),
                ..Default::default()
            },
            ..Default::default()
        },
    )
        .args(args)
        .run()
}
