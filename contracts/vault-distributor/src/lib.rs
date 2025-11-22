#![no_std]

//! # Vault Distributor Contract
//!
//! A smart contract for equitable token distribution to multiple recipients.
//!
//! ## Architecture
//! - Modular design following SOLID principles
//! - Separation of concerns across multiple modules
//! - Clean error handling with custom error types
//! - Comprehensive validation layer
//!
//! ## Modules
//! - `auth`: Authentication and authorization
//! - `storage`: Data persistence layer
//! - `validation`: Input validation and business rules
//! - `token_operations`: Token transfer operations
//! - `events`: Event emission
//! - `errors`: Error definitions

use soroban_sdk::{contract, contractimpl, Address, Env, Vec};

mod auth;
mod errors;
mod events;
mod storage;
mod token_operations;
mod validation;

use auth::Auth;
use errors::ContractError;
use events::Events;
use storage::Storage;
use token_operations::TokenOperations;
use validation::Validator;

/// Main contract struct
#[contract]
pub struct VaultDistributor;

#[contractimpl]
impl VaultDistributor {
    /// Initialize the contract with an admin address
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `admin` - Admin wallet address
    ///
    /// # Authorization
    /// Requires signature from the admin address
    ///
    /// # Errors
    /// * `AlreadyInitialized` - If admin is already set
    ///
    /// # Events
    /// Emits `admin_set` event on success
    pub fn init(env: Env, admin: Address) {
        Auth::initialize_admin(&env, &admin)
            .unwrap_or_else(|e| panic!("{}", e.as_str()));

        Events::emit_admin_set(&env, &admin);
    }

    /// Get the registered admin address
    ///
    /// # Returns
    /// Admin address
    ///
    /// # Errors
    /// Panics if admin not found
    pub fn get_admin(env: Env) -> Address {
        Storage::get_admin(&env)
            .unwrap_or_else(|| panic!("{}", ContractError::AdminNotFound.as_str()))
    }

    /// Distribute tokens equitably to multiple recipients
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `token` - Token contract address
    /// * `recipients` - List of recipient addresses
    /// * `total_amount` - Total amount to distribute
    ///
    /// # Authorization
    /// Only admin can execute this function
    ///
    /// # Validation
    /// - Recipients list must not be empty
    /// - Total amount must be positive
    /// - Amount per recipient must be greater than zero
    ///
    /// # Behavior
    /// - Calculates equal distribution amount
    /// - Transfers tokens from contract to each recipient
    /// - Emits distribution event
    ///
    /// # Events
    /// Emits `vault/distributed` event with distribution details
    ///
    /// # Panics
    /// - If caller is not admin
    /// - If validation fails
    /// - If any token transfer fails
    pub fn distribute(env: Env, token: Address, recipients: Vec<Address>, total_amount: i128) {
        // 1. Authorization check
        Auth::require_admin(&env).unwrap_or_else(|e| panic!("{}", e.as_str()));

        // 2. Validate inputs and calculate distribution
        let amount_each = Validator::validate_distribution(&recipients, total_amount)
            .unwrap_or_else(|e| panic!("{}", e.as_str()));

        // 3. Execute token transfers
        TokenOperations::distribute_to_recipients(&env, &token, &recipients, amount_each);

        // 4. Emit success event
        Events::emit_distribution(&env, total_amount, recipients.len(), amount_each);
    }
}

#[cfg(test)]
mod test;
