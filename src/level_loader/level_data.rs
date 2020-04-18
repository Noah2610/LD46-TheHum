use super::ObjectType;
use super::TileType;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
pub struct PosData {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Deserialize)]
pub struct SizeData {
    pub w: f32,
    pub h: f32,
}

#[derive(Clone, Deserialize)]
pub struct Props {
    pub z:     Option<f32>,
    #[serde(flatten)]
    pub other: HashMap<String, JsonValue>,
}

#[derive(Clone, Deserialize)]
pub struct Level {
    pub level:   LevelData,
    pub tiles:   Vec<TileData>,
    pub objects: Vec<ObjectData>,
}

#[derive(Clone, Deserialize)]
pub struct LevelData {
    pub size:      SizeData,
    pub tile_size: SizeData,
}

#[derive(Clone, Deserialize)]
pub struct TileData {
    pub id:        i64,
    #[serde(rename = "type")]
    pub tile_type: TileType,
    pub ts:        String,
    pub pos:       PosData,
    pub props:     Props,
}

#[derive(Clone, Deserialize)]
pub struct ObjectData {
    #[serde(rename = "type")]
    pub object_type: ObjectType,
    pub pos:         PosData,
    pub size:        SizeData,
    pub props:       Props,
}
