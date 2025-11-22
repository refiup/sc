use soroban_sdk::{token, Address, Env, Vec};

/// Token operations module - Single Responsibility Principle
pub struct TokenOperations;

impl TokenOperations {
    /// Transfer tokens from contract to a single recipient
    fn transfer_to_recipient(
        env: &Env,
        token: &Address,
        recipient: &Address,
        amount: i128,
    ) {
        let token_client = token::Client::new(env, token);
        token_client.transfer(&env.current_contract_address(), recipient, &amount);
    }

    /// Distribute tokens equally to all recipients
    /// 
    /// # Arguments
    /// * `env` - Contract environment
    /// * `token` - Token contract address
    /// * `recipients` - List of recipient addresses
    /// * `amount_each` - Amount to send to each recipient
    /// 
    /// # Panics
    /// Will panic if any transfer fails
    pub fn distribute_to_recipients(
        env: &Env,
        token: &Address,
        recipients: &Vec<Address>,
        amount_each: i128,
    ) {
        for recipient in recipients.iter() {
            Self::transfer_to_recipient(env, token, &recipient, amount_each);
        }
    }
}
