use super::component_prelude::*;

#[derive(Clone, Deserialize)]
pub enum MoveAction {
    Walk(f32),
    Jump(f32),
}

#[derive(Component, Clone, Deserialize)]
pub struct Movable {
    acceleration:  f32,
    jump_strength: f32,
    #[serde(skip)]
    actions:       Vec<MoveAction>,
}

impl ActionQueue for Movable {
    type Action = MoveAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}
