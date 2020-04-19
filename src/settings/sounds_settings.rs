// resources/settings/sounds.ron

use crate::resources::prelude::SoundKey;
use rand::seq::SliceRandom;

#[derive(Clone, Deserialize)]
pub struct SoundsSettings {
    pub update_interval_ms: u64,
    pub play_chance:        f32,
    pub sound_groups:       Vec<SoundGroup>,
}

impl SoundsSettings {
    pub fn random_sound_key(&self) -> Option<SoundKey> {
        let mut rng = rand::thread_rng();
        self.sound_groups.choose(&mut rng).and_then(|group| {
            group.sounds.choose(&mut rng).map(|sound| sound.key.clone())
        })
    }
}

#[derive(Clone, Deserialize)]
pub struct SoundGroup {
    pub name:   String,
    pub sounds: Vec<SoundSettings>,
}

#[derive(Clone, Deserialize)]
#[serde(from = "SoundKey")]
pub struct SoundSettings {
    pub key: SoundKey,
}

impl From<SoundKey> for SoundSettings {
    fn from(key: SoundKey) -> Self {
        Self { key }
    }
}
