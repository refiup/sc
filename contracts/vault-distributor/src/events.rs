use soroban_sdk::{contracttype, Address, Env};

/// Admin set event data
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdminSetEvent {
    pub admin: Address,
}

/// Distribution completed event data
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistributionEvent {
    pub total_amount: i128,
    pub num_recipients: u32,
    pub amount_each: i128,
}

/// Event manager - Single Responsibility Principle
pub struct Events;

impl Events {
    /// Emit admin initialized event
    pub fn emit_admin_set(env: &Env, admin: &Address) {
        env.events().publish(
            ("VaultDistributor", "admin_set"),
            AdminSetEvent {
                admin: admin.clone(),
            },
        );
    }

    /// Emit distribution completed event
    /// 
    /// # Arguments
    /// * `total_amount` - Total amount distributed
    /// * `num_recipients` - Number of recipients
    /// * `amount_each` - Amount per recipient
    pub fn emit_distribution(
        env: &Env,
        total_amount: i128,
        num_recipients: u32,
        amount_each: i128,
    ) {
        env.events().publish(
            ("VaultDistributor", "distribution_completed"),
            DistributionEvent {
                total_amount,
                num_recipients,
                amount_each,
            },
        );
    }
}
