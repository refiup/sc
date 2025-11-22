#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, token, Address, Env, String};

fn create_test_env() -> (Env, Address, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    env.budget().reset_unlimited();

    let admin = Address::generate(&env);
    let human1 = Address::generate(&env);
    let human2 = Address::generate(&env);
    let human3 = Address::generate(&env);

    (env, admin, human1, human2, human3)
}

fn setup_contract(env: &Env, admin: &Address) -> Address {
    let contract_id = env.register_contract(None, EventDistributor);
    let client = EventDistributorClient::new(env, &contract_id);
    client.init(admin);
    contract_id
}

// ========== INIT TESTS ==========

#[test]
fn test_init_success() {
    let (env, admin, _, _, _) = create_test_env();
    let client = EventDistributorClient::new(&env, &env.register_contract(None, EventDistributor));
    
    client.init(&admin);
    
    let stored_admin = client.get_admin();
    assert_eq!(stored_admin, admin);
}

#[test]
#[should_panic]
fn test_init_already_initialized() {
    let (env, admin, _, _, _) = create_test_env();
    let client = EventDistributorClient::new(&env, &env.register_contract(None, EventDistributor));
    
    client.init(&admin);
    client.init(&admin); // Should panic
}

#[test]
#[should_panic]
fn test_get_admin_not_initialized() {
    let (env, _, _, _, _) = create_test_env();
    let client = EventDistributorClient::new(&env, &env.register_contract(None, EventDistributor));
    
    client.get_admin(); // Should panic
}

// ========== HUMAN MANAGEMENT TESTS ==========

#[test]
fn test_add_human_success() {
    let (env, admin, human1, _, _) = create_test_env();
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    let ipfs_hash = String::from_str(&env, "QmTest123");
    client.add_human(&human1, &ipfs_hash);
    
    let stored_human = client.get_human(&human1);
    assert_eq!(stored_human.address, human1);
    assert_eq!(stored_human.validated, false);
    assert_eq!(stored_human.ipfs_hash, ipfs_hash);
}

#[test]
#[should_panic]
fn test_add_human_already_exists() {
    let (env, admin, human1, _, _) = create_test_env();
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    let ipfs_hash = String::from_str(&env, "QmTest123");
    client.add_human(&human1, &ipfs_hash);
    client.add_human(&human1, &ipfs_hash); // Should panic
}

#[test]
fn test_update_human_validation() {
    let (env, admin, human1, _, _) = create_test_env();
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    let ipfs_hash = String::from_str(&env, "QmTest123");
    client.add_human(&human1, &ipfs_hash);
    
    client.update_human_validation(&human1, &true);
    
    let stored_human = client.get_human(&human1);
    assert_eq!(stored_human.validated, true);
}

#[test]
#[should_panic]
fn test_update_validation_nonexistent_human() {
    let (env, admin, human1, _, _) = create_test_env();
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    client.update_human_validation(&human1, &true); // Should panic
}

#[test]
fn test_update_human_image() {
    let (env, admin, human1, _, _) = create_test_env();
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    let ipfs_hash1 = String::from_str(&env, "QmTest123");
    client.add_human(&human1, &ipfs_hash1);
    
    let ipfs_hash2 = String::from_str(&env, "QmNewImage456");
    client.update_human_image(&human1, &ipfs_hash2);
    
    let stored_human = client.get_human(&human1);
    assert_eq!(stored_human.ipfs_hash, ipfs_hash2);
}

#[test]
#[should_panic]
fn test_get_human_not_found() {
    let (env, admin, human1, _, _) = create_test_env();
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    client.get_human(&human1); // Should panic
}

#[test]
fn test_get_all_humans() {
    let (env, admin, human1, human2, human3) = create_test_env();
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    client.add_human(&human1, &String::from_str(&env, "QmTest1"));
    client.add_human(&human2, &String::from_str(&env, "QmTest2"));
    client.add_human(&human3, &String::from_str(&env, "QmTest3"));
    
    let all_humans = client.get_all_humans(&0, &10);
    assert_eq!(all_humans.len(), 3);
}

// ========== EVENT MANAGEMENT TESTS ==========

#[test]
fn test_create_event_success() {
    let (env, admin, _, _, _) = create_test_env();
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    let event_id = String::from_str(&env, "event_001");
    let location = String::from_str(&env, "Buenos Aires");
    let event_name = String::from_str(&env, "EthGlobal BA 2025");
    let pool = 1000000000i128; // 100 XLM
    
    client.create_event(&event_id, &location, &event_name, &pool);
    
    let stored_event = client.get_event(&event_id);
    assert_eq!(stored_event.location, location);
    assert_eq!(stored_event.event_name, event_name);
    assert_eq!(stored_event.pool, pool);
    assert_eq!(stored_event.humans.len(), 0);
}

