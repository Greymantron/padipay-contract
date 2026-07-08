use soroban_sdk::{contracttype, Address};

/// Represents the lifecycle states of an escrow.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EscrowStatus {
    /// The escrow has been initialized but funds are not yet locked.
    Created,
    /// Funds have been locked in the escrow contract.
    Locked,
    /// Funds have been successfully released to the seller.
    Released,
    /// Funds have been returned to the buyer.
    Refunded,
}

/// Represents the state of an escrow agreement.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EscrowState {
    pub buyer: Address,
    pub seller: Address,
    pub token: Address,
    pub amount: i128,
    pub status: EscrowStatus,
}
