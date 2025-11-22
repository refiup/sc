#![no_std]

//! # Event Distributor Contract
//!
//! Smart contract for managing validated participants (Humans) and distributing
//! funds from event pools to verified recipients.
//!
//! ## Features
//! - On-chain Human registry with validation status
//! - Event management with associated participants
//! - Automatic distribution to validated participants only
//! - IPFS-based image storage for efficiency
//! - Comprehensive event emission for auditability

use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

mod errors;
mod events;
mod models;
mod storage;

use errors::ContractError;
use events::Events;
use models::{Event, Human};
use storage::Storage;

/// Main contract struct
#[contract]
pub struct EventDistributor;

#[contractimpl]
impl EventDistributor {
    /// Initialize the contract with an admin address
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `admin` - Admin wallet address
    ///
    /// # Authorization
    /// Requires signature from the admin address
    pub fn init(env: Env, admin: Address) -> Result<(), ContractError> {
        admin.require_auth();

        if Storage::has_admin(&env) {
            return Err(ContractError::AlreadyInitialized);
        }

        Storage::set_admin(&env, &admin);
        Events::admin_set(&env, &admin);

        Ok(())
    }

    /// Get the admin address
    pub fn get_admin(env: Env) -> Result<Address, ContractError> {
        Storage::get_admin(&env).ok_or(ContractError::AdminNotFound)
    }

    // ========== HUMAN MANAGEMENT ==========

    /// Add a new human to the registry
    ///
    /// # Arguments
    /// * `address` - Human's wallet address
    /// * `ipfs_hash` - IPFS hash of the human's image (e.g., "Qm...")
    ///
    /// # Authorization
    /// Requires admin authentication
    pub fn add_human(
        env: Env,
        address: Address,
        ipfs_hash: String,
    ) -> Result<(), ContractError> {
        let admin = Storage::get_admin(&env).ok_or(ContractError::AdminNotFound)?;
        admin.require_auth();

        // Check if human already exists
        if Storage::has_human(&env, &address) {
            return Err(ContractError::HumanAlreadyExists);
        }

        let human = Human {
            address: address.clone(),
            validated: false,
            ipfs_hash: ipfs_hash.clone(),
        };

        Storage::set_human(&env, &address, &human);
        Events::human_added(&env, &address, &ipfs_hash);

        Ok(())
    }

    /// Update human's validation status
    ///
    /// # Arguments
    /// * `address` - Human's wallet address
    /// * `validated` - New validation status
    ///
    /// # Authorization
    /// Requires admin authentication
    pub fn update_human_validation(
        env: Env,
        address: Address,
        validated: bool,
    ) -> Result<(), ContractError> {
        let admin = Storage::get_admin(&env).ok_or(ContractError::AdminNotFound)?;
        admin.require_auth();

        let mut human = Storage::get_human(&env, &address).ok_or(ContractError::HumanNotFound)?;
        human.validated = validated;

        Storage::set_human(&env, &address, &human);
        Events::human_validation_updated(&env, &address, validated);

        Ok(())
    }

    /// Update human's IPFS image hash
    ///
    /// # Arguments
    /// * `address` - Human's wallet address
    /// * `ipfs_hash` - New IPFS hash
    ///
    /// # Authorization
    /// Requires admin authentication
    pub fn update_human_image(
        env: Env,
        address: Address,
        ipfs_hash: String,
    ) -> Result<(), ContractError> {
        let admin = Storage::get_admin(&env).ok_or(ContractError::AdminNotFound)?;
        admin.require_auth();

        let mut human = Storage::get_human(&env, &address).ok_or(ContractError::HumanNotFound)?;
        human.ipfs_hash = ipfs_hash.clone();

        Storage::set_human(&env, &address, &human);
        Events::human_image_updated(&env, &address, &ipfs_hash);

        Ok(())
    }

    /// Get human information
    pub fn get_human(env: Env, address: Address) -> Result<Human, ContractError> {
        Storage::get_human(&env, &address).ok_or(ContractError::HumanNotFound)
    }

    /// Get all humans (paginated)
    pub fn get_all_humans(env: Env, start: u32, limit: u32) -> Vec<Human> {
        Storage::get_all_humans(&env, start, limit)
    }

    // ========== EVENT MANAGEMENT ==========

