// resources/settings/beartrap.ron

use crate::components::prelude::*;
use crate::resources::prelude::AnimationKey;

#[derive(Clone, Deserialize)]
pub struct BeartrapSettings {
    pub size:       Size,
    pub hitbox:     Hitbox,
    pub beartrap:   Beartrap,
    pub animations: AnimationsContainer<AnimationKey>,
}
