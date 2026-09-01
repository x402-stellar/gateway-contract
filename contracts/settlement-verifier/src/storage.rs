use crate::error::Error;
use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Initialized,
    Admin,
    FeeBps,
    FeeRecipient,
    Nonce(Address),
}

pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 518_400; // ~30 days
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = 431_000;
pub(crate) const PERSISTENT_BUMP_AMOUNT: u32 = 518_400;
pub(crate) const PERSISTENT_LIFETIME_THRESHOLD: u32 = 431_000;

pub(crate) fn extend_instance_ttl(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub(crate) fn is_initialized(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Initialized)
}

pub(crate) fn set_initialized(env: &Env) {
    env.storage().instance().set(&DataKey::Initialized, &true);
}

pub(crate) fn get_admin(env: &Env) -> Result<Address, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(Error::NotInitialized)
}

pub(crate) fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub(crate) fn get_fee_bps(env: &Env) -> u32 {
    env.storage().instance().get(&DataKey::FeeBps).unwrap_or(0)
}

pub(crate) fn set_fee_bps(env: &Env, fee_bps: u32) {
    env.storage().instance().set(&DataKey::FeeBps, &fee_bps);
}

pub(crate) fn get_fee_recipient(env: &Env) -> Result<Address, Error> {
    env.storage()
        .instance()
        .get(&DataKey::FeeRecipient)
        .ok_or(Error::NotInitialized)
}

pub(crate) fn set_fee_recipient(env: &Env, recipient: &Address) {
    env.storage()
        .instance()
        .set(&DataKey::FeeRecipient, recipient);
}

pub(crate) fn get_nonce(env: &Env, account: &Address) -> u64 {
    let key = DataKey::Nonce(account.clone());
    let nonce: u64 = env.storage().persistent().get(&key).unwrap_or(0);
    if nonce > 0 {
        env.storage().persistent().extend_ttl(
            &key,
            PERSISTENT_LIFETIME_THRESHOLD,
            PERSISTENT_BUMP_AMOUNT,
        );
    }
    nonce
}

pub(crate) fn set_nonce(env: &Env, account: &Address, nonce: u64) {
    let key = DataKey::Nonce(account.clone());
    env.storage().persistent().set(&key, &nonce);
    env.storage().persistent().extend_ttl(
        &key,
        PERSISTENT_LIFETIME_THRESHOLD,
        PERSISTENT_BUMP_AMOUNT,
    );
}
