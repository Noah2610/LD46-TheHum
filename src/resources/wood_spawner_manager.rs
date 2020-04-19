#[derive(Clone, Deserialize)]
pub struct WoodSpawnerManager {
    pub update_interval_ms: u64,
    pub active_percentage:  f32,
}
