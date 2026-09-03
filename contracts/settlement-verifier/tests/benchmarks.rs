#![cfg(test)]

use settlement_verifier::{SettlementVerifier, SettlementVerifierClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::token::{Client as TokenClient, StellarAssetClient};
use soroban_sdk::{vec, Address, Env};

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
fn benchmark_settle_payment_resource_consumption() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SettlementVerifier, ());
    let client = SettlementVerifierClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let fee_recipient = Address::generate(&env);
    let merchant = Address::generate(&env);
    let payer = Address::generate(&env);

    client.initialize(&admin, &25, &fee_recipient);

    let (token_addr, stellar_client, _) = setup_token(&env, &admin);
    stellar_client.mint(&payer, &50_000_000);

    let mut budget = env.cost_estimate().budget();
    budget.reset_unlimited();

    let cpu_before = budget.cpu_instruction_cost();
    let mem_before = budget.memory_bytes_cost();

    let net = client.settle_payment(&payer, &merchant, &token_addr, &10_000_000, &1);
    assert_eq!(net, 9_975_000);

    let cpu_used = budget.cpu_instruction_cost() - cpu_before;
    let mem_used = budget.memory_bytes_cost() - mem_before;

    // Verify ultra-low resource consumption thresholds
    assert!(
        cpu_used < 1_500_000,
        "CPU instruction count exceeds threshold: {}",
        cpu_used
    );
    assert!(
        mem_used < 300_000,
        "Memory bytes exceed threshold: {}",
        mem_used
    );

    std::println!("\n================ BENCHMARK METRICS ================");
    std::println!("Operation: settle_payment (single merchant, 25 bps fee)");
    std::println!("CPU Instructions: {}", cpu_used);
    std::println!("Memory Bytes: {}", mem_used);
    std::println!("====================================================\n");
}

#[test]
fn benchmark_verify_and_split_resource_consumption() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SettlementVerifier, ());
    let client = SettlementVerifierClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let fee_recipient = Address::generate(&env);
    let recipient_a = Address::generate(&env);
    let recipient_b = Address::generate(&env);
    let payer = Address::generate(&env);

    client.initialize(&admin, &25, &fee_recipient);

    let (token_addr, stellar_client, _) = setup_token(&env, &admin);
    stellar_client.mint(&payer, &100_000_000);

    let mut budget = env.cost_estimate().budget();
    budget.reset_unlimited();

    let cpu_before = budget.cpu_instruction_cost();
    let mem_before = budget.memory_bytes_cost();

    let splits = vec![
        &env,
        (recipient_a.clone(), 7_000u32),
        (recipient_b.clone(), 3_000u32),
    ];

    client.verify_and_split(&payer, &token_addr, &10_000_000, &splits, &1);

    let cpu_used = budget.cpu_instruction_cost() - cpu_before;
    let mem_used = budget.memory_bytes_cost() - mem_before;

    assert!(
        cpu_used < 2_500_000,
        "CPU instruction count exceeds threshold: {}",
        cpu_used
    );
    assert!(
        mem_used < 500_000,
        "Memory bytes exceed threshold: {}",
        mem_used
    );

    std::println!("\n================ BENCHMARK METRICS ================");
    std::println!("Operation: verify_and_split (2 recipients split 70/30)");
    std::println!("CPU Instructions: {}", cpu_used);
    std::println!("Memory Bytes: {}", mem_used);
    std::println!("====================================================\n");
}

#[test]
fn benchmark_get_nonce_resource_consumption() {
    let env = Env::default();

    let contract_id = env.register(SettlementVerifier, ());
    let client = SettlementVerifierClient::new(&env, &contract_id);

    let payer = Address::generate(&env);

    let mut budget = env.cost_estimate().budget();
    budget.reset_unlimited();

    let cpu_before = budget.cpu_instruction_cost();
    let mem_before = budget.memory_bytes_cost();

    let nonce = client.get_nonce(&payer);
    assert_eq!(nonce, 0);

    let cpu_used = budget.cpu_instruction_cost() - cpu_before;
    let mem_used = budget.memory_bytes_cost() - mem_before;

    assert!(
        cpu_used < 100_000,
        "CPU instruction count exceeds threshold: {}",
        cpu_used
    );
    assert!(
        mem_used < 50_000,
        "Memory bytes exceed threshold: {}",
        mem_used
    );

    std::println!("\n================ BENCHMARK METRICS ================");
    std::println!("Operation: get_nonce (read-only view)");
    std::println!("CPU Instructions: {}", cpu_used);
    std::println!("Memory Bytes: {}", mem_used);
    std::println!("====================================================\n");
}
