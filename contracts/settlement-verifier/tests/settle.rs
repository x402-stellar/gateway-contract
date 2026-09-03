#![cfg(test)]

use settlement_verifier::{Error, SettlementVerifier, SettlementVerifierClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::token::{Client as TokenClient, StellarAssetClient};
use soroban_sdk::{Address, Env};

fn setup_token<'a>(
    env: &Env,
    admin: &Address,
) -> (Address, StellarAssetClient<'a>, TokenClient<'a>) {
    let token_contract = env.register_stellar_asset_contract_v2(admin.clone());
    let token_client = TokenClient::new(env, &token_contract.address());
    let stellar_client = StellarAssetClient::new(env, &token_contract.address());
    (token_contract.address(), stellar_client, token_client)
}

#[test]
fn test_settle_payment_happy_path() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SettlementVerifier, ());
    let client = SettlementVerifierClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let fee_recipient = Address::generate(&env);
    let merchant = Address::generate(&env);
    let payer = Address::generate(&env);

    client.initialize(&admin, &100, &fee_recipient); // 1.00% fee

    let (token_addr, stellar_client, token_client) = setup_token(&env, &admin);
    stellar_client.mint(&payer, &10_000_000); // $1.00

    let net = client.settle_payment(&payer, &merchant, &token_addr, &10_000_000, &1);

    // 1% of 10,000,000 is 100,000. Net is 9,900,000
    assert_eq!(net, 9_900_000);
    assert_eq!(token_client.balance(&merchant), 9_900_000);
    assert_eq!(token_client.balance(&fee_recipient), 100_000);
    assert_eq!(token_client.balance(&payer), 0);
    assert_eq!(client.get_nonce(&payer), 1);
}

#[test]
fn test_settle_payment_zero_amount_rejected() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SettlementVerifier, ());
    let client = SettlementVerifierClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let fee_recipient = Address::generate(&env);
    let merchant = Address::generate(&env);
    let payer = Address::generate(&env);
    let (token_addr, _, _) = setup_token(&env, &admin);

    client.initialize(&admin, &100, &fee_recipient);

    let res = client.try_settle_payment(&payer, &merchant, &token_addr, &0, &1);
    assert_eq!(res, Err(Ok(Error::InvalidAmount)));
}
