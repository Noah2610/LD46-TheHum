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
            .with(VisibleInFlame::default())
            .with(Loadable::default())
            .with(Loaded::default());

        if let Some(tile_settings) = tiles_settings.get(&tile.tile_type) {
            if tile_settings.is_solid.unwrap_or(false) {
                entity_builder = entity_builder
                    .with(Collidable::new(CollisionTag::Solid))
                    .with(Solid::new(SolidTag::Solid))
                    .with(Hitbox::from(vec![Rect::from(&tile_size)]));
            }

            if let Some(reactive_animations) =
                &tile_settings.reactive_animations
            {
                let mut animations = reactive_animations.clone();
                if let Err(_) = animations.play(ReactiveAnimationKey::Default) {
                    eprintln!(
                        "[WARNING]\n    A tile with `reactive_animations` \
                         should define the `Default` animation.\n    Without, \
                         once the player has triggered any animation,\n    it \
                         will stay at the final frame of the last played \
                         animation,\n    which is probably not what you want."
                    );
                }
                entity_builder = entity_builder
                    .with(animations)
                    .with(Collidable::new(CollisionTag::ReactiveTile))
                    .with(Hitbox::from(vec![Rect::from(&tile_size)]));
            }
        }

        entity_builder.build();
    }

    Ok(())
}
