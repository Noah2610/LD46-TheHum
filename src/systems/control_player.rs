use super::system_prelude::*;
use deathframe::physics::query::prelude::*;

#[derive(Default)]
pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Read<'a, InputManager<IngameBindings>>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Movable>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(
        &mut self,
        (
            input_manager,
            mut player_store,
            mut movable_store,
            collider_store,
        ): Self::SystemData,
    ) {
        for (player, movable, collider) in
            (&mut player_store, &mut movable_store, &collider_store).join()
        {
            // UPDATE player.on_ground
            // this shouldn't be here but whatever dude
            player.on_ground = {
                use deathframe::physics::query::exp::prelude_variants::*;
                collider
                    .query::<FindQuery<CollisionTag>>()
                    .exp(&And(vec![IsSide(Bottom), IsTag(CollisionTag::Solid)]))
                    .run()
                    .is_some()
            };

            // MOVE
            if let Some(x) = input_manager.axis_value(IngameAxis::MoveX) {
                if x != 0.0 {
                    movable.add_action(MoveAction::Walk(x));
                }
            }

            // JUMP
            if input_manager.is_down(IngameAction::Jump) {
                if player.on_ground {
                    movable.add_action(MoveAction::Jump);
                }
            } else if input_manager.is_up(IngameAction::Jump) {
                movable.add_action(MoveAction::KillJump);
            }
        }
    }
}
