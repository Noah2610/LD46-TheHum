// resources/ui/main_menu.ron

use super::menu_prelude::*;
use super::state_prelude::*;

const DEFAULT_LEVEL: &str = "dev.json";

#[derive(Default)]
pub struct MainMenu {
    ui_data: UiData,
}

impl MainMenu {
    fn start<'a, 'b>(&mut self, data: &mut StateData<GameData<'a, 'b>>) {
        self.create_ui(data, resource("ui/main_menu.ron").to_str().unwrap());
        let _ = data
            .world
            .write_resource::<Songs<SongKey>>()
            .play(&SongKey::MainMenu);
    }

    fn stop<'a, 'b>(&mut self, data: &mut StateData<GameData<'a, 'b>>) {
        self.delete_ui(data);
        let _ = data
            .world
            .write_resource::<Songs<SongKey>>()
            .stop(&SongKey::MainMenu);
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for MainMenu {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        if let Err(e) = enter_fullscreen(data.world) {
            eprintln!("[WARNING]\n    Couldn't enter fullscreen:\n    {}", e);
        }

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
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data
            .update(data.world, DispatcherId::MainMenu)
            .unwrap();

        let input_manager =
            data.world.read_resource::<InputManager<MenuBindings>>();
        if input_manager.is_down(MenuAction::Enter) {
            return Trans::Push(Box::new(Ingame::new(
                DEFAULT_LEVEL.to_string(),
            )));
        } else if input_manager.is_down(MenuAction::Back) {
            return Trans::Quit;
        }

        Trans::None
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
                "btn_start" => Some(Trans::Push(Box::new(Ingame::new(
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

fn enter_fullscreen(world: &mut World) -> Result<(), String> {
    use amethyst::ecs::{Read, ReadExpect, SystemData};
    use amethyst::renderer::rendy::wsi::winit::Window;
    use amethyst::window::{MonitorIdent, MonitorsAccess};
    use deathframe::amethyst;

    // let window = world.read_resource::<Window>();
    let window = <ReadExpect<'_, Window>>::fetch(world);
    let monitor_ident = MonitorIdent::from_primary(&*window);
    let monitor_id = monitor_ident.monitor_id(&*window);

    dbg!(&monitor_ident);
    dbg!(&monitor_id);

    window.set_fullscreen(Some(monitor_id));

    Ok(())
}
