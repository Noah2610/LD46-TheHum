use super::system_prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct UpdateBonfireHaloSizeSystem {
    cache: HashMap<Entity, f32>,
}

impl<'a> System<'a> for UpdateBonfireHaloSizeSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Halo>,
        ReadStorage<'a, Flame>,
        WriteStorage<'a, Size>,
        WriteStorage<'a, ScaleOnce>,
    );

    fn run(
        &mut self,
        (
            entities,
            halo_store,
            flame_store,
            mut size_store,
            mut scale_once_store,
        ): Self::SystemData,
    ) {
        for (halo_entity, halo, halo_size) in
            (&entities, &halo_store, &mut size_store).join()
        {
            if let Some(bonfire_entity) = halo.bonfire_entity {
                if let Some(bonfire_flame) = flame_store.get(bonfire_entity) {
                    let cached_flame_radius =
                        self.cache.entry(bonfire_entity).or_default();
                    if bonfire_flame.radius != *cached_flame_radius {
                        let new_size =
                            bonfire_flame.radius * 2.0 + halo.size_margin;
                        halo_size.w = new_size;
                        halo_size.h = new_size;
                        scale_once_store
                            .insert(halo_entity, ScaleOnce::default())
                            .unwrap();
                        *cached_flame_radius = bonfire_flame.radius;
                    }
                }
            } else {
                eprintln!(
                    "[WARNING]\n[UpdateBonfireHaloSizeSystem]\n    \
                     BonfireHalo has no Bonfire enity saved.\n    Cannot \
                     update halo size."
                );
            }
        }
    }
}