    /// Create a new event
    ///
    /// # Arguments
    /// * `event_id` - Unique identifier for the event
    /// * `location` - Event location
    /// * `event_name` - Name of the event
    /// * `pool` - Total pool amount to distribute
    ///
    /// # Authorization
    /// Requires admin authentication
    pub fn create_event(
        env: Env,
        event_id: String,
        location: String,
        event_name: String,
        pool: i128,
    ) -> Result<(), ContractError> {
        let admin = Storage::get_admin(&env).ok_or(ContractError::AdminNotFound)?;
        admin.require_auth();

        if pool <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        if Storage::has_event(&env, &event_id) {
            return Err(ContractError::EventAlreadyExists);
        }

        let event = Event {
            location: location.clone(),
            event_name: event_name.clone(),
            humans: Vec::new(&env),
            pool,
        };

        Storage::set_event(&env, &event_id, &event);
        Events::event_created(&env, &event_id, &location, &event_name, pool);

        Ok(())
    }

    /// Add a human to an event
    ///
    /// # Arguments
    /// * `event_id` - Event identifier
    /// * `human_address` - Address of the human to add
    ///
    /// # Authorization
    /// Requires admin authentication
    pub fn add_human_to_event(
        env: Env,
        event_id: String,
        human_address: Address,
    ) -> Result<(), ContractError> {
        let admin = Storage::get_admin(&env).ok_or(ContractError::AdminNotFound)?;
        admin.require_auth();

        // Verify human exists
        if !Storage::has_human(&env, &human_address) {
            return Err(ContractError::HumanNotFound);
        }

        let mut event = Storage::get_event(&env, &event_id).ok_or(ContractError::EventNotFound)?;

        // Check if human is already in the event
        for addr in event.humans.iter() {
            if addr == human_address {
                return Err(ContractError::HumanAlreadyInEvent);
            }
        }

        event.humans.push_back(human_address.clone());
        Storage::set_event(&env, &event_id, &event);
        Events::human_added_to_event(&env, &event_id, &human_address);

        Ok(())
    }

    /// Get event information
    pub fn get_event(env: Env, event_id: String) -> Result<Event, ContractError> {
        Storage::get_event(&env, &event_id).ok_or(ContractError::EventNotFound)
    }

    /// Get all humans in an event (returns only validated ones)
    pub fn get_event_validated_humans(env: Env, event_id: String) -> Result<Vec<Address>, ContractError> {
        let event = Storage::get_event(&env, &event_id).ok_or(ContractError::EventNotFound)?;
        let mut validated_humans = Vec::new(&env);

        for human_addr in event.humans.iter() {
            if let Some(human) = Storage::get_human(&env, &human_addr) {
                if human.validated {
                    validated_humans.push_back(human_addr);
                }
            }
        }

        Ok(validated_humans)
    }

    // ========== DISTRIBUTION ==========

    /// Distribute event pool to validated humans
    ///
    /// # Arguments
    /// * `event_id` - Event identifier
    /// * `token` - Token contract address
    ///
    /// # Authorization
    /// Requires admin authentication
    ///
    /// # Logic
    /// 1. Get event from storage
    /// 2. Filter only validated humans
    /// 3. Calculate amount per recipient
    /// 4. Distribute tokens equitably
    pub fn distribute_event_pool(
        env: Env,
        event_id: String,
        token: Address,
    ) -> Result<(), ContractError> {
        let admin = Storage::get_admin(&env).ok_or(ContractError::AdminNotFound)?;
        admin.require_auth();

        let event = Storage::get_event(&env, &event_id).ok_or(ContractError::EventNotFound)?;

        // Get validated humans only
        let mut validated_humans = Vec::new(&env);
        for human_addr in event.humans.iter() {
            if let Some(human) = Storage::get_human(&env, &human_addr) {
                if human.validated {
                    validated_humans.push_back(human_addr);
                }
            }
        }

        if validated_humans.is_empty() {
            return Err(ContractError::NoValidatedHumans);
        }

        let recipient_count = validated_humans.len() as i128;
        let amount_per_recipient = event
            .pool
            .checked_div(recipient_count)
            .ok_or(ContractError::MathError)?;

        if amount_per_recipient == 0 {
            return Err(ContractError::ZeroAmountPerRecipient);
        }

        // Execute transfers
        use soroban_sdk::token;
        let contract_address = env.current_contract_address();
        let token_client = token::Client::new(&env, &token);

        for recipient in validated_humans.iter() {
            token_client.transfer(&contract_address, &recipient, &amount_per_recipient);
        }

        Events::distribution_completed(&env, &event_id, event.pool, recipient_count as u32);

        Ok(())
    }
}

#[cfg(test)]
mod test;
