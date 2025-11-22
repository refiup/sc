use soroban_sdk::{contracttype, Address, Env, String, Vec};

use crate::models::{Event, Human};

/// Storage keys for contract data
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Admin address
    Admin,
    /// Human by address
    Human(Address),
    /// Event by ID
    Event(String),
    /// Counter for total humans
    HumanCount,
    /// Human address by index (for iteration)
    HumanIndex(u32),
}

/// Storage manager following Repository pattern
pub struct Storage;

impl Storage {
    // ========== ADMIN ==========

    /// Set admin address in persistent storage
    pub fn set_admin(env: &Env, admin: &Address) {
        env.storage().instance().set(&DataKey::Admin, admin);
    }

    /// Get admin address from storage
    pub fn get_admin(env: &Env) -> Option<Address> {
        env.storage().instance().get(&DataKey::Admin)
    }

    /// Check if admin is already initialized
    pub fn has_admin(env: &Env) -> bool {
        env.storage().instance().has(&DataKey::Admin)
    }

    // ========== HUMANS ==========

    /// Save human to storage
    pub fn set_human(env: &Env, address: &Address, human: &Human) {
        let key = DataKey::Human(address.clone());
        
        // If human doesn't exist, increment counter and add to index
        if !env.storage().persistent().has(&key) {
            let count = Self::get_human_count(env);
            env.storage()
                .persistent()
                .set(&DataKey::HumanIndex(count), address);
            env.storage().persistent().set(&DataKey::HumanCount, &(count + 1));
        }
        
        env.storage().persistent().set(&key, human);
    }

    /// Get human from storage
    pub fn get_human(env: &Env, address: &Address) -> Option<Human> {
        env.storage()
            .persistent()
            .get(&DataKey::Human(address.clone()))
    }

    /// Check if human exists
    pub fn has_human(env: &Env, address: &Address) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::Human(address.clone()))
    }

    /// Get total human count
    pub fn get_human_count(env: &Env) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::HumanCount)
            .unwrap_or(0)
    }

    /// Get all humans (paginated)
    pub fn get_all_humans(env: &Env, start: u32, limit: u32) -> Vec<Human> {
        let mut humans = Vec::new(env);
        let total = Self::get_human_count(env);
        let end = core::cmp::min(start + limit, total);

        for i in start..end {
            if let Some(address) = env
                .storage()
                .persistent()
                .get::<DataKey, Address>(&DataKey::HumanIndex(i))
            {
                if let Some(human) = Self::get_human(env, &address) {
                    humans.push_back(human);
                }
            }
        }

        humans
    }

    // ========== EVENTS ==========

    /// Save event to storage
    pub fn set_event(env: &Env, event_id: &String, event: &Event) {
        env.storage()
            .persistent()
            .set(&DataKey::Event(event_id.clone()), event);
    }

    /// Get event from storage
    pub fn get_event(env: &Env, event_id: &String) -> Option<Event> {
        env.storage()
            .persistent()
            .get(&DataKey::Event(event_id.clone()))
    }

    /// Check if event exists
    pub fn has_event(env: &Env, event_id: &String) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::Event(event_id.clone()))
    }
}
