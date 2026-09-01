#![cfg(test)]

use settlement_verifier::{Error, SettlementVerifier, SettlementVerifierClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env};

#[test]
fn test_initialize_happy_path() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SettlementVerifier, ());
    let client = SettlementVerifierClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let fee_recipient = Address::generate(&env);

    client.initialize(&admin, &50, &fee_recipient);

    assert_eq!(client.get_admin(), admin);
    assert_eq!(client.get_fee_config(), (50, fee_recipient));
}

#[test]
fn test_initialize_double_init_rejected() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SettlementVerifier, ());
    let client = SettlementVerifierClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let fee_recipient = Address::generate(&env);

    client.initialize(&admin, &50, &fee_recipient);

    let result = client.try_initialize(&admin, &50, &fee_recipient);
    assert_eq!(result, Err(Ok(Error::AlreadyInitialized)));
}

#[test]
fn test_initialize_excessive_fee_rejected() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SettlementVerifier, ());
    let client = SettlementVerifierClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let fee_recipient = Address::generate(&env);

    let result = client.try_initialize(&admin, &1_001, &fee_recipient);
    assert_eq!(result, Err(Ok(Error::InvalidFeeBps)));
}
