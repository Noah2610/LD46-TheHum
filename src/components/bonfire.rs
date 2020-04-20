use super::component_prelude::*;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct Bonfire {
    pub flame_decrease: FlameDecreaseConfig,
}

#[derive(Clone, Deserialize)]
pub struct FlameDecreaseConfig {
    pub step:          f32,
    pub interval_ms:   u64,
    pub wood_decrease: f32,
}
