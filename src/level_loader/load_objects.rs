use super::load_prelude::*;

pub(super) fn load_objects(
    world: &mut World,
    objects: Vec<ObjectData>,
) -> amethyst::Result<()> {
    for object in objects {
        let transform: Transform = (&object).into();

        match &object.object_type {
            ObjectType::Player => {
                let _player = entities::init_player(world, transform);
            }

            ObjectType::Bonfire => {
                let _bonfire = entities::init_bonfire(world, transform);
            }

            ObjectType::Wood => {
                let _wood = entities::init_wood(world, transform);
            }

            ObjectType::WoodSpawner => {
                let _wood_spawner =
                    entities::init_wood_spawner(world, transform);
            }

            ObjectType::Ladder => {
                let size: Size = (&object.size).into();
                let _ladder = entities::init_ladder(world, transform, size);
            }
        }
    }

    Ok(())
}
