use super::init_prelude::*;

pub fn init_halo_for(
    world: &mut World,
    parent: Entity,
    halo_settings: HaloSettings,
    parent_transform: Transform,
) -> Entity {
    let sprite_render = {
        let sprite_sheet = world
            .write_resource::<SpriteSheetHandles<PathBuf>>()
            .get_or_load(resource("spritesheets/bonfire_halo.png"), world);
        SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        }
    };

    let transform = {
        let mut transform = parent_transform.clone();
        transform.translation_mut().z += 0.01;
        transform
    };

    let mut halo_builder = world
        .create_entity()
        .with(halo_settings.halo.with_entity(parent))
        .with(transform)
        .with(Size::new(0.0, 0.0))
        .with(sprite_render)
        .with(ScaleOnce::default())
        .with(Transparent)
        .with(Follow::new(parent));

    if let Some(mut animation) = halo_settings.animation {
        animation.play_cycle();
        halo_builder = halo_builder.with(animation);
    }

    halo_builder.build()
}