#[test]
#[should_panic]
fn test_create_event_invalid_pool() {
    let (env, admin, _, _, _) = create_test_env();
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    let event_id = String::from_str(&env, "event_001");
    client.create_event(
        &event_id,
        &String::from_str(&env, "Location"),
        &String::from_str(&env, "Event"),
        &0, // Invalid pool
    );
}

#[test]
#[should_panic]
fn test_create_event_already_exists() {
    let (env, admin, _, _, _) = create_test_env();
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    let event_id = String::from_str(&env, "event_001");
    client.create_event(
        &event_id,
        &String::from_str(&env, "Location"),
        &String::from_str(&env, "Event"),
        &1000,
    );
    client.create_event(
        &event_id,
        &String::from_str(&env, "Location2"),
        &String::from_str(&env, "Event2"),
        &2000,
    ); // Should panic
}

#[test]
fn test_add_human_to_event() {
    let (env, admin, human1, _, _) = create_test_env();
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    // Create human and event
    client.add_human(&human1, &String::from_str(&env, "QmTest1"));
    let event_id = String::from_str(&env, "event_001");
    client.create_event(
        &event_id,
        &String::from_str(&env, "Location"),
        &String::from_str(&env, "Event"),
        &1000,
    );
    
    // Add human to event
    client.add_human_to_event(&event_id, &human1);
    
    let stored_event = client.get_event(&event_id);
    assert_eq!(stored_event.humans.len(), 1);
    assert_eq!(stored_event.humans.get(0).unwrap(), human1);
}

#[test]
#[should_panic]
fn test_add_nonexistent_human_to_event() {
    let (env, admin, human1, _, _) = create_test_env();
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    let event_id = String::from_str(&env, "event_001");
    client.create_event(
        &event_id,
        &String::from_str(&env, "Location"),
        &String::from_str(&env, "Event"),
        &1000,
    );
    
    client.add_human_to_event(&event_id, &human1); // Human doesn't exist, should panic
}

#[test]
#[should_panic]
fn test_add_human_to_nonexistent_event() {
    let (env, admin, human1, _, _) = create_test_env();
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    client.add_human(&human1, &String::from_str(&env, "QmTest1"));
    
    let event_id = String::from_str(&env, "nonexistent");
    client.add_human_to_event(&event_id, &human1); // Should panic
}

#[test]
#[should_panic]
fn test_add_human_to_event_twice() {
    let (env, admin, human1, _, _) = create_test_env();
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    client.add_human(&human1, &String::from_str(&env, "QmTest1"));
    let event_id = String::from_str(&env, "event_001");
    client.create_event(
        &event_id,
        &String::from_str(&env, "Location"),
        &String::from_str(&env, "Event"),
        &1000,
    );
    
    client.add_human_to_event(&event_id, &human1);
    client.add_human_to_event(&event_id, &human1); // Should panic
}

#[test]
fn test_get_event_validated_humans() {
    let (env, admin, human1, human2, human3) = create_test_env();
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    // Add humans
    client.add_human(&human1, &String::from_str(&env, "QmTest1"));
    client.add_human(&human2, &String::from_str(&env, "QmTest2"));
    client.add_human(&human3, &String::from_str(&env, "QmTest3"));
    
    // Validate only human1 and human3
    client.update_human_validation(&human1, &true);
    client.update_human_validation(&human3, &true);
    
    // Create event and add all humans
    let event_id = String::from_str(&env, "event_001");
    client.create_event(
        &event_id,
        &String::from_str(&env, "Location"),
        &String::from_str(&env, "Event"),
        &1000,
    );
    client.add_human_to_event(&event_id, &human1);
    client.add_human_to_event(&event_id, &human2);
    client.add_human_to_event(&event_id, &human3);
    
    // Get validated humans (should be 2)
    let validated = client.get_event_validated_humans(&event_id);
    assert_eq!(validated.len(), 2);
}

// ========== DISTRIBUTION TESTS ==========

