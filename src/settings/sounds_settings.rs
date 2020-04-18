// resources/settings/sounds.ron

use crate::resources::prelude::SoundKey;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
#[serde(from = "HashMap<SoundKey, SoundSettings>")]
pub struct SoundsSettings {
    pub sounds: HashMap<SoundKey, SoundSettings>,
}

#[derive(Clone, Deserialize)]
pub struct SoundSettings {
    pub file: String,
}

impl From<HashMap<SoundKey, SoundSettings>> for SoundsSettings {
    fn from(sounds: HashMap<SoundKey, SoundSettings>) -> Self {
        Self { sounds }
    }
}
