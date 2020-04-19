use super::component_prelude::*;

#[derive(Clone)]
pub enum WoodInventoryAction {
    Add(usize),
    Remove(usize),
}

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct WoodInventory {
    #[serde(skip)]
    pub woods:     usize,
    pub max_woods: usize,
    #[serde(skip)]
    actions:       Vec<WoodInventoryAction>,
}

impl WoodInventory {
    pub fn is_empty(&self) -> bool {
        self.woods == 0
    }

    pub fn is_at_max(&self) -> bool {
        self.woods == self.max_woods
    }
}

impl ActionQueue for WoodInventory {
    type Action = WoodInventoryAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}
