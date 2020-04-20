use super::component_prelude::*;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct Flame {
    pub radius:           f32,
    pub min_radius:       f32,
    wood_radius_increase: f32,
}

impl Flame {
    pub fn increase_radius_by_steps(&mut self, steps: isize) {
        self.radius += self.wood_radius_increase * steps as f32;
    }

    pub fn is_at_min_radius(&self) -> bool {
        self.radius <= self.min_radius
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct VisibleInFlame;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct Halo {
    #[serde(skip)]
    pub bonfire_entity: Option<Entity>,
    pub size_margin:    f32,
}

impl Halo {
    pub fn with_entity(mut self, bonfire_entity: Entity) -> Self {
        self.bonfire_entity = Some(bonfire_entity);
        self
    }
}
