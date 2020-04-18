use super::load_prelude::*;

pub(super) fn load_objects(
    world: &mut World,
    objects: Vec<ObjectData>,
) -> amethyst::Result<()> {
    for object in objects {
        match object.object_type.as_str() {
            "Player" => {
                let transform: Transform = (&object).into();
                let _player = entities::init_player(world, transform.clone());
            }

            "Bonfire" => {
                let transform: Transform = (&object).into();
                let _bonfire = entities::init_bonfire(world, transform.clone());
            }

            unknown => {
                eprintln!("[WARNING]\n    Unknown object type: {}", unknown)
            }
        }
    }

    Ok(())
}
