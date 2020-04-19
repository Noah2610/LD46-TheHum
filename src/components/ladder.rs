use super::component_prelude::*;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Ladder;

#[derive(Default, Component)]
pub struct LadderClimber {
    pub on_ladder: bool,
}
