#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token, vec, Address, Env, IntoVal,
};

#[test]
fn test_init_success() {
    let env = Env::default();
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    // Crear una dirección de admin de prueba
    let admin = Address::generate(&env);

    // Mock de la autenticación
    env.mock_all_auths();

    // Inicializar el contrato
    client.init(&admin);

    // Verificar que el admin fue almacenado correctamente
    let stored_admin = client.get_admin();
    assert_eq!(stored_admin, admin);
}

#[test]
#[should_panic(expected = "Admin already initialized")]
fn test_init_already_initialized() {
    let env = Env::default();
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    env.mock_all_auths();

    // Primera inicialización - debe funcionar
    client.init(&admin);

    // Segunda inicialización - debe fallar con panic
    let admin2 = Address::generate(&env);
    client.init(&admin2);
}

#[test]
#[should_panic(expected = "Admin not found")]
fn test_get_admin_not_found() {
    let env = Env::default();
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    // Intentar obtener admin sin inicializar - debe fallar con panic
    client.get_admin();
}

#[test]
fn test_init_requires_auth() {
    let env = Env::default();
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    // Mockear todas las autenticaciones
    env.mock_all_auths();
    
    client.init(&admin);
    
    // Verificar que se requirió autenticación del admin
    use soroban_sdk::symbol_short;
    assert_eq!(
        env.auths(),
        std::vec![(
            admin.clone(),
            soroban_sdk::testutils::AuthorizedInvocation {
                function: soroban_sdk::testutils::AuthorizedFunction::Contract((
                    contract_id.clone(),
                    symbol_short!("init"),
                    (admin.clone(),).into_val(&env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
}

// ============= Tests para distribute =============

#[test]
fn test_distribute_success() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    // Inicializar contrato
    client.init(&admin);

    // Crear token de prueba
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_id = token_contract.address();
    let token_client = token::StellarAssetClient::new(&env, &token_id);

    // Mintear tokens al contrato
    token_client.mint(&contract_id, &1000);

    // Crear destinatarios
    let recipient1 = Address::generate(&env);
    let recipient2 = Address::generate(&env);
    let recipient3 = Address::generate(&env);
    let recipients = vec![&env, recipient1.clone(), recipient2.clone(), recipient3.clone()];

    // Distribuir 900 tokens (300 cada uno)
    client.distribute(&token_id, &recipients, &900);

    // Verificar balances - cada uno recibe 300 (900 / 3)
    assert_eq!(token_client.balance(&recipient1), 300);
    assert_eq!(token_client.balance(&recipient2), 300);
    assert_eq!(token_client.balance(&recipient3), 300);
    
    // Verificar que el contrato tiene el sobrante
    assert_eq!(token_client.balance(&contract_id), 100);
}

#[test]
#[should_panic(expected = "Recipients list cannot be empty")]
fn test_distribute_empty_recipients() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    client.init(&admin);

    let token_id = Address::generate(&env);
    let recipients = vec![&env]; // Lista vacía

    // Debe fallar
    client.distribute(&token_id, &recipients, &1000);
}

#[test]
#[should_panic(expected = "Total amount must be positive")]
fn test_distribute_invalid_amount() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    client.init(&admin);

    let token_id = Address::generate(&env);
    let recipient = Address::generate(&env);
    let recipients = vec![&env, recipient];

    // Debe fallar con monto 0
    client.distribute(&token_id, &recipients, &0);
}

#[test]
#[should_panic(expected = "Total amount must be positive")]
fn test_distribute_negative_amount() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    client.init(&admin);

    let token_id = Address::generate(&env);
    let recipient = Address::generate(&env);
    let recipients = vec![&env, recipient];

    // Debe fallar con monto negativo
    client.distribute(&token_id, &recipients, &-100);
}

#[test]
#[should_panic(expected = "Amount per recipient must be greater than zero")]
fn test_distribute_amount_too_small() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    client.init(&admin);

    let token_id = Address::generate(&env);
    let recipient1 = Address::generate(&env);
    let recipient2 = Address::generate(&env);
    let recipient3 = Address::generate(&env);
    let recipients = vec![&env, recipient1, recipient2, recipient3];

    // 2 / 3 = 0, debe fallar
    client.distribute(&token_id, &recipients, &2);
}

#[test]
fn test_distribute_requires_admin_auth() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);

    client.init(&admin);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_id = token_contract.address();
    let token_client = token::StellarAssetClient::new(&env, &token_id);
    token_client.mint(&contract_id, &1000);

    let recipient = Address::generate(&env);
    let recipients = vec![&env, recipient];

    client.distribute(&token_id, &recipients, &500);

    // Verificar que el admin fue autenticado
    let auths = env.auths();
    let admin_auth = auths.iter().find(|(addr, _)| addr == &admin);
    assert!(admin_auth.is_some());
}

// ============= Edge Cases Tests =============

#[test]
fn test_distribute_single_recipient() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);
    client.init(&admin);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_id = token_contract.address();
    let token_client = token::StellarAssetClient::new(&env, &token_id);
    token_client.mint(&contract_id, &10000);

    let recipient = Address::generate(&env);
    let recipients = vec![&env, recipient.clone()];

    // Distribuir todo a un solo recipient
    client.distribute(&token_id, &recipients, &5000);

    assert_eq!(token_client.balance(&recipient), 5000);
    assert_eq!(token_client.balance(&contract_id), 5000);
}

