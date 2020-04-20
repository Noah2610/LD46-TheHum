mod beartrap;
mod bonfire;
mod camera;
mod halo;
mod ladder;
mod player;
mod radio;
mod wood;
mod wood_spawner;

mod init_prelude {
    pub use crate::components::prelude::*;
    pub use crate::resource;
    pub use crate::resources::prelude::*;
    pub use crate::settings::prelude::*;
    pub use amethyst::ecs::{Entity, World, WorldExt};
    pub use amethyst::prelude::Builder;
    pub use deathframe::amethyst;
    pub use std::path::PathBuf;
}

pub use beartrap::init_beartrap;
pub use bonfire::init_bonfire;
pub use camera::init_camera;
pub use halo::init_halo_for;
pub use ladder::init_ladder;
pub use player::init_player;
pub use radio::init_radio;
pub use wood::{init_wood, init_wood_with_storages, InitWoodStorages};
pub use wood_spawner::init_wood_spawner;
