use crate::resource;
use crate::settings::Settings;
use crate::states::aliases::GameData;
use crate::states::prelude::Startup;
use amethyst::core::frame_limiter::FrameRateLimitConfig;
use amethyst::utils::app_root_dir::application_root_dir;
use amethyst::ApplicationBuilder;
use deathframe::amethyst;

mod init_game_data;

pub fn run() -> amethyst::Result<()> {
    start_logger();

    let settings = Settings::load()?;
    let game_data = init_game_data::build_game_data(&settings)?;

    let mut game: amethyst::CoreApplication<GameData> =
        ApplicationBuilder::new(application_root_dir()?, Startup::default())?
            .with_frame_limit_config(frame_rate_limit_config()?)
            .with_resource(settings)
            .build(game_data)?;

    game.run();

    Ok(())
}

fn start_logger() {
    use amethyst::{LogLevelFilter, LoggerConfig};
    amethyst::start_logger(LoggerConfig {
        level_filter: LogLevelFilter::Error,
        ..Default::default()
    });
}

fn frame_rate_limit_config() -> amethyst::Result<FrameRateLimitConfig> {
    use std::fs::File;
    Ok(ron::de::from_reader(File::open(resource(
        "config/frame_limiter.ron",
    ))?)?)
}
