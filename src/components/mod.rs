pub mod prelude {
    pub use super::flame::{Flame, VisibleInFlame};
    pub use super::movable::{Movable, MovableData, MoveAction};
    pub use super::player::Player;
    pub use deathframe::components::prelude::*;
}

mod component_prelude {
    pub use super::prelude::*;
    pub use deathframe::components::component_prelude::*;
}

mod flame;
mod movable;
mod player;
