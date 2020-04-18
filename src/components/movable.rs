use super::component_prelude::*;

#[derive(Clone, Deserialize)]
pub enum MoveAction {
    Walk(f32),
    Jump,
    KillJump,
}

#[derive(Component, Clone, Deserialize)]
#[serde(from = "MovableData")]
pub struct Movable {
    pub data: MovableData,
    actions:  Vec<MoveAction>,
}

impl From<MovableData> for Movable {
    fn from(data: MovableData) -> Self {
        Self {
            data,
            actions: Default::default(),
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct MovableData {
    pub acceleration:           f32,
    pub max_velocity:           f32,
    pub jump_strength:          f32,
    pub kill_jump_strength:     f32,
    pub kill_jump_min_velocity: f32,
}

impl ActionQueue for Movable {
    type Action = MoveAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}
