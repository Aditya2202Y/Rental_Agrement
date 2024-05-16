#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::{Address as _};
use soroban_sdk::{token, Address, Env};
use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let contract_address = e.register_stellar_asset_contract(admin.clone());
    (
        TokenClient::new(e, &contract_address),
        TokenAdminClient::new(e, &contract_address),
    )
}

struct AutomaticCharityDonationsTest<'a> {
    env: Env,
    admin_address: Address,
    user_address: Address,
    charity_address: Address,
    token_address: Address,
    contract: AutomaticCharityDonationsClient<'a>,
}

impl<'a> AutomaticCharityDonationsTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let admin_address = Address::generate(&env);
        let user_address = Address::generate(&env);
        let charity_address = Address::generate(&env);
        let token_address = Address::generate(&env);

        let (_token, token_admin_client) = create_token_contract(&env, &admin_address);
        token_admin_client.mint(&user_address, &1000);

        let contract_id = env.register_contract(None, AutomaticCharityDonations);
        let contract = AutomaticCharityDonationsClient::new(&env, &contract_id);

        contract.set_admin(&admin_address);
        contract.add_charity_account(&1, &charity_address);

        AutomaticCharityDonationsTest {
            env,
            admin_address,
            user_address,
            charity_address,
            token_address,
            contract,
        }
    }
}

#[test]
fn test_set_admin() {
    let test = AutomaticCharityDonationsTest::setup();
    assert_eq!(test.contract.admin(), test.admin_address);
}

#[test]
fn test_add_charity_account() {
    let test = AutomaticCharityDonationsTest::setup();
    assert_eq!(test.contract.admin(), test.admin_address);
    assert_eq!(test.contract.get_charity_donation_amoutn(&1, &test.token_address), 0);

    let new_charity_address = Address::generate(&test.env);
    test.contract.add_charity_account(&2, &new_charity_address);

    assert_eq!(test.contract.get_charity_donation_amoutn(&2, &test.token_address), 0);
}

#[test]
fn test_remove_charity_account() {
    let test = AutomaticCharityDonationsTest::setup();
    assert_eq!(test.contract.admin(), test.admin_address);

    test.contract.remove_charity_account(&1);
    assert_eq!(test.contract.get_charity_donation_amoutn(&1, &test.token_address), 0);
}

#[test]
fn test_get_user_donation_amount() {
    let test = AutomaticCharityDonationsTest::setup();
    assert_eq!(test.contract.get_user_donation_amount(&test.user_address, &test.token_address), 0);
}

#[test]
fn test_get_charity_donation_amoutn() {
    let test = AutomaticCharityDonationsTest::setup();
    assert_eq!(test.contract.get_charity_donation_amoutn(&1, &test.token_address), 0);
}
