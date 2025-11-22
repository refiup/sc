use soroban_sdk::{contracttype, Address, Env, String, Vec};

/// Human participant with validation status and IPFS image
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Human {
    /// Wallet address of the human
    pub address: Address,
    /// Validation status (true = can receive distributions)
    pub validated: bool,
    /// IPFS hash of the human's image (e.g., "QmXxxx...")
    pub ipfs_hash: String,
}

/// Event with location, name, participants, and distribution pool
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    /// Event location
    pub location: String,
    /// Event name
    pub event_name: String,
    /// List of human addresses participating in the event
    pub humans: Vec<Address>,
    /// Total pool amount to distribute
    pub pool: i128,
}
