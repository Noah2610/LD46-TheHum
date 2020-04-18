pub mod prelude {
    pub use super::control_player::ControlPlayerSystem;
    pub use super::handle_flame_visibility::HandleFlameVisibilitySystem;
    pub use super::handle_movables::HandleMovablesSystem;
    pub use super::update_player_animation::UpdatePlayerAnimationSystem;
    pub use super::update_reactive_animations::UpdateReactiveAnimationsSystem;
    pub use deathframe::systems::prelude::*;
}

mod system_prelude {
    pub use crate::components::prelude::*;
    pub use crate::input::prelude::*;
    pub use crate::resources::prelude::*;
    pub use crate::settings::prelude::*;
    pub use deathframe::core::geo::prelude::*;
    pub use deathframe::systems::system_prelude::*;
}

mod control_player;
mod handle_flame_visibility;
mod handle_movables;
mod update_player_animation;
mod update_reactive_animations;
