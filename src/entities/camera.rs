use super::init_prelude::*;

pub fn init_camera(world: &mut World, level_size: Size) -> Entity {
    use deathframe::amethyst::renderer::Camera;
    use deathframe::amethyst::utils::ortho_camera::{
        CameraNormalizeMode,
        CameraOrtho,
        CameraOrthoWorldCoordinates,
    };

    let camera_settings = world.read_resource::<Settings>().camera.clone();

    let size = level_size;

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

    let mut transform = Transform::default();
    transform.set_translation_xyz(half_size.w, half_size.h, camera_settings.z);

    world
        .create_entity()
        .with(transform)
        .with(size)
        .with(camera)
        .build()
}
