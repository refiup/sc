use soroban_sdk::{symbol_short, Address, Env, String};

/// Event emission utilities
pub struct Events;

impl Events {
    /// Emit AdminSetEvent when admin is initialized
    pub fn admin_set(env: &Env, admin: &Address) {
        env.events().publish(
            (symbol_short!("event_dst"), symbol_short!("admin_set")),
            admin,
        );
    }

    /// Emit HumanAddedEvent when a new human is added
    pub fn human_added(env: &Env, address: &Address, ipfs_hash: &String) {
        env.events().publish(
            (symbol_short!("event_dst"), symbol_short!("human_add")),
            (address, ipfs_hash),
        );
    }

    /// Emit HumanValidationUpdatedEvent
    pub fn human_validation_updated(env: &Env, address: &Address, validated: bool) {
        env.events().publish(
            (symbol_short!("event_dst"), symbol_short!("human_val")),
            (address, validated),
        );
    }

    /// Emit HumanImageUpdatedEvent
    pub fn human_image_updated(env: &Env, address: &Address, ipfs_hash: &String) {
        env.events().publish(
            (symbol_short!("event_dst"), symbol_short!("human_img")),
            (address, ipfs_hash),
        );
    }

    /// Emit EventCreatedEvent
    pub fn event_created(
        env: &Env,
        event_id: &String,
        location: &String,
        event_name: &String,
        pool: i128,
    ) {
        env.events().publish(
            (symbol_short!("event_dst"), symbol_short!("evt_crt")),
            (event_id, location, event_name, pool),
        );
    }

    /// Emit HumanAddedToEventEvent
    pub fn human_added_to_event(env: &Env, event_id: &String, human_address: &Address) {
        env.events().publish(
            (symbol_short!("event_dst"), symbol_short!("evt_add")),
            (event_id, human_address),
        );
    }

    /// Emit DistributionCompletedEvent
    pub fn distribution_completed(env: &Env, event_id: &String, total_amount: i128, recipient_count: u32) {
        env.events().publish(
            (symbol_short!("event_dst"), symbol_short!("distribtd")),
            (event_id, total_amount, recipient_count),
        );
    }
}
