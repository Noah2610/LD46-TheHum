use super::system_prelude::*;
use deathframe::physics::query::prelude::*;

#[derive(Default)]
pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Read<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Movable>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(
        &mut self,
        (
            input_manager,
            player_store,
            mut movable_store,
            collider_store,
        ): Self::SystemData,
    ) {
        for (_, movable, collider) in
            (&player_store, &mut movable_store, &collider_store).join()
        {
            // MOVE
            if let Some(x) = input_manager.axis_value(IngameAxis::MoveX) {
                if x != 0.0 {
                    movable.add_action(MoveAction::Walk(x));
                }
            }

            // JUMP
            if input_manager.is_down(IngameAction::Jump) {
                let on_ground = {
                    use deathframe::physics::query::exp::prelude_variants::*;
                    collider
                        .query::<FindQuery<CollisionTag>>()
                        .exp(&And(vec![
                            IsSide(Bottom),
                            IsTag(CollisionTag::Solid),
                        ]))
                        .run()
                        .is_some()
                };
                if on_ground {
                    movable.add_action(MoveAction::Jump);
                }
            } else if input_manager.is_up(IngameAction::Jump) {
                movable.add_action(MoveAction::KillJump);
            }
        }
    }
}
