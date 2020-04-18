use crate::components::prelude::*;
use crate::resource;
use crate::resources::prelude::*;
use crate::settings::prelude::*;
use amethyst::ecs::{Entity, World, WorldExt};
use amethyst::prelude::Builder;
use deathframe::amethyst;
use std::path::PathBuf;

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
        .with(transform)
        .with(player_settings.size)
        .with(player_settings.hitbox)
        .with(sprite_render)
        .with(Velocity::default())
        .with(ScaleOnce::default())
        .with(Lifecycle::default())
        .build()
}

pub fn init_camera(world: &mut World, player: Entity) -> Entity {
    use deathframe::amethyst::renderer::Camera;
    use deathframe::amethyst::utils::ortho_camera::{
        CameraNormalizeMode,
        CameraOrtho,
        CameraOrthoWorldCoordinates,
    };

    let camera_settings = world.read_resource::<Settings>().camera.clone();

    let mut transform = Transform::default();
    transform.set_translation_z(camera_settings.z);

    let size = Size::from(camera_settings.size);

    let camera = Camera::standard_2d(size.w, size.h);
    let mut camera_ortho =
        CameraOrtho::normalized(CameraNormalizeMode::Contain);
    let half_size = size.half();
    camera_ortho.world_coordinates = CameraOrthoWorldCoordinates {
        top:    half_size.h,
        bottom: -half_size.h,
        left:   -half_size.w,
        right:  half_size.w,
    };

    world
        .create_entity()
        .with(transform)
        .with(size)
        .with(camera)
        .with(Follow::new(player))
        .build()
}
