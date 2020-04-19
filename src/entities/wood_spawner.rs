use super::init_prelude::*;

pub fn init_wood_spawner(world: &mut World, transform: Transform) -> Entity {
    let wood_spawner_settings =
        world.read_resource::<Settings>().wood_spawner.clone();

    world
        .create_entity()
        .with(transform)
        .with(wood_spawner_settings.wood_spawner)
        .build()
}
