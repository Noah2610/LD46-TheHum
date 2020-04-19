pub mod prelude {
    pub use super::flame::{Flame, VisibleInFlame};
    pub use super::movable::{Movable, MovableData, MoveAction};
    pub use super::player::Player;
    pub use super::trigger_reactive_animation::TriggerReactiveAnimation;
    pub use super::wood_inventory::{WoodInventory, WoodInventoryAction};
    pub use super::wood_spawner::WoodSpawner;
    pub use deathframe::components::prelude::*;
}

mod component_prelude {
    pub use super::prelude::*;
    pub use deathframe::components::component_prelude::*;
}

mod flame;
mod movable;
mod player;
mod trigger_reactive_animation;
mod wood_inventory;
mod wood_spawner;
