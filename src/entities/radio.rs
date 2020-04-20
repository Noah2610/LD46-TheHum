use super::init_prelude::*;

pub fn init_radio(world: &mut World, transform: Transform) -> Entity {
    let radio_settings = world.read_resource::<Settings>().radio.clone();

    let sprite_render = {
        let sprite_sheet = world
            .write_resource::<SpriteSheetHandles<PathBuf>>()
            .get_or_load(resource("spritesheets/radio.png"), world);
        SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        }
    };

    let mut radio_builder = world
        .create_entity()
        .with(Radio::default())
        .with(transform)
        .with(radio_settings.size)
        .with(sprite_render)
        .with(ScaleOnce::default())
        .with(Transparent)
        .with(VisibleInFlame::default());

    if let Some(mut animation) = radio_settings.animation {
        animation.play_cycle();
        radio_builder = radio_builder.with(animation);
    }

    radio_builder.build()
}
