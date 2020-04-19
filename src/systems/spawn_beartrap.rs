use super::system_prelude::*;

#[derive(Default)]
pub struct SpawnBeartrapSystem;

impl<'a> System<'a> for SpawnBeartrapSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Beartrap>,
        ReadStorage<'a, Bonfire>,
        ReadStorage<'a, WoodInventory>,
        WriteStorage<'a, VisibleInFlame>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut beartrap_store,
            bonfire_store,
            wood_inventory_store,
            mut visible_in_flame_store,
        ): Self::SystemData,
    ) {
        if let Some(bonfire_woods) = (&bonfire_store, &wood_inventory_store)
            .join()
            .next()
            .map(|(_, inventory)| inventory.woods)
        {
            for (entity, beartrap) in (&entities, &mut beartrap_store).join() {
                if !beartrap.is_active {
                    if bonfire_woods >= beartrap.spawn_at_woods_amount {
                        beartrap.is_active = true;
                        visible_in_flame_store
                            .insert(entity, Default::default())
                            .unwrap();
                    }
                }
            }
        }
    }
}
