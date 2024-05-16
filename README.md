# Rental Contract on Soroban

This repository contains a smart contract for managing rental agreements on the Soroban blockchain. The contract allows users to create, execute, pay rent, terminate, and refund deposits for rental agreements. 

## Features

- **Create Rental Agreement**: Allows authorized users to create a new rental agreement.
- **Execute Agreement**: Executes the rental agreement, transferring the first month's rent.
- **Pay Rent**: Allows tenants to pay their rent.
- **Terminate Agreement**: Allows landlords to terminate the agreement.
- **Refund Deposit**: Allows landlords to refund the deposit to the tenant.

## Contract Structure

The contract consists of the following main components:

### StorageKey

An enumeration to define storage keys.

```rust
#[derive(Clone)]
#[contracttype]
pub enum StorageKey {
    AgreementID,
}
```

### RentalAgreement

A struct to represent a rental agreement.

```rust
#[derive(Clone)]
#[contracttype]
pub struct RentalAgreement {
    property_address: String,
    landlord: Address,
    tenant: Address,
    rent_amount: i128,
    duration_months: u32,
    start_date: u64,
    executed: bool,
    deposit_paid: bool,
}
```

### RentalContract

The main contract that implements the rental agreement logic.

```rust
#[contract]
pub struct RentalContract;
```

## Contract Methods

### `create_agreement`

Creates a new rental agreement.

```rust
pub fn create_agreement(
    env: Env,
    from: Address,
    property_address: String,
    landlord: Address,
    tenant: Address,
    rent_amount: i128,
    duration_months: u32,
    start_date: u64,
);
```

### `execute_agreement`

Executes an existing rental agreement.

```rust
pub fn execute_agreement(
    env: Env,
    from: Address,
    token_address: Address,
    agreement_id: u64,
    current_date: u64
);
```

### `pay_rent`

Allows tenants to pay rent for an executed agreement.

```rust
pub fn pay_rent(
    env: Env,
    from: Address,
    token_address: Address,
    agreement_id: u64,
    amount: i128
);
```

### `terminate_agreement`

Terminates an executed agreement. Only the landlord can terminate the agreement.

```rust
pub fn terminate_agreement(
    env: Env,
    from: Address,
    agreement_id: u64
);
```

### `refund_deposit`

Refunds the deposit to the tenant. Only the landlord can refund the deposit.

```rust
pub fn refund_deposit(
    env: Env,
    from: Address,
    token_address: Address,
    agreement_id: u64,
    deposit_amount: i128
);
```

## Getting Started

### Prerequisites

- Rust
- Soroban SDK

### Building the Contract

To build the contract, run:

```sh
cargo build --target wasm32-unknown-unknown --release
```

### Testing the Contract

To test the contract, run:

```sh
cargo test
```

### Deploying the Contract

To deploy the contract to the Soroban blockchain, follow the Soroban SDK documentation.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

---

Feel free to reach out if you have any questions or need further assistance!