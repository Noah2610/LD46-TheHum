// resources/ui/load_ingame.ron

use super::menu_prelude::*;
use super::state_prelude::*;
use crate::level_loader;

pub struct LoadIngame {
    level_name: String,
    ui_data:    UiData,
}

impl LoadIngame {
    pub fn new(level_name: String) -> Self {
        Self {
            level_name,
            ui_data: Default::default(),
        }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for LoadIngame {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        data.world.delete_all();
        self.create_ui(
            &mut data,
            resource("ui/load_ingame.ron").to_str().unwrap(),
        );
        level_loader::load_level(
            resource(format!("levels/{}", &self.level_name)),
            data.world,
        )
        .unwrap();
    }

    fn on_stop(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.delete_ui(&mut data);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data
            .update(data.world, DispatcherId::LoadIngame)
            .unwrap();

        Trans::Switch(Box::new(Ingame::default()))
    }

    fn fixed_update(
        &mut self,
        mut data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        if let Some(trans) = self.update_ui_events(&mut data) {
            trans
        } else {
            Trans::None
        }
    }
}

impl<'a, 'b> Menu<GameData<'a, 'b>, StateEvent> for LoadIngame {
    fn event_triggered(
        &mut self,
        _data: &mut StateData<GameData<'a, 'b>>,
        _event_name: String,
        _event: UiEvent,
    ) -> Option<Trans<GameData<'a, 'b>, StateEvent>> {
        None
    }

    fn ui_data(&self) -> &UiData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UiData {
        &mut self.ui_data
    }
}
