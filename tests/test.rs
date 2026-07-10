#![cfg(test)]

use soroban_escrow_contracts::{PadiPayEscrowContract, PadiPayEscrowContractClient};
use soroban_sdk::{
    testutils::{Address as _, Events},
    vec, Address, Env, IntoVal, Symbol,
};

pub struct TestSetup<'a> {
    pub contract_id: Address,
    pub client: PadiPayEscrowContractClient<'a>,
    pub buyer: Address,
    pub seller: Address,
    pub token: Address,
    pub token_admin: Address,
    pub token_client: soroban_sdk::token::StellarAssetClient<'a>,
    pub token_client_basic: soroban_sdk::token::Client<'a>,
}

pub fn setup_test<'a>(env: &'a Env) -> TestSetup<'a> {
    let contract_id = env.register(PadiPayEscrowContract, ());
    let client = PadiPayEscrowContractClient::new(env, &contract_id);

    let buyer = Address::generate(env);
    let seller = Address::generate(env);

    let token_admin = Address::generate(env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token = token_contract.address();
    let token_client = soroban_sdk::token::StellarAssetClient::new(env, &token);
    let token_client_basic = soroban_sdk::token::Client::new(env, &token);

    TestSetup {
        contract_id,
        client,
        buyer,
        seller,
        token,
        token_admin,
        token_client,
        token_client_basic,
    }
}

