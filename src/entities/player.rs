use super::init_prelude::*;

pub fn init_player(world: &mut World) -> Entity {
    let player_settings = world.read_resource::<Settings>().player.clone();

    let mut transform = Transform::default();
    transform.set_translation_z(player_settings.z);

    let sprite_render = {
        let sprite_sheet = world
            .write_resource::<SpriteSheetHandles<PathBuf>>()
            .get_or_load(resource("spritesheets/player.png"), world);
        SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        }
    };

    world
        .create_entity()
        .with(Player::default())
        .with(transform)
        .with(player_settings.size)
        .with(player_settings.hitbox)
        .with(player_settings.movable)
        .with(sprite_render)
        .with(Velocity::default())
        .with(ScaleOnce::default())
        .with(Lifecycle::default())
        .build()
}
