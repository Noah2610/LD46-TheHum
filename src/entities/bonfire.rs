use super::init_prelude::*;
use std::path::PathBuf;

pub fn init_bonfire(world: &mut World, transform: Transform) -> Entity {
    let bonfire_settings = world.read_resource::<Settings>().bonfire.clone();

    let sprite_render = {
        let sprite_sheet = world
            .write_resource::<SpriteSheetHandles<PathBuf>>()
            .get_or_load(resource("spritesheets/bonfire.png"), world);
        SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        }
    };

    let mut animations = bonfire_settings.animations;
    animations
        .play(AnimationKey::Idle)
        .expect("Bonfire should have `Idle` animation");

    world
        .create_entity()
        .with(transform)
        .with(bonfire_settings.size)
        .with(bonfire_settings.hitbox)
        .with(bonfire_settings.flame)
        .with(animations)
        .with(sprite_render)
        .with(Solid::new(SolidTag::Solid))
        .with(Transparent)
        .with(ScaleOnce::default())
        .build()
}
