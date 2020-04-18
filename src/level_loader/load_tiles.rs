use super::load_prelude::*;
use deathframe::core::geo::prelude::Rect;

pub(super) fn load_tiles(
    world: &mut World,
    tiles: Vec<TileData>,
    tile_size: Size,
) -> amethyst::Result<()> {
    let tiles_settings = world.read_resource::<Settings>().tiles.clone();

    for tile in tiles {
        let transform: Transform = (&tile).into();

        let sprite_render = {
            let sprite_sheet = world
                .write_resource::<SpriteSheetHandles<PathBuf>>()
                .get_or_load(
                    resource(format!("spritesheets/tilesets/{}", &tile.ts)),
                    world,
                );
            SpriteRender {
                sprite_sheet,
                sprite_number: tile.id,
            }
        };

        let mut entity_builder = world
            .create_entity()
            .with(transform)
            .with(tile_size.clone())
            .with(ScaleOnce::default())
            .with(sprite_render)
            .with(Transparent)
            .with(VisibleInFlame::default());

        if let Some(tile_settings) = tiles_settings.get(&tile.tile_type) {
            if tile_settings.is_solid.unwrap_or(false) {
                entity_builder = entity_builder
                    .with(Collidable::new(CollisionTag::Solid))
                    .with(Solid::new(SolidTag::Solid))
                    .with(Hitbox::from(vec![Rect::from(&tile_size)]));
            }
        }

        entity_builder.build();
    }

    Ok(())
}
