use super::component_prelude::*;
use climer::Timer;
use deathframe::amethyst::ecs::world::Index;

#[derive(Component, Clone, Deserialize)]
#[storage(DenseVecStorage)]
pub struct WoodSpawner {
    pub spawn_interval_ms: u64,
    #[serde(skip)]
    pub timer:             Option<Timer>,
    #[serde(skip)]
    pub spawned_wood_id:   Option<Index>,
}
