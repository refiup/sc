use soroban_sdk::contracterror;

/// Contract error codes
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    /// Admin already initialized
    AlreadyInitialized = 1,
    /// Admin not found
    AdminNotFound = 2,
    /// Unauthorized: caller is not admin
    Unauthorized = 3,
    /// Recipients list is empty
    EmptyRecipients = 4,
    /// Total amount must be positive
    InvalidAmount = 5,
    /// Amount per recipient is zero
    ZeroAmountPerRecipient = 6,
    /// Division overflow or underflow
    MathError = 7,
}

impl ContractError {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AlreadyInitialized => "Admin already initialized",
            Self::AdminNotFound => "Admin not found",
            Self::Unauthorized => "Unauthorized: caller is not admin",
            Self::EmptyRecipients => "Recipients list cannot be empty",
            Self::InvalidAmount => "Total amount must be positive",
            Self::ZeroAmountPerRecipient => "Amount per recipient must be greater than zero",
            Self::MathError => "Math operation failed",
        }
    }
}
