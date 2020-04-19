use super::load_prelude::*;

pub(super) fn load_objects(
    world: &mut World,
    objects: Vec<ObjectData>,
) -> amethyst::Result<()> {
    for object in objects {
        let transform: Transform = (&object).into();

        match &object.object_type {
            ObjectType::Player => {
                let _player = entities::init_player(world, transform.clone());
            }

            ObjectType::Bonfire => {
                let _bonfire = entities::init_bonfire(world, transform.clone());
            }

            ObjectType::Wood => {
                let _wood = entities::init_wood(world, transform.clone());
            }
        }
    }

    Ok(())
}
