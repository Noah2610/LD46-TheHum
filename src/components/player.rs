use super::component_prelude::*;

#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct Player {
    pub on_ground: bool,
}
