use crate::components::prelude::*;

#[derive(Clone, Deserialize)]
pub struct WoodSettings {
    pub size:   Size,
    pub hitbox: Hitbox,
}
