use soroban_sdk::{contractevent, Address, Env};

#[contractevent(topics = ["init"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializedEvent {
    pub admin: Address,
    pub fee_bps: u32,
    pub fee_recipient: Address,
}

#[contractevent(topics = ["settle"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettlementReceipt {
    #[topic]
    pub merchant: Address,
    #[topic]
    pub payer: Address,
    pub token: Address,
    pub amount: i128,
    pub fee: i128,
    pub nonce: u64,
}

#[contractevent(topics = ["split"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitReceipt {
    #[topic]
    pub payer: Address,
    pub token: Address,
    pub total_amount: i128,
    pub recipient_count: u32,
    pub nonce: u64,
}

#[contractevent(topics = ["admin_change"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdminChangedEvent {
    pub old_admin: Address,
    pub new_admin: Address,
}

pub(crate) fn emit_initialized(env: &Env, admin: Address, fee_bps: u32, fee_recipient: Address) {
    InitializedEvent {
        admin,
        fee_bps,
        fee_recipient,
    }
    .publish(env);
}

pub(crate) fn emit_settlement(
    env: &Env,
    merchant: Address,
    payer: Address,
    token: Address,
    amount: i128,
    fee: i128,
    nonce: u64,
) {
    SettlementReceipt {
        merchant,
        payer,
        token,
        amount,
        fee,
        nonce,
    }
    .publish(env);
}

pub(crate) fn emit_split(
    env: &Env,
    payer: Address,
    token: Address,
    total_amount: i128,
    recipient_count: u32,
    nonce: u64,
) {
    SplitReceipt {
        payer,
        token,
        total_amount,
        recipient_count,
        nonce,
    }
    .publish(env);
}

pub(crate) fn emit_admin_changed(env: &Env, old_admin: Address, new_admin: Address) {
    AdminChangedEvent {
        old_admin,
        new_admin,
    }
    .publish(env);
}
