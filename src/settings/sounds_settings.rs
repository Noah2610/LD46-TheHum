// resources/settings/sounds.ron

use crate::resources::prelude::SoundKey;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
#[serde(from = "Vec<SoundGroup>")]
pub struct SoundsSettings {
    pub sound_groups: Vec<SoundGroup>,
}

#[derive(Clone, Deserialize)]
pub struct SoundGroup {
    pub name:   String,
    pub sounds: Vec<SoundSettings>,
}

#[derive(Clone, Deserialize)]
#[serde(from = "String")]
pub struct SoundSettings {
    pub file: String,
}

impl From<Vec<SoundGroup>> for SoundsSettings {
    fn from(sound_groups: Vec<SoundGroup>) -> Self {
        Self { sound_groups }
    }
}

impl From<String> for SoundSettings {
    fn from(file: String) -> Self {
        Self { file }
    }
}
