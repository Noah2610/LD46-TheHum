use super::load_prelude::*;

pub(super) fn load_objects(
    world: &mut World,
    objects: Vec<ObjectData>,
) -> amethyst::Result<()> {
    let mut player_transform = None;

    for object in objects {
        match object.object_type.as_str() {
            "Player" => {
                let transform: Transform = (&object).into();
                let _player = entities::init_player(world, transform.clone());
                player_transform = Some(transform);
            }
            unknown => {
                eprintln!("[WARNING]\n    Unknown object type: {}", unknown)
            }
        }
    }

    if let Some(camera_transform) = player_transform {
        let _camera = entities::init_camera(world, camera_transform);
    }

    Ok(())
}
