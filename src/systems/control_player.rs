use super::system_prelude::*;

#[derive(Default)]
pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        Read<'a, InputManager<IngameBindings>>,
    );

    fn run(&mut self, (player_store, input_manager): Self::SystemData) {
        for _ in player_store.join() {
            if let Some(x) = input_manager.axis_value(IngameAxis::MoveX) {
                dbg!(x);
            }
        }
    }
}
