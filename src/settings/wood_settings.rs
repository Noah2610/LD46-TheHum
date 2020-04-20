// resources/settings/wood.ron

use crate::components::prelude::*;

#[derive(Clone, Deserialize)]
pub struct WoodSettings {
    pub size:      Size,
    pub hitbox:    Hitbox,
    pub indicator: WoodIndicatorSettings,
}

#[derive(Clone, Deserialize)]
pub struct WoodIndicatorSettings {
    pub size:      Size,
    pub animation: Animation,
}
