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
    pub max_woods: Option<usize>,
    #[serde(skip)]
    actions:       Vec<WoodInventoryAction>,
}

impl WoodInventory {
    pub fn is_empty(&self) -> bool {
        self.woods == 0
    }

    pub fn is_at_max(&self) -> bool {
        self.max_woods.map(|max| self.woods == max).unwrap_or(false)
    }
}

impl ActionQueue for WoodInventory {
    type Action = WoodInventoryAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}