#[test]
fn test_create_escrow() {
    let env = Env::default();
    env.mock_all_auths();
    let setup = setup_test(&env);
    let amount = 1000;

    setup
        .client
        .create_escrow(&setup.buyer, &setup.seller, &setup.token, &amount);

    let events = env.events().all();
    assert_eq!(
        events,
        vec![
            &env,
            (
                setup.contract_id.clone(),
                (
                    Symbol::new(&env, "EscrowCreated"),
                    setup.buyer.clone(),
                    setup.seller.clone()
                )
                    .into_val(&env),
                amount.into_val(&env)
            )
        ]
    );

    env.as_contract(&setup.contract_id, || {
        let state = soroban_escrow_contracts::storage::read_escrow_state(&env).unwrap();
        assert_eq!(state.buyer, setup.buyer);
        assert_eq!(state.seller, setup.seller);
        assert_eq!(state.token, setup.token);
        assert_eq!(state.amount, amount);
        assert_eq!(
            state.status,
            soroban_escrow_contracts::types::EscrowStatus::Created
        );
    });
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn test_create_escrow_unauthorized() {
    let env = Env::default();
    let setup = setup_test(&env);
    let amount = 1000;

    // This should panic because buyer didn't authorize
    setup
        .client
        .create_escrow(&setup.buyer, &setup.seller, &setup.token, &amount);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_create_escrow_invalid_amount() {
    let env = Env::default();
    env.mock_all_auths();
    let setup = setup_test(&env);
    let amount = 0; // Invalid amount

    setup
        .client
        .create_escrow(&setup.buyer, &setup.seller, &setup.token, &amount);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #6)")]
fn test_create_escrow_invalid_addresses() {
    let env = Env::default();
    env.mock_all_auths();
    let setup = setup_test(&env);
    let amount = 1000;

    // Buyer == seller
    setup
        .client
        .create_escrow(&setup.buyer, &setup.buyer, &setup.token, &amount);
}

#[test]
fn test_lock_funds() {
    let env = Env::default();
    env.mock_all_auths();
    let setup = setup_test(&env);
    let amount = 1000;

    // Mint tokens to buyer
    setup.token_client.mint(&setup.buyer, &10000);
    assert_eq!(setup.token_client_basic.balance(&setup.buyer), 10000);

    // Create escrow
    setup
        .client
        .create_escrow(&setup.buyer, &setup.seller, &setup.token, &amount);

    // Lock funds
    setup.client.lock_funds();

    let events = env.events().all().filter_by_contract(&setup.contract_id);
    assert_eq!(
        events,
        vec![
            &env,
            (
                setup.contract_id.clone(),
                (
                    Symbol::new(&env, "FundsLocked"),
                    setup.buyer.clone(),
                    setup.seller.clone()
                )
                    .into_val(&env),
                amount.into_val(&env)
            )
        ]
    );

    // Check balances
    assert_eq!(setup.token_client_basic.balance(&setup.buyer), 9000);
    assert_eq!(setup.token_client_basic.balance(&setup.contract_id), 1000);

    env.as_contract(&setup.contract_id, || {
        let state = soroban_escrow_contracts::storage::read_escrow_state(&env).unwrap();
        assert_eq!(
            state.status,
            soroban_escrow_contracts::types::EscrowStatus::Locked
        );
    });
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #5)")]
fn test_lock_funds_already_funded() {
    let env = Env::default();
    env.mock_all_auths();
    let setup = setup_test(&env);
    let amount = 1000;

    setup.token_client.mint(&setup.buyer, &10000);

    setup
        .client
        .create_escrow(&setup.buyer, &setup.seller, &setup.token, &amount);
    setup.client.lock_funds();

    // This should panic with AlreadyFunded
    setup.client.lock_funds();
}

#[test]
fn test_release_funds() {
    let env = Env::default();
    env.mock_all_auths();
    let setup = setup_test(&env);
    let amount = 1000;

    // Mint tokens to buyer
    setup.token_client.mint(&setup.buyer, &10000);

    // Create escrow
    setup
        .client
        .create_escrow(&setup.buyer, &setup.seller, &setup.token, &amount);

    // Lock funds
    setup.client.lock_funds();

    // Release funds
    setup.client.release_funds();

    let events = env.events().all().filter_by_contract(&setup.contract_id);
    assert_eq!(
        events,
        vec![
            &env,
            (
                setup.contract_id.clone(),
                (
                    Symbol::new(&env, "FundsReleased"),
                    setup.buyer.clone(),
                    setup.seller.clone()
                )
                    .into_val(&env),
                amount.into_val(&env)
            )
        ]
    );

    // Check balances
    assert_eq!(setup.token_client_basic.balance(&setup.contract_id), 0);
    assert_eq!(setup.token_client_basic.balance(&setup.seller), 1000);

    env.as_contract(&setup.contract_id, || {
        let state = soroban_escrow_contracts::storage::read_escrow_state(&env).unwrap();
        assert_eq!(
            state.status,
            soroban_escrow_contracts::types::EscrowStatus::Released
        );
    });
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_release_funds_already_released() {
    let env = Env::default();
    env.mock_all_auths();
    let setup = setup_test(&env);
    let amount = 1000;

    setup.token_client.mint(&setup.buyer, &10000);

    setup
        .client
        .create_escrow(&setup.buyer, &setup.seller, &setup.token, &amount);
    setup.client.lock_funds();
    setup.client.release_funds();

    // Releasing again should panic with InvalidState (Error 2)
    setup.client.release_funds();
}

#[test]
fn test_refund() {
    let env = Env::default();
    env.mock_all_auths();
    let setup = setup_test(&env);
    let amount = 1000;

    // Mint tokens to buyer
    setup.token_client.mint(&setup.buyer, &10000);

    // Create and lock
    setup
        .client
        .create_escrow(&setup.buyer, &setup.seller, &setup.token, &amount);
    setup.client.lock_funds();

    // Check balance before refund
    assert_eq!(setup.token_client_basic.balance(&setup.buyer), 9000);

    // Refund
    setup.client.refund();

    let events = env.events().all().filter_by_contract(&setup.contract_id);
    assert_eq!(
        events,
        vec![
            &env,
            (
                setup.contract_id.clone(),
                (
                    Symbol::new(&env, "EscrowRefunded"),
                    setup.buyer.clone(),
                    setup.seller.clone()
                )
                    .into_val(&env),
                amount.into_val(&env)
            )
        ]
    );

    // Check balances after refund
    assert_eq!(setup.token_client_basic.balance(&setup.contract_id), 0);
    assert_eq!(setup.token_client_basic.balance(&setup.buyer), 10000);

    env.as_contract(&setup.contract_id, || {
        let state = soroban_escrow_contracts::storage::read_escrow_state(&env).unwrap();
        assert_eq!(
            state.status,
            soroban_escrow_contracts::types::EscrowStatus::Refunded
        );
    });
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_refund_already_released() {
    let env = Env::default();
    env.mock_all_auths();
    let setup = setup_test(&env);
    let amount = 1000;

    setup.token_client.mint(&setup.buyer, &10000);

    setup
        .client
        .create_escrow(&setup.buyer, &setup.seller, &setup.token, &amount);
    setup.client.lock_funds();
    setup.client.release_funds();

    // Try to refund after released
    setup.client.refund();
}

#[test]
fn test_resolve_dispute() {
    let _env = Env::default();
    // TODO: Set up environment, register contract, and mock tokens.
    // TODO: Lock funds first.
    // TODO: Call client.resolve_dispute(&mediator, &Symbol::new(&env, "refund_buyer")).
    // TODO: Assert that the funds were routed correctly based on the outcome.
}

#[test]
fn test_escrow_lifecycle_happy_path_release() {
    let env = Env::default();
    env.mock_all_auths();
    let setup = setup_test(&env);
    let amount = 5000;

    // 1. Initial State
    setup.token_client.mint(&setup.buyer, &10000);
    assert_eq!(setup.token_client_basic.balance(&setup.buyer), 10000);

    // 2. Create Escrow
    setup
        .client
        .create_escrow(&setup.buyer, &setup.seller, &setup.token, &amount);

    let events = env.events().all().filter_by_contract(&setup.contract_id);
    assert_eq!(
        events,
        vec![
            &env,
            (
                setup.contract_id.clone(),
                (
                    Symbol::new(&env, "EscrowCreated"),
                    setup.buyer.clone(),
                    setup.seller.clone()
                )
                    .into_val(&env),
                amount.into_val(&env)
            )
        ]
    );

    env.as_contract(&setup.contract_id, || {
        let state = soroban_escrow_contracts::storage::read_escrow_state(&env).unwrap();
        assert_eq!(state.buyer, setup.buyer);
        assert_eq!(state.seller, setup.seller);
        assert_eq!(state.token, setup.token);
        assert_eq!(state.amount, amount);
        assert_eq!(
            state.status,
            soroban_escrow_contracts::types::EscrowStatus::Created
        );
    });

    // 3. Lock Funds
    setup.client.lock_funds();

    let events = env.events().all().filter_by_contract(&setup.contract_id);
    assert_eq!(
        events,
        vec![
            &env,
            (
                setup.contract_id.clone(),
                (
                    Symbol::new(&env, "FundsLocked"),
                    setup.buyer.clone(),
                    setup.seller.clone()
                )
                    .into_val(&env),
                amount.into_val(&env)
            )
        ]
    );

    assert_eq!(setup.token_client_basic.balance(&setup.buyer), 5000);
    assert_eq!(setup.token_client_basic.balance(&setup.contract_id), 5000);

    env.as_contract(&setup.contract_id, || {
        let state = soroban_escrow_contracts::storage::read_escrow_state(&env).unwrap();
        assert_eq!(
            state.status,
            soroban_escrow_contracts::types::EscrowStatus::Locked
        );
    });

    // 4. Release Funds
    setup.client.release_funds();

    let events = env.events().all().filter_by_contract(&setup.contract_id);
    assert_eq!(
        events,
        vec![
            &env,
            (
                setup.contract_id.clone(),
                (
                    Symbol::new(&env, "FundsReleased"),
                    setup.buyer.clone(),
                    setup.seller.clone()
                )
                    .into_val(&env),
                amount.into_val(&env)
            )
        ]
    );

    assert_eq!(setup.token_client_basic.balance(&setup.contract_id), 0);
    assert_eq!(setup.token_client_basic.balance(&setup.seller), 5000);

    env.as_contract(&setup.contract_id, || {
        let state = soroban_escrow_contracts::storage::read_escrow_state(&env).unwrap();
        assert_eq!(
            state.status,
            soroban_escrow_contracts::types::EscrowStatus::Released
        );
    });
}

#[test]
fn test_escrow_lifecycle_happy_path_refund() {
    let env = Env::default();
    env.mock_all_auths();
    let setup = setup_test(&env);
    let amount = 5000;

    // 1. Initial State
    setup.token_client.mint(&setup.buyer, &10000);

    // 2. Create Escrow
    setup
        .client
        .create_escrow(&setup.buyer, &setup.seller, &setup.token, &amount);

    env.as_contract(&setup.contract_id, || {
        let state = soroban_escrow_contracts::storage::read_escrow_state(&env).unwrap();
        assert_eq!(
            state.status,
            soroban_escrow_contracts::types::EscrowStatus::Created
        );
    });

    // 3. Lock Funds
    setup.client.lock_funds();

    env.as_contract(&setup.contract_id, || {
        let state = soroban_escrow_contracts::storage::read_escrow_state(&env).unwrap();
        assert_eq!(
            state.status,
            soroban_escrow_contracts::types::EscrowStatus::Locked
        );
    });

    // 4. Refund Funds
    setup.client.refund();

    let events = env.events().all().filter_by_contract(&setup.contract_id);
    assert_eq!(
        events,
        vec![
            &env,
            (
                setup.contract_id.clone(),
                (
                    Symbol::new(&env, "EscrowRefunded"),
                    setup.buyer.clone(),
                    setup.seller.clone()
                )
                    .into_val(&env),
                amount.into_val(&env)
            )
        ]
    );

    assert_eq!(setup.token_client_basic.balance(&setup.contract_id), 0);
    assert_eq!(setup.token_client_basic.balance(&setup.buyer), 10000);

    env.as_contract(&setup.contract_id, || {
        let state = soroban_escrow_contracts::storage::read_escrow_state(&env).unwrap();
        assert_eq!(
            state.status,
            soroban_escrow_contracts::types::EscrowStatus::Refunded
        );
    });
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn test_lock_funds_unauthorized() {
    let env = Env::default();
    let setup = setup_test(&env);

    env.as_contract(&setup.contract_id, || {
        let state = soroban_escrow_contracts::types::EscrowState {
            buyer: setup.buyer.clone(),
            seller: setup.seller.clone(),
            token: setup.token.clone(),
            amount: 1000,
            status: soroban_escrow_contracts::types::EscrowStatus::Created,
        };
        soroban_escrow_contracts::storage::write_escrow_state(&env, &state);
    });

    setup.client.lock_funds();
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn test_release_funds_unauthorized() {
    let env = Env::default();
    let setup = setup_test(&env);

    env.as_contract(&setup.contract_id, || {
        let state = soroban_escrow_contracts::types::EscrowState {
            buyer: setup.buyer.clone(),
            seller: setup.seller.clone(),
            token: setup.token.clone(),
            amount: 1000,
            status: soroban_escrow_contracts::types::EscrowStatus::Locked,
        };
        soroban_escrow_contracts::storage::write_escrow_state(&env, &state);
    });

    setup.client.release_funds();
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn test_refund_unauthorized() {
    let env = Env::default();
    let setup = setup_test(&env);

    env.as_contract(&setup.contract_id, || {
        let state = soroban_escrow_contracts::types::EscrowState {
            buyer: setup.buyer.clone(),
            seller: setup.seller.clone(),
            token: setup.token.clone(),
            amount: 1000,
            status: soroban_escrow_contracts::types::EscrowStatus::Locked,
        };
        soroban_escrow_contracts::storage::write_escrow_state(&env, &state);
    });

    setup.client.refund();
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_release_funds_invalid_state() {
    let env = Env::default();
    env.mock_all_auths();
    let setup = setup_test(&env);
    let amount = 1000;

    setup
        .client
        .create_escrow(&setup.buyer, &setup.seller, &setup.token, &amount);

    // Try to release while still 'Created' (invalid state)
    setup.client.release_funds();
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_refund_invalid_state() {
    let env = Env::default();
    env.mock_all_auths();
    let setup = setup_test(&env);
    let amount = 1000;

    setup
        .client
        .create_escrow(&setup.buyer, &setup.seller, &setup.token, &amount);

    // Try to refund while still 'Created' (invalid state)
    setup.client.refund();
}
