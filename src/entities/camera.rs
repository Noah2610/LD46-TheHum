use super::init_prelude::*;
use deathframe::core::geo::prelude::{Point, Rect};

pub fn init_camera(world: &mut World, level_size: Size) -> Entity {
    use deathframe::amethyst::renderer::Camera;
    use deathframe::amethyst::utils::ortho_camera::{
        CameraNormalizeMode,
        CameraOrtho,
        CameraOrthoWorldCoordinates,
    };

    let camera_settings = world.read_resource::<Settings>().camera.clone();

    let mut transform = Transform::default();
    transform.set_translation_z(camera_settings.z);

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

    world
        .create_entity()
        .with(Confined::from(
            Rect::from(&size)
                .with_offset(&Point::new(half_size.w, half_size.h)),
        ))
        .with(transform)
        .with(size)
        .with(camera)
        .build()
}
