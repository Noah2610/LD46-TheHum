use deathframe::amethyst::input::{BindingTypes, InputBundle};
use std::fmt;

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct IngameBindings;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IngameAxis {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IngameAction {}

impl BindingTypes for IngameBindings {
    type Axis = IngameAxis;
    type Action = IngameAction;
}

impl IngameBindings {
    pub fn bundle() -> deathframe::amethyst::Result<InputBundle<IngameBindings>>
    {
        Ok(
            InputBundle::<IngameBindings>::new().with_bindings_from_file(
                crate::resource("config/ingame_bindings.ron"),
            )?,
        )
    }
}

impl fmt::Display for IngameAxis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for IngameAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
