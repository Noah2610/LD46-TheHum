use super::init_prelude::*;

pub fn init_wood(world: &mut World, transform: Transform) -> Entity {
    let wood_settings = world.read_resource::<Settings>().wood.clone();

    let sprite_render = {
        let sprite_sheet = world
            .write_resource::<SpriteSheetHandles<PathBuf>>()
            .get_or_load(resource("spritesheets/wood.png"), world);
        SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        }
    };

    world
        .create_entity()
        .with(transform)
        .with(wood_settings.size)
        .with(wood_settings.hitbox)
        .with(sprite_render)
        .with(ScaleOnce::default())
        .with(Collidable::new(CollisionTag::Wood))
        .with(Transparent)
        .with(VisibleInFlame::default())
        .build()
}
