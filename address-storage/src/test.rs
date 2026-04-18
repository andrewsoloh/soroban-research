#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(AddressStorage, ());
    let client = AddressStorageClient::new(&env, &contract_id);
}
