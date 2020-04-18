use super::component_prelude::*;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct Flame {
    pub radius: f32,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct VisibleInFlame;
