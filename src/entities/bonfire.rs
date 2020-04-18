use super::init_prelude::*;
use std::path::PathBuf;

pub fn init_bonfire(world: &mut World, transform: Transform) -> Entity {
    let bonfire_settings = world.read_resource::<Settings>().bonfire.clone();

    let (sprite_render, halo_sprite_render) = {
        let mut sprite_sheet_handles =
            world.write_resource::<SpriteSheetHandles<PathBuf>>();
        let sprite_sheet = sprite_sheet_handles
            .get_or_load(resource("spritesheets/bonfire.png"), world);
        let halo_sprite_sheet = sprite_sheet_handles
            .get_or_load(resource("spritesheets/bonfire_halo.png"), world);
        (
            SpriteRender {
                sprite_sheet,
                sprite_number: 0,
            },
            SpriteRender {
                sprite_sheet:  halo_sprite_sheet,
                sprite_number: 0,
            },
        )
    };

    let mut animations = bonfire_settings.animations;
    animations
        .play(AnimationKey::Idle)
        .expect("Bonfire should have `Idle` animation");

    let halo_transform = {
        let mut halo_transform = transform.clone();
        halo_transform.translation_mut().z += 0.01;
        halo_transform
    };
    let halo_size = Size::from({
        let s = bonfire_settings.flame.radius * 2.0;
        (s, s)
    });

    let _halo = world
        .create_entity()
        .with(halo_transform)
        .with(halo_size)
        .with(halo_sprite_render)
        .with(ScaleOnce::default())
        .with(Transparent)
        .build();

    world
        .create_entity()
        .with(transform)
        .with(bonfire_settings.size)
        .with(bonfire_settings.hitbox)
        .with(bonfire_settings.flame)
        .with(animations)
        .with(sprite_render)
        .with(Collidable::new(CollisionTag::Bonfire))
        .with(Transparent)
        .with(ScaleOnce::default())
        .build()
}
