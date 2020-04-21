// resources/settings/player.ron

use super::prelude::HaloSettings;
use crate::components::prelude::*;
use crate::resources::prelude::AnimationKey;

#[derive(Clone, Deserialize)]
pub struct PlayerSettings {
    pub size:           Size,
    pub hitbox:         Hitbox,
    pub movable:        Movable,
    pub base_friction:  BaseFriction,
    pub gravity:        Gravity,
    pub flame:          Flame,
    pub halo:           HaloSettings,
    pub wood_inventory: WoodInventory,
    pub animations:     AnimationsContainer<AnimationKey>,
    pub loader:         Loader,
}
