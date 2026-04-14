#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct AddressStorage;

#[contractimpl]
impl AddressStorage {
    pub fn address_verify(env: Env, address: Address) -> Address {
        address.require_auth();
        env.storage().persistent().set(&address, &true);
        address
    }

    pub fn get_address(env: Env, address: Address) -> bool {
        let verified = env.storage().persistent().get(&address).unwrap_or(false);
        verified
    }
}

mod test;
