pub mod prelude {
    pub use super::movable::{Movable, MoveAction};
    pub use super::player::Player;
    pub use deathframe::components::prelude::*;
}

mod component_prelude {
    pub use super::prelude::*;
    pub use deathframe::components::component_prelude::*;
}

mod movable;
mod player;
