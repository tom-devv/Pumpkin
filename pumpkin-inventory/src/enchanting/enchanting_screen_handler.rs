use std::{any::Any, sync::Arc};

use async_trait::async_trait;
use pumpkin_data::{item::Item, screen::WindowType};
use pumpkin_world::{inventory::Inventory, item::ItemStack};

use crate::{
    player::player_inventory::PlayerInventory,
    screen_handler::{InventoryPlayer, ScreenHandler, ScreenHandlerBehaviour},
};

pub struct EnchantingTableScreenHandler {
    behaviour: ScreenHandlerBehaviour,
    enchantment_inventory: Arc<dyn 
}

impl EnchantingTableScreenHandler {
    pub async fn new(sync_id: u8, player_inventory: &Arc<PlayerInventory>) -> Self {
        let mut handler = EnchantingTableScreenHandler {
            behaviour: ScreenHandlerBehaviour::new(sync_id, Some(WindowType::Enchantment)),
        };

        let player_inventory: Arc<dyn Inventory> = player_inventory.clone();
        handler.add_player_slots(&player_inventory);

        handler
    }
}

#[async_trait]
impl ScreenHandler for EnchantingTableScreenHandler {

    async fn on_closed(&mut self, player: &dyn InventoryPlayer) {
        self.default_on_closed(player).await;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_behaviour(&self) -> &ScreenHandlerBehaviour {
        &self.behaviour
    }

    fn get_behaviour_mut(&mut self) -> &mut ScreenHandlerBehaviour {
        &mut self.behaviour
    }

    async fn quick_move(&mut self, player: &dyn InventoryPlayer, slot_index: i32) -> ItemStack {
        let slot = self.get_behaviour().slots[slot_index as usize].clone();

        // if slot.has_stack().await {
        //     let slot_stack = slot.get_stack().await;
        //     let mut slot_stack = slot_stack.lock().await;
        //     let stack_prev = *slot_stack;

        //     if slot_index == 0 {
        //         if !self.insert_item(&mut slot_stack, 2, 36, true).await {
        //             return ItemStack::EMPTY;
        //         }
        //     } else if slot_index == 1 {
        //         if !self.insert_item(&mut slot_stack, 2, 36, true).await {
        //             return ItemStack::EMPTY;
        //         }
        //     } else if slot_stack.get_item().id == Item::LAPIS_LAZULI.id {
        //         if !self.insert_item(&mut slot_stack, 1, 2, true).await {
        //             return ItemStack::EMPTY;
        //         }
        //     } else {
        //         if let Some(first_slot) = self.get_behaviour().slots.get(0) {
        //             if first_slot.has_stack().await || !first_slot.can_insert(&slot_stack).await {
        //                 return ItemStack::EMPTY;
        //             }
        //             let temp_slot_stack = slot_stack.copy_with_count(1);
        //             slot_stack.decrement(1);
        //             first_slot.set_stack(temp_slot_stack).await;
        //         }
        //     }

        //     let stack = *slot_stack;
        //     drop(slot_stack);
        //     if stack.is_empty() {
        //         slot.set_stack_prev(ItemStack::EMPTY, stack_prev).await;
        //     } else {
        //         slot.mark_dirty().await;
        //     }

        //     if stack.item_count == stack_prev.item_count {
        //         return ItemStack::EMPTY;
        //     }

        //     slot.on_take_item(player, &stack).await;

        //     return stack_prev;
        // }
        ItemStack::EMPTY
    }
}
