// resources/settings/player.ron

use crate::components::prelude::*;

#[derive(Clone, Deserialize)]
pub struct PlayerSettings {
    pub size:          Size,
    pub hitbox:        Hitbox,
    pub movable:       Movable,
    pub base_friction: BaseFriction,
    pub gravity:       Gravity,
}
