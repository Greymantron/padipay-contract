# Smart Contract Architecture

This document describes the architecture of the **PadiPay Soroban Escrow Contracts**, including the contract components, escrow lifecycle, storage model, authentication model, and planned evolution of the protocol.

> **Note**
>
> This document reflects the current **v0.1.0 MVP** implementation. Features planned for future milestones are documented separately to distinguish implemented functionality from the long-term vision.

---

# Overview

The PadiPay escrow contract enables buyers and sellers to securely exchange digital assets using a trust-minimized escrow model on the Stellar network.

The contract is intentionally designed to be:

* Modular
* Extensible
* Testable
* Contributor-friendly

Rather than implementing every feature at once, the protocol evolves incrementally through milestone-based development.

---

# Design Principles

The contract follows several guiding principles:

* Keep the MVP intentionally small.
* Build reusable components.
* Prefer explicit state transitions.
* Never mutate escrow state without validation.
* Require authorization before performing sensitive operations.
* Emit events for important lifecycle actions.
* Keep business logic separated from storage helpers.
* Maintain comprehensive unit test coverage.

---

# Contract Components

The contract is organized into several logical responsibilities.

## Escrow State

Responsible for representing an escrow agreement.

Examples include:

* EscrowState
* EscrowStatus
* DataKey

---

## Storage Layer

Responsible for reading and writing escrow data.

Responsibilities include:

* Persisting escrow records
* Updating escrow state
* Retrieving existing escrows

---

## Authentication Layer

Responsible for validating transaction authorization.

Current roles:

* Buyer
* Seller

Future roles:

* Mediator
* Oracle

---

## Token Layer

Responsible for interacting with Soroban token contracts.

Responsibilities include:

* Locking funds
* Releasing funds
* Refunding buyers

---

## Event Layer

Responsible for publishing contract events for off-chain consumers.

---

# Escrow Lifecycle (v0.1.0)

The MVP supports a simple escrow lifecycle.

```text
Buyer

↓

Create Escrow

↓

Lock Funds

↓

Release Funds

OR

Refund Buyer
```

---

# State Machine

## Current (v0.1.0)

```text
Created
    │
    ▼
Locked
 ┌──┴─────┐
 ▼        ▼
Released Refunded
```

State transitions are strictly validated.

Allowed transitions:

* Created → Locked
* Locked → Released
* Locked → Refunded

All other transitions should be rejected.

---

## Future Extension (v0.3.0)

The Human-in-the-Loop Oracle introduces dispute resolution.

```text
Created
    │
    ▼
Locked
   │
   ▼
Disputed
 ┌──┴─────┐
 ▼        ▼
Released Refunded
```

This functionality is intentionally outside the scope of the MVP.

---

# Storage Layout

Escrow data is stored using Soroban contract storage.

Core data structures include:

* DataKey
* EscrowState
* EscrowStatus

Each escrow record stores:

* Buyer address
* Seller address
* Token contract address
* Escrow amount
* Current status

Future versions may introduce:

* Escrow expiration
* Metadata
* Oracle assignments
* Evidence references

---

# Authentication Model

The contract currently supports two participant roles.

## Buyer

Responsible for:

* Creating escrows
* Locking funds

## Seller

Responsible for:

* Receiving released funds

Future versions will introduce additional roles.

### Mediator

Responsible for:

* Resolving disputes
* Approving refunds after disputes

### Oracle

Responsible for:

* Providing trusted dispute outcomes

---

# Token Flow

Funds are managed through the Soroban Token Interface.

Current flow:

```text
Buyer Wallet
      │
      ▼
Escrow Contract
      │
 ┌────┴─────┐
 ▼          ▼
Seller    Buyer
Release   Refund
```

The contract never mints assets.

It only transfers existing tokens between participants.

---

# Event Model

The contract emits events to allow off-chain applications to observe escrow activity.

Planned MVP events:

* EscrowCreated
* FundsLocked
* FundsReleased
* EscrowRefunded

Future releases may introduce:

* DisputeCreated
* MediatorAssigned
* DisputeResolved

---

# Error Model

The contract returns explicit errors instead of panicking.

Examples include:

* Unauthorized
* EscrowNotFound
* InvalidState
* InvalidAmount
* EscrowAlreadyFunded

Future releases may expand this list as new features are introduced.

---

# Future Architecture

## v0.2.0 — Contract Hardening

Planned improvements:

* Escrow expiration
* Additional validation
* Better error handling
* Storage optimization
* Security improvements

---

## v0.3.0 — Human Oracle

Planned additions:

* Oracle registry
* Mediator permissions
* Dispute workflow
* Evidence handling
* Dispute events

---

## v0.4.0 — Production Readiness

Planned additions:

* Milestone payments
* Partial releases
* Protocol fees
* Multi-party escrow
* Performance optimization
* Fuzz testing
* Security audit