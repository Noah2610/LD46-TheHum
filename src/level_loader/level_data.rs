use super::ObjectType;
use super::TileType;
use crate::components::prelude::{Size, Transform};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
pub struct PosData {
    pub x: f32,
    pub y: f32,
}

impl Into<Transform> for &PosData {
    fn into(self) -> Transform {
        let mut transform = Transform::default();
        transform.set_translation_x(self.x);
        transform.set_translation_y(self.y);
        transform
    }
}

#[derive(Clone, Deserialize)]
pub struct SizeData {
    pub w: f32,
    pub h: f32,
}

impl Into<Size> for &SizeData {
    fn into(self) -> Size {
        Size::new(self.w, self.h)
    }
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
    pub id:        usize,
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

impl Into<Transform> for &TileData {
    fn into(self) -> Transform {
        let mut transform: Transform = (&self.pos).into();
        if let Some(z) = self.props.z {
            transform.set_translation_z(z);
        }
        transform
    }
}

impl Into<Transform> for &ObjectData {
    fn into(self) -> Transform {
        let mut transform: Transform = (&self.pos).into();
        if let Some(z) = self.props.z {
            transform.set_translation_z(z);
        }
        transform
    }
}
