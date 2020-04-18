#[derive(PartialEq, Eq, Hash, Clone, Deserialize, Debug)]
pub enum AnimationKey {
    Idle,
    Walk,
    Jump,
}

// DOESN'T SEEM RIGHT STUPID
impl Default for AnimationKey {
    fn default() -> Self {
        Self::Idle
    }
}
