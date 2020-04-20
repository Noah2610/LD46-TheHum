use super::load_prelude::*;
use deathframe::core::geo::prelude::Rect;

pub(super) fn load_objects(
    world: &mut World,
    objects: Vec<ObjectData>,
    level_rect: Rect,
) -> amethyst::Result<()> {
    for object in objects {
        let transform: Transform = (&object).into();

        match &object.object_type {
            ObjectType::Player => {
                let _player =
                    entities::init_player(world, transform, level_rect.clone());
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

            ObjectType::Beartrap(spawn_at_woods_amount) => {
                let _beartrap = entities::init_beartrap(
                    world,
                    transform,
                    *spawn_at_woods_amount,
                );
            }
        }
    }

    Ok(())
}
