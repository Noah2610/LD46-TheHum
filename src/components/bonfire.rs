use super::component_prelude::*;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Bonfire;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct BonfireHalo {
    pub size_margin: f32,
}
