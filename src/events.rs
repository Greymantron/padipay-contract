#![allow(deprecated)]
use crate::types::{EscrowId, EscrowState};
use soroban_sdk::{Env, Symbol};

pub fn publish_escrow_created(env: &Env, escrow_id: EscrowId, state: &EscrowState) {
    let topics = (
        Symbol::new(env, "EscrowCreated"),
        escrow_id,
        state.buyer.clone(),
        state.seller.clone(),
    );
    env.events().publish(topics, state.amount);
}

pub fn publish_funds_locked(env: &Env, escrow_id: EscrowId, state: &EscrowState) {
    let topics = (
        Symbol::new(env, "FundsLocked"),
        escrow_id,
        state.buyer.clone(),
        state.seller.clone(),
    );
    env.events().publish(topics, state.amount);
}

pub fn publish_funds_released(env: &Env, escrow_id: EscrowId, state: &EscrowState) {
    let topics = (
        Symbol::new(env, "FundsReleased"),
        escrow_id,
        state.buyer.clone(),
        state.seller.clone(),
    );
    env.events().publish(topics, state.amount);
}

pub fn publish_escrow_refunded(env: &Env, escrow_id: EscrowId, state: &EscrowState) {
    let topics = (
        Symbol::new(env, "EscrowRefunded"),
        escrow_id,
        state.buyer.clone(),
        state.seller.clone(),
    );
    env.events().publish(topics, state.amount);
}
