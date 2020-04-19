// resources/settings/beartrap.ron

use crate::components::prelude::*;

#[derive(Clone, Deserialize)]
pub struct BeartrapSettings {
    pub size:     Size,
    pub hitbox:   Hitbox,
    pub beartrap: Beartrap,
}
