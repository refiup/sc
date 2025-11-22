use soroban_sdk::contracterror;

/// Contract error types
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    /// Admin already initialized
    AlreadyInitialized = 1,
    /// Admin not found
    AdminNotFound = 2,
    /// Unauthorized access
    Unauthorized = 3,
    /// Human already exists
    HumanAlreadyExists = 4,
    /// Human not found
    HumanNotFound = 5,
    /// Event already exists
    EventAlreadyExists = 6,
    /// Event not found
    EventNotFound = 7,
    /// Human already in event
    HumanAlreadyInEvent = 8,
    /// No validated humans found
    NoValidatedHumans = 9,
    /// Invalid amount (zero or negative)
    InvalidAmount = 10,
    /// Amount per recipient is zero
    ZeroAmountPerRecipient = 11,
    /// Math error (overflow/underflow)
    MathError = 12,
}

impl ContractError {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContractError::AlreadyInitialized => "Admin already initialized",
            ContractError::AdminNotFound => "Admin not found",
            ContractError::Unauthorized => "Unauthorized access",
            ContractError::HumanAlreadyExists => "Human already exists",
            ContractError::HumanNotFound => "Human not found",
            ContractError::EventAlreadyExists => "Event already exists",
            ContractError::EventNotFound => "Event not found",
            ContractError::HumanAlreadyInEvent => "Human already in event",
            ContractError::NoValidatedHumans => "No validated humans found",
            ContractError::InvalidAmount => "Invalid amount (zero or negative)",
            ContractError::ZeroAmountPerRecipient => "Amount per recipient is zero",
            ContractError::MathError => "Math error (overflow/underflow)",
        }
    }
}
