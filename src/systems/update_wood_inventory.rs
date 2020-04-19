use super::system_prelude::*;

#[derive(Default)]
pub struct UpdateWoodInventorySystem;

impl<'a> System<'a> for UpdateWoodInventorySystem {
    type SystemData = WriteStorage<'a, WoodInventory>;

    fn run(&mut self, mut wood_inventory_store: Self::SystemData) {
        for wood_inventory in (&mut wood_inventory_store).join() {
            for action in wood_inventory
                .drain_actions()
                .collect::<Vec<WoodInventoryAction>>()
                .into_iter()
            {
                match action {
                    WoodInventoryAction::Add(amount) => {
                        let _ = wood_inventory.woods.checked_add(amount);
                    }
                    WoodInventoryAction::Remove(amount) => {
                        let _ = wood_inventory.woods.checked_sub(amount);
                    }
                }
            }
        }
    }
}
