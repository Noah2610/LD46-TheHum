// resources/settings/tiles.ron

use crate::level_loader::TileType;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
#[serde(from = "HashMap<TileType, TileSettings>")]
pub struct TilesSettings {
    pub tiles: HashMap<TileType, TileSettings>,
}

impl TilesSettings {
    pub fn get(&self, key: &TileType) -> Option<&TileSettings> {
        self.tiles.get(key)
    }
}

#[derive(Clone, Deserialize)]
pub struct TileSettings {
    pub is_solid: Option<bool>,
}

impl From<HashMap<TileType, TileSettings>> for TilesSettings {
    fn from(tiles: HashMap<TileType, TileSettings>) -> Self {
        Self { tiles }
    }
}
