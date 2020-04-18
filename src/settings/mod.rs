pub mod prelude {
    pub use super::camera_settings::CameraSettings;
    pub use super::general_settings::GeneralSettings;
    pub use super::player_settings::PlayerSettings;
    pub use super::Settings;
}

mod camera_settings;
mod general_settings;
mod player_settings;

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
}

impl Settings {
    pub fn load() -> amethyst::Result<Self> {
        Ok(Self {
            general: load_settings("general.ron")?,
            player:  load_settings("player.ron")?,
            camera:  load_settings("camera.ron")?,
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
