use super::system_prelude::*;
use crate::entities::{init_wood_with_storages, InitWoodStorages};

#[derive(Default)]
pub struct SpawnWoodSystem;

impl<'a> System<'a> for SpawnWoodSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WoodSpawner>,
        InitWoodStorages<'a>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut wood_spawner_store,
            mut init_wood_storages,
        ): Self::SystemData,
    ) {
    }
}
