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
        .with(Confined::from(Rect::from(&level_size).with_offset(&{
            let half_size = level_size.half();
            Point::new(half_size.w, half_size.h)
        })))
        .build()
}