#[test]
#[should_panic]
fn test_distribute_no_validated_humans() {
    let (env, admin, human1, human2, _) = create_test_env();
    let token_admin = Address::generate(&env);
    let token = env.register_stellar_asset_contract_v2(token_admin.clone());
    
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    // Add humans but don't validate them
    client.add_human(&human1, &String::from_str(&env, "QmTest1"));
    client.add_human(&human2, &String::from_str(&env, "QmTest2"));
    
    // Create event and add humans
    let event_id = String::from_str(&env, "event_001");
    client.create_event(
        &event_id,
        &String::from_str(&env, "Location"),
        &String::from_str(&env, "Event"),
        &1000000000,
    );
    client.add_human_to_event(&event_id, &human1);
    client.add_human_to_event(&event_id, &human2);
    
    // Try to distribute (should fail - no validated humans)
    client.distribute_event_pool(&event_id, &token.address());
}

#[test]
#[should_panic]
fn test_distribute_nonexistent_event() {
    let (env, admin, _, _, _) = create_test_env();
    let token_admin = Address::generate(&env);
    let token = env.register_stellar_asset_contract_v2(token_admin.clone());
    
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    let event_id = String::from_str(&env, "nonexistent");
    client.distribute_event_pool(&event_id, &token.address()); // Should panic
}

#[test]
fn test_distribute_success_single_recipient() {
    let (env, admin, human1, _, _) = create_test_env();
    let token_admin = Address::generate(&env);
    let token = env.register_stellar_asset_contract_v2(token_admin.clone());
    
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    // Mint tokens to contract using StellarAssetClient
    token::StellarAssetClient::new(&env, &token.address()).mint(&contract_id, &1000000000);
    
    // Add and validate human
    client.add_human(&human1, &String::from_str(&env, "QmTest1"));
    client.update_human_validation(&human1, &true);
    
    // Create event
    let event_id = String::from_str(&env, "event_001");
    client.create_event(
        &event_id,
        &String::from_str(&env, "Location"),
        &String::from_str(&env, "Event"),
        &1000000000,
    );
    client.add_human_to_event(&event_id, &human1);
    
    // Distribute
    client.distribute_event_pool(&event_id, &token.address());
    
    // Verify balance
    let balance = token::Client::new(&env, &token.address()).balance(&human1);
    assert_eq!(balance, 1000000000);
}

#[test]
fn test_distribute_success_multiple_recipients() {
    let (env, admin, human1, human2, human3) = create_test_env();
    let token_admin = Address::generate(&env);
    let token = env.register_stellar_asset_contract_v2(token_admin.clone());
    
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    // Mint tokens to contract using StellarAssetClient
    token::StellarAssetClient::new(&env, &token.address()).mint(&contract_id, &1000000000);
    
    // Add and validate humans
    client.add_human(&human1, &String::from_str(&env, "QmTest1"));
    client.add_human(&human2, &String::from_str(&env, "QmTest2"));
    client.add_human(&human3, &String::from_str(&env, "QmTest3"));
    
    client.update_human_validation(&human1, &true);
    client.update_human_validation(&human2, &true);
    // human3 not validated
    
    // Create event
    let event_id = String::from_str(&env, "event_001");
    client.create_event(
        &event_id,
        &String::from_str(&env, "Location"),
        &String::from_str(&env, "Event"),
        &1000000000,
    );
    client.add_human_to_event(&event_id, &human1);
    client.add_human_to_event(&event_id, &human2);
    client.add_human_to_event(&event_id, &human3);
    
    // Distribute
    client.distribute_event_pool(&event_id, &token.address());
    
    // Verify balances (only validated humans receive)
    let token_client = token::Client::new(&env, &token.address());
    assert_eq!(token_client.balance(&human1), 500000000); // 50 XLM each
    assert_eq!(token_client.balance(&human2), 500000000);
    assert_eq!(token_client.balance(&human3), 0); // Not validated
}

#[test]
#[should_panic]
fn test_distribute_amount_too_small() {
    let (env, admin, human1, human2, _) = create_test_env();
    let token_admin = Address::generate(&env);
    let token = env.register_stellar_asset_contract_v2(token_admin.clone());
    
    let contract_id = setup_contract(&env, &admin);
    let client = EventDistributorClient::new(&env, &contract_id);
    
    // Add and validate humans
    client.add_human(&human1, &String::from_str(&env, "QmTest1"));
    client.add_human(&human2, &String::from_str(&env, "QmTest2"));
    client.update_human_validation(&human1, &true);
    client.update_human_validation(&human2, &true);
    
    // Create event with tiny pool
    let event_id = String::from_str(&env, "event_001");
    client.create_event(
        &event_id,
        &String::from_str(&env, "Location"),
        &String::from_str(&env, "Event"),
        &1, // 1 stroops for 2 people = 0 each
    );
    client.add_human_to_event(&event_id, &human1);
    client.add_human_to_event(&event_id, &human2);
    
    // Try to distribute (should fail)
    client.distribute_event_pool(&event_id, &token.address());
}
