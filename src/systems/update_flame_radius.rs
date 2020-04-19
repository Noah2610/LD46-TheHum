use super::system_prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct UpdateFlameRadiusSystem {
    cache: HashMap<Entity, usize>,
}

impl<'a> System<'a> for UpdateFlameRadiusSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Flame>,
        ReadStorage<'a, WoodInventory>,
    );

    fn run(
        &mut self,
        (entities, mut flame_store, wood_inventory_store): Self::SystemData,
    ) {
        for (entity, flame, inventory) in
            (&entities, &mut flame_store, &wood_inventory_store).join()
        {
            let cached = self.cache.entry(entity).or_default();
            if inventory.woods != *cached {
                let difference = inventory.woods as isize - *cached as isize;
                flame.increase_radius_by_steps(difference);
                *cached = inventory.woods;
            }
        }
    }
}
