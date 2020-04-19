use super::component_prelude::*;

pub enum WoodInventoryAction {
    Add(usize),
    Remove(usize),
}

#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct WoodInventory {
    pub woods: usize,
    actions:   Vec<WoodInventoryAction>,
}

impl WoodInventory {
    pub fn is_empty(&self) -> bool {
        self.woods == 0
    }
}

impl ActionQueue for WoodInventory {
    type Action = WoodInventoryAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}
