//! Assorted functionality relating to blocks, including:
//! * The block notify system, where a block update "notifies"
//! adjacent blocks of the update. This is used for spawning
//! falling blocks, for example.
//!
//! The block notify system works as follows: when a block
//! is updated, `on_block_update_notify_adjacent` is called,
//! which checks the blocks adjacent to the updated block.
//! For each adjacent block, `notify_entity_for_block` is called
//! which returns an `Option<EntityBuilder>` containing the components
//! to create for the notify entity. For example, `Some(EntityBuilder::new().with(FallingBlockNotify)`
//! could be returned for `Sand` and `Gravel` variants.
//!
//! `on_block_update_notify_adjacent` then creates an entity with those components.
//! The "notify entity," in this case,
//! acts as a sort of event, as other systems can check for these entities
//! and perform actions based on their components.

use crate::adjacent_blocks;
use feather_core::blocks::BlockId;
use feather_core::util::BlockPosition;
use feather_server_types::{BlockUpdateEvent, Game};
use fecs::{EntityBuilder, World};
use std::iter;

/// Marker component stating that an entity is a notify entity.
#[derive(Copy, Clone, Debug)]
pub struct BlockNotify;

/// Component storing the position of a block for a block notify entity.
#[derive(Copy, Clone, Debug)]
pub struct BlockNotifyPosition(pub BlockPosition);

/// Component storing the type of block notified.
#[derive(Copy, Clone, Debug)]
pub struct BlockNotifyBlock(pub BlockId);

/// Marker component for block notify entities created for falling
/// blocks, such as sand and gravel.
#[derive(Copy, Clone, Debug)]
pub struct BlockNotifyFallingBlock;

/// Marker component for block notify entities created for falling
/// blocks, such as sand and gravel.
#[derive(Copy, Clone, Debug)]
pub struct BlockNotifySupportedBlock;

/// Returns an `EntityBuilder` to create the block notify entity for
/// the given block type.
fn notify_entity_for_block(block: BlockId, pos: BlockPosition) -> Option<EntityBuilder> {
    let builder = EntityBuilder::new()
        .with(BlockNotify)
        .with(BlockNotifyPosition(pos))
        .with(BlockNotifyBlock(block));

    if block.can_fall() {
        Some(builder.with(BlockNotifyFallingBlock))
    } else if block.needs_support().is_some() {
        Some(builder.with(BlockNotifySupportedBlock))
    } else {
        None
    }
}

/// When a block is updated, spawns notify entities
/// for adjacent blocks.
#[fecs::event_handler]
pub fn on_block_update_notify_adjacent(
    event: &BlockUpdateEvent,
    game: &mut Game,
    world: &mut World,
) {
    adjacent_blocks(event.pos)
        .into_iter()
        .chain(iter::once(event.pos))
        .filter_map(|adjacent_pos| {
            if let Some(adjacent_block) = game.block_at(adjacent_pos) {
                Some((adjacent_block, adjacent_pos))
            } else {
                None
            }
        })
        .filter_map(|(adjacent_block, adjacent_pos)| {
            notify_entity_for_block(adjacent_block, adjacent_pos)
        })
        .for_each(|builder| {
            builder.build().spawn_in(world);
        })
}
