use super::component_prelude::*;
use climer::Timer;
use std::time::Duration;

#[derive(Component, Clone, Deserialize)]
#[storage(DenseVecStorage)]
pub struct WoodSpawner {
    spawn_interval_ms:       u64,
    #[serde(skip)]
    timer:                   Option<Timer>,
    #[serde(skip)]
    pub spawned_wood_entity: Option<Entity>,
    #[serde(skip)]
    is_active:               bool,
}

impl WoodSpawner {
    pub fn should_spawn_wood(&mut self) -> bool {
        if !self.is_active {
            return false;
        }

        let spawn_interval_ms = self.spawn_interval_ms;
        let timer = self.timer.get_or_insert_with(|| {
            Timer::new(
                Some(Duration::from_millis(spawn_interval_ms).into()),
                None,
            )
        });

        if timer.state.is_stopped() || timer.state.is_finished() {
            timer.start().unwrap();
        }
        timer.update().unwrap();

        timer.state.is_finished()
    }
}
