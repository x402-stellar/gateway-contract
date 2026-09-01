#![cfg(test)]

use settlement_verifier::{Error, SettlementVerifier, SettlementVerifierClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::token::{Client as TokenClient, StellarAssetClient};
use soroban_sdk::{vec, Address, Env};

fn setup_token<'a>(env: &Env, admin: &Address) -> (Address, StellarAssetClient<'a>, TokenClient<'a>) {
    let token_contract = env.register_stellar_asset_contract_v2(admin.clone());
    let token_client = TokenClient::new(env, &token_contract.address());
    let stellar_client = StellarAssetClient::new(env, &token_contract.address());
    (token_contract.address(), stellar_client, token_client)
}

#[test]
fn test_verify_and_split_happy_path() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SettlementVerifier, ());
    let client = SettlementVerifierClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let fee_recipient = Address::generate(&env);
    let model_creator = Address::generate(&env);
    let compute_host = Address::generate(&env);
    let gateway_operator = Address::generate(&env);
    let payer = Address::generate(&env);

    client.initialize(&admin, &0, &fee_recipient);

    let (token_addr, stellar_client, token_client) = setup_token(&env, &admin);
    stellar_client.mint(&payer, &100_000_000); // $10.00

    // Split: 70% model, 20% compute, 10% gateway
    let recipients = vec![
        &env,
        (model_creator.clone(), 7_000u32),
        (compute_host.clone(), 2_000u32),
        (gateway_operator.clone(), 1_000u32),
    ];

    client.verify_and_split(&payer, &token_addr, &100_000_000, &recipients, &1);

    assert_eq!(token_client.balance(&model_creator), 70_000_000);
    assert_eq!(token_client.balance(&compute_host), 20_000_000);
    assert_eq!(token_client.balance(&gateway_operator), 10_000_000);
    assert_eq!(token_client.balance(&payer), 0);
    assert_eq!(client.get_nonce(&payer), 1);
}

#[test]
fn test_verify_and_split_invalid_bps_rejected() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SettlementVerifier, ());
    let client = SettlementVerifierClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let fee_recipient = Address::generate(&env);
    let r1 = Address::generate(&env);
    let payer = Address::generate(&env);

    client.initialize(&admin, &0, &fee_recipient);
    let (token_addr, _, _) = setup_token(&env, &admin);

    // Sum is only 9000 bps (90%), not 10,000 bps
    let recipients = vec![&env, (r1.clone(), 9_000u32)];

    let res = client.try_verify_and_split(&payer, &token_addr, &100_000_000, &recipients, &1);
    assert_eq!(res, Err(Ok(Error::InvalidSplit)));
}
