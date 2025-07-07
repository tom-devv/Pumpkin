use pumpkin_util::text::TextComponent;
use std::sync::{Arc, LazyLock};
use tokio::sync::Mutex;

use async_trait::async_trait;
use chrono::offset;
use pumpkin_data::block_properties::{
    BlockProperties, CactusLikeProperties, EnumVariants, Integer0To15,
};
use pumpkin_data::damage::DamageType;
use pumpkin_data::particle::Particle;
use pumpkin_data::tag::Tagable;
use pumpkin_data::{Block, BlockDirection};
use pumpkin_inventory::enchanting::enchanting_screen_handler::EnchantingTableScreenHandler;
use pumpkin_inventory::player::player_inventory::PlayerInventory;
use pumpkin_inventory::screen_handler::{InventoryPlayer, ScreenHandler, ScreenHandlerFactory};
use pumpkin_macros::pumpkin_block;
use pumpkin_util::math::position::{BlockPos, BlockPosIterator};
use pumpkin_world::BlockStateId;
use pumpkin_world::chunk::TickPriority;
use pumpkin_world::world::{BlockAccessor, BlockFlags};
use rand::Rng;

use crate::block::pumpkin_block::{
    CanPlaceAtArgs, GetStateForNeighborUpdateArgs, NormalUseArgs, OnEntityCollisionArgs,
    OnScheduledTickArgs, PumpkinBlock, RandomTickArgs, UseWithItemArgs,
};
use crate::block::registry::BlockActionResult;
use crate::world::World;

#[pumpkin_block("minecraft:enchanting_table")]
pub struct EnchantingTableBlock;

pub static POWER_PROVIDER_OFFSETS: LazyLock<Vec<BlockPos>> = LazyLock::new(|| {
    BlockPosIterator::new(-2, 0, -2, 2, 1, 2)
        .filter(|pos| pos.0.x == 2 || pos.0.z == 2)
        .collect()
});

#[async_trait]
impl PumpkinBlock for EnchantingTableBlock {
    async fn normal_use(&self, args: NormalUseArgs<'_>) {
        args.player
            .open_handled_screen(&EnchantingTableScreenFactory)
            .await;
    }

    async fn use_with_item(&self, args: UseWithItemArgs<'_>) -> BlockActionResult {
        args.player
            .open_handled_screen(&EnchantingTableScreenFactory)
            .await;
        BlockActionResult::Consume
    }
}

async fn can_access_power_provider(
    world: Arc<World>,
    table_pos: BlockPos,
    offset_pos: BlockPos,
) -> bool {
    let provider_block = world.get_block(&table_pos.add_pos(&offset_pos)).await;
    let transmitter_block = world
        .get_block(&table_pos.add(offset_pos.0.x / 2, offset_pos.0.y, offset_pos.0.z / 2))
        .await;

    provider_block
        .is_tagged_with("minecraft:enchantment_power_provider")
        .unwrap()
        && transmitter_block
            .is_tagged_with("minecraft:enchantment_power_transmitter")
            .unwrap()
}

struct EnchantingTableScreenFactory;

#[async_trait]
impl ScreenHandlerFactory for EnchantingTableScreenFactory {
    async fn create_screen_handler(
        &self,
        sync_id: u8,
        player_inventory: &Arc<PlayerInventory>,
        _player: &dyn InventoryPlayer,
    ) -> Option<Arc<Mutex<dyn ScreenHandler>>> {
        Some(Arc::new(Mutex::new(
            EnchantingTableScreenHandler::new(sync_id, player_inventory).await,
        )))
    }

    fn get_display_name(&self) -> TextComponent {
        TextComponent::translate("container.crafting", &[])
    }
}
