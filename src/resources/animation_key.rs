#[derive(PartialEq, Eq, Hash, Clone, Deserialize, Debug)]
pub enum AnimationKey {
    Idle,
    Walk,
    Jump,
    CrippledIdle,
    CrippledWalk,
    ClimbingIdle,
    Climbing,
    BeartrapHit,
    BonfireBurnt,
}

#[derive(PartialEq, Eq, Hash, Clone, Deserialize, Debug)]
pub enum ReactiveAnimationKey {
    Default,
    OnEnter,
    OnSteady,
    OnLeave,
}

// DOESN'T SEEM RIGHT STUPID
impl Default for AnimationKey {
    fn default() -> Self {
        Self::Idle
    }
}

impl Default for ReactiveAnimationKey {
    fn default() -> Self {
        Self::Default
    }
}
