pub mod prelude {
    pub use super::Settings;
}

#[derive(Clone, Deserialize)]
pub struct Settings {}

impl Settings {
    pub fn load() -> deathframe::amethyst::Result<Self> {
        Ok(Self {})
    }
}
