use soroban_sdk::contracttype;

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
