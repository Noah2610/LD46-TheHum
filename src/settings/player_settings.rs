// resources/settings/player.ron

use crate::components::prelude::*;

#[derive(Clone, Deserialize)]
pub struct PlayerSettings {
    pub z:       f32,
    pub size:    Size,
    pub hitbox:  Hitbox,
    pub movable: Movable,
}
