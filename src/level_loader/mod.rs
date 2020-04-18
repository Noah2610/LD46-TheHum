pub type ObjectType = String;
pub type TileType = String;

mod level_data;

use crate::components::prelude::*;
use crate::entities;
use crate::resource;
use crate::resources::prelude::*;
use amethyst::ecs::{World, WorldExt};
use amethyst::prelude::Builder;
use deathframe::amethyst;
use level_data::*;
use std::fs::File;
use std::path::PathBuf;

pub fn load_level(
    level_path: PathBuf,
    world: &mut World,
) -> amethyst::Result<()> {
    let level_file = File::open(level_path)?;
    let level = serde_json::de::from_reader::<_, Level>(level_file)?;

    let tile_size: Size = (&level.level.tile_size).into();

    load_tiles(world, level.tiles, tile_size)?;
    load_objects(world, level.objects)?;

    Ok(())
}

fn load_tiles(
    world: &mut World,
    tiles: Vec<TileData>,
    tile_size: Size,
) -> amethyst::Result<()> {
    for tile in tiles {
        let transform: Transform = (&tile).into();

        let sprite_render = {
            let sprite_sheet = world
                .write_resource::<SpriteSheetHandles<PathBuf>>()
                .get_or_load(
                    resource(format!("spritesheets/tiles/{}", &tile.ts)),
                    world,
                );
            SpriteRender {
                sprite_sheet,
                sprite_number: tile.id,
            }
        };

        world
            .create_entity()
            .with(transform)
            .with(tile_size.clone())
            .with(ScaleOnce::default())
            .with(sprite_render)
            .with(Transparent)
            .build();
    }

    Ok(())
}

fn load_objects(
    world: &mut World,
    objects: Vec<ObjectData>,
) -> amethyst::Result<()> {
    unimplemented!()
}
