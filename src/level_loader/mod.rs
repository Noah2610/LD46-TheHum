mod level_data;
mod load_objects;
mod load_tiles;

pub type TileType = String;

#[derive(Clone, Deserialize)]
pub enum ObjectType {
    Player,
    Bonfire,
    Wood,
    WoodSpawner,
    Ladder,
    Beartrap(usize),
    Radio,
}

mod load_prelude {
    pub(super) use super::level_data::*;
    pub(super) use super::ObjectType;
    pub(super) use super::TileType;
    pub(super) use crate::components::prelude::*;
    pub(super) use crate::entities;
    pub(super) use crate::resource;
    pub(super) use crate::resources::prelude::*;
    pub(super) use crate::settings::prelude::*;
    pub(super) use amethyst::ecs::{World, WorldExt};
    pub(super) use amethyst::prelude::Builder;
    pub(super) use deathframe::amethyst;
    pub(super) use std::path::PathBuf;
}

use crate::components::prelude::*;
use crate::entities;
use amethyst::ecs::World;
use deathframe::amethyst;
use deathframe::core::geo::prelude::{Point, Rect};
use level_data::*;
use std::fs::File;
use std::path::PathBuf;

pub fn load_level(
    level_path: PathBuf,
    world: &mut World,
) -> amethyst::Result<()> {
    let level_file = File::open(level_path)?;
    let level = serde_json::de::from_reader::<_, Level>(level_file)?;

    let level_rect = {
        let size: Size = (&level.level.size).into();
        let center = {
            let half = size.half();
            Point::new(half.w, half.h)
        };
        Rect::from(&size).with_offset(&center)
    };
    let tile_size: Size = (&level.level.tile_size).into();

    let _camera = entities::init_camera(world, (&level.level.size).into());
    load_tiles::load_tiles(world, level.tiles, tile_size)?;
    load_objects::load_objects(world, level.objects, level_rect)?;

    Ok(())
}
