use super::component_prelude::*;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct Beartrap {
    #[serde(skip)]
    spawn_at_woods_amount: usize,
}

impl Beartrap {
    pub fn with_spawn_at_woods_amount(
        mut self,
        spawn_at_woods_amount: usize,
    ) -> Self {
        self.spawn_at_woods_amount = spawn_at_woods_amount;
        self
    }
}
