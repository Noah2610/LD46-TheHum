use deathframe::amethyst::input::{BindingTypes, InputBundle};
use std::fmt;

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct MenuBindings;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MenuAxis {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MenuAction {}

impl BindingTypes for MenuBindings {
    type Axis = MenuAxis;
    type Action = MenuAction;
}

impl MenuBindings {
    pub fn bundle() -> deathframe::amethyst::Result<InputBundle<MenuBindings>> {
        Ok(InputBundle::<MenuBindings>::new().with_bindings_from_file(
            crate::resource("config/menu_bindings.ron"),
        )?)
    }
}

impl fmt::Display for MenuAxis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for MenuAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
