pub mod prelude {
    pub use super::ingame::Ingame;
    pub use super::main_menu::MainMenu;
    pub use super::startup::Startup;
}

pub mod aliases {
    use crate::resources::prelude::DispatcherId;
    use deathframe::core::custom_game_data::prelude::*;

    pub type CustomData = ();

    pub type GameData<'a, 'b> =
        CustomGameData<'a, 'b, DispatcherId, CustomData>;

    pub type GameDataBuilder<'a, 'b> =
        CustomGameDataBuilder<'a, 'b, DispatcherId, CustomData>;
}

mod state_prelude {
    pub use super::aliases::*;
    pub use super::prelude::*;
    pub use crate::resource;
    pub use crate::resources::prelude::*;
    pub use crate::settings::prelude::*;
    pub use deathframe::states::state_prelude::*;
}

mod menu_prelude {
    pub use deathframe::amethyst::ui::{UiEvent, UiEventType};
    pub use deathframe::core::menu::prelude::*;
}

mod ingame;
mod main_menu;
mod startup;
