#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct AddressVerify;

#[contractimpl]
impl AddressVerify {
    pub fn address_verify(env: Env, address: Address) -> Address {
        address.require_auth();
        env.storage().persistent().set(&address, &true);
        address
    }

    pub fn get_address(env: Env, address: Address) -> bool {
        env.storage().persistent().get(&address).unwrap_or(false)
    }
}

mod test;
