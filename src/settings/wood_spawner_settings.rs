// resources/settings/wood_spawner.ron

use crate::components::prelude::WoodSpawner;
use crate::resources::prelude::WoodSpawnerManager;

#[derive(Clone, Deserialize)]
pub struct WoodSpawnerSettings {
    pub wood_spawner:         WoodSpawner,
    pub wood_spawner_manager: WoodSpawnerManager,
}
