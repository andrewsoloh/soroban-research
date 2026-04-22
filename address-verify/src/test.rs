#![cfg(test)]
use crate::{AddressVerify, AddressVerifyClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(AddressVerify, ());
    let client = AddressVerifyClient::new(&env, &contract_id);

    env.mock_all_auths();
    let address = Address::generate(&env);

    assert_eq!(client.address_verify(&address), address);
    assert_eq!(client.get_address(&address), true);
}
