#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_initialize_token() {
    // Setup environment
    let env = Env::default();
    let contract_id = env.register(TokenContract, ());
    let client = TokenContractClient::new(&env, &contract_id);

    // Create admin address
    let admin = Address::generate(&env);

    // Mock authorization
    env.mock_all_auths();

    // Initialize token
    let name = String::from_str(&env, "Indonesian Rupiah");
    let symbol = String::from_str(&env, "IDR");
    let supply = 1_000_000_000i128; // 1 miliar

    client.initialize(&admin, &name, &symbol, &supply);

    // Verify token info
    assert_eq!(client.get_name(), name);
    assert_eq!(client.get_symbol(), symbol);
    assert_eq!(client.get_total_supply(), supply);
    assert_eq!(client.get_balance(&admin), supply);
}

#[test]
fn test_get_token_info() {
    let env = Env::default();
    let contract_id = env.register(TokenContract, ());
    let client = TokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    env.mock_all_auths();

    let name = String::from_str(&env, "Workshop Token");
    let symbol = String::from_str(&env, "WST");
    let supply = 5_000_000i128;

    client.initialize(&admin, &name, &symbol, &supply);

    // Test getter functions
    assert_eq!(client.get_name(), name);
    assert_eq!(client.get_symbol(), symbol);
    assert_eq!(client.get_total_supply(), supply);
}

#[test]
#[should_panic(expected = "Total supply harus lebih dari 0")]
fn test_initialize_invalid_supply() {
    let env = Env::default();
    let contract_id = env.register(TokenContract, ());
    let client = TokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    env.mock_all_auths();

    let name = String::from_str(&env, "Bad Token");
    let symbol = String::from_str(&env, "BAD");
    let supply = 0i128; // Invalid!

    // Should panic
    client.initialize(&admin, &name, &symbol, &supply);
}

#[test]
fn test_transfer() {
    let env = Env::default();
    let contract_id = env.register(TokenContract, ());
    let client = TokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    env.mock_all_auths();

    let name = String::from_str(&env, "Test Token");
    let symbol = String::from_str(&env, "TST");
    let supply = 1_000_000i128;

    client.initialize(&admin, &name, &symbol, &supply);

    // Transfer tokens
    let transfer_amount = 100_000i128;
    client.transfer(&admin, &user, &transfer_amount);

    // Balance should be reduced
    assert_eq!(client.get_balance(&admin), supply - transfer_amount);
    assert_eq!(client.get_balance(&user), transfer_amount);
}

#[test]
fn test_mint_burn_and_allowance() {
    let env = Env::default();
    let contract_id = env.register(TokenContract, ());
    let client = TokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    env.mock_all_auths();

    let name = String::from_str(&env, "Test Token");
    let symbol = String::from_str(&env, "TST");
    let supply = 1_000_000i128;

    client.initialize(&admin, &name, &symbol, &supply);

    // Mint to Alice
    let mint_amount = 10_000i128;
    client.mint(&admin, &alice, &mint_amount);
    assert_eq!(client.get_balance(&alice), mint_amount);
    assert_eq!(client.get_total_supply(), supply + mint_amount);

    // Alice approves Bob
    let approve_amount = 5_000i128;
    client.approve(&alice, &bob, &approve_amount);
    assert_eq!(client.allowance(&alice, &bob), approve_amount);

    // Bob transfers from Alice to admin
    let transfer_from_amount = 3_000i128;
    client.transfer_from(&bob, &alice, &admin, &transfer_from_amount);
    assert_eq!(client.get_balance(&alice), mint_amount - transfer_from_amount);
    assert_eq!(client.get_balance(&admin), supply + transfer_from_amount);
    assert_eq!(client.allowance(&alice, &bob), approve_amount - transfer_from_amount);

    // Burn some admin tokens
    let burn_amount = 1_000i128;
    client.burn(&admin, &burn_amount);
    assert_eq!(client.get_total_supply(), supply + mint_amount - burn_amount);
    assert_eq!(client.get_balance(&admin), supply + transfer_from_amount - burn_amount);
}