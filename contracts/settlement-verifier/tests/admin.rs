#![cfg(test)]

use settlement_verifier::{Error, SettlementVerifier, SettlementVerifierClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env};

#[test]
fn test_admin_update_happy_path() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SettlementVerifier, ());
    let client = SettlementVerifierClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let new_admin = Address::generate(&env);
    let fee_recipient = Address::generate(&env);
    let new_fee_recipient = Address::generate(&env);

    client.initialize(&admin, &50, &fee_recipient);

    client.set_admin(&new_admin);
    assert_eq!(client.get_admin(), new_admin);

    client.set_fee_config(&25, &new_fee_recipient);
    assert_eq!(client.get_fee_config(), (25, new_fee_recipient));
}

#[test]
fn test_admin_excessive_fee_update_rejected() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SettlementVerifier, ());
    let client = SettlementVerifierClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let fee_recipient = Address::generate(&env);

    client.initialize(&admin, &50, &fee_recipient);

    let res = client.try_set_fee_config(&1_500, &fee_recipient);
    assert_eq!(res, Err(Ok(Error::InvalidFeeBps)));
}
