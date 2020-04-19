// resources/settings/wood_spawner.ron

use crate::components::prelude::WoodSpawner;

#[derive(Clone, Deserialize)]
pub struct WoodSpawnerSettings {
    pub wood_spawner: WoodSpawner,
}
