use super::state_prelude::*;
use std::path::PathBuf;

#[derive(Default)]
pub struct Startup;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Startup {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        insert_resources(&mut data.world);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update_core(data.world);
        Trans::Switch(Box::new(MainMenu::default()))
    }
}

fn insert_resources(world: &mut World) {
    let sprite_sheet_handles = SpriteSheetHandles::<PathBuf>::default();
    world.insert(sprite_sheet_handles);
}
