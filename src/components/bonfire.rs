use super::component_prelude::*;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct Bonfire {
    pub flame_decrease: FlameDecreaseConfig,
}

#[derive(Clone, Deserialize)]
pub struct FlameDecreaseConfig {
    pub step:        f32,
    pub interval_ms: u64,
    pub woods_mult:  f32,
}

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct BonfireHalo {
    #[serde(skip)]
    pub bonfire_entity: Option<Entity>,
    pub size_margin:    f32,
}

impl BonfireHalo {
    pub fn with_bonfire_entity(mut self, bonfire_entity: Entity) -> Self {
        self.bonfire_entity = Some(bonfire_entity);
        self
    }
}
