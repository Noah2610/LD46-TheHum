// resources/settings/bonfire.ron

use crate::components::prelude::*;
use crate::resources::prelude::AnimationKey;

#[derive(Clone, Deserialize)]
pub struct BonfireSettings {
    pub size:           Size,
    pub hitbox:         Hitbox,
    pub bonfire:        Bonfire,
    pub flame:          Flame,
    pub wood_inventory: WoodInventory,
    pub animations:     AnimationsContainer<AnimationKey>,
    pub halo:           HaloSettings,
}

#[derive(Clone, Deserialize)]
pub struct HaloSettings {
    pub halo:      Halo,
    pub animation: Option<Animation>,
}
