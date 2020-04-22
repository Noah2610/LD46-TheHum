// resources/ui/game_over.ron

use super::menu_prelude::*;
use super::state_prelude::*;
// TODO terrible
use crate::systems::system_prelude::*;
use amethyst::assets::ProgressCounter;
use deathframe::amethyst;

const WOODS_UI_TRANSFORM_ID: &str = "game_over_woods";
const WOODS_UI_TEXT_REPL: &str = "<WOODS>";

#[derive(Default)]
pub struct GameOver {
    ui_data:     UiData,
    ui_progress: Option<ProgressCounter>,
}

impl GameOver {
    fn set_label(&self, world: &mut World) {
        use amethyst::ui::{UiText, UiTransform};

        world.exec(
            |(
                bonfire_store,
                wood_inventory_store,
                ui_transforms,
                mut ui_texts,
            ): (
                ReadStorage<Bonfire>,
                ReadStorage<WoodInventory>,
                ReadStorage<UiTransform>,
                WriteStorage<UiText>,
            )| {
                if let Some(bonfire_woods) =
                    (&bonfire_store, &wood_inventory_store)
                        .join()
                        .next()
                        .map(|(_, inventory)| inventory.woods)
                {
                    if let Some((_, text)) = (&ui_transforms, &mut ui_texts)
                        .join()
                        .find(|(transform, _)| {
                            transform.id.as_str() == WOODS_UI_TRANSFORM_ID
                        })
                    {
                        let new_text = text.text.replace(
                            WOODS_UI_TEXT_REPL,
                            bonfire_woods.to_string().as_str(),
                        );
                        text.text = new_text;
                    }
                }
            },
        );
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for GameOver {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.ui_progress = Some(self.create_ui(
            &mut data,
            resource("ui/game_over.ron").to_str().unwrap(),
        ));

        {
            let mut sound_player =
                data.world.write_resource::<SoundPlayer<SoundKey>>();
            sound_player.add_action(SoundAction::Play(
                "play_with_headphones".to_string(),
            ));
        }

        data.world.exec(
            |(entities, bonfire_store, mut animations_store, halo_store): (
                Entities,
                ReadStorage<Bonfire>,
                WriteStorage<AnimationsContainer<AnimationKey>>,
                ReadStorage<Halo>,
            )| {
                for (bonfire_entity, _, animations) in
                    (&entities, &bonfire_store, &mut animations_store).join()
                {
                    let _ = animations.play(AnimationKey::BonfireBurnt);
                    for (halo_entity, halo) in (&entities, &halo_store).join() {
                        if let Some(halo_bonfire_entity) = halo.bonfire_entity {
                            if bonfire_entity == halo_bonfire_entity {
                                let _ = entities.delete(halo_entity);
                            }
                        }
                    }
                }
            },
        );
    }

    fn on_stop(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.delete_ui(&mut data);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data
            .update(data.world, DispatcherId::GameOver)
            .unwrap();

        if let Some(progress) = self.ui_progress.as_ref() {
            if progress.is_complete() {
                self.set_label(data.world);
                self.ui_progress = None;
            }
        }

        let input_manager =
            data.world.read_resource::<InputManager<MenuBindings>>();
        if input_manager.is_down(MenuAction::Enter)
            || input_manager.is_down(MenuAction::Back)
        {
            return Trans::Replace(Box::new(MainMenu::default()));
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

impl<'a, 'b> Menu<GameData<'a, 'b>, StateEvent> for GameOver {
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