#[test]
fn test_distribute_large_number_of_recipients() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);
    client.init(&admin);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_id = token_contract.address();
    let token_client = token::StellarAssetClient::new(&env, &token_id);
    
    // Mintear 10 millones
    token_client.mint(&contract_id, &10_000_000);

    // Crear 10 recipients
    let mut recipients = vec![&env];
    for _ in 0..10 {
        recipients.push_back(Address::generate(&env));
    }

    // Distribuir 1 millón (100k cada uno)
    client.distribute(&token_id, &recipients, &1_000_000);

    // Verificar cada uno recibe 100k
    for recipient in recipients.iter() {
        assert_eq!(token_client.balance(&recipient), 100_000);
    }
}

#[test]
fn test_distribute_exact_division() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);
    client.init(&admin);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_id = token_contract.address();
    let token_client = token::StellarAssetClient::new(&env, &token_id);
    token_client.mint(&contract_id, &9000);

    let r1 = Address::generate(&env);
    let r2 = Address::generate(&env);
    let r3 = Address::generate(&env);
    let recipients = vec![&env, r1.clone(), r2.clone(), r3.clone()];

    // 9000 / 3 = 3000 exacto
    client.distribute(&token_id, &recipients, &9000);

    assert_eq!(token_client.balance(&r1), 3000);
    assert_eq!(token_client.balance(&r2), 3000);
    assert_eq!(token_client.balance(&r3), 3000);
    assert_eq!(token_client.balance(&contract_id), 0);
}

#[test]
fn test_distribute_with_remainder() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);
    client.init(&admin);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_id = token_contract.address();
    let token_client = token::StellarAssetClient::new(&env, &token_id);
    token_client.mint(&contract_id, &10000);

    let r1 = Address::generate(&env);
    let r2 = Address::generate(&env);
    let r3 = Address::generate(&env);
    let recipients = vec![&env, r1.clone(), r2.clone(), r3.clone()];

    // 1000 / 3 = 333, remainder 1
    client.distribute(&token_id, &recipients, &1000);

    assert_eq!(token_client.balance(&r1), 333);
    assert_eq!(token_client.balance(&r2), 333);
    assert_eq!(token_client.balance(&r3), 333);
    // Contrato retiene el remainder
    assert_eq!(token_client.balance(&contract_id), 9001);
}

#[test]
fn test_distribute_minimum_amount() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);
    client.init(&admin);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_id = token_contract.address();
    let token_client = token::StellarAssetClient::new(&env, &token_id);
    token_client.mint(&contract_id, &10);

    let recipient = Address::generate(&env);
    let recipients = vec![&env, recipient.clone()];

    // Mínimo: 1 token
    client.distribute(&token_id, &recipients, &1);

    assert_eq!(token_client.balance(&recipient), 1);
}

#[test]
fn test_distribute_multiple_times() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);
    client.init(&admin);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_id = token_contract.address();
    let token_client = token::StellarAssetClient::new(&env, &token_id);
    token_client.mint(&contract_id, &10000);

    let r1 = Address::generate(&env);
    let r2 = Address::generate(&env);
    let recipients = vec![&env, r1.clone(), r2.clone()];

    // Primera distribución: 2000
    client.distribute(&token_id, &recipients, &2000);
    assert_eq!(token_client.balance(&r1), 1000);
    assert_eq!(token_client.balance(&r2), 1000);

    // Segunda distribución: 4000
    client.distribute(&token_id, &recipients, &4000);
    assert_eq!(token_client.balance(&r1), 3000);
    assert_eq!(token_client.balance(&r2), 3000);

    // Balance restante
    assert_eq!(token_client.balance(&contract_id), 4000);
}

#[test]
fn test_distribute_same_recipient_multiple_times() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);
    client.init(&admin);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_id = token_contract.address();
    let token_client = token::StellarAssetClient::new(&env, &token_id);
    token_client.mint(&contract_id, &10000);

    let recipient = Address::generate(&env);
    // Mismo recipient aparece 3 veces en la lista
    let recipients = vec![&env, recipient.clone(), recipient.clone(), recipient.clone()];

    // 900 / 3 = 300, pero se envía 3 veces al mismo
    client.distribute(&token_id, &recipients, &900);

    // Recipient debe recibir 300 * 3 = 900
    assert_eq!(token_client.balance(&recipient), 900);
}

#[test]
fn test_distribute_max_i128_amount() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);
    client.init(&admin);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_id = token_contract.address();
    let token_client = token::StellarAssetClient::new(&env, &token_id);
    
    // Mintear cantidad muy grande
    let large_amount: i128 = 1_000_000_000_000_000;
    token_client.mint(&contract_id, &large_amount);

    let r1 = Address::generate(&env);
    let r2 = Address::generate(&env);
    let recipients = vec![&env, r1.clone(), r2.clone()];

    client.distribute(&token_id, &recipients, &large_amount);

    assert_eq!(token_client.balance(&r1), large_amount / 2);
    assert_eq!(token_client.balance(&r2), large_amount / 2);
}

// ============= Tests de reinicialización =============

#[test]
fn test_admin_persists_after_operations() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register(VaultDistributor, ());
    let client = VaultDistributorClient::new(&env, &contract_id);
    client.init(&admin);

    // Verificar admin antes de operaciones
    assert_eq!(client.get_admin(), admin);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_id = token_contract.address();
    let token_client = token::StellarAssetClient::new(&env, &token_id);
    token_client.mint(&contract_id, &10000);

    let recipient = Address::generate(&env);
    let recipients = vec![&env, recipient];

    // Ejecutar distribución
    client.distribute(&token_id, &recipients, &1000);

    // Verificar que admin sigue siendo el mismo
    assert_eq!(client.get_admin(), admin);
}
