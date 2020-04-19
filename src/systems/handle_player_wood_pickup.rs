use super::system_prelude::*;
use deathframe::physics::query::prelude::*;

#[derive(Default)]
pub struct HandlePlayerWoodPickupSystem;

impl<'a> System<'a> for HandlePlayerWoodPickupSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, WoodInventory>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(
        &mut self,
        (
            entities,
            input_manager,
            player_store,
            mut wood_inventory_store,
            collider_store,
        ): Self::SystemData,
    ) {
        for (_, wood_inventory, collider) in
            (&player_store, &mut wood_inventory_store, &collider_store).join()
        {
            if !wood_inventory.is_at_max() {
                let query_exp = {
                    use deathframe::physics::query::exp::prelude_variants::*;

                    And(vec![IsTag(CollisionTag::Wood), IsSide(Inner)])
                };

                if input_manager.is_down(IngameAction::Interact) {
                    if let Some(wood_collision) = collider
                        .query::<FindQuery<CollisionTag>>()
                        .exp(&query_exp)
                        .run()
                    {
                        wood_inventory.add_action(WoodInventoryAction::Add(1));
                        let _ =
                            entities.delete(entities.entity(wood_collision.id));
                    }
                }
            }
        }
    }
}
