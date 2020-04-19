use climer::Timer;
use std::time::Duration;

#[derive(Clone, Deserialize)]
pub struct WoodSpawnerManager {
    update_interval_ms:    u64,
    pub active_percentage: f32,
    #[serde(skip)]
    timer:                 Option<Timer>,
}

impl WoodSpawnerManager {
    pub fn should_update_wood_spawners(&mut self) -> bool {
        let update_interval_ms = self.update_interval_ms;
        let timer = self.timer.get_or_insert_with(|| {
            Timer::new(
                Some(Duration::from_millis(update_interval_ms).into()),
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
