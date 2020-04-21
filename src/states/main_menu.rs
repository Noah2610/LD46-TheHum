// resources/ui/main_menu.ron

use super::menu_prelude::*;
use super::state_prelude::*;
use crate::components::prelude::{
    ActionQueue,
    Movable,
    MoveAction,
    Player,
    Transform,
};
use crate::level_loader::level_data::SizeData;
use crate::level_loader::load_level;
use amethyst::ecs::{Join, ReadStorage, WriteStorage};
use deathframe::amethyst;

const DEFAULT_LEVEL: &str = "dev.json";
const MAIN_MENU_LEVEL: &str = "main_menu.json";
const CUTSCENE_PLAYER_WALK_DIR: f32 = 1.0;

#[derive(Default)]
pub struct MainMenu {
    ui_data:       UiData,
    level_size:    Option<SizeData>,
    init_cutscene: bool,
}

impl MainMenu {
    fn start<'a, 'b>(&mut self, data: &mut StateData<GameData<'a, 'b>>) {
        self.create_ui(data, resource("ui/main_menu.ron").to_str().unwrap());
        let _ = data
            .world
            .write_resource::<Songs<SongKey>>()
            .play(&SongKey::MainMenu);

        self.level_size = load_level(
            resource(format!("levels/{}", MAIN_MENU_LEVEL)),
            data.world,
        )
        .ok()
        .map(|level| level.size);

        data.world.exec(|mut player_store: WriteStorage<Player>| {
            (&mut player_store)
                .join()
                .next()
                .map(|player| player.on_ground = true)
        });
    }

    fn stop<'a, 'b>(&mut self, data: &mut StateData<GameData<'a, 'b>>) {
        data.world.delete_all();
        let _ = data
            .world
            .write_resource::<Songs<SongKey>>()
            .stop(&SongKey::MainMenu);
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for MainMenu {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.start(&mut data);
    }

    fn on_resume(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.start(&mut data);
    }

    fn on_stop(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.stop(&mut data);
    }

    fn on_pause(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.stop(&mut data);
    }

    fn update(
        &mut self,
        mut data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data
            .update(data.world, DispatcherId::MainMenu)
            .unwrap();

        {
            let input_manager =
                data.world.read_resource::<InputManager<MenuBindings>>();
            if input_manager.is_down(MenuAction::Enter) {
                self.init_cutscene = true;
            } else if input_manager.is_down(MenuAction::Back) {
                return Trans::Quit;
            }
        }

        let mut should_start = false;

        if let Some(level_size) = self.level_size.as_ref() {
            if self.init_cutscene {
                data.world.exec(
                    |(mut player_store, mut movable_store, transform_store): (
                        WriteStorage<Player>,
                        WriteStorage<Movable>,
                        ReadStorage<Transform>,
                    )| {
                        if let Some((_, movable, transform)) = (
                            &mut player_store,
                            &mut movable_store,
                            &transform_store,
                        )
                            .join()
                            .next()
                        {
                            let pos = transform.translation();
                            if pos.x > level_size.w || pos.x < 0.0 {
                                should_start = true;
                            } else {
                                movable.add_action(MoveAction::Walk(
                                    CUTSCENE_PLAYER_WALK_DIR,
                                ));
                            }
                        } else {
                            should_start = true;
                        }
                    },
                )
            }
        } else {
            should_start = self.init_cutscene;
        }

        if should_start {
            self.delete_ui(&mut data);
            Trans::Push(Box::new(LoadIngame::new(DEFAULT_LEVEL.to_string())))
        } else {
            Trans::None
        }
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

impl<'a, 'b> Menu<GameData<'a, 'b>, StateEvent> for MainMenu {
    fn event_triggered(
        &mut self,
        _data: &mut StateData<GameData<'a, 'b>>,
        event_name: String,
        event: UiEvent,
    ) -> Option<Trans<GameData<'a, 'b>, StateEvent>> {
        if let UiEventType::ClickStop = event.event_type {
            match event_name.as_str() {
                "btn_start" => Some(Trans::Push(Box::new(LoadIngame::new(
                    DEFAULT_LEVEL.to_string(),
                )))),
                "btn_quit" => Some(Trans::Quit),
                _ => None,
            }
        } else {
            None
        }
    }

    fn ui_data(&self) -> &UiData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UiData {
        &mut self.ui_data
    }
}
