#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, token, String};

#[derive(Clone)]
#[contracttype]
pub enum StorageKey {
    AgreementID,
}

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

#[contract]
pub struct RentalContract;

#[contractimpl]
impl RentalContract {
    pub fn create_agreement(
        env: Env,
        from: Address,
        property_address: String,
        landlord: Address,
        tenant: Address,
        rent_amount: i128,
        duration_months: u32,
        start_date: u64,
    ) {
        // Ensure that the transaction sender is authorized to create agreements
        from.require_auth();

        // Ensure landlord and tenant are not the same
        if landlord == tenant {
            panic!("Landlord and tenant cannot be the same");
        }

        // Validate rent amount and duration
        if rent_amount <= 0 {
            panic!("Rent amount must be greater than zero");
        }
        if duration_months == 0 {
            panic!("Duration must be greater than zero");
        }

        // Generate a new unique agreement ID
        let mut agreement_id = env.storage().instance().get(&StorageKey::AgreementID).unwrap_or(0);

        agreement_id += 1;

        // Create the rental agreement
        let agreement = RentalAgreement {
            property_address,
            landlord,
            tenant,
            rent_amount,
            duration_months,
            start_date,
            executed: false,
            deposit_paid: false,
        };

        // Store the agreement in the contract storage
        env.storage().instance().set(&StorageKey::AgreementID, &agreement_id);
        env.storage().instance().set(&agreement_id, &agreement);
    }

    pub fn execute_agreement(env: Env, from: Address, token_address: Address, agreement_id: u64, current_date: u64) {
        // Retrieve the agreement from the contract storage
        let agreement: RentalAgreement = env.storage().instance().get(&agreement_id).unwrap();

        // Ensure that the agreement exists
        if agreement.start_date == 0 {
            panic!("Agreement does not exist");
        }

        // Ensure that the transaction sender is authorized to execute the agreement
        from.require_auth();

        // Ensure that the agreement has not been executed already
        if agreement.executed {
            panic!("Agreement has already been executed");
        }

        // Ensure current date is start date or later
        if current_date < agreement.start_date {
            panic!("Cannot execute agreement before start date");
        }

        // Transfer the first month's rent payment
        token::Client::new(&env, &token_address).transfer(&agreement.tenant, &agreement.landlord, &agreement.rent_amount);

        // Mark the agreement as executed
        let updated_agreement = RentalAgreement {
            executed: true,
            ..agreement
        };
        env.storage().instance().set(&agreement_id, &updated_agreement);
    }

    pub fn pay_rent(env: Env, from: Address, token_address: Address, agreement_id: u64, amount: i128) {
        // Retrieve the agreement from the contract storage
        let agreement: RentalAgreement = env.storage().instance().get(&agreement_id).unwrap();

        // Ensure that the transaction sender is authorized to pay rent
        from.require_auth();

        // Ensure the agreement has been executed
        if !agreement.executed {
            panic!("Agreement has not been executed");
        }

        // Ensure the amount matches the rent amount
        if amount != agreement.rent_amount {
            panic!("Incorrect rent amount");
        }

        // Transfer the rent payment
        token::Client::new(&env, &token_address).transfer(&agreement.tenant, &agreement.landlord, &amount);
    }

    pub fn terminate_agreement(env: Env, from: Address, agreement_id: u64) {
        // Retrieve the agreement from the contract storage
        let agreement: RentalAgreement = env.storage().instance().get(&agreement_id).unwrap();

        // Ensure that the transaction sender is authorized to terminate the agreement
        from.require_auth();

        // Ensure the agreement has been executed
        if !agreement.executed {
            panic!("Agreement has not been executed");
        }

        // Only landlord can terminate the agreement
        if from != agreement.landlord {
            panic!("Only the landlord can terminate the agreement");
        }

        // Remove the agreement from storage
        env.storage().instance().remove(&agreement_id);
    }

    pub fn refund_deposit(env: Env, from: Address, token_address: Address, agreement_id: u64, deposit_amount: i128) {
        // Retrieve the agreement from the contract storage
        let agreement: RentalAgreement = env.storage().instance().get(&agreement_id).unwrap();

        // Ensure that the transaction sender is authorized to refund the deposit
        from.require_auth();

        // Ensure the agreement has been executed and deposit paid
        if !agreement.executed || !agreement.deposit_paid {
            panic!("Agreement has not been executed or deposit not paid");
        }

        // Only landlord can refund the deposit
        if from != agreement.landlord {
            panic!("Only the landlord can refund the deposit");
        }

        // Transfer the deposit back to the tenant
        token::Client::new(&env, &token_address).transfer(&agreement.landlord, &agreement.tenant, &deposit_amount);

        // Mark the deposit as refunded
        let updated_agreement = RentalAgreement {
            deposit_paid: false,
            ..agreement
        };
        env.storage().instance().set(&agreement_id, &updated_agreement);
    }
}

