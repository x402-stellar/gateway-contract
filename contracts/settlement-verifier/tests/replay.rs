#![cfg(test)]

use settlement_verifier::{Error, SettlementVerifier, SettlementVerifierClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::token::{Client as TokenClient, StellarAssetClient};
use soroban_sdk::{Address, Env};

fn setup_token<'a>(env: &Env, admin: &Address) -> (Address, StellarAssetClient<'a>, TokenClient<'a>) {
    let token_contract = env.register_stellar_asset_contract_v2(admin.clone());
    let token_client = TokenClient::new(env, &token_contract.address());
    let stellar_client = StellarAssetClient::new(env, &token_contract.address());
    (token_contract.address(), stellar_client, token_client)
}

#[test]
fn test_replay_attack_rejected() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SettlementVerifier, ());
    let client = SettlementVerifierClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let fee_recipient = Address::generate(&env);
    let merchant = Address::generate(&env);
    let payer = Address::generate(&env);

    client.initialize(&admin, &0, &fee_recipient);

    let (token_addr, stellar_client, _) = setup_token(&env, &admin);
    stellar_client.mint(&payer, &20_000_000);

    // First call with nonce 1 succeeds
    let net = client.settle_payment(&payer, &merchant, &token_addr, &10_000_000, &1);
    assert_eq!(net, 10_000_000);

    // Replay attack with same nonce 1 fails
    let replay_res = client.try_settle_payment(&payer, &merchant, &token_addr, &10_000_000, &1);
    assert_eq!(replay_res, Err(Ok(Error::InvalidNonce)));

    // Skipped nonce (e.g. nonce 3 instead of 2) fails
    let skip_res = client.try_settle_payment(&payer, &merchant, &token_addr, &10_000_000, &3);
    assert_eq!(skip_res, Err(Ok(Error::InvalidNonce)));

    // Next sequential nonce 2 succeeds
    let valid_next = client.settle_payment(&payer, &merchant, &token_addr, &10_000_000, &2);
    assert_eq!(valid_next, 10_000_000);
}
