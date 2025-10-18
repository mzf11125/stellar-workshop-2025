#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, symbol_short};

// Storage keys
const BALANCE: soroban_sdk::Symbol = symbol_short!("BALANCE");
const NAME: soroban_sdk::Symbol = symbol_short!("NAME");
const SYMBOL: soroban_sdk::Symbol = symbol_short!("SYMBOL");
const TOTAL: soroban_sdk::Symbol = symbol_short!("TOTAL");
const ALLOWANCE: soroban_sdk::Symbol = symbol_short!("ALLOWANCE");

// Contract struct
#[contract]
pub struct TokenContract;

// Token data structure
#[contracttype]
#[derive(Clone)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub total_supply: i128,
}

#[contractimpl]
impl TokenContract {

    // Initialize token dengan nama, symbol, dan supply
    pub fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        total_supply: i128,
    ) {
        // Verify admin authorization
        admin.require_auth();

        // Validasi input
        if total_supply <= 0 {
            panic!("Total supply harus lebih dari 0");
        }

        // Simpan token info
        env.storage().instance().set(&NAME, &name);
        env.storage().instance().set(&SYMBOL, &symbol);
        env.storage().instance().set(&TOTAL, &total_supply);

        // Set balance admin = total supply (per-address storage)
        env.storage()
            .instance()
            .set(&(&BALANCE, admin.clone()), &total_supply);
    }

    // Get nama token
    pub fn get_name(env: Env) -> String {
        env.storage().instance().get(&NAME).unwrap()
    }

    // Get symbol token
    pub fn get_symbol(env: Env) -> String {
        env.storage().instance().get(&SYMBOL).unwrap()
    }

    // Get total supply
    pub fn get_total_supply(env: Env) -> i128 {
        env.storage().instance().get(&TOTAL).unwrap()
    }

    // Get balance for an address
    pub fn get_balance(env: Env, addr: Address) -> i128 {
        env.storage()
            .instance()
            .get(&(&BALANCE, addr))
            .unwrap_or(0)
    }

    // Transfer token (simplified - real token contract lebih kompleks)
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        // Verify authorization
        from.require_auth();

        // Validasi amount
        if amount <= 0 {
            panic!("Amount harus lebih dari 0");
        }

        // Get balances
    let from_balance: i128 = env.storage().instance().get(&(&BALANCE, from.clone())).unwrap_or(0);
    let to_balance: i128 = env.storage().instance().get(&(&BALANCE, to.clone())).unwrap_or(0);

        // Check sufficient balance
        if from_balance < amount {
            panic!("Balance tidak cukup");
        }

        // Update balances
        env.storage()
            .instance()
            .set(&(&BALANCE, from), &(from_balance - amount));
        env.storage()
            .instance()
            .set(&(&BALANCE, to), &(to_balance + amount));
    }

    // Mint new tokens to an address. Only callable by admin (caller must authorize).
    pub fn mint(env: Env, admin: Address, to: Address, amount: i128) {
        admin.require_auth();

        if amount <= 0 {
            panic!("Amount harus lebih dari 0");
        }

        // increase total supply
        let total: i128 = env.storage().instance().get(&TOTAL).unwrap_or(0);
        let new_total = total.checked_add(amount).expect("Total overflow");
        env.storage().instance().set(&TOTAL, &new_total);

        // increase recipient balance
        let bal: i128 = env.storage().instance().get(&(&BALANCE, to.clone())).unwrap_or(0);
        env.storage()
            .instance()
            .set(&(&BALANCE, to), &(bal + amount));
    }

    // Burn tokens from caller's address
    pub fn burn(env: Env, owner: Address, amount: i128) {
        owner.require_auth();

        if amount <= 0 {
            panic!("Amount harus lebih dari 0");
        }

    let bal: i128 = env.storage().instance().get(&(&BALANCE, owner.clone())).unwrap_or(0);
        if bal < amount {
            panic!("Balance tidak cukup untuk burn");
        }

        env.storage()
            .instance()
            .set(&(&BALANCE, owner.clone()), &(bal - amount));

        // decrease total supply
        let total: i128 = env.storage().instance().get(&TOTAL).unwrap_or(0);
        env.storage().instance().set(&TOTAL, &(total - amount));
    }

    // Approve spender to spend owner's tokens
    pub fn approve(env: Env, owner: Address, spender: Address, amount: i128) {
        owner.require_auth();

        if amount < 0 {
            panic!("Amount tidak boleh negatif");
        }

        env.storage()
            .instance()
            .set(&(&ALLOWANCE, owner.clone(), spender.clone()), &amount);
    }

    // Get allowance
    pub fn allowance(env: Env, owner: Address, spender: Address) -> i128 {
        env.storage()
            .instance()
            .get(&(&ALLOWANCE, owner, spender))
            .unwrap_or(0)
    }

    // Transfer from owner's account by approved spender
    pub fn transfer_from(env: Env, spender: Address, owner: Address, to: Address, amount: i128) {
        spender.require_auth();

        if amount <= 0 {
            panic!("Amount harus lebih dari 0");
        }

    let mut allowed: i128 = env.storage().instance().get(&(&ALLOWANCE, owner.clone(), spender.clone())).unwrap_or(0);
        if allowed < amount {
            panic!("Allowance tidak cukup");
        }

        // Deduct allowance
        allowed = allowed - amount;
        env.storage()
            .instance()
            .set(&(&ALLOWANCE, owner.clone(), spender.clone()), &allowed);

        // Move funds
    let owner_bal: i128 = env.storage().instance().get(&(&BALANCE, owner.clone())).unwrap_or(0);
        if owner_bal < amount {
            panic!("Balance owner tidak cukup");
        }

    let to_bal: i128 = env.storage().instance().get(&(&BALANCE, to.clone())).unwrap_or(0);

        env.storage()
            .instance()
            .set(&(&BALANCE, owner), &(owner_bal - amount));
        env.storage()
            .instance()
            .set(&(&BALANCE, to), &(to_bal + amount));
    }
}

mod test;