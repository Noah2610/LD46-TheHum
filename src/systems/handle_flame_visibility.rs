use super::system_prelude::*;
use deathframe::core::resources::entity_component_inserter::InsertionAction;

#[derive(Default)]
pub struct HandleFlameVisibilitySystem;

impl<'a> System<'a> for HandleFlameVisibilitySystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Flame>,
        ReadStorage<'a, VisibleInFlame>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Hidden>,
        ReadStorage<'a, Loadable>,
        WriteStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            flame_store,
            visible_in_flame_store,
            transform_store,
            mut hidden_store,
            loadable_store,
            mut loaded_store,
        ): Self::SystemData,
    ) {
        let mut visibility_loader = EntityComponentInserter::default()
            .with_priority(InsertionAction::Remove);
        let mut loaded_loader = EntityComponentInserter::default()
            .with_priority(InsertionAction::Insert);

        for (flame, flame_transform, loadable_opt) in
            (&flame_store, &transform_store, loadable_store.maybe()).join()
        {
            let flame_pos = {
                let trans = flame_transform.translation();
                (trans.x, trans.y)
            };
            let is_pos_in_radius = |pos: (f32, f32)| -> bool {
                // https://stackoverflow.com/a/481150/10927893
                (pos.0 - flame_pos.0).powf(2.0)
                    + (pos.1 - flame_pos.1).powf(2.0)
                    < flame.radius.powf(2.0)
            };

            for (target_entity, _, target_transform) in
                (&entities, &visible_in_flame_store, &transform_store).join()
            {
                let target_pos = {
                    let trans = target_transform.translation();
                    (trans.x, trans.y)
                };
                if is_pos_in_radius(target_pos) {
                    visibility_loader.remove(target_entity);
                    if loadable_opt.is_some() {
                        loaded_loader.insert(target_entity);
                    }
                } else {
                    visibility_loader.insert(target_entity);
                    if loadable_opt.is_some() {
                        loaded_loader.remove(target_entity);
                    }
                }
            }
        }

        visibility_loader.run(&mut hidden_store).unwrap();
        loaded_loader.run(&mut loaded_store).unwrap();
    }
}
