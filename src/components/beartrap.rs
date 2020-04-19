use super::component_prelude::*;
use climer::Timer;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct Beartrap {
    pub crippled_duration_ms:  u64,
    #[serde(skip)]
    pub is_active:             bool,
    #[serde(skip)]
    pub spawn_at_woods_amount: usize,
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

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct BeartrapAffected {
    pub is_crippled: bool,
    pub timer:       Option<Timer>,
}
