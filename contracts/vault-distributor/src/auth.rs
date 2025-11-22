use soroban_sdk::{Address, Env};

use crate::errors::ContractError;
use crate::storage::Storage;

/// Authentication and authorization module - Single Responsibility Principle
pub struct Auth;

impl Auth {
    /// Verify caller is the admin
    /// 
    /// # Returns
    /// * `Ok(Address)` - Admin address if authorized
    /// * `Err(ContractError)` - If admin not found or unauthorized
    pub fn require_admin(env: &Env) -> Result<Address, ContractError> {
        let admin = Storage::get_admin(env).ok_or(ContractError::AdminNotFound)?;
        
        admin.require_auth();
        
        Ok(admin)
    }

    /// Initialize admin with authentication
    /// 
    /// # Errors
    /// * `AlreadyInitialized` - If admin already set
    pub fn initialize_admin(env: &Env, admin: &Address) -> Result<(), ContractError> {
        if Storage::has_admin(env) {
            return Err(ContractError::AlreadyInitialized);
        }

        admin.require_auth();
        
        Storage::set_admin(env, admin);
        
        Ok(())
    }
}
