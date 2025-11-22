use soroban_sdk::{contracttype, Address, Env};

/// Storage keys for contract data
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
}

/// Storage manager - Single Responsibility Principle
pub struct Storage;

impl Storage {
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
}
