use soroban_sdk::{Address, Vec};

use crate::errors::ContractError;

/// Input validation module - Single Responsibility Principle
pub struct Validator;

impl Validator {
    /// Validate recipients list is not empty
    pub fn validate_recipients(recipients: &Vec<Address>) -> Result<(), ContractError> {
        if recipients.is_empty() {
            return Err(ContractError::EmptyRecipients);
        }
        Ok(())
    }

    /// Validate total amount is positive
    pub fn validate_amount(total_amount: i128) -> Result<(), ContractError> {
        if total_amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }
        Ok(())
    }

    /// Validate and calculate amount per recipient
    /// Returns the amount each recipient should receive
    pub fn calculate_amount_per_recipient(
        total_amount: i128,
        num_recipients: u32,
    ) -> Result<i128, ContractError> {
        if num_recipients == 0 {
            return Err(ContractError::EmptyRecipients);
        }

        let amount_each = total_amount
            .checked_div(num_recipients as i128)
            .ok_or(ContractError::MathError)?;

        if amount_each <= 0 {
            return Err(ContractError::ZeroAmountPerRecipient);
        }

        Ok(amount_each)
    }

    /// Validate all distribution parameters
    pub fn validate_distribution(
        recipients: &Vec<Address>,
        total_amount: i128,
    ) -> Result<i128, ContractError> {
        Self::validate_recipients(recipients)?;
        Self::validate_amount(total_amount)?;
        Self::calculate_amount_per_recipient(total_amount, recipients.len())
    }
}
