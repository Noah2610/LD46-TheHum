// resources/settings/songs.ron

use crate::resources::prelude::SongKey;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
pub struct SongsSettings {
    pub songs:           HashMap<SongKey, SongSettings>,
    pub songs_proximity: HashMap<SongKey, SongProximitySettings>,
}

#[derive(Clone, Deserialize)]
pub struct SongSettings {
    pub file:        String,
    pub should_loop: bool,
}

#[derive(Clone, Deserialize)]
pub struct SongProximitySettings {
    pub factor: f32,
}
