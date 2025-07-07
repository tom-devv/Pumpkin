use std::sync::{Arc, LazyLock};

use async_trait::async_trait;
use chrono::offset;
use pumpkin_data::block_properties::{
    BlockProperties, CactusLikeProperties, EnumVariants, Integer0To15,
};
use pumpkin_data::damage::DamageType;
use pumpkin_data::particle::Particle;
use pumpkin_data::tag::Tagable;
use pumpkin_data::{Block, BlockDirection};
use pumpkin_macros::pumpkin_block;
use pumpkin_util::math::position::{BlockPos, BlockPosIterator};
use pumpkin_world::BlockStateId;
use pumpkin_world::chunk::TickPriority;
use pumpkin_world::world::{BlockAccessor, BlockFlags};
use rand::Rng;

use crate::block::pumpkin_block::{
    CanPlaceAtArgs, GetStateForNeighborUpdateArgs, OnEntityCollisionArgs, OnScheduledTickArgs,
    PumpkinBlock, RandomTickArgs,
};
use crate::world::World;

#[pumpkin_block("minecraft:enchanting_table")]
pub struct EnchantingTableBlock;

pub static POWER_PROVIDER_OFFSETS: LazyLock<Vec<BlockPos>> = LazyLock::new(|| {
    BlockPosIterator::new(-2, 0, -2, 2, 1, 2)
        .filter(|pos| pos.0.x == 2 || pos.0.z == 2)
        .collect()
});

#[async_trait]
impl PumpkinBlock for EnchantingTableBlock {}

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
