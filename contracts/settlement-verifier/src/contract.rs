use crate::error::Error;
use crate::events::{emit_admin_changed, emit_initialized, emit_settlement, emit_split};
use crate::storage::{
    extend_instance_ttl, get_admin, get_fee_bps, get_fee_recipient, get_nonce, is_initialized,
    set_admin as store_set_admin, set_fee_bps, set_fee_recipient, set_initialized, set_nonce,
};
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::{contract, contractimpl, Address, Env, Vec};

#[contract]
pub struct SettlementVerifier;

#[contractimpl]
impl SettlementVerifier {
    /// Initializes the settlement verifier contract with an admin and protocol fee config.
    pub fn initialize(
        env: Env,
        admin: Address,
        fee_bps: u32,
        fee_recipient: Address,
    ) -> Result<(), Error> {
        admin.require_auth();

        if is_initialized(&env) {
            return Err(Error::AlreadyInitialized);
        }

        if fee_bps > 1_000 {
            // Max protocol fee is 10% (1000 bps)
            return Err(Error::InvalidFeeBps);
        }

        set_initialized(&env);
        store_set_admin(&env, &admin);
        set_fee_bps(&env, fee_bps);
        set_fee_recipient(&env, &fee_recipient);
        extend_instance_ttl(&env);

        emit_initialized(&env, admin, fee_bps, fee_recipient);
        Ok(())
    }

    /// Verifies and settles a single merchant payment with replay protection and protocol fee split.
    pub fn settle_payment(
        env: Env,
        payer: Address,
        merchant: Address,
        token: Address,
        amount: i128,
        nonce: u64,
    ) -> Result<i128, Error> {
        payer.require_auth();

        if !is_initialized(&env) {
            return Err(Error::NotInitialized);
        }

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        // Monotonic Nonce Replay Protection
        let current_nonce = get_nonce(&env, &payer);
        if nonce != current_nonce + 1 {
            return Err(Error::InvalidNonce);
        }

        set_nonce(&env, &payer, nonce);
        extend_instance_ttl(&env);

        let fee_bps = get_fee_bps(&env);
        let fee: i128 = if fee_bps > 0 {
            (amount * fee_bps as i128) / 10_000
        } else {
            0
        };

        let net_amount = amount - fee;
        let token_client = TokenClient::new(&env, &token);

        // Transfer net amount to merchant
        token_client.transfer(&payer, &merchant, &net_amount);

        // Transfer fee to protocol fee recipient if applicable
        if fee > 0 {
            let fee_recipient = get_fee_recipient(&env)?;
            token_client.transfer(&payer, &fee_recipient, &fee);
        }

        emit_settlement(&env, merchant, payer, token, amount, fee, nonce);
        Ok(net_amount)
    }

    /// Verifies and splits a payment across multiple arbitrary recipients with basis point allocation.
    pub fn verify_and_split(
        env: Env,
        payer: Address,
        token: Address,
        amount: i128,
        recipients: Vec<(Address, u32)>,
        nonce: u64,
    ) -> Result<(), Error> {
        payer.require_auth();

        if !is_initialized(&env) {
            return Err(Error::NotInitialized);
        }

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        if recipients.is_empty() {
            return Err(Error::InvalidSplit);
        }

        let current_nonce = get_nonce(&env, &payer);
        if nonce != current_nonce + 1 {
            return Err(Error::InvalidNonce);
        }

        set_nonce(&env, &payer, nonce);
        extend_instance_ttl(&env);

        let mut total_bps: u32 = 0;
        for i in 0..recipients.len() {
            let (_, bps) = recipients.get(i).unwrap();
            total_bps += bps;
        }

        if total_bps != 10_000 {
            return Err(Error::InvalidSplit);
        }

        let token_client = TokenClient::new(&env, &token);
        let mut distributed_total: i128 = 0;
        let recipient_count = recipients.len();

        for i in 0..recipient_count {
            let (recipient, bps) = recipients.get(i).unwrap();
            let share: i128 = (amount * bps as i128) / 10_000;
            if share > 0 {
                token_client.transfer(&payer, &recipient, &share);
                distributed_total += share;
            }
        }

        // Remainder adjustment (if integer division leaves residual wei)
        let remainder = amount - distributed_total;
        if remainder > 0 {
            let (primary_recipient, _) = recipients.get(0).unwrap();
            token_client.transfer(&payer, &primary_recipient, &remainder);
        }

        emit_split(&env, payer, token, amount, recipient_count, nonce);
        Ok(())
    }

    /// Read view: returns the current nonce for a payer account.
    pub fn get_nonce(env: Env, account: Address) -> u64 {
        get_nonce(&env, &account)
    }

    /// Read view: returns the contract admin.
    pub fn get_admin(env: Env) -> Result<Address, Error> {
        get_admin(&env)
    }

    /// Read view: returns the current fee configuration (fee_bps, fee_recipient).
    pub fn get_fee_config(env: Env) -> Result<(u32, Address), Error> {
        let bps = get_fee_bps(&env);
        let recipient = get_fee_recipient(&env)?;
        Ok((bps, recipient))
    }

    /// Admin view: updates the admin address.
    pub fn set_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        let current_admin = get_admin(&env)?;
        current_admin.require_auth();

        store_set_admin(&env, &new_admin);
        extend_instance_ttl(&env);
        emit_admin_changed(&env, current_admin, new_admin);
        Ok(())
    }

    /// Admin view: updates protocol fee parameters.
    pub fn set_fee_config(env: Env, new_fee_bps: u32, new_recipient: Address) -> Result<(), Error> {
        let current_admin = get_admin(&env)?;
        current_admin.require_auth();

        if new_fee_bps > 1_000 {
            return Err(Error::InvalidFeeBps);
        }

        set_fee_bps(&env, new_fee_bps);
        set_fee_recipient(&env, &new_recipient);
        extend_instance_ttl(&env);
        Ok(())
    }
}
