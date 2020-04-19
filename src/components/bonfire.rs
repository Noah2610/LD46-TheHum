use super::component_prelude::*;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Bonfire;

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
