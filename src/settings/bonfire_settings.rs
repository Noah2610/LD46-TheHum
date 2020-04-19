// resources/settings/bonfire.ron

use crate::components::prelude::*;
use crate::resources::prelude::AnimationKey;

#[derive(Clone, Deserialize)]
pub struct BonfireSettings {
    pub size:           Size,
    pub hitbox:         Hitbox,
    pub flame:          Flame,
    pub flame_decrease: FlameDecreaseSettings,
    pub animations:     AnimationsContainer<AnimationKey>,
    pub halo:           HaloSettings,
}

#[derive(Clone, Deserialize)]
pub struct FlameDecreaseSettings {
    pub step:        f32,
    pub interval_ms: f32,
}

#[derive(Clone, Deserialize)]
pub struct HaloSettings {
    pub bonfire_halo: BonfireHalo,
    pub animation:    Option<Animation>,
}
