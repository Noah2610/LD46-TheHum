use super::system_prelude::*;
use deathframe::physics::query::prelude::*;

#[derive(Default)]
pub struct HandlePlayerFeedBonfireSystem;

impl<'a> System<'a> for HandlePlayerFeedBonfireSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, SoundPlayer<SoundKey>>,
        Read<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Collider<CollisionTag>>,
        ReadStorage<'a, Bonfire>,
        WriteStorage<'a, WoodInventory>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut sound_player,
            input_manager,
            player_store,
            collider_store,
            bonfire_store,
            mut wood_inventory_store,
        ): Self::SystemData,
    ) {
        for (player_entity, _, player_collider) in
            (&entities, &player_store, &collider_store).join()
        {
            if input_manager.is_down(IngameAction::Interact) {
                let query_exp = {
                    use deathframe::physics::query::exp::prelude_variants::*;
                    And(vec![IsTag(CollisionTag::Bonfire), IsSide(Inner)])
                };

                if let Some(bonfire_collision) = player_collider
                    .query::<FindQuery<CollisionTag>>()
                    .exp(&query_exp)
                    .run()
                {
                    let player_inventory = wood_inventory_store
                        .get_mut(player_entity)
                        .expect("Player needs WoodInventory");

                    let should_transfer_wood = !player_inventory.is_empty();

                    if should_transfer_wood {
                        sound_player.add_action(SoundAction::Play(
                            "woodblock".to_string(),
                        ));

                        player_inventory
                            .add_action(WoodInventoryAction::Remove(1));

                        for (bonfire_entity, _, bonfire_inventory) in (
                            &entities,
                            &bonfire_store,
                            &mut wood_inventory_store,
                        )
                            .join()
                        {
                            if bonfire_entity.id() == bonfire_collision.id {
                                bonfire_inventory
                                    .add_action(WoodInventoryAction::Add(1));
                            }
                        }
                    }
                }
            }
        }
    }
}
