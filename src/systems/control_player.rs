use super::system_prelude::*;

#[derive(Default)]
pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Read<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Movable>,
    );

    fn run(
        &mut self,
        (input_manager, player_store, mut movable_store): Self::SystemData,
    ) {
        for (_, movable) in (&player_store, &mut movable_store).join() {
            if let Some(x) = input_manager.axis_value(IngameAxis::MoveX) {
                if x != 0.0 {
                    movable.add_action(MoveAction::Walk(x));
                }
            }
        }
    }
}
