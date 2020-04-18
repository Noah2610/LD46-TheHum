pub mod prelude {
    pub use super::bonfire_settings::BonfireSettings;
    pub use super::camera_settings::CameraSettings;
    pub use super::general_settings::GeneralSettings;
    pub use super::player_settings::PlayerSettings;
    pub use super::songs_settings::SongsSettings;
    pub use super::sounds_settings::SoundsSettings;
    pub use super::tiles_settings::{TileSettings, TilesSettings};
    pub use super::Settings;
}

mod bonfire_settings;
mod camera_settings;
mod general_settings;
mod player_settings;
mod songs_settings;
mod sounds_settings;
mod tiles_settings;

use crate::resource;
use deathframe::amethyst;
use prelude::*;
use std::fmt;
use std::fs::File;

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub general: GeneralSettings,
    pub player:  PlayerSettings,
    pub camera:  CameraSettings,
    pub tiles:   TilesSettings,
    pub songs:   SongsSettings,
    pub sounds:  SoundsSettings,
    pub bonfire: BonfireSettings,
}

impl Settings {
    pub fn load() -> amethyst::Result<Self> {
        Ok(Self {
            general: load_settings("general.ron")?,
            player:  load_settings("player.ron")?,
            camera:  load_settings("camera.ron")?,
            tiles:   load_settings("tiles.ron")?,
            songs:   load_settings("songs.ron")?,
            sounds:  load_settings("sounds.ron")?,
            bonfire: load_settings("bonfire.ron")?,
        })
    }
}

fn load_settings<S, P>(path: P) -> amethyst::Result<S>
where
    for<'de> S: serde::Deserialize<'de>,
    P: fmt::Display,
{
    let file = File::open(resource(format!("settings/{}", &path)))?;
    Ok(ron::de::from_reader(file).map_err(|e| {
        amethyst::Error::from_string(format!(
            "Failed parsing ron settings file: {}\n{:#?}",
            path, e
        ))
    })?)
}
