use async_trait::async_trait;
use pumpkin_data::block_properties::{BlockProperties, WallTorchLikeProperties};
use pumpkin_world::BlockStateId;

use crate::block::pumpkin_block::{BlockMetadata, OnPlaceArgs, PumpkinBlock};

pub struct CarvedPumpkinBlock;

static TAGS: &[&str; 2] = &["carved_pumpkin", "jack_o_lantern"];

impl BlockMetadata for CarvedPumpkinBlock {
    fn namespace(&self) -> &'static str {
        "minecraft"
    }

    fn ids(&self) -> &'static [&'static str] {
        TAGS
    }
}

#[async_trait]
impl PumpkinBlock for CarvedPumpkinBlock {
    async fn on_place(&self, args: OnPlaceArgs<'_>) -> BlockStateId {
        let mut props = WallTorchLikeProperties::default(args.block);
        props.facing = args
            .player
            .living_entity
            .entity
            .get_horizontal_facing()
            .opposite();
        props.to_state_id(args.block)
    }
}
