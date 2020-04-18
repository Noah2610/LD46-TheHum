use super::load_prelude::*;

pub(super) fn load_tiles(
    world: &mut World,
    tiles: Vec<TileData>,
    tile_size: Size,
) -> amethyst::Result<()> {
    for tile in tiles {
        dbg!(&tile.tile_type);

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
