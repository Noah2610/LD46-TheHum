// resources/settings/songs.ron

use crate::resources::prelude::SongKey;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
#[serde(from = "HashMap<SongKey, SongSettings>")]
pub struct SongsSettings {
    pub songs: HashMap<SongKey, SongSettings>,
}

#[derive(Clone, Deserialize)]
pub struct SongSettings {
    pub file: String,
}

impl From<HashMap<SongKey, SongSettings>> for SongsSettings {
    fn from(songs: HashMap<SongKey, SongSettings>) -> Self {
        Self { songs }
    }
}
