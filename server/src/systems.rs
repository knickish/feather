//! Constant string names for each system.
//! This file does not actually contain the systems
//! themselves.

// Chunk logic
pub const CHUNK_LOAD: &str = "chunk_load";
pub const CHUNK_OPTIMIZE: &str = "chunk_optimize";

pub const CHUNK_UNLOAD: &str = "chunk_unload";
pub const CHUNK_HOLD_REMOVE: &str = "chunk_hold_remove";

// Player
pub const PLAYER_DIGGING: &str = "player_digging";
pub const PLAYER_ANIMATION: &str = "player_animation";
pub const CREATIVE_INVENTORY: &str = "creative_inventory";
pub const HELD_ITEM_CHANGE: &str = "held_item_change";
pub const PLAYER_MOVEMENT: &str = "player_movement";

pub const CHUNK_CROSS: &str = "chunk_cross";
pub const PLAYER_INIT: &str = "player_init";
pub const CLIENT_CHUNK_UNLOAD: &str = "client_chunk_unload";

pub const HELD_ITEM_BROADCAST: &str = "held_item_broadcast";
pub const JOIN_BROADCAST: &str = "join_broadcast";
pub const DISCONNECT_BROADCAST: &str = "disconnect_broadcast";
pub const ANIMATION_BROADCAST: &str = "animation_broadcast";
pub const EQUIPMENT_SEND: &str = "equipment_send";
pub const RESOURCE_PACK_SEND: &str = "resource_pack_send";
pub const CHUNK_SEND: &str = "chunk_send";
pub const BLOCK_BREAK_BROADCAST: &str = "block_break_broadcast";

// Entity
pub const CHUNK_ENTITIES_UPDATE: &str = "chunk_entities_update";
pub const ENTITY_DESTROY: &str = "entity_destroy";
pub const ITEM_SPAWN: &str = "item_spawn";
pub const SPAWNER: &str = "spawner";

pub const ENTITY_MOVE_BROADCAST: &str = "entity_move_broadcast";
pub const ENTITY_SPAWN_BROADCAST: &str = "entity_spawn_broadcast";
pub const ENTITY_VELOCITY_BROADCAST: &str = "entity_velocity_broadcast";
pub const ENTITY_DESTROY_BROADCAST: &str = "entity_destroy_broadcast";

// Physics
pub const ENTITY_PHYSICS: &str = "entity_physics";

pub const PHYSICS_INIT: &str = "physics_init";

// Other
pub const JOIN_HANDLER: &str = "join_handler";
pub const NETWORK: &str = "network";
