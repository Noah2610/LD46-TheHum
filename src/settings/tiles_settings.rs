// resources/settings/tiles.ron

use crate::level_loader::TileType;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
#[serde(from = "HashMap<TileType, TileSettings>")]
pub struct TilesSettings {
    pub tiles: HashMap<TileType, TileSettings>,
}

#[derive(Clone, Deserialize)]
pub struct TileSettings {}

impl From<HashMap<TileType, TileSettings>> for TilesSettings {
    fn from(tiles: HashMap<TileType, TileSettings>) -> Self {
        Self { tiles }
    }
}
