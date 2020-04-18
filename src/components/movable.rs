use super::component_prelude::*;

#[derive(Clone, Deserialize)]
pub enum MoveAction {
    Walk(f32),
    Jump,
}

#[derive(Component, Clone, Deserialize)]
pub struct Movable {
    pub data: MovableData,
    #[serde(skip)]
    actions:  Vec<MoveAction>,
}

#[derive(Clone, Deserialize)]
pub struct MovableData {
    pub acceleration:  f32,
    pub max_velocity:  f32,
    pub jump_strength: f32,
}

impl ActionQueue for Movable {
    type Action = MoveAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}
